// a-oc-isolate.js：opencli eval 用。把页面里指定的几个原始元素（段落、表格、图）按顺序移到一个干净容器顶部、隐藏页面其余部分，
// 避免截图时因视口缩放而滚动错位；元素本身不改文字与数据，只去掉导航栏与翻译插件浮层。
// SPECS 为 JSON 数组：{"t":"锚点文字","up":"祖先选择器或空","tag":"限定元素标签或空","cls":"限定class子串或空"}；ZOOM、WIDTH(容器 CSS px) 由 run 脚本替换。
(() => {
  const SPECS = __SPECS__, Z = __ZOOM__, W = __WIDTH__;
  const pop = document.getElementById('immersive-translate-popup'); if (pop) pop.remove();
  document.querySelectorAll('[id^=immersive-translate]').forEach(e => e.remove());
  const findEl = (s) => {
    if (s.q) return document.querySelectorAll(s.q)[s.i || 0] || null;
    const all = [...document.querySelectorAll(s.tag || '*')].filter(e => !['SCRIPT','STYLE'].includes(e.tagName) && (e.textContent || '').includes(s.t) && (!s.cls || (e.className||'').toString().includes(s.cls)));
    const inner = all.filter(e => ![...e.children].some(c => (c.textContent || '').includes(s.t)));
    let el = inner[s.nth || 0]; if (!el) return null;
    return s.up ? el.closest(s.up) : el;
  };
  const els = SPECS.map(findEl);
  SPECS.forEach((sp, k) => { if (sp.maxRows && els[k]) [...els[k].querySelectorAll('tr')].forEach((r, j) => { if (j >= sp.maxRows) r.style.display = 'none'; }); }); // 长榜只截前 N 行，其余行隐藏（节选）
  if (els.some(e => !e)) return 'missing: ' + JSON.stringify(els.map(e => !!e));
  const box = document.createElement('div'); box.id = 'iso';
  box.style.cssText = 'position:absolute;top:0;left:0;background:' + getComputedStyle(document.body).backgroundColor + ';padding:12px 14px;box-sizing:border-box;width:' + W + 'px;zoom:' + Z + ';z-index:99999';
  [...document.body.children].forEach(c => { c.style.display = 'none'; });
  document.body.appendChild(box);
  document.body.style.margin = '0';
  els.forEach(e => {
    e.style.display = ''; e.style.maxWidth = 'none';
    const cs = getComputedStyle(e); // 移动前固化原排版，保持字体、字号、行距与原页一致
    if (e.tagName === 'P') { e.style.fontFamily = cs.fontFamily; e.style.fontSize = cs.fontSize; e.style.lineHeight = cs.lineHeight; e.style.color = cs.color; e.style.margin = '0 0 20px 0'; }
    box.appendChild(e);
  });
  document.querySelectorAll('#iso [class*=layer]').forEach(l => { l.style.overflow = 'visible'; });
  // 后台标签页不跑 CSS 入场动画，图上的点与线停在透明初始态；这里只把动画结束态强制为可见，不改数据。
  const st = document.createElement('style'); st.textContent = '#iso svg *{animation:none !important;} #iso svg [class*=_node],#iso svg [class*=_mark],#iso svg [class*=pointLabel],#iso svg [class*=_line],#iso svg [class*=_series],#iso svg path,#iso svg polyline{opacity:1 !important;stroke-dashoffset:0 !important;}'; document.head.appendChild(st);
  window.scrollTo(0, 0);
  const r = box.getBoundingClientRect();
  return JSON.stringify({w: r.width, h: r.height, n: els.length});
})()
