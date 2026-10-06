import fs from 'node:fs';
import {homedir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {execFileSync} from 'node:child_process';

const mode = process.argv[2];
if (mode !== 'current' && mode !== 'prior' && mode !== 'reddit') {
  throw new Error('usage: node b-shots.mjs current|prior|reddit');
}
const session = '1004-b';
const now = () => new Intl.DateTimeFormat('sv-SE', {
  timeZone: 'Asia/Shanghai',
  year: 'numeric',
  month: '2-digit',
  day: '2-digit',
  hour: '2-digit',
  minute: '2-digit',
  second: '2-digit',
  hourCycle: 'h23'
}).format(new Date()).replace(' ', 'T') + '+08:00';
const urls = {
  current: 'https://support.google.com/gemini/answer/17004136?hl=en',
  prior: 'https://web.archive.org/web/20260930071648/https://support.google.com/gemini/answer/16275805?hl=en',
  reddit: 'https://old.reddit.com/r/GoogleGeminiAI/comments/1wwimbm/gemini_flash_and_pro_will_only_be_available_by/'
};
const {sendCommand} = await import(pathToFileURL(join(
  homedir(),
  'scoop/persist/bun/install/cache/@jackwener/opencli@1.8.7@@@1/dist/src/browser/daemon-client.js'
)).href);
const cli = (...args) => execFileSync('opencli.exe', ['browser', session, ...args], {
  encoding: 'utf8',
  stdio: ['ignore', 'pipe', 'pipe']
});
const cdp = (cdpMethod, cdpParams) => sendCommand('cdp', {
  session,
  surface: 'browser',
  cdpMethod,
  cdpParams
});
const parseEval = raw => {
  const match = raw.match(/\{[\s\S]*\}/);
  if (!match) throw new Error('OpenCLI eval returned no JSON: ' + raw.slice(0, 300));
  let value = JSON.parse(match[0]);
  if (typeof value === 'string') value = JSON.parse(value);
  return value;
};

cli('open', urls[mode]);
await new Promise(resolve => setTimeout(resolve, 900));
const viewportWidth = mode === 'reddit' ? 1024 : 700;
await cdp('Emulation.setDeviceMetricsOverride', {
  width: viewportWidth,
  height: 2400,
  deviceScaleFactor: 2,
  mobile: false
});
await new Promise(resolve => setTimeout(resolve, 700));

const expression = 'JSON.stringify({heads:[...document.querySelectorAll("h1,h2,h3")].filter(e=>e.innerText.trim()).map(e=>({tag:e.tagName,text:e.innerText.trim(),top:e.getBoundingClientRect().top+scrollY,bottom:e.getBoundingClientRect().bottom+scrollY})),tables:[...document.querySelectorAll("table")].map((e,i)=>({i,top:e.getBoundingClientRect().top+scrollY,bottom:e.getBoundingClientRect().bottom+scrollY,x:e.getBoundingClientRect().x,width:e.getBoundingClientRect().width})),posts:[...document.querySelectorAll("shreddit-post")].map(e=>({top:e.getBoundingClientRect().top+scrollY,bottom:e.getBoundingClientRect().bottom+scrollY,text:e.innerText.slice(0,2600)})),things:[...document.querySelectorAll(".thing.link")].map(e=>({top:e.getBoundingClientRect().top+scrollY,bottom:e.getBoundingClientRect().bottom+scrollY,text:e.innerText.slice(0,2600)}))})';
const geometry = parseEval(cli('eval', expression));
const table = geometry.tables;
let shots;
if (mode === 'current') {
  const heading = geometry.heads.find(x => x.tag === 'H1' && x.text === 'Changes to Gemini model access and limits');
  const previous = geometry.heads.find(x => x.tag === 'H2' && x.text === 'Previous changes');
  if (!heading || table.length < 2 || !previous) throw new Error('Current page layout did not match expected article.');
  shots = [
    {file: '10-gemini-access-table.png', top: Math.max(0, heading.top - 24), bottom: table[0].bottom + 24},
    {file: '11-gemini-rollout-scope.png', top: Math.max(0, heading.top - 24), bottom: table[0].top + 32},
    {file: '12-gemini-usage-limits.png', top: Math.max(0, previous.top - 24), bottom: table[1].bottom + 24}
  ];
} else {
  if (mode === 'reddit') {
    const post = geometry.things.find(x => x.text.includes('Gemini Flash and Pro will only be available by paid subscription from October 9th onwards'));
    if (!post) throw new Error('Target Reddit post was not found in the rendered page.');
    shots = [{
      file: '14-reddit-access-claim.png',
      top: Math.max(0, post.top),
      bottom: post.bottom + 16
    }];
  } else {
  const heading = geometry.heads.find(x => /Model Access/i.test(x.text));
  const accessTable = table.find(x => heading && x.top > heading.bottom);
  if (!heading || !accessTable) throw new Error('Archived model access table was not found.');
  shots = [{
    file: '13-gemini-old-model-access.png',
    top: Math.max(0, heading.top - 30),
    bottom: accessTable.bottom + 30
  }];
  }
}

for (const shot of shots) {
  const height = Math.ceil(shot.bottom - shot.top);
  const widthCss = 700;
  const result = await cdp('Page.captureScreenshot', {
    format: 'png',
    captureBeyondViewport: true,
    clip: {x: 0, y: shot.top, width: widthCss, height, scale: 1}
  });
  fs.writeFileSync('docs/2610/1004/images/' + shot.file, Buffer.from(result.data, 'base64'));
  fs.appendFileSync('docs/2610/1004/sources/b-capture-records.jsonl', JSON.stringify({
    time: now(),
    url: urls[mode],
    tool: 'OpenCLI browser 1004-b + CDP Page.captureScreenshot',
    result: `${widthCss} CSS px at DPR 2; cropped to the relevant public source region`,
    file: shot.file
  }) + '\n', 'utf8');
  console.log(JSON.stringify({file: shot.file, xCss: 0, yCss: shot.top, widthCss, widthPixels: widthCss * 2, heightPixels: height * 2, dpr: 2}));
}
