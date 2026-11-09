# 1108 期 B 组（高盛收回 Claude Code）抓取与截图日志

时间一律北京时间（本机 UTC+8），2026-11-09。工具：`barking fetch`（curl 类批量存档，限时 60/180 秒）、`opencli browser`（会话 `b-shot`，后台窗口，未登录任何站点，未点击任何同意/订阅/登录按钮）、`barking heat`、Exa 网页搜索（只用于找线索，搜索摘要不作为存档）。各清单文件：`b-fetch-list-1.tsv` … `b-fetch-list-7.tsv`；批量结果见 `b-fetch-log.jsonl`；截图动作见 `b-shots-log.jsonl`；截图脚本 `b-shot.sh`。

## 一、barking fetch 批次

| 北京时间 | 清单 | 条目与结果 |
|---|---|---|
| 18:06:25 | list-1 | bbg-original（彭博原页，200，付费墙，仅导语）、bbglaw（Bloomberg Law，200，仅公开导语）、yahoo-edge（Yahoo 新加坡/The Edge 转载，200）、fa-mag（FA Magazine 彭博转载，200，**全文**）、pymnts（200）、decoder（200）、stateofai（200）、tnw-argenti（200）、iposcoop（200）；webpronews **blocked**（403 Cloudflare 质询，未绕过） |
| 18:07:20 | list-2 | sec-s1a1 **blocked**（403，SEC 要求声明身份的 User-Agent，未伪装；改用 opencli）、anthropic-s1-news（200）、bbglaw-ipo（Bloomberg Law 承销商报道，200，仅导语）、cnbc-goldman-anthropic（200）、pymnts-bny（200）、qz **blocked**（403 Cloudflare）、finextra **blocked**（403 Cloudflare）、fstech（200）、bbg-oddlots（Bloomberg 节目页，200，仅简介） |
| 18:16:55 | list-3 | bny-1q26-deck（PDF，200，19 页）、bny-aibank（与 pymnts-bny 为同一 URL，重复存档，见待删除） |
| 18:17:29 | list-4 | anthropic-barclays、thestack-news、toi、lavx、financefeeds、econotimes、cellcog、cls-cn、hkyahoo-cn 均 200；ndtv-profit **blocked**（403）、wscn-cn **blocked**（正文过短，需 JS） |
| 18:17:48 | list-5 | gs-waldron（高盛官网 Exchanges 访谈页，200）、bi-jpm-devspace（Business Insider，200） |
| 18:18:29 | list-6 | icwork-cn、winzheng-cn、ainvest、airtracker 均 200；youtube-oddlots（200，但 txt 仅 7 行，没有字幕正文，字幕只在 Exa 搜索摘要里见到，未存档） |
| 18:19:23 | list-7 | newsfilter-raise（RAISE Summit 摘要，200）、raise-linkedin（LinkedIn 公开帖，200，无需登录；仅含活动方与公司/记者的公开发言）、wavebyvento（官网，200，无高盛条目）、awsstack-claude（State of AI Barclays 文，200） |

`ok` 条目已逐个核对 txt 是正文（抽查 `b-fa-mag.txt`、`b-bbglaw.txt`、`b-bbg-original.txt`、`b-tnw-argenti.txt`、`b-cnbc-goldman-anthropic.txt`、`b-bi-jpm-devspace.txt`、`b-gs-waldron.txt`、`b-toi.txt`、`b-bny-1q26-deck.txt`）。`b-bbg-original.*` 与 `b-bbglaw.*` 是付费墙页，只含导语。

## 二、opencli 浏览器（截图与 DOM 摘录）

| 北京时间 | 页面 | 结果 |
|---|---|---|
| 约 18:09–18:10 | news.bloomberglaw.com 导语页 | 打开成功；页面有“已移除广告”占位与 Cookie 横幅，**只用 CSS 隐藏浮层，未点击 Options/Continue**；截图 2800 px 宽（zoom 2，= 1400 CSS px，页面版式需要 >700 CSS px，否则右侧被截）；页面显示时间为浏览器本地化的 “Nov. 2, 2026, 9:52 PM GMT+8”。裁出 `13-bbglaw-lede.png` |
| 约 18:10 | www.bloomberg.com 原页 | 打开成功，付费墙（Subscribe 横幅在下）；裁出 `14-bbg-original-lede.png`（标题、EXCLUSIVE 标签、署名、时间 “November 2, 2026 at 9:52 PM GMT+8”、被淡出的首段）。未使用 Gift 链接 |
| 约 18:11 | sg.finance.yahoo.com（The Edge 转载） | 打开成功但右栏与图片为加载占位、版面有大块空白，不适合作配图，只用于文本存档；`tmp-yahoo.png` 作废 |
| 约 18:12 | fa-mag.com 彭博转载 | 首次截图被广告弹窗遮罩变暗；隐藏全部 position:fixed 浮层后重截；裁出 `15-famag-bbg-reprint-full.png`（标题、日期、署名、全文至 “This article was provided by Bloomberg News.”）。页面为无样式排版，字小但 2800 px 宽可读 |
| 约 18:13 | www.sec.gov S-1/A-1 | opencli 可打开（页面约 1.58 MB 文本）；裁出 `16-s1a-cover-underwriters.png`（封面至承销商排位）与 `17-s1a-title-page.png`（“As filed … November 2, 2026. Registration No. 333-299633”）；承销节与封面段落用 DOM textContent 摘录存 `b-sec-s1a1-excerpt.txt`。封面最上方的 Anthropic 黑色页眉被截掉一部分（原页该处即页首边缘） |
| 约 18:14 | thenextweb.com Argenti 稿 | 首次被 Cookie 遮罩变暗，隐藏浮层后重截；裁出 `19-tnw-argenti-headline.png`（标题、导语、日期 October 9, 2026 - 6:09 pm）与 `18-tnw-argenti-first-wave.png`（“Three waves” 第一段）；页尾 “Published October 9, 2026 - 6:09 pm UTC”。第一次按文字滚动时命中了页面下方位置，已重截，中间图 `tmp-tnw2.png` 为中间产物 |
| 约 18:14 | cnbc.com 2026-02-06 稿 | 裁出 `20-cnbc-goldman-anthropic-embedded.png`（标题、日期、Key points、六个月段）。第一版右侧截断了标题，已作废重裁（见待删除） |
| 约 18:14 | news.ycombinator.com/item?id=50415628 | 未登录（页头 “login”）；裁出 `22-hn-50415628-top.png`（标题、782 points、413 comments、最高回复评论之一）。第一版（`tmp-v1-21-hn.png`）带出评论框边线，作废，重裁为 22 号 |
| 约 18:15 | timesofindia 标题夸大实例 | 页面含广告占位；裁出 `23-toi-thousands-headline.png`（标题、更新时间、正文前几段）。页面显示 “Updated: Nov 07, 2026, 09:56 IST” |
| 18:17 | BNY 1Q26 业绩演示 PDF 第 4 页 | `barking shot pdf` 渲 144 dpi（1920×1080，=1 倍满幅幻灯片，宽度超过 1400 px，但原文是整页幻灯片，不是网页 CSS 像素），裁出 `24-bny-1q26-deck-p4.png` |

