// Local-only extraction and arithmetic; no network, browser launch, or deletion.
import fs from 'node:fs';
const out = 'docs/2610/1006/sources/';
const bj = () => new Date(Date.now()+28800000).toISOString().replace('Z','+08:00');
const write = (name,data) => fs.writeFileSync(out+name,JSON.stringify(data,null,2)+'\n',{flag:'wx'});
const records=[];
for(const id of ['2104823812042940713','2104951965184925941']) {
  const path=`docs/2609/0929/sources/e-x-${id}.json`;
  const p=JSON.parse(fs.readFileSync(path,'utf8').replace(/^\uFEFF/,'' )).find(x=>x.id===id);
  records.push({source_file:path,status:'Historical local public-post archive; current live retrieval failed',url:p.url,id:p.id,author:p.author,text:p.text,created_at_original:p.created_at,created_at_bj:new Date(new Date(p.created_at).getTime()+28800000).toISOString().replace('Z','+08:00')});
}
const path='docs/2610/1005/sources/d-tibo-day1-note.json';
const p=JSON.parse(fs.readFileSync(path,'utf8').replace(/^\uFEFF/,''));
const dom=JSON.parse(fs.readFileSync('docs/2610/1005/sources/d-tibo-day1.json','utf8').replace(/^\uFEFF/,''));
const t=dom.cards.flatMap(x=>x.times).find(x=>x.datetime.startsWith('2026-10-05'));
records.push({source_file:path,status:'Historical local public-post archive; current live retrieval failed',url:p.url,id:p.id,author:'thsottiaux',text:p.note.text,original_capture_bj:p.captured_bj,created_at_original:t.datetime,created_at_bj:new Date(new Date(t.datetime).getTime()+28800000).toISOString().replace('Z','+08:00')});
write('b-historical-posts.json',{compiled_bj:bj(),warning:'Not a new X capture; no contemporary engagement counts copied',records});
const rows=[{usd:200,api_ant:11726,api_oai:2084,tokens_b_ant:28.6,tokens_b_oai:10.2},{usd:100,api_ant:5725,api_oai:1055,tokens_b_ant:14,tokens_b_oai:5.1},{usd:20,api_ant:1178,api_oai:211,tokens_b_ant:2.9,tokens_b_oai:1}];
for(const r of rows){r.api_ratio=r.api_ant/r.api_oai;r.token_ratio=r.tokens_b_ant/r.tokens_b_oai;}
write('b-conditional-ratios.json',{compiled_bj:bj(),source:'.handoff/2026-10-06-1006-evidence.md B known situation',status:'Arithmetic verified; input numbers NOT independently verified against original charts',rows,four_x_candidate:{formula:'11726/2897',value:11726/2897,status:'Candidate interpretation only; scatterplot not retrieved'},workload_percent_sum:0.4+96.6+2.6+0.3});
console.log('Created b-historical-posts.json and b-conditional-ratios.json');
