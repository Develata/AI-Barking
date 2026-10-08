// usage: node d-html2txt.mjs <in.html> <out.txt>
// Crude HTML->text: drops script/style/noscript/svg, turns block tags into newlines, decodes common entities.
// Also prints <title>, meta (published/modified/description) and JSON-LD dates to stdout head of the .txt as "META:" lines.
import fs from 'node:fs';
const [inp,out]=process.argv.slice(2);
let h=fs.readFileSync(inp,'utf8');
const meta=[];
for(const m of h.matchAll(/<meta[^>]+>/gi)){const s=m[0];if(/(property|name)=["'](article:|og:title|og:description|og:url|description|date|pubdate|twitter:title|twitter:description|last-modified)/i.test(s))meta.push(s)}
for(const m of h.matchAll(/<link[^>]+rel=["']canonical["'][^>]*>/gi))meta.push(m[0]);
const title=(h.match(/<title[^>]*>([\s\S]*?)<\/title>/i)||[])[1]||'';
const times=[...h.matchAll(/<time[^>]*>[\s\S]*?<\/time>/gi)].map(m=>m[0].replace(/\s+/g,' '));
const ld=[...h.matchAll(/<script[^>]+type=["']application\/ld\+json["'][^>]*>([\s\S]*?)<\/script>/gi)].map(m=>m[1].trim());
h=h.replace(/<(script|style|noscript|svg|template)[\s\S]*?<\/\1>/gi,'');
h=h.replace(/<!--[\s\S]*?-->/g,'');
h=h.replace(/<(br|\/p|\/div|\/li|\/h[1-6]|\/tr|\/section|\/article|\/ul|\/ol|\/table|\/blockquote|\/pre)[^>]*>/gi,'\n');
h=h.replace(/<li[^>]*>/gi,'- ').replace(/<[^>]+>/g,'');
const ent={'&amp;':'&','&lt;':'<','&gt;':'>','&quot;':'"','&#39;':"'",'&apos;':"'",'&nbsp;':' ','&rsquo;':'’','&lsquo;':'‘','&ldquo;':'“','&rdquo;':'”','&mdash;':'—','&ndash;':'–','&hellip;':'…'};
h=h.replace(/&(#x?[0-9a-f]+|[a-z]+);/gi,(m,e)=>{if(ent[m])return ent[m];if(e[0]==='#'){const n=e[1].toLowerCase()==='x'?parseInt(e.slice(2),16):parseInt(e.slice(1),10);return String.fromCodePoint(n)}return m});
h=h.split('\n').map(l=>l.replace(/[ \t]+/g,' ').trim()).filter(l=>l&&l!=='-');
let body=h.join('\n').replace(/\n{3,}/g,'\n\n');
const head=['TITLE: '+title.trim(),...meta.map(m=>'META: '+m),...times.map(t=>'TIME: '+t),...ld.map(l=>'JSONLD: '+l.slice(0,3000))].join('\n');
fs.writeFileSync(out,head+'\n\n=====BODY=====\n'+body+'\n');
console.log(out,body.length,'chars');
