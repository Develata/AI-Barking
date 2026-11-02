#!/usr/bin/env python3
# a-gdpx-shot.py: 站点特定脚本（取证用）——切换 GDPx 榜单视图并用 opencli 截全页（zoom 2、宽 1800），由 barking shot cropbox 裁出榜单区。
import importlib.util, time, subprocess, sys
spec = importlib.util.spec_from_file_location('g', 'docs/2611/1101/sources/a-gdpx-domains.py')
g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)
def dom_open(): return g.js("[...document.querySelectorAll('[role=menuitemcheckbox]')].some(x=>x.textContent.startsWith('Chip Design'))") is True
def set_metric(label):
    g.click_text("#leaderboard button", "Metric"); time.sleep(0.6)
    g.click_text("[role=menuitemradio]", label); time.sleep(0.8)
def set_domain(label, radio=False):
    if not dom_open(): g.open_domain_menu()
    g.click_text("[role=menuitemradio],[role=menuitemcheckbox]", "All domains"); time.sleep(0.6)
    if label != "All domains":
        if not dom_open(): g.open_domain_menu()
        g.click_text("[role=menuitemradio],[role=menuitemcheckbox]", label); time.sleep(0.9)
    if dom_open(): g.open_domain_menu(); time.sleep(0.4)
def shot(path):
    g.js("document.documentElement.style.zoom='2'; 'ok'"); time.sleep(1)
    subprocess.run(["opencli","browser","--session","a-shot","screenshot",path,"--width","1800","--full-page"],capture_output=True)
    g.js("document.documentElement.style.zoom='1'; 'ok'"); time.sleep(0.5)
