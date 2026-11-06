/* Tabs and highlighting for the infographics (.wi). The markup is complete without this file: it only adds roles, keys and classes. */
(function () {
  'use strict';
  function all(root, sel) { return Array.prototype.slice.call(root.querySelectorAll(sel)); }
  function keyboardFocus(el) { try { return el.matches(':focus-visible'); } catch (e) { return true; } }
  function has(el, token) { return (' ' + (el.getAttribute('data-in') || '') + ' ').indexOf(' ' + token + ' ') >= 0; }

  function setup(root) {
    var tabs = all(root, '.wi-tab');
    var panels = tabs.map(function (t) { return document.getElementById(t.getAttribute('data-panel')); });
    var list = root.querySelector('.wi-tabs');
    var parts = all(root, '[data-in]');
    var triggers = all(root, '[data-hl]');
    var pinned = null, hovered = null, current = -1;

    function paint() {
      var on = hovered || pinned;
      parts.forEach(function (el) { el.classList.toggle('wi-dim', !!on && !has(el, on)); });
      triggers.forEach(function (el) {
        el.classList.toggle('wi-on', el.getAttribute('data-hl') === on);
        if (el.hasAttribute('aria-pressed')) el.setAttribute('aria-pressed', String(el.getAttribute('data-hl') === pinned));
      });
    }

    function select(i, focus) {
      if (i === current) { if (focus) tabs[i].focus(); return; }
      current = i;
      tabs.forEach(function (t, j) {
        t.setAttribute('aria-selected', String(j === i));
        t.tabIndex = j === i ? 0 : -1;
        panels[j].classList.toggle('wi-off', j !== i);
        panels[j].classList.toggle('wi-active', j === i);
      });
      /* a pinned highlight stays only if the new panel has something to show for it */
      if (pinned && !panels[i].querySelector('[data-in~="' + pinned + '"]')) pinned = null;
      hovered = null;
      paint();
      if (focus) tabs[i].focus();
    }

    if (list && tabs.length) {
      list.setAttribute('role', 'tablist');
      tabs.forEach(function (t, i) {
        if (!t.id) t.id = panels[i].id + '-tab';
        t.setAttribute('role', 'tab');
        t.setAttribute('aria-controls', panels[i].id);
        panels[i].setAttribute('role', 'tabpanel');
        panels[i].setAttribute('aria-labelledby', t.id);
        if (!panels[i].querySelector('a[href], button')) panels[i].tabIndex = 0;
        t.addEventListener('click', function () {
          select(i, false);
          /* tabs low in the window: the panel that just changed is mostly out of sight, so bring the tabs to the top */
          if (panels[i].getBoundingClientRect().top > window.innerHeight * 0.7) {
            var calm = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
            list.scrollIntoView({ block: 'start', behavior: calm ? 'auto' : 'smooth' });
          }
        });
        t.addEventListener('keydown', function (e) {
          var n = tabs.length, to = -1;
          if (e.key === 'ArrowRight' || e.key === 'ArrowDown') to = (i + 1) % n;
          else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') to = (i + n - 1) % n;
          else if (e.key === 'Home') to = 0;
          else if (e.key === 'End') to = n - 1;
          if (to >= 0) { e.preventDefault(); select(to, true); }
        });
      });
      select(0, false);
    }

    triggers.forEach(function (el) {
      var token = el.getAttribute('data-hl');
      el.addEventListener('pointerenter', function (e) { if (e.pointerType === 'mouse') { hovered = token; paint(); } });
      el.addEventListener('pointerleave', function (e) { if (e.pointerType === 'mouse' && hovered === token) { hovered = null; paint(); } });
      if (el.tagName !== 'BUTTON') return;
      el.addEventListener('focus', function () { if (keyboardFocus(el)) { hovered = token; paint(); } });
      el.addEventListener('blur', function () { if (hovered === token) { hovered = null; paint(); } });
      el.addEventListener('click', function () { pinned = pinned === token ? null : token; if (!pinned && !el.matches(':hover')) hovered = null; paint(); });
    });
    root.addEventListener('keydown', function (e) { if (e.key === 'Escape' && (pinned || hovered)) { pinned = hovered = null; paint(); } });

    root.classList.add('wi-js');
  }

  function start() { all(document, '.wi[data-wi]').forEach(setup); }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start); else start();
})();
