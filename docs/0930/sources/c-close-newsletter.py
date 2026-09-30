import sys,importlib
sys.path.insert(0,'docs/0930/sources');b=importlib.import_module('a-browser-capture');b.SESSION='evidence0930qa'
b.call('eval','document.querySelector("[id^=mcforms]").shadowRoot.querySelector("button[aria-label=Close]").click();document.querySelector("button[aria-label=Dismiss]")?.click();true')
b.log(dict(id='c-newsletter-dismiss',url='https://www.whitehouse.gov/presidential-actions/2026/09/inaugurating-the-era-of-super-intelligence/',time_bjt=b.stamp(),tool='opencli browser eval; native Close button inside newsletter shadow root',status='success; newsletter closed, no form submission'))
b.shot('17-whitehouse-section-one.png',560,1000)
b.shot('18-whitehouse-sections-two-three.png',1260,1170)
b.call('eval','window.scrollTo({top:0,behavior:"instant"});[...document.querySelectorAll("button")].find(e=>e.innerText.trim()==="MENU")?.click();true')
b.call('hover','#menu-item-29161 > a');b.call('hover','#menu-item-33121 > a')
b.shot('26-whitehouse-ai-navigation.png',0,900)
