$ErrorActionPreference='Stop'
$base=Split-Path $PSScriptRoot -Parent
function Stamp($name){(Get-Item "$PSScriptRoot/$name").LastWriteTimeUtc.AddHours(8).ToString('yyyy-MM-dd HH:mm:ss')+' UTC+8'}
$heat=@(
@('a-hn-search.json','HN 49881850','183 points / 133 comments（索引）'),
@('a-hn-item.json','HN 49881850','215 points / 147 comments（Firebase；标题 Sonnet 5.5）'),
@('b-hn-item.json','HN 49879079','3 points / 0 comments'),
@('b-hn-search.json','HN 49881803','4 points / 1 comment（Guardian 链接）'),
@('b-author-x.json','X 2104578598115885530','183 likes / 23609 views；未返回转发'),
@('b-author-thread.json','X 2104578598115885530','183 likes / 46 retweets；与 search 分开'),
@('b-reddit.json','Reddit 1wshicc','score 1；read 接口'),
@('b-reddit-dom-metrics.json','Reddit 1wshicc','score 3 / 74 comments；页面 DOM'),
@('c-x.json','X 2104251067039191324','1231 likes / 83 retweets；views 未返回'),
@('c-reddit.json','Reddit 1wrq71o','score 132；read'),
@('c-reddit-metrics.json','Reddit 1wrq71o','score 128 / 12 comments；search'),
@('c-opencode-search.json','Reddit 1wrtyfb','score 90 / 23 comments；search'),
@('c-before-reports.json','Reddit 1wrtyfb','score 96 / 23 comments；另一 search'),
@('c-reddit-metrics.json','Reddit 1woa5d3','score 96 / 85 comments；早期批评帖，不当作已核实复读复现'),
@('c-before-report-detail.json','Reddit 1woa5d3','score 95；正文接口有截断'),
@('d-reddit.json','Reddit 1wo4vzy','score 759；read'),
@('d-reddit-metrics.json','Reddit 1wo4vzy','score 762 / 103 comments；search'),
@('e-a-bilibili.txt','B站 BV1bZac6NEdd','510 likes / 36743 views；页面另显示3.7万')
)
$lines=@('','## 热度数据快照','','时间为该次本地存档完成时间（文件 UTC 时间换算），不是平台发布时间。','', '| 帖子 | 数值/接口 | 抓取完成时间（北京时间） | 原始存档 |','|---|---|---|---|')
foreach($h in $heat){$lines+="| $($h[1]) | $($h[2]) | $(Stamp $h[0]) | $($h[0]) |"}
Add-Content "$PSScriptRoot/evidence.md" ($lines -join "`n")
$urls=@{
'a-aa'='https://artificialanalysis.ai/models/claude-sonnet-5-5';'a-aa-tb'='https://artificialanalysis.ai/evaluations/terminalbench-4-0';'a-aa-home'='https://artificialanalysis.ai/';'a-hn'='https://news.ycombinator.com/item?id=49881850';'a-system'='https://www.anthropic.com/claude-sonnet-5-5-system-card';'a-'='https://www.anthropic.com/claude-sonnet-5-5';
'b-author'='https://x.com/_achan96_/status/2104578598115885530';'b-authors'='https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf';'b-paper'='https://casp.ac/__l5e/assets-v1/5efd4b41-deb5-4513-a0a3-b4f82d2b79ea/intelligence-explosion.pdf';'b-render'='https://casp.ac/reports/intelligence-explosion';'b-casp'='https://casp.ac/reports/intelligence-explosion';'b-axios'='https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion';'b-guardian'='https://www.theguardian.com/technology/2026/sep/28/ai-godfathers-warn-of-runaway-intelligence-explosion';'b-wsj'='https://www.wsj.com/tech/ai/top-ai-researchers-call-for-urgent-oversight-of-self-improving-systems-49bae9b4';'b-hn'='https://news.ycombinator.com/item?id=49879079';'b-reddit'='https://www.reddit.com/r/technology/comments/1wshicc/';
'c-mimo'='https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition';'c-font'='https://mimo.xiaomi.com/blog/mimo-v2-6-tool-call-repetition';'c-x'='https://x.com/XiaomiMiMoDevs/status/2104251067039191324';'c-reddit'='https://www.reddit.com/r/LocalLLaMA/comments/1wrq71o/';'c-opencode'='https://www.reddit.com/r/opencodeCLI/comments/1wrtyfb/';'c-before-report-detail'='https://www.reddit.com/r/LocalLLaMA/comments/1woa5d3/';'c-before-reports'='https://www.reddit.com/search/?q=mimo%20loop';
'd-inc'='https://www.inc.com/jason-aten/metas-new-muse-ai-agent-read-my-private-messages-i-never-asked-it-to/91408202';'d-meta'='https://about.fb.com/news/2026/09/introducing-muse-personal-ai-agent/';'d-download'='https://ai.meta.com/muse/download/';'d-9to5mac'='https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/';'d-response'='https://9to5mac.com/2026/09/28/yeah-dont-give-metas-muse-app-access-to-your-mac/';'d-singleton-bug'='https://www.threads.com/@davidsingleton/post/Ddml0eHEj3E';'d-singleton'='https://www.threads.com/share/JEVmarjjh/';'d-reddit'='https://www.reddit.com/r/privacy/comments/1wo4vzy/';
'e-a'='https://www.bilibili.com/video/BV1bZac6NEdd/';'e-b'='https://www.axios.com/2026/09/28/ai-pioneers-intelligence-explosion';'e-c'='https://chanmeng.org/newsletter/2026-09-28';'e-d'='https://www.ic.work/article/meta-muse-hits-mac-agent-privacy-dilemma'
}
function Url($name){foreach($k in ($urls.Keys | Sort-Object Length -Descending)){if($name.StartsWith($k)){return $urls[$k]}};return '本地整理；原始URL见关联存档'}
$log=@('# 0928 抓取日志','','北京时间 UTC+8。逐文件时间采用写入完成时的 LastWriteTimeUtc + 8；它是抓取/处理完成近似时间，不冒充请求发起时间。curl 请求开始时间、HTTP状态与重定向另见 http-capture.tsv。派生图记录加工时间，原始截图时间另列。','','## 工具与失败边界','','- opencli 1.8.7；已执行 browser --help、doctor；浏览器扩展连接成功。agent-reach doctor --json 后按可用 OpenCLI 后端访问 X/Reddit/B站。普通网页用 curl.exe 与 opencli browser；PDF 用 pdftotext / pdftoppm，图像用 PowerShell System.Drawing 1:1 裁剪拼接。未安装软件、未登录、未输入凭据、未接受非必要 Cookie。','- Inc 和 Axios 的 curl HTML 返回403，保留响应体；同一原站浏览器可读，另存逐字 Markdown 与浏览器 HTML。Inc 点击原页面 Expand to continue reading 后保存 expanded 版本；前一版本不冒充全文。','- 两个 Threads 原链接均仅见登录提示，失败；高管回应只能由 Inc 内嵌原图转录，单独标媒体截图。WSJ 仅抓到标题、日期、导语和开头，未取到全文。','- Anthropic full-page 截图发生视口高度/布局重排，出现黑区或错位；a-sonnet-full*.png 及 a-footnotes*、a-availability*、a-end-crop、a-crop-test 等保留为未采用尝试。最终用可见视口截图裁剪；02采用 a-safeguards-direct 和 a-notes-good。','- b-reddit-metrics.json 搜索没有命中目标，不用于目标热度；后续 b-reddit-dom-metrics.json 从目标帖 DOM 取得 score/comments。c-before-report-detail.json 正文被接口截断，不作为全文。','- a-font-audit.json / c-font-audit.json 返回空数组（会话页面已变化），不构成字号验收。d-9to5mac-time.json 未取得时间，使用可见页头。PDF字体提取有乱码，作者名按原页图转录；render-errors 文件保留渲染警告。','- Python 调用曾被自动审批机制拒绝：approval required by policy, but AskForApproval is set to Never。改用现有 PowerShell/.NET 完成，没有申请安装或绕过审批。','','## 截图规则与未满足项','','14张PNG全部为原站/原PDF裁剪，未重绘、改字、调色、缩放；拼接线为2px灰线，空白补齐宽度。精确像素坐标在 crop-captures.ps1。浏览器默认100%缩放；浅色正文，但 Anthropic 标题区沿用原站深色底，未强制改站点样式。MiMo表格原生小字部分可能低于14px，本轮未能证明所有引文字高均≥14px，因此保留为候选图，不声称图像规格全部验收。Inc标题在原页面有自然重叠，截图忠实保留。CASP网页无日级日期，03不补造；PDF封面只有September2026。','','## 全部源文件','','“成功/原始返回”仅表示取得文件；全文性及失败响应按上方和 evidence.md 判断。HTML外链资源未打包，不能保证离线复原外观；可读Markdown另存。原文Markdown可能带浏览器提取的结构标记，不是改写正文。','','| 文件 | URL | 完成时间（北京） | 工具/结果 |','|---|---|---|---|')
foreach($f in (Get-ChildItem $PSScriptRoot -File | Sort-Object Name)){
 $tool=switch($f.Extension){'.html'{'curl / browser HTML；状态见 http-capture.tsv'} '.pdf'{'curl 原PDF；成功'} '.png'{'opencli browser screenshot；PDF命名者为pdftoppm；原始/尝试图'} '.webp'{'curl 原站嵌图；成功'} '.json'{'opencli extract/eval/social；HN为curl API；原始返回'} '.ps1'{'本地处理脚本'} default{'浏览器逐字提取/PDF文本或本地记录；按文件名区分'}}
 $log+="| $($f.Name) | $(Url $f.Name) | $(Stamp $f.Name) | $tool |"
}
$log+=@('','## 候选截图与裁剪/拼接','','下表源文件名可回溯上表URL与原始抓取时间；每行对应且仅对应images内一张PNG。','', '| PNG | 来源文件与裁剪坐标(x,y,w,h) | 加工时间（北京） | URL |','|---|---|---|---|')
foreach($line in (Get-Content "$PSScriptRoot/crop-captures.ps1" | Where-Object {$_ -like 'Join-Crops *'})){
 $name=[regex]::Match($line,"Join-Crops '([^']+)'").Groups[1].Value
 $src=[regex]::Match($line,"sources/([^']+)'").Groups[1].Value
 $when=(Get-Item "$base/images/$name").LastWriteTimeUtc.AddHours(8).ToString('yyyy-MM-dd HH:mm:ss')
 $log+="| $name | $($line.Substring($line.IndexOf('@')))；多段时竖拼2px分隔 | $when UTC+8 | $(Url $src) |"
}
Set-Content "$PSScriptRoot/capture-log.md" ($log -join "`n")
