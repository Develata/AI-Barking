$ErrorActionPreference='Stop'
$root='E:/gitclone/AI-Barking/docs/1001'
function Stamp($f){$f.LastWriteTimeUtc.AddHours(8).ToString('yyyy-MM-dd HH:mm:ss')+'（UTC+8）'}
$lines=[Collections.Generic.List[string]]::new()
$lines.Add("# 1001 抓取记录`n`n全部时间为北京时间（UTC+8）。逐次浏览器记录来自d-browser-log.jsonl；未即时记入日志的原始档案以写盘时刻补记，明确标记，精度不代表HTTP完成时刻。搜索/失败中没有保存秒级时刻的项目只报告执行区间，见d-search-scope.md。`n`n## 即时记录`n`n| 时间（北京） | URL | 工具 | 结果 | 文件 |`n|---|---|---|---|---|")
foreach($line in Get-Content "$root/sources/d-browser-log.jsonl"){
 $x=$line|ConvertFrom-Json
 $url=([string]$x.url).Replace("`n",' ').Replace('|','\|')
 $lines.Add('| '+$x.time+' | '+$url+' | '+$x.tool+' | '+$x.result+' | '+$x.file+' |')
}
$lines.Add("`n## 档案写入时间补充索引`n`n时间是原档案最后写入，不推测更早开始时刻；与上表重合的不是另一次请求。页面正文真实性/付费墙限制以evidence.md为准。`n`n| 写入时间（北京） | 原站URL / 查询入口 | 工具 | 结果 | 档案 |`n|---|---|---|---|---|")
foreach($f in Get-ChildItem "$root/sources/*-extract.json" | Sort-Object Name){
 try{$j=Get-Content $f.FullName -Raw|ConvertFrom-Json;$result=if($j.total_chars -gt 0){'已提取正文/可见内容'}else{'空DOM；PDF另存或见缺口'};$lines.Add('| '+(Stamp $f)+' | '+$j.url+' | opencli browser open/state/extract | '+$result+' | '+$f.Name+' |')}catch{$lines.Add('| '+(Stamp $f)+' | 见相邻布局档案 | opencli browser | 提取解析失败 | '+$f.Name+' |')}
}
foreach($f in Get-ChildItem "$root/sources/d-hn-*.json"){
 $j=Get-Content $f.FullName -Raw|ConvertFrom-Json
 $lines.Add('| '+(Stamp $f)+' | https://hn.algolia.com/api/v1/search?'+$j.params+' | 浏览器读取公开Algolia API | nbHits='+$j.nbHits+'；只保存响应，不补历史热度 | '+$f.Name+' |')
}
foreach($f in Get-ChildItem "$root/sources/d-reddit-*.json"){$lines.Add('| '+(Stamp $f)+' | https://www.reddit.com/search/ （参数见d-search-scope.md） | opencli reddit search | JSON成功；无关结果已排除于事实表 | '+$f.Name+' |')}
$social=@{'a-official-x.json'='https://x.com/GoogleDeepMind/status/2105388084154056939';'b-kimi-profile.json'='https://x.com/Kimi_Moonshot';'b-openai-x-profile.json'='https://x.com/OpenAI';'c-official-x-profile.json'='https://x.com/AnthropicAI';'c-official-x-search.json'='https://x.com/search?q=from%3AAnthropicAI%20robots&f=live';'c-official-x-search-retry.json'='https://x.com/search?q=from%3AAnthropicAI%20robots&f=live'}
foreach($name in $social.Keys){$f=Get-Item "$root/sources/$name";$j=Get-Content $f.FullName -Raw|ConvertFrom-Json;$lines.Add('| '+(Stamp $f)+' | '+$social[$name]+' | opencli browser公开帖子卡片DOM | '+$(if(@($j).Count -eq 0){'空数组；搜索未成功'}else{'公开卡片存档；非完整历史'})+' | '+$name+' |')}
$lines.Add("`n## 最终图像处理与纠错`n`n- 08、19、20：Google原图下载后等比缩放，未重绘；12、13、15：AA原图下载后等比缩放，保留全图。`n- 09–10：a-methodology.pdf物理页2、3渲染。`n- 17：AA原站完整比较表区域；18：同页完整概览三图。截图裁切只改变周边区域，不改变文字数字。`n- 25：关闭原站广告遮罩后重新截标题，未操作Cookie授权。`n- 28：b-update.pdf页1；29：页3+4纵向拼接。`n- 30–39：c-anthropic.pdf页1+2、4+5、6+16、7、8+9、15、17、18、19+20、22；数字为物理页码，32跨页拼接不是连续正文。`n- 原始错误截图保留sources/d-rejected-*，不得当候选配图使用。早期同名图的日志不是最终图片内容；最终对应关系以上表与evidence.md为准。`n- 截图和原页渲染都用原站/原PDF内容；未用AI图像生成或重画表格。`n`n## 失败与非抓取操作`n`n见[d-search-scope.md](d-search-scope.md)。未即时记秒级时间的命令被阻止、适配器超时等事件，发生于本次执行区间（北京时间2026-10-02凌晨），不伪造精确时刻。执行前已检查opencli doctor、agent-reach doctor --json；浏览器扩展已连接。未登录、未社交写入、未绕过付费墙。")
$lines -join "`n" | Set-Content "$root/sources/capture-log.md"
Add-Type -AssemblyName System.Drawing
$rows=@('# 最终候选图索引','','此文件位于sources，供核验用，不是发布配图说明。所有宽度≤1400px。PDF渲染/拼接页码见capture-log.md。','','| 文件 | 宽×高 | SHA256 |','|---|---|---|')
foreach($f in Get-ChildItem "$root/images/*.png" | Sort-Object Name){$im=[Drawing.Image]::FromFile($f.FullName);$w=$im.Width;$h=$im.Height;$im.Dispose();if($w -gt 1400){throw "oversize $($f.Name)"};$rows+='| ['+$f.Name+'](../images/'+$f.Name+') | '+$w+'×'+$h+' | '+(Get-FileHash $f.FullName -Algorithm SHA256).Hash+' |'}
$rows|Set-Content "$root/sources/d-image-index.md"
$manifest=@('# 新增文件清单','','仅docs/1001/内文件；未执行的脚本、诊断失败图单独保留，不是已成功取证的凭据。','','| 文件 | 字节 | SHA256 |','|---|---:|---|')
foreach($f in Get-ChildItem $root -Recurse -File | Where-Object {$_.Name -ne 'd-file-manifest.md'} | Sort-Object FullName){$rel=$f.FullName.Substring($root.Length+1).Replace('\','/');$manifest+='| '+$rel+' | '+$f.Length+' | '+(Get-FileHash $f.FullName -Algorithm SHA256).Hash+' |'}
$manifest|Set-Content "$root/sources/d-file-manifest.md"
