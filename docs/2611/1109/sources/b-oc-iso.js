// b-oc-iso.js：opencli eval 用。只保留 START 到 END 之间（含）的 DOM，其余 display:none，滚到顶，返回文档高度。参数占位：__START__ __END__ __ZOOM__ __W__
(async () => {
  const START = __START__, END = __END__;
  for (let i = 0; i < 30 && !document.body.textContent.includes(START); i++) await new Promise(r => setTimeout(r, 500));
  await new Promise(r => setTimeout(r, 800));
  const p = document.getElementById('immersive-translate-popup'); if (p) p.remove();
  const inner = t => { const all = [...document.querySelectorAll('body *:not(script):not(style)')].filter(e => (e.textContent || '').includes(t)); return all.filter(e => ![...e.children].some(c => (c.textContent || '').includes(t))); };
  let s = inner(START)[0]; for (let k=0;k<__UP__&&s;k++) s=s.parentElement; if (!s) return JSON.stringify({err: 'start not found'});
  const ends = inner(END).filter(e => e === s || s.contains(e) || (s.compareDocumentPosition(e) & Node.DOCUMENT_POSITION_FOLLOWING));
  const e = ends[0]; if (!e) return JSON.stringify({err: 'end not found'});
  document.querySelectorAll('body *').forEach(x => { const c = getComputedStyle(x).position; if (c === 'fixed' || c === 'sticky') { if (x.contains(s)) { x.style.position = 'static'; x.style.height = 'auto'; } else x.style.display = 'none'; } });
  const hide = n => { n.style.display = 'none'; };
  // 共同祖先
  let C = s; while (!C.contains(e)) C = C.parentElement;
  if (s === e || C === s) { // 单元素：逐层隐藏兄弟
    let n = s; while (n !== document.body) { [...n.parentElement.children].forEach(c => { if (c !== n) hide(c); }); n = n.parentElement; }
  } else {
    let sc = s; while (sc.parentElement !== C) sc = sc.parentElement;
    let ec = e; while (ec.parentElement !== C) ec = ec.parentElement;
    const kids = [...C.children]; const i = kids.indexOf(sc), j = kids.indexOf(ec);
    kids.forEach((c, k) => { if (k < i || k > j) hide(c); });
    let n = s; while (n !== sc) { let seen = false; [...n.parentElement.children].forEach(c => { if (c === n) { seen = true; } else if (!seen) hide(c); }); n = n.parentElement; }
    n = e; while (n !== ec) { let seen = false; [...n.parentElement.children].forEach(c => { if (c === n) { seen = true; } else if (seen) hide(c); }); n = n.parentElement; }
    let n2 = C; while (n2 !== document.body) { [...n2.parentElement.children].forEach(c => { if (c !== n2) hide(c); }); n2 = n2.parentElement; }
  }
  document.querySelectorAll('body *').forEach(e=>{const c=getComputedStyle(e); if(parseFloat(c.opacity)<1){e.style.opacity='1';e.style.transform='none';} if(c.visibility==='hidden'){e.style.visibility='visible';}});
  for (let n = s; n && n !== document.documentElement; n = n.parentElement) { n.style.transform = 'none'; n.style.translate = 'none'; n.style.opacity = '1'; n.style.visibility = 'visible'; n.style.overflow = 'visible'; n.style.height = n.style.height; }
  document.documentElement.style.zoom = '__ZOOM__';
  document.documentElement.style.width = (__W__ / __ZOOM__) + 'px';
  window.scrollTo(0, 0);
  const r = s.getBoundingClientRect(); return JSON.stringify({ok: 1, r: [r.left, r.top, r.width, r.height], tag: s.tagName, sh: document.documentElement.scrollHeight, bh: document.body.getBoundingClientRect().height});
})()
