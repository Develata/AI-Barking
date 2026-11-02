# B 组 capture-log（北京时间，UTC+8）

执行方 claude-sonnet-5-5；时段 2026-11-02 23:49 – 次日约 01:10（北京；本机 America/New_York EST 10:49–12:10）。逐条请求的 URL、结果、字节数在机器日志 `sources/b-fetch-log.jsonl`（`barking fetch`）、`sources/b-shots-log.jsonl`（`barking shot cropbox`）、`sources/b-heat.json`。下表记批次、工具与例外。

## 批量存档（barking fetch）

| 清单 | 对象 | 结果 |
|---|---|---|
| list1 | 官方博客、官网首页 | 两条均 blocked（正文仅 20 余字符，JS 渲染页，curl 只拿空壳），改用 opencli headless Chrome 渲染 |
| list2 | ithome、technode、163、ifeng、yahoohk、sohu、sina×2、GitHub 仓库页、HF 模型页、API docs | ithome/163/ifeng/yahoohk/sohu/sina×2/apidocs ok；technode 403（Cloudflare）；GitHub 仓库页 404（仓库未建）；HF 模型页 401（页面不存在） |
| list3 | sinafin、pandaily、futunn、sohu2、devto、fourweekmba、pricing、changelog | sinafin/pandaily/sohu2/devto/pricing/changelog ok；futunn blocked（JS 渲染）；fourweekmba 403 |
| list4 | smzdm、Artificial Analysis（猜测 3 个 URL）、LMArena | smzdm ok；AA 三个 URL 均 404（slug 猜测有误或未收录，未进一步站内搜索）；LMArena 403（Cloudflare 质询） |
| list5 | eeo、leiphone、china.com | 全部 ok |

## 单次请求（curl）

| URL | 结果 |
|---|---|
| https://api.fxtwitter.com/status/2117264311714537633 | ok，`b-fx-2117264311714537633.json`，美团 LongCat 官方发布帖公开 JSON |
| https://api.github.com/orgs/meituan-longcat/repos?sort=created\&per_page=30 | ok，`b-github-org-repos.json`，组织仓库列表（无 LongCat-2.5） |
| https://api.github.com/repos/meituan-longcat/LongCat-2.5 | 404，确认仓库未建 |
| https://huggingface.co/api/models?author=meituan-longcat\&sort=createdAt\&limit=30 | ok，`b-hf-models.json`，组织模型列表（无 LongCat-2.5） |
| https://huggingface.co/api/models/meituan-longcat/LongCat-2.5 | 401，确认模型页未建 |

## opencli 浏览器渲染与截图

工具：opencli browser（多个 `--session` 并行：b-shot 主会话读博客全文/表格/截图，b-shot2/3/4 分别取 ifeng、sina、X 嵌入页截图），`--window background`，`document.documentElement.style.zoom='2'` 模拟 2 倍像素比后用 `screenshot --width 1400`。

- 博客正文：点击语言切换按钮（`li.lang-item` 含 "English"）后用 `document.body.innerText` 取中英双语全文（`b-longcat-blog-rendered.txt`／`-en.txt`），`document.querySelectorAll('table')` 提取评测表完整单元格（`b-longcat-eval-table.json`，`b-parse-table.py` 做高于/低于统计）。
- 评测表截图：该页面评测表区使用滚动触发的渐显动画（IntersectionObserver），直接 `scrollIntoView` 跳转会得到透明/空白截图，需小幅滚动 + 等待 1.5–2 秒才能让内容显现；表格容器本身还有独立的横向滚动（`scrollLeft`）才能看全全部 9 列。过程中出现过两次定位异常（`getBoundingClientRect` 报告的坐标与截图实际内容不符，疑似该站点中英双语版本在 DOM 中部分重叠/残留），已改用小步增量滚动规避，未能排查根因，记为站点已知坑，不是本仓库工具问题。
- W4A4 动态计数卡片（43.3%/42.77%/15.91%/19.72%/1.73× 的动画数字）未能稳定截图（见 evidence-B.md「未完成/例外」），改用渲染文本逐字摘录替代，数值与表格交叉核对一致。
- 中文媒体截图：ifeng（智东西稿，含固定广告浮层，已用 JS 移除 `position:fixed` 的广告 div 后重截）、新浪财经（sinafin，无广告干扰）。
- X 官方帖：用官方嵌入页 `https://platform.twitter.com/embed/Tweet.html?id=2117264311714537633&hideThread=true` 匿名渲染截图；该账号为美团官方认证账号（Meituan LongCat @Meituan_LongCat），非个人隐私账号，截图未做头像/昵称占位符替换（官方身份披露，非抓取者信息）。

