# 1025 期 B 组抓取记录

执行方 claude-sonnet-5-5。时间为北京时间（UTC+8），本机美东（EDT），北京 = 本机 + 12 小时。机器记录另见 `b-fetch-log.jsonl`、`b-shots-log.jsonl`、`b-heat.json`。

## 一、网页与 PDF 存档（`barking fetch`，前缀 b）

| 北京时间 | name | URL | 结果 |
|---|---|---|---|
| 20:34 | code-member-en / code-member-zh | https://www.kimi.com/code/docs/en/kimi-code/membership.html ；…/code/docs/kimi-code/membership.html | ok |
| 同 | code-whatsnew(-zh)、code-errref、code-faq、code-agents、code-tower、code-models | https://www.kimi.com/code/docs/… | ok |
| 同 | kimi-code-page | https://www.kimi.ai/code/ | ok（套餐卡片 JS 渲染，txt 无价格） |
| 同 | kimi-blog | https://www.kimi.ai/blog/kimi-k3-1-pro | ok |
| 同 | kimi-quickstart | https://platform.kimi.ai/docs/guide/kimi-k3-1-pro-quickstart | ok（首次连接失败，重试成功） |
| 同 | kimi-membership | https://www.kimi.com/membership/pricing | blocked（需 JS，正文过短）→ 改 opencli |
| 同 | kimi-reuters | https://www.reuters.com/world/asia-pacific/moonshot-resets-subscriber-quotas-after-kimi-k31-pro-launch-issues-2026-10-26/ | blocked（HTTP 401），未绕 |
| 20:41 | 21jingji、thepaper、sina-finance、sohu、sina-weibo-1…4、yahoo-reuters、tradingview-reuters、deccan、panews、chaincatcher | 见 `b-fetch-log.jsonl` | 全部 ok；tradingview-reuters 只有标题与付费墙提示；sina-weibo-1…4 是新浪新闻聚合页，只用其中与本事件相关的几条 |
| 20:46 | qq-news、sohu-zuiqiang、sohu-shoudao、sohu-zuixin、ifeng、zhongxin、sina-opinion、weibo-detail、21jingji-2 | 同上 | 全部 ok |
| 21:18 | sina-weibo-5、biggo | 同上 | ok |

清单文件：`b-list1.tsv`、`b-list2.tsv`、`b-list3.tsv`、`b-list4.tsv`。失败/重试：fetch 过程中 21jingji、sohu、chaincatcher、kimi-quickstart 首次连接失败，由工具自动重试成功。

## 二、其他请求

| 北京时间 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 20:33 | https://api.fxtwitter.com/status/2114450610724708492 | curl | 200，存 `b-fx-2114450610724708492.json`（只含官方账号公开字段） |
| 21:0x | fxtwitter：@KimiDevs 四条（2114327830465237001、2114327832646004990、2114327835930472778、2114327838140592411） | curl | 200，存 `b-fx-<id>.json` |
| 20:4x | https://www.kimi.ai/membership/pricing 、https://www.kimi.com/membership/pricing | opencli browser（背景窗口，只读页面文本，未点击订阅、未登录） | 文本存 `b-kimiai-pricing-opencli.txt`、`b-kimicom-pricing-opencli.txt`；页面界面为中文（浏览器语言）；浏览器是否带登录态未知，页面未显示账号信息 |
| 20:5x | https://weibo.com/u/7909392214 （月之暗面Kimi） ；`opencli weibo search/user/user-posts`；m.weibo.cn container API | opencli / curl | `weibo user` 取到公开资料；帖子列表需要登录（页面“前方有点拥堵，请登录后使用”，`opencli weibo whoami` 报 AUTH_REQUIRED），**未登录、未绕**，官方微博原帖读不到 |
| 21:0x | WebSearch：中文/英文关键词（额度补偿、致歉、夸大类） | WebSearch | 用于找线索与媒体；未把搜索摘要当来源 |
| 21:04 | `barking heat`（HN 条目 50222093；Reddit 两次搜索） | barking | HN 成功，`b-hn-50222093.json`、`b-heat.json`；Reddit 搜索两次均被重定向到登录页（非 JSON），记入 `b-heat.json` errors，改用 opencli |
| 21:0x | https://hn.algolia.com/api/v1/items/50222093 | curl | 200，存 `b-algolia-item-50222093.json`（含评论时间戳） |
| 21:0x | `opencli reddit subreddit kimi --sort new --limit 40`、`reddit search`、`reddit read 1xerert / 1xflkq6` | opencli | 成功，存 `b-reddit-kimi-new-opencli.json`（仅公开帖子，无抓取者信息）；评论正文只摘录进 evidence，未另存 |
| 21:0x | `opencli twitter thread 2114450610724708492`（回复）、`opencli twitter tweets Kimi_Moonshot / KimiDevs --limit 25` | opencli | 成功；回复只保留他人公开回复字段，存 `b-x-replies-opencli.json`（不含抓取者信息）；两个时间线 JSON 只放在临时目录，未入库 |

