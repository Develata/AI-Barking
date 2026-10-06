import fs from "node:fs";
import { execFileSync } from "node:child_process";

const sourceDir = "docs/2610/1004/sources";
const urls = [
  ["a-reports-index", "https://alignment.openai.com/misalignment-reports/"],
  ["a-report-perl", "https://alignment.openai.com/misalignment-reports/command-injecting-a-reference-tool-to-copy-a-source-file/"],
  ["a-report-eda", "https://alignment.openai.com/misalignment-reports/reaching-an-internal-eda-host-through-a-reference-tool/"],
  ["a-report-slack", "https://alignment.openai.com/misalignment-reports/preparing-for-a-restart-after-reading-slack/"],
  ["a-atlantic", "https://www.theatlantic.com/technology/2026/10/openai-safety-team-resignation/688881/"],
  ["a-reuters", "https://www.reuters.com/legal/litigation/openai-safety-employee-quits-says-time-trial-error-is-over-2026-10-03/"],
];
const strip = (u) => {
  const x = new URL(u);
  for (const k of [...x.searchParams.keys()]) if (k.toLowerCase().startsWith("utm_")) x.searchParams.delete(k);
  return x.toString();
};
const nowBJ = () => new Date(Date.now() + 8 * 60 * 60 * 1000).toISOString().replace("Z", "+08:00");
const records = [];
for (const [name, url] of urls) {
  const started = nowBJ();
  try {
    const opened = execFileSync("opencli.exe", ["browser", "1004-a", "open", url], { encoding: "utf8", timeout: 90000 });
    const js = "JSON.stringify({url:location.href,title:document.title,text:document.body.innerText,meta:[...document.querySelectorAll('meta[name],meta[property],time,link[rel=canonical]')].map(x=>x.outerHTML),links:[...document.querySelectorAll('a')].map(x=>({text:x.innerText,url:x.href}))})";
    let out = execFileSync("opencli.exe", ["browser", "1004-a", "eval", js], { encoding: "utf8", timeout: 90000 });
    out = out.split("\n  Update available:")[0].trim();
    const data = JSON.parse(out);
    data.url = strip(data.url);
    data.links = (data.links || []).map((x) => ({ ...x, url: strip(x.url) }));
    fs.writeFileSync(`${sourceDir}/${name}.json`, JSON.stringify(data, null, 2), "utf8");
    fs.writeFileSync(`${sourceDir}/${name}.txt`, data.text || "", "utf8");
    records.push({ started_bj: started, url: strip(url), tool: "opencli browser 1004-a open/eval", result: `成功；${(data.text || "").length} 字符；标题=${data.title}`, files: [`${name}.json`, `${name}.txt`] });
    console.log(name, started, data.title, (data.text || "").length, data.url);
  } catch (error) {
    records.push({ started_bj: started, url: strip(url), tool: "opencli browser 1004-a open/eval", result: `失败：${String(error.message || error).slice(0, 600)}`, files: [] });
    console.log(name, started, "FAILED", String(error.message || error).slice(0, 400));
  }
  fs.writeFileSync(`${sourceDir}/a-archive-capture.jsonl`, records.map((x) => JSON.stringify(x)).join("\n") + "\n", "utf8");
}

