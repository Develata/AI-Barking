// B 组：Playwright 截页面顶部（视口），DPR 2
const { chromium } = await import(`${process.env.HOME}/.bun/install/global/node_modules/playwright/index.mjs`);
const [url, out, w = '1000', h = '900'] = process.argv.slice(2);
const b = await chromium.launch({ channel: 'chrome', executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' });
const ctx = await b.newContext({ viewport: { width: Number(w), height: Number(h) }, deviceScaleFactor: 2 });
const p = await ctx.newPage();
await p.goto(url, { waitUntil: 'networkidle', timeout: 60000 });
await p.screenshot({ path: out });
await b.close();
