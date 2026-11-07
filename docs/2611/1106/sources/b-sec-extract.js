(async () => {
  const url = "/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm";
  const r = await fetch(url);
  const status = r.status;
  const html = await r.text();
  const doc = new DOMParser().parseFromString(html, "text/html");
  const text = doc.body.innerText || doc.body.textContent;
  const pats = ["imminently", "dramatically accelerated", "closing the loop", "Responsible Scaling Policy"];
  const out = {status, htmlLen: html.length, textLen: text.length, hits: {}};
  for (const p of pats) {
    const idxs = []; let i = -1;
    while ((i = text.indexOf(p, i+1)) !== -1 && idxs.length < 12) idxs.push(i);
    out.hits[p] = idxs.length;
    out["ctx_"+p] = idxs.slice(0,6).map(i => text.slice(Math.max(0,i-900), i+900));
  }
  window.__s1a = out;
  return JSON.stringify({status, htmlLen: html.length, textLen: text.length, hits: out.hits});
})()
