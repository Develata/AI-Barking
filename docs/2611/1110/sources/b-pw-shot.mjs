// B 组：用 Playwright（系统 Chrome）按文本锚点截取文档页段落，DPR 2，宽 1000 CSS px
const { chromium } = await import(`${process.env.HOME}/.bun/install/global/node_modules/playwright/index.mjs`);
const [url, text, out, before = '0', after = '0'] = process.argv.slice(2);
const b = await chromium.launch({ channel: 'chrome', executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' });
const ctx = await b.newContext({ viewport: { width: Number(process.env.W || 1000), height: 1400 }, deviceScaleFactor: 2 });
const p = await ctx.newPage();
await p.goto(url, { waitUntil: 'networkidle', timeout: 60000 });
const loc = p.getByText(text, { exact: false }).first();
await loc.scrollIntoViewIfNeeded();
const box0 = await loc.boundingBox();
const sy = await p.evaluate(() => window.scrollY);
const box = { ...box0, y: box0.y + sy };
console.log(JSON.stringify(box));
await p.screenshot({ path: out, clip: { x: 0, y: Math.max(0, box.y - Number(before)), width: Number(process.env.W || 1000), height: box.height + Number(before) + Number(after) }, fullPage: true });
await b.close();
