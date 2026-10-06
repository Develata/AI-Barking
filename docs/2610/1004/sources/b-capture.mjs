import fs from 'node:fs';
import path from 'node:path';
import {execFileSync} from 'node:child_process';

const [name, url] = process.argv.slice(2);
if (!name || !/^b-[a-z0-9-]+$/.test(name) || !url) {
  throw new Error('usage: node b-capture.mjs b-name https://public-url');
}

const root = path.resolve('docs/2610/1004/sources');
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

function browser(args) {
  const raw = execFileSync('opencli.exe', ['browser', session, ...args], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe']
  });
  const match = raw.match(/\{[\s\S]*\}/);
  if (!match) throw new Error('OpenCLI returned no JSON payload: ' + raw.slice(0, 400));
  let result = JSON.parse(match[0]);
  if (typeof result === 'string') result = JSON.parse(result);
  return result;
}

browser(['open', url]);
const expression = 'JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,tables:[...document.querySelectorAll("table")].map(t=>[...t.rows].map(r=>[...r.cells].map(c=>({text:c.innerText,checkmark:[...c.querySelectorAll("img")].some(i=>i.alt==="Checkmark"),imageCount:c.querySelectorAll("img").length}))))})';
const data = browser(['eval', expression]);
if (!data || typeof data.text !== 'string' || data.text.length < 80) {
  throw new Error('Page body text missing or too short; not archived');
}

const lines = [
  'Capture time (Beijing): ' + now(),
  'URL: ' + data.url,
  'Title: ' + data.title,
  'Method: OpenCLI browser 1004-b; visible page text and public table DOM.',
  '',
  '## Full visible page text',
  data.text
];
if (data.tables?.length) {
  lines.push('', '## Table cell transcription from public DOM');
  for (let i = 0; i < data.tables.length; i++) {
    lines.push('', 'Table ' + (i + 1));
    for (const row of data.tables[i]) {
      lines.push(row.map(cell => {
        if (cell.checkmark) return '✓';
        if (cell.imageCount) return '✗';
        return String(cell.text || '').replace(/\s+/g, ' ').trim();
      }).join(' | '));
    }
  }
}

fs.writeFileSync(path.join(root, name + '.txt'), lines.join('\n') + '\n', 'utf8');
fs.appendFileSync(path.join(root, 'b-capture-records.jsonl'), JSON.stringify({
  time: now(),
  url: data.url,
  title: data.title,
  tool: 'OpenCLI browser 1004-b open/eval',
  result: 'visible body text saved; table cells transcribed from public DOM',
  file: name + '.txt'
}) + '\n', 'utf8');
console.log(JSON.stringify({url: data.url, title: data.title, textLength: data.text.length, file: name + '.txt'}));
