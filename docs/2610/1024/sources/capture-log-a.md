# A 组抓取日志（1024 期，Kimi K3.1 Pro 宣传风波）

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8）。机器记录另见 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json`。本机时区美东（EDT）：北京 = 本机 +12 小时。

## 一、barking fetch 批量存档（`a-<name>.html|.txt|.headers`）

| 时间 | name | URL | 结果 |
|---|---|---|---|
| 10-25 23:29 | kimi-blog | https://kimi.ai/blog/kimi-k3-1-pro | ok（跳转至 www.kimi.ai/blog/…）|
| 同 | venturebeat | https://venturebeat.com/technology/moonshots-kimi-k3-1-pro-flirts-with-open-weights-leadership-amid-paid-influencer-scandal-allegations | ok |
| 同 | pekingnology | https://www.pekingnology.com/p/kimis-global-momentand-a-very-chinese | ok |
| 同 | ifeng | https://tech.ifeng.com/c/8xUJ5Olz7tc | ok |
| 同 | kr36 | https://www.36kr.com/p/4039236862514306 | ok |
| 同 | smzdm | https://post.smzdm.com/p/a82pd23q/ | ok |
| 同 | hf-moonshot | https://huggingface.co/moonshotai | ok |
| 同 | hf-api | https://huggingface.co/api/models?author=moonshotai&sort=lastModified&limit=20 | ok（JSON）|
| 同 | aa-model | https://artificialanalysis.ai/models/kimi-k3-1-pro | ok（HTML 4.4 MB，仅本地）|
| 同 | ftc-faq | https://www.ftc.gov/business-guidance/resources/ftcs-endorsement-guides-what-people-are-asking | failed：404 |
| 同 | ecfr255 | https://www.ecfr.gov/current/title-16/chapter-I/subchapter-B/part-255 | ok（整章；不含 255.5 正文）|
| 同 | xpaid | https://help.x.com/en/rules-and-policies/paid-partnerships-policy | blocked：Cloudflare 质询 403 → 改用 opencli 浏览器读文本（见下）|
| 同 | tripo-models | https://platform.tripo3d.ai/docs/model | blocked：正文过短（需 JS）|
| 10-26 00:00 前后 | arena-blog | https://arena.ai/blog | ok（无 Kimi 相关）|
| 同 | arena-changelog | https://arena.ai/changelog | blocked（JS）；opencli 打开为 404 页 |
| 同 | arena-agent-lb | https://arena.ai/leaderboard/agent | blocked：Cloudflare 质询 403 → 改用 opencli 浏览器 |
| 00:08 | aa-opus55 / aa-astra / aa-cmp / aa-article | https://artificialanalysis.ai/models/claude-opus-5-5 、/models/gpt-6-astra 、/models/comparisons/kimi-k3-1-pro-vs-claude-opus-5-5 、/articles/kimi-k3-1-pro-launch-benchmarks | 均 ok（前三者 HTML 1.5–1.7 MB，仅本地）|
| 同 | kimi-pricing | https://platform.kimi.ai/docs/pricing/chat | ok（无 5 小时额度信息）|
| 同 | kimi-member | https://www.kimi.com/membership/pricing | blocked：需 JS（改用 opencli 浏览器读公开套餐文本，未存档）|
| 同 | infoq-search | https://www.infoq.cn/search?q=Kimi%20K3.1 | blocked（JS），未找到 infoq.cn 首发页 URL |
| 同 | latent | https://latent.space/p/ainews-kimi-31/ | ok（日期 Oct 24；仅存档，未精读）|
| 同 | tripo-docs | https://platform.tripo3d.ai/docs/generation | blocked（JS）|
| 00:09 | kimi-plans | https://platform.kimi.ai/docs/guide/product-plans | ok（无额度数字）|
| 同 | kimi-code-docs | https://www.kimi.com/code/docs/en/ | ok |
| 同 | kimi-status | https://x.com/KimiDevs | ok（匿名未登录页，无抓取者信息；内容仅简介）|
| 00:10 | kimi-code-member | https://www.kimi.com/code/docs/en/kimi-code/membership.html | ok |
| 00:1x | ftc-disclosures | https://www.ftc.gov/business-guidance/resources/disclosures-101-social-media-influencers | failed：404（VentureBeat 引用的链接）|
| 00:1x | ecfr2555 | https://www.ecfr.gov/current/title-16/section-255.5 | ok |

## 二、X / Reddit / 其他（opencli 与 curl）

| 时间 | 对象 | 工具 | 结果 |
|---|---|---|---|
| 10-25 23:27 | api.fxtwitter.com/status/<id>：2113710752741892476、2113903850641842589、2113940886480454040、2113963710389592503、2113998747730383067、2113801452783284724、2113680701770068067 | curl（60 s）| 全部 200，存 `a-fx-<id>.json`（含公开作者信息，不含抓取者）|
| 同 | 同上补取：2113734602032312731（Post 2）、2113751173476049138（Post 3）、2113849838731210836、2113682513197514888、2113695782003950040、2113923642983690743（Hesamation）、2113725151556837499（LLMJunky）、2113872950214438955（kimmonismus）、2113910532876955680、2114022983387791600 | curl | 200 |
| 同 | pbs.twimg.com 配图原件（HVVouYUW4AAjkex、HVYYWGKXsAAhiOc、HVVMjBoaIAAZncD、HVZuo8MWQAAyETA、HVZupZYWgAEWDKb、HVZup0rXAAA40wp、HVW6wRYa8AAHnsV、HVW6w24aIAAs4NR、HVW6y33bEAACCgn、HVV-albWUAAlSfj、HVWNfFJW0AAiZ3x）| curl | 200，存 `a-media-<id>.jpg`（本地，不入库）。HVW6* 三张是 @0xLogicrw 转述帖里的中文翻译截图，**带 X 翻译界面，仅供阅读，不作候选配图**；HVVMjBoaIAAZncD 为官方发布帖的基准图 |
| 23:3x | `opencli twitter tweets elshayib_ --limit 60` | opencli | 成功，存 `a-elshayib-timeline-opencli.json`（含他的转推原文片段）|
| 00:1x | `opencli twitter tweets Kimi_Moonshot --limit 15`、`KimiDevs --limit 10` | opencli | 成功，存 `a-kimi-moonshot-timeline-opencli.json`、`a-kimidevs-timeline-opencli.json` |
| 00:1x | `opencli twitter search`（5 组关键词）| opencli | 成功，汇总 `a-twitter-search-opencli.txt` |
| 23:4x | `opencli reddit search "Kimi K3.1 model3d_generation"`、`"Kimi K3.1 Tripo"`；`opencli reddit read 1xe7cbr --limit 15` | opencli | 成功，存 `a-reddit-search-tripo.json`、`a-reddit-1xe7cbr-comments.json`；直接 curl reddit.com 被 “Blocked” 页拦截，barking heat 的 Reddit 搜索被登录墙拦截（1 个请求失败，已记入 a-heat.json）|
| 00:16 | `barking heat 2610/1024 a --hn …` | barking | HN 2 组成功（见 evidence 第 15 条），Reddit 失败 |
| 00:10 | https://huggingface.co/api/models?author=moonshotai&sort=lastModified&limit=8 | curl | 200，存 `a-hf-api-2026-10-26T0010.json`；`moonshotai/Kimi-K3.1-Pro` 页面 HTTP 401 |
| 00:0x | https://help.x.com/en/rules-and-policies/paid-partnerships-policy | opencli browser（背景窗口）| 成功，文本存 `a-xpaid-policy-opencli.txt` |
| 00:0x | https://arena.ai/leaderboard/agent | opencli browser | 第一次停在 Cloudflare 验证页（“正在进行安全验证”），**未做任何绕过**，再等待/重开后页面自行通过；文本存 `a-arena-agent-lb-opencli.txt`；之后在后台窗口截图 `Page.captureScreenshot` 多次超时，改前台窗口 1 次成功（04 号图），“View all”展开后的截图 2 次超时失败 |
| 00:1x | https://platform.tripo3d.ai/docs/model（opencli）| opencli browser | 页面无 model 表（只有“Tripo API V2 is being retired”横幅与侧栏）|
| 00:1x | https://www.kimi.com/membership/pricing | opencli browser | 只读页面文本做核对，未存档、未点击 |

## 三、截图（`images/`）

| 文件 | 来源 | 工具与步骤 | 备注 |
|---|---|---|---|
| 01-kimi-brief-raw.png | https://platform.twitter.com/embed/Tweet.html?id=2113710752741892476&theme=light | opencli browser（背景窗口）打开 X 官方匿名嵌入页，页面 zoom 2，截图 1400 px 宽，再 `barking shot cropbox` 裁 [0,0,1100,1796] | 卡片本身；不含抓取者信息（嵌入页为匿名）；原始截图在 `%TEMP%\a-tw1.png` |
| 02-kimi-post4-raw.png | 同上 id=2113903850641842589 | 同上，裁 [0,0,1100,1282] | |
| 03-kimi-official-raw.png | 同上 id=2113680701770068067（加 `&lang=en&dnt=true` 才渲染出内容）| 同上，裁 [0,0,1100,1848] | |
| 04-arena-agent-raw.png | https://arena.ai/leaderboard/agent（Arena Agent 榜 Overall，页面日期 Oct 23, 2026；截取时刻北京 2026-10-26 00:26–00:30）| opencli browser（前台窗口）打开，点“Hide Filters”，页面 zoom 2（700 CSS px 版式，2× 像素），把表格容器横向滚动到三个位置各截一张 1400 px 宽图：Net Improvement 列、Sessions 列、Cost/Task (P50) 列（粘性 Rank/Model 列常驻，一次只能露出一列）；每张裁 [170,570,1310,1940]（表头+第 1–12 名），页头（标题、日期、会话数、模型数）裁 [170,150,1310,420]；用 `barking shot stack` 竖拼四段（gap 24）成 1140×4452。重截版（2×）覆盖了我自己先前的 1× 版（mv 覆盖，旧版无残留）| 第 10 名 Claude Fable 5 (High) 与 Kimi K3.1 Pro (Max)（第 11，Preliminary）两行均在三段里：Net Improvement 7.75%/7.61%，Sessions 43,567/4,071，Cost/Task $3.45/$1.22。局限：Net Improvement 表头被截成“Improvement”（左半“Net”被粘性列压住）；Cost/Task 表头为“ost/Task (P50)”；各列数字完整可读。三段来自同一页面同一次加载（只滚动未刷新）；opencli 后台窗口截图多次超时，故用前台窗口 |
| 05-aa-cost-raw.png | https://artificialanalysis.ai/models/comparisons/kimi-k3-1-pro-vs-claude-opus-5-5-vs-gpt-6-astra（AA 三方对比页，标题 “Kimi K3.1 Pro, Claude Opus 5.5 (Max, Default Fallback), and GPT-6 Astra (Max)”；渲染时刻北京 2026-10-26 00:23）| `barking shot render`（headless Chrome，700 CSS px、DPR 2，高 6000，页高 12000），再 `stack` 竖拼表头条 [0,1120,1400,1345] 与 Cost+Token Use 段 [0,2340,1400,3470]，gap 16，成 1400×1371；覆盖了我自己先前仅含 Kimi/Opus 两列的旧版（mv 覆盖）| 三列同框：Cost per Task $2.11（Kimi）/ $5.98（Opus 5.5）/ $3.26（Astra）；Cost to Run Intelligence Index $3,117 / $8,708 / $5,324；Output Tokens per Task 55k / 119k / 27k；Output Tokens to Run 148M / 260M / 60M；Price per 1M Tokens $1.54 / $2.94 / $7.70。表头无遮挡；Opus 列名在表头被页面缩写为 “Claude Opus 5.5 (Max, Default…)”（全名见页面标题）；Astra 列贴近图右缘但数字完整 |
| 06-kimi-3d-raw.png | 同 01，id=2113998747730383067 | opencli，裁 [0,0,1100,1105] | |
| 07–12 | — | 未做 | |

逐张目视确认：01–06 均已打开核对。06 是 X 嵌入页把三张附图拼接并裁切的结果，文字不完整，只适合作“帖子存在”的证明；读 Reddit 内容请用 sources 里的原图 a-media-HVZup0rXAAA40wp.jpg。所有 X 卡片中显示的时间为 EDT。

## 四、一次性脚本与试验文件

- `sources/a-embed-shot.sh`：包装 opencli 嵌入页截图（一次性，实际后续多数为手动调用）。
- `%TEMP%` 下的中间图（`a-tw1.png`、`a-tw4.png`、`a-tw-official.png`、`a-tw-reddit.png`、`a-arena1.png`、`a-arena2.png`，以及 `…\barking-shot\26101024-a\{tw1,aacmp}.png`）不在仓库内。
- opencli 浏览器会话（`a-shot`、`a-arena`、`a-xp`、`a-mem`、`a-tripo`）留有已打开标签页，未关闭。

## 五、待删除文件（按协调者要求，本组未调用 rm）

- `E:\gitclone\AI-Barking\.claude_tmp_none`（空文件，误建于仓库根目录；**不在 docs/2610/1024/ 下**，务必删除，否则 `git status` 会多出一项）
- `E:\gitclone\AI-Barking\docs\2610\1024\sources\a-embed-shot.sh`（一次性脚本；若要保留作为抓取脚本可不删）
- `E:\gitclone\AI-Barking\docs\2610\1024\sources\a-fx-2113695782003950040.json`（Dario 帖，与本组无关的探查）
- `E:\gitclone\AI-Barking\docs\2610\1024\sources\a-media-HVW6wRYa8AAHnsV.jpg`、`a-media-HVW6w24aIAAs4NR.jpg`、`a-media-HVW6y33bEAACCgn.jpg`（第三方中文翻译截图，含 X 翻译界面，仅读过，建议不留）
- `E:\gitclone\AI-Barking\docs\2610\1024\sources\a-kimi-status.html`、`a-kimi-status.txt`、`a-kimi-status.headers`（/KimiDevs 匿名页，信息已被 opencli 时间线取代）

## 六、泄露自查

- 对 `docs/2610/1024/sources/a-*` 与 `evidence-a.md` grep “develata”“C:\Users\QQ”：0 命中。
- fxtwitter / opencli 输出只含被抓取帖的公开作者字段，无抓取者信息；X 卡片截图来自匿名嵌入页（显示“Follow”按钮，非登录态）。
- 大于 1 MB 的文件（仅 HTML 原件，本地不入库）：`a-aa-model.html` 4.4 MB、`a-aa-astra.html` 1.6 MB、`a-aa-opus55.html` 1.7 MB、`a-aa-cmp.html` 1.5 MB；所有 .txt/.json/.md 均 < 1 MB。

## 七、补截（04、05 重截）追加的待删除文件

- 检查用中间图，误写进 images/ 目录，必须删，否则会被当作成品：`E:\gitclone\AI-Barking\docs\2610\1024\images\a-chk-net.png`、`a-chk-cost.png`、`a-chk-net2.png`、`a-chk-cost2.png`（均为我生成）
- 仓库外：`C:\Users\QQ\AppData\Local\Temp$2`、`C:\Users\QQ\AppData\Local\Temp$n.png`（脚本引号写错，截图被写进 AppData\Local 下的这两个文件）