**图号用满 13–24**：21 号空缺（第一版 HN 截图作废后改为 22 号）；未截 State of AI 页面（L5 线索，不配图）。所有截图均已逐张打开目视确认文字清楚、范围正确；截图内无抓取者账号信息（未登录；浏览器翻译插件小图标位于裁切框之外；HN、SEC、Bloomberg 页均为匿名态）。

## 三、热度

| 北京时间 | 命令 | 结果 |
|---|---|---|
| 18:07:24 | `barking heat docs/2611/1108 b --item 50415628` | 780 分，405 条嵌套评论 → `b-heat.json`、`b-hn-50415628.json`（Algolia items，评论无分数字段，HN 不对外给评论分） |
| 18:18:06 | `barking heat docs/2611/1108 b2 --hn "Goldman Sachs Claude Code" --hn "Goldman restricts traders bankers AI tools" --reddit "Goldman Sachs Claude Code"` | 50415628：782 分/413 评；50341065（彭博原链接）：31 分/5 评；50346504（moneycontrol）：4 分/1 评；Reddit 被登录页拦截（失败）→ `b2-heat.json` |
| 约 18:14 | opencli 打开 HN 页 | 页面显示 782 points、413 comments、“15 hours ago” |

（因 `barking heat` 拒绝覆盖已有 `b-heat.json`，第二次用组前缀 `b2`。）

## 四、Exa 网页搜索（线索，非存档）

约 18:08–18:19，共 9 组：Argenti 7 月 vibe coding 采访；彭博 + 7 月 Argenti 采访；Anthropic S-1 承销商；Anthropic 工程师驻高盛六个月；高盛事故/泄露/Anthropic 回应；Barclays/BNY/JPMorgan 同业；中文标题（禁用/封杀）两组；JPMorgan Devspace；高盛 Waldron 访谈。线索页已逐个用 fetch 取得原文后才引用。

## 五、假设与限制

- 彭博全文以 FA Magazine 转载页为准，彭博原页与 Bloomberg Law 只作前两段比对。
- 评论分数：HN 对外不给评论分，用子评论数作热度代理。
- 没有自称高盛现职员工的评论；自称金融从业者的评论见 evidence-b.md（均未核实身份）。
- `b-yahoo-edge.txt` 含 The Edge 页面署名 “Uploaded by …”，为转载站编辑署名，非本案当事人；入库前由 Claude 决定是否保留该行。
- 本组所有时间以本机时钟为准（`date` 显示 UTC+8）。

## 六、待删除文件（精确路径，均在 `docs/2611/1108/` 下）

作废或中间产物（图片不入库，仅本地）：
- `images/tmp-v1-20-cnbc-cut.png`（CNBC 第一版，标题被截断）
- `images/tmp-v1-21-hn.png`（HN 第一版，带评论框边线）
- `images/tmp-yahoo.png`（Yahoo 截图，未用）
- `images/tmp-tnw2.png`（TNW 中间图；18 号已从它裁出，裁完即可删）
- 重复存档：`sources/b-bny-aibank.html`、`sources/b-bny-aibank.headers`、`sources/b-bny-aibank.txt`（与 `b-pymnts-bny` 同源，该 URL 已在 list-2 取过）

裁图用的底图（Claude 如需从中重裁批注底图则保留，用完再删）：
- `images/tmp-bbglaw-full.png`、`images/tmp-bbg.png`、`images/tmp-famag.png`、`images/tmp-s1.png`、`images/tmp-s1b.png`、`images/tmp-s1c.png`、`images/tmp-tnw.png`、`images/tmp-cnbc.png`、`images/tmp-hn.png`、`images/tmp-toi.png`

说明：`tmp-s1b.png` 为 SEC 页滚动到承销商栏的中间图，未用。

文本存档单文件均小于 1 MB（最大 `b-hn-50415628.json` 约 214 KB）；超过 1 MB 的只有 html/pdf 原件（不入库，如 `b-thestack-news.html` 1.2 MB、`b-youtube-oddlots.html` 1.7 MB），`b-bny-1q26-deck.pdf` 约 227 KB。