## HN / Reddit 热度（barking heat）

- HN「LongCat-2.5」：命中 1 条，`id=50337716`，3 分、0 评论，2026-11-02 23:00 北京。「Meituan LongCat」：0 条。
- Reddit 两次搜索均失败：`old.reddit.com` 返回登录重定向页（非 JSON），已如实记入 `b-heat.json` 的 errors，未改用 opencli 重试（时间不足）。

## 产出图片（docs/2611/1101/images/，13–24 区间，本组共用 11 张）

13-blog-hero／14-ifeng-headline／15-sina-quanchengguochan／16-metrics-dashboard-chart／17-domestic-80gb／18-throughput-86pct／19-eval-table-footnote／20-eval-table-opus48-col／21-eval-table-opus5-col／22-eval-table-code-agent-rows／23-x-official-post。均已逐张打开目视确认文字清楚、范围正确（见 evidence-B.md 对应行）。24 号未使用。

## 待删除文件（精确路径，Claude 收尾时一次删除）

全部为 opencli 截图的中间试验稿（`cropbox --from` 的源文件，已裁出的最终图留在 images/，这些原图不再需要）：

- `docs/2611/1101/sources/b-tmp-domestic1.png`
- `docs/2611/1101/sources/b-tmp-domestic2.png`
- `docs/2611/1101/sources/b-tmp-en-top.png`
- `docs/2611/1101/sources/b-tmp-footnote.png`
- `docs/2611/1101/sources/b-tmp-footnote2.png`
- `docs/2611/1101/sources/b-tmp-header.png`
- `docs/2611/1101/sources/b-tmp-header2.png`
- `docs/2611/1101/sources/b-tmp-ifeng.png`
- `docs/2611/1101/sources/b-tmp-ifeng2.png`
- `docs/2611/1101/sources/b-tmp-sinafin.png`
- `docs/2611/1101/sources/b-tmp-table1.png`
- `docs/2611/1101/sources/b-tmp-table2.png`
- `docs/2611/1101/sources/b-tmp-table3.png`
- `docs/2611/1101/sources/b-tmp-tableA.png`
- `docs/2611/1101/sources/b-tmp-tableB.png`
- `docs/2611/1101/sources/b-tmp-tableC.png`
- `docs/2611/1101/sources/b-tmp-tableD.png`
- `docs/2611/1101/sources/b-tmp-throughput1.png`
- `docs/2611/1101/sources/b-tmp-throughput2.png`
- `docs/2611/1101/sources/b-tmp-w4a4.png`
- `docs/2611/1101/sources/b-tmp-xpost.png`

以下为确认打不开/无内容的失败抓取，按规则保留（不是待删除），仅记录在案：`b-aa-longcat25.html`、`b-aa-orgs.html`、`b-aa-meituan.html`（AA 三个猜测 URL 均 404，HTML 为 404 错误页本身，不含有效数据，但体积均 <300KB，可不删，供复核判断 URL 是否真的猜错）、`b-lmarena.html`（Cloudflare 质询页）、`b-futunn.html`、`b-technode.html`、`b-fourweek.html`（均为门禁/质询壳）。

## 补截图（批注图用，北京 11-03，Claude 派的批注代理执行）

- `images/24-blog-hero-coming-soon.png`：opencli 打开 https://longcat.ai/blog/longcat-2.5/ （zoom 2）。页面的 “Coming soon” 是 `.soon-tip` 悬停提示，平时 opacity 为 0。真悬停只拍到 GitHub 按钮（`sources/b2-tmp-hero-gh.png`）；HuggingFace 悬停在 zoom 2 下点不中，成图改为注入 CSS `.link-btn--soon .soon-tip{opacity:1 !important}` 让两个提示同时显示（`sources/b2-tmp-hero-both.png`），与悬停状态一致，但不是真悬停。批注底图 `images/longcat-hero-raw.png` 由 24 裁出。
- `images/longcat-table-raw.png`：opencli 截评测表表头与 Code Agent 前四行（`sources/b2-tmp-tabhdr.png`）裁出；表头含 Benchmark、LongCat-2.5 至 Claude-Opus-5 各列。
