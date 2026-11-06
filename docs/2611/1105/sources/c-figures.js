/* Interactive figures: highlighting and tooltips. No dependencies, no network, no HTML built from strings.
   The markup is complete without this file; it only adds behaviour.
   - .wf-key[data-m]  a model's key: hover or focus highlights that model in every figure; click pins it (Escape or a second click clears).
                      In a figure whose root has data-local-m the colours stand for something other than the responding model,
                      so its model keys act within that figure only, and highlights from other figures pass it by.
   - .wf-key[data-s]  a series' key: the same, within its own figure.
   - .wf-mark         a data mark: hover, focus or tap shows a tooltip built from its data-t1 (what it is), data-t2 and data-t3 (its value or values). */
(function () {
  'use strict';
  var figs = Array.prototype.slice.call(document.querySelectorAll('.wf'));
  if (!figs.length) return;
  var pin = null, hov = null, openMark = null, fresh = false, downX = 0, downY = 0, lastW = window.innerWidth;

  function same(a, b) { return !!a && !!b && a.attr === b.attr && a.val === b.val && a.fig === b.fig; }
  function keyState(key) {
    var fig = key.closest('.wf');
    return key.hasAttribute('data-m') ? { attr: 'data-m', val: key.getAttribute('data-m'), fig: fig.hasAttribute('data-local-m') ? fig : null }
                                      : { attr: 'data-s', val: key.getAttribute('data-s'), fig: fig };
  }
  function apply() {
    var act = hov || pin;
    figs.forEach(function (fig) {
      var on = !!act && (act.fig ? act.fig === fig : !fig.hasAttribute('data-local-m')) && !!fig.querySelector('[' + act.attr + '="' + act.val + '"]');
      Array.prototype.forEach.call(fig.querySelectorAll('[data-m], [data-s]'), function (el) {
        var has = on && el.hasAttribute(act.attr);
        var hit = has && el.getAttribute(act.attr) === act.val;
        el.classList.toggle('wf-dim', has && !hit);
        el.classList.toggle('wf-on', hit);
      });
      Array.prototype.forEach.call(fig.querySelectorAll('.wf-key'), function (k) {
        k.setAttribute('aria-pressed', same(pin, keyState(k)) ? 'true' : 'false');
      });
    });
  }

  function tipOf(fig) {
    var t = fig.querySelector('.wf-tip');
    if (!t) {
      t = document.createElement('div'); t.className = 'wf-tip'; t.hidden = true;
      t.setAttribute('aria-hidden', 'true');   // each mark's aria-label already says the same
      fig.appendChild(t);
    }
    return t;
  }
  function hideTip() {
    if (!openMark) return;
    openMark.classList.remove('wf-open');
    tipOf(openMark.closest('.wf')).hidden = true;
    openMark = null;
  }
  function showTip(mark) {
    if (openMark === mark) return;
    hideTip();
    var fig = mark.closest('.wf'), tip = tipOf(fig), i, s, v;
    while (tip.firstChild) tip.removeChild(tip.firstChild);
    for (i = 1; i <= 3; i++) {
      v = mark.getAttribute('data-t' + i);
      if (!v) continue;
      s = document.createElement('span'); s.className = 'wf-t' + i; s.textContent = v; tip.appendChild(s);
    }
    if (!tip.firstChild) return;
    tip.style.left = '0px'; tip.style.top = '0px';   // measure at the left edge: a box left over at the far right would be squeezed
    tip.hidden = false; openMark = mark; fresh = true; mark.classList.add('wf-open');
    var a = (mark.querySelector('.wf-anchor') || mark).getBoundingClientRect(), f = fig.getBoundingClientRect();
    var w = tip.offsetWidth, h = tip.offsetHeight;
    var left = Math.max(0, Math.min(a.left + a.width / 2 - f.left - w / 2, f.width - w));
    var top = a.top - f.top - h - 9;
    // Above the mark (it may rise over the text above the figure), or below it where the window's top edge is in the way.
    if (a.top - h - 9 < 4) top = a.bottom - f.top + 9;
    tip.style.left = Math.floor(left) + 'px'; tip.style.top = Math.round(top) + 'px';
  }
  function focusVisible(el) { try { return el.matches(':focus-visible'); } catch (e) { return true; } }
  function visibleMarks(plot) {
    return Array.prototype.filter.call(plot.querySelectorAll('.wf-mark'), function (m) { return m.getClientRects().length > 0; });
  }

  figs.forEach(function (fig) {
    fig.classList.add('wf-js');
    // One tab stop per plot; the arrow keys walk its marks.
    Array.prototype.forEach.call(fig.querySelectorAll('.wf-plot'), function (plot) {
      Array.prototype.forEach.call(plot.querySelectorAll('.wf-mark'), function (m, i) { m.setAttribute('tabindex', i ? '-1' : '0'); });
    });
    fig.addEventListener('pointerover', function (e) {
      if (e.pointerType !== 'mouse') return;
      var k = e.target.closest('.wf-key'), m = e.target.closest('.wf-mark');
      if (k) { hov = keyState(k); apply(); }
      if (m) showTip(m);
    });
    fig.addEventListener('pointerout', function (e) {
      if (e.pointerType !== 'mouse') return;
      var to = e.relatedTarget, k = e.target.closest('.wf-key'), m = e.target.closest('.wf-mark');
      if (k && !(to && k.contains(to))) { hov = null; apply(); }
      if (m && !(to && m.contains(to))) hideTip();
    });
    fig.addEventListener('focusin', function (e) {
      var k = e.target.closest('.wf-key'), m = e.target.closest('.wf-mark');
      if (k && focusVisible(k)) { hov = keyState(k); apply(); }
      if (m) showTip(m);
    });
    fig.addEventListener('focusout', function (e) {
      if (e.target.closest('.wf-key')) { hov = null; apply(); }
      if (e.target.closest('.wf-mark')) hideTip();
    });
    fig.addEventListener('click', function (e) {
      var k = e.target.closest('.wf-key'), m = e.target.closest('.wf-mark');
      if (k) { var s = keyState(k); pin = same(pin, s) ? null : s; hov = null; apply(); }
      if (m) {   // a click or tap on the mark whose tooltip was already open closes it
        if (openMark === m && !fresh) hideTip(); else showTip(m);
        fresh = false; e.stopPropagation();
      }
    });
    fig.addEventListener('keydown', function (e) {
      var m = e.target.closest('.wf-mark'), d = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[e.key];
      if (!m || !d || e.altKey || e.ctrlKey || e.metaKey) return;
      var list = visibleMarks(m.closest('.wf-plot') || fig), n = list[list.indexOf(m) + d];
      if (n) { m.setAttribute('tabindex', '-1'); n.setAttribute('tabindex', '0'); n.focus(); }
      e.preventDefault();
    });
  });
  document.addEventListener('click', hideTip);
  // iOS Safari sends no click for a tap on plain text, so a press and release in one place outside a mark also closes the tooltip.
  // (A swipe that scrolls the page ends in pointercancel, not pointerup, and leaves it open.)
  document.addEventListener('pointerdown', function (e) { downX = e.clientX; downY = e.clientY; });
  document.addEventListener('pointerup', function (e) {
    var still = Math.abs(e.clientX - downX) < 10 && Math.abs(e.clientY - downY) < 10;
    if (still && !(e.target.closest && e.target.closest('.wf-mark'))) hideTip();
  });
  window.addEventListener('resize', function () {   // the mark moves, or its layout gives way to the other one; a change of height alone (a phone's address bar) is nothing
    if (window.innerWidth !== lastW) { lastW = window.innerWidth; hideTip(); }
  });
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') { pin = null; hov = null; hideTip(); apply(); }
  });
})();
