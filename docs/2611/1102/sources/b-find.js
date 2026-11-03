(function(q,k,off){
  var w=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT);var n,c=0,hit=null,list=[];
  while(n=w.nextNode()){ if(n.nodeValue.replace(/\s+/g,' ').indexOf(q)>=0){list.push(n); } }
  hit=list[k||0]; if(!hit) return 'notfound count='+list.length;
  var e=hit.parentElement; e.scrollIntoView({block:'start'}); window.scrollBy(0,-(off||40));
  var r=e.getBoundingClientRect();
  return JSON.stringify({count:list.length,tag:e.tagName,r:[r.x,r.y,r.width,r.height],sy:window.scrollY,txt:e.parentElement.textContent.slice(0,200)});
})