## 三、截图（`images/`）

| 文件 | 来源 | 方法 | 备注 |
|---|---|---|---|
| 13-kimi-apology-card-raw.png | https://platform.twitter.com/embed/Tweet.html?id=2114450610724708492 | `b-embed-shot.sh`（opencli 背景窗口，页面 zoom 2，1400 宽）再 `barking shot cropbox` [0,0,1100,740] | X 官方匿名嵌入卡，显示“Follow”按钮（非登录态）。**嵌入卡把长帖截断（“(usage below Show more”）**；“Show more”是跳转链接，无法在嵌入里展开。尝试过用 opencli 打开 x.com 原帖页截全文：浏览器为登录态且自动翻译成中文、左侧栏带账号信息，**不合格，未采用**；匿名 headless Chrome 打开 x.com 帖子页失败（无输出）。全文请用 fxtwitter JSON + 14 号图。2× 像素，1100×740 |
| 24-kimi-apology-fulltext-raw.png | https://x.com/Kimi_Moonshot/status/2114450610724708492 | opencli 背景窗口打开，点“显示原文”，页面 zoom 1.4，截图后 `barking shot cropbox` [458,82,1272,775] | 补 13 号图被嵌入卡截断的问题。浏览器为登录态：裁切只含帖子正文与互动计数（372 回复/452 转/4,828 赞/357 书签/53.7万 查看），已打开确认不含侧栏导航、账号头像/昵称、回复框；约 1.4× 像素（宽 814）。页面界面文字为中文（“显示翻译”标签、时间“下午4:14·2026年10月25日”按浏览器本地时区） |
| 14-kimi-cn-notice-raw.png | https://tech.ifeng.com/c/8xWrSDBLrWR | opencli 背景窗口 zoom 2，1400 宽，裁 [0,470,1290,2090] | IT之家转录的官方中文公告全文（页面标题、IT之家署名、2026年10月26日 07:17:51）。页面截图里右侧有浏览器翻译插件图标，已被裁掉；原图下方的官方通知截图（Kimi智能助手帖）未截入。来源是转录页，不是官方原页 |
| 15-kimicode-quota-rules-raw.png | https://www.kimi.com/code/docs/en/kimi-code/membership.html | `barking shot render`（700 CSS px、DPR 2）后 `stack`：公告块 [0,380,1400,830] + Quota and Limits 段 [0,3380,1400,4470] | 中间被略去的是 Overview 与 Membership Tiers 表（表在窄屏里被横向截断）；套餐对照表在 `b-code-member-en.txt` |
| 16-kimicode-subagents-raw.png | https://www.kimi.com/code/docs/en/kimi-code-cli/customization/agents.html | `barking shot render` + `crop --start "Sub-agents are scheduled automatically" --end "tighten the corresponding"` | 完整保留“Context Isolation and Resource Cost”“Permission Inheritance”两节；第一次裁切下沿落在文字行内，已改锚点重裁，旧版移到临时目录（见待删除清单） |
| 17-kimiai-pricing-raw.png | https://www.kimi.ai/membership/pricing | opencli 背景窗口，**不加 zoom（1× 像素）**，1400 宽，裁 [0,0,1400,1330] | 700 px 宽下是手机轮播版式，只显示两张卡，故用桌面版式；像素比 1×，低于规范的 2×。海外价目表，Plus/Pro/Max/Ultra（美元），界面中文，包年折算价与原价同显 |
| 18-kimiai-plan-table-raw.png | 同上（滚动到“订阅计划”对比表） | 同上，裁 [0,0,1400,1495] | 含“集群功能运行子任务数”（Plus 2、Pro 4、Max 8、Ultra 8）“Agent 任务并行”等行；1× 像素 |
| 19-kimicom-pricing-raw.png | https://www.kimi.com/membership/pricing | 同上，裁 [0,0,1400,1330] | 国内价目表（人民币，Go/Plus/Pro/Max，Ultra 卡片被轮播右缘截断，完整文本见 `b-kimicom-pricing-opencli.txt`）；1× 像素 |
| 20-kimidevs-subagent-opt-raw.png | https://platform.twitter.com/embed/Tweet.html?id=2114327830465237001 | 同 13，裁 [0,0,1100,550] | @KimiDevs 首帖，923 赞 |
| 21-kimidevs-cli232-swarm-raw.png | …id=2114327835930472778 | 同 13，裁 [0,0,1100,955] | 嵌入卡里同时带着父帖（CLI 2.3.2）与本帖（Swarm 仅开启 swarm 模式时启动）；2× |
| 22-kimidevs-desktop113-raw.png | …id=2114327838140592411 | 同 13，裁 [0,0,1100,1068] | Desktop v1.1.3；带父帖上下文；2× |
| 23-hn-kimicode-5h-limit-raw.png | https://news.ycombinator.com/item?id=50222093 | `barking shot render`（DPR 2），裁 [0,80,1400,2200] | 匿名页，含发帖作者的竞品披露与两条回复；“add comment”空输入框在图中，无个人信息 |

