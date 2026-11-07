(async () => {
  const res = {};
  for (const [k,url] of [["s1","/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm"],["s1a","/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm"]]) {
    const r = await fetch(url); const html = await r.text();
    const doc = new DOMParser().parseFromString(html, "text/html");
    const text = doc.body.textContent;
    const o = {status:r.status, htmlLen: html.length, imminently: (text.match(/imminently/g)||[]).length, dramatic: (text.match(/dramatically accelerated progress/g)||[]).length};
    const i = text.indexOf("dramatically accelerated");
    o.prior = i>=0 ? text.slice(Math.max(0,i-2600), i-900) : null;
    const j = text.indexOf("closing the loop");
    o.loop = j>=0 ? text.slice(Math.max(0,j-600), j+900): null;
    const k2 = text.indexOf("Rule 424") ; 
    res[k]=o;
  }
  return JSON.stringify(res);
})()
