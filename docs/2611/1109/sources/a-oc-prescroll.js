// a-oc-prescroll.js：opencli eval 用。滚到含某段文字的最内层元素处并抖动一下滚动，触发图表进入视口后的绘制动画。
(() => {
  const t = __T__;
  const all = [...document.querySelectorAll('body *')].filter(e => !['SCRIPT','STYLE'].includes(e.tagName) && (e.textContent||'').includes(t));
  const inner = all.filter(e => ![...e.children].some(c => (c.textContent||'').includes(t)));
  if (!inner[0]) return 'no pre';
  inner[0].scrollIntoView({block: 'center', behavior: 'instant'});
  return 'ok';
})()