**未采用**（第一次 x.com 截图为翻译态且带侧栏账号信息，已弃；第二次点“显示原文”并裁切后得 24 号）：Reddit 帖子页截图（opencli 浏览器为登录态：中文翻译叠加、左栏显示“最近访问”社区），已弃，Reddit 只以 opencli 文本为据；b-x-page 截图同理弃用。两张弃图只在临时目录（scratchpad），不在仓库内。

目视核对：13、14、15、16、19、20（卡片）、21、22、23 已逐张打开确认文字清楚、范围正确；17、18 是对已目视的整页截图的矩形裁切（17 的底边在卡片下沿之后，18 含完整表格）。

## 四、自查（隐私）

- fxtwitter / opencli 输出只含被抓取帖的公开作者字段，无抓取者信息；X 卡片来自匿名嵌入页。
- `b-x-replies-opencli.json` 为他人公开回复（账号名为公开 handle），已确认不含抓取者账号；对 `docs/2610/1025/` 以抓取者在屏幕里出现过的 handle 做过 grep，b 组文件无命中（其他组文件里的普通英文词命中与本检查无关）。
- 官方微博、公众号、小红书原帖均未登录读取；定价页读的是公开文本。

## 五、本组新增脚本（`sources/`）

- `b-embed-shot.sh`：包装 opencli 的 X 匿名嵌入页截图。
- `b-shotpage.sh`：opencli 打开网页、zoom 2 截图（实际部分页面改为 1×，见上）。

## 六、待删除文件清单（供 Claude 收尾一次删除）

- 无仓库内待删文件（试验图都在临时目录）。
- 仓库外临时文件：`C:\Users\QQ\AppData\Local\Temp\claude\E--gitclone-AI-Barking\fe074772-c75d-4f5a-a7fa-06e50cbe7378\scratchpad\` 下的 `b-*.png`、`old16.png`、`kimicode.png`（可留可删）；`C:\Users\QQ\AppData\Local\Temp\b-thread.json`、`b-tl-Kimi_Moonshot.json`、`b-tl-KimiDevs.json`（含登录态读取的时间线，建议删）。
- opencli 浏览器会话（`bshot`、`b-shot`、`b-s`）留有已打开标签页，未关闭。
