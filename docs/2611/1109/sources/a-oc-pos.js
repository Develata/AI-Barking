// a-oc-pos.js：opencli eval 用。按文本锚点定位元素、去掉翻译插件浮层、滚到视口 top 偏移处，返回元素 rect（视口坐标，CSS zoom 后的像素）。
// 用法：把 ANCHOR/PAD 替换后传给 opencli browser eval。
(() => {
  const ANCHOR = __ANCHOR__, PAD = __PAD__, NTH = __NTH__;
  const p = document.getElementById('immersive-translate-popup'); if (p) p.remove();
  const hd = document.querySelector('.site-header'); if (hd) hd.style.display = 'none';
  document.documentElement.style.zoom = '__ZOOM__';
  document.documentElement.style.width = (1400 / __ZOOM__) + 'px';
  const all = [...document.querySelectorAll('body *:not(script):not(style)')]
    .filter(e => (e.textContent||"").includes(ANCHOR));
  // 取最内层命中元素
  const inner = all.filter(e => ![...e.children].some(c => (c.textContent||"").includes(ANCHOR)));
  const el = inner[NTH]; if (!el) return JSON.stringify({err: 'not found', n: inner.length});
  el.scrollIntoView({block: 'start', behavior: 'instant'});
  window.scrollBy({top: -PAD, behavior: 'instant'});
  const r = el.getBoundingClientRect();
  return JSON.stringify({n: inner.length, tag: el.tagName, top: r.top, left: r.left, w: r.width, h: r.height, sy: window.scrollY});
})()
