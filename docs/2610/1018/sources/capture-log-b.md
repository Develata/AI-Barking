# 1018 期 B 组抓取日志

时区：北京时间（UTC+8）；本机为美东（EDT），北京时间 = 本机时间 + 12 小时。执行模型 Sonnet 5.5（claude-sonnet-5-5）。未登录、未接受 Cookie（页面自带的“切换到中文”弹窗只用 CSS 隐藏，未点击）、未用镜像或代理域名、未读取浏览器 Cookie；未使用 firecrawl。派工开始约 17:08，交回约 17:50。

## 一、网页存档（barking fetch，清单 `b-manifest.tsv`、`b-manifest-2.tsv`、`b-manifest-3.tsv`；明细 `b-fetch-log.jsonl`）

| 开始 | 条目（URL） | 结果 | 存档 |
|---|---|---|---|
| 17:08:35 | aa-model（https://artificialanalysis.ai/models/deepseek-v4-1-flash-1017） | 200 ok，4.3 MB，737 行文本 | b-aa-model.html/.txt/.headers |
| 同上 | aa-lb（https://artificialanalysis.ai/leaderboards/models） | 200 ok，5,636 行文本（SSR 表含 FAQ 文字） | b-aa-lb.* |
| 同上 | aa-old（…/models/deepseek-v4-1-flash） | 200 ok | b-aa-old.* |
| 同上 | aa-open（…/leaderboards/models?open_weights=open_source） | 200 ok，**与 aa-lb 字节数相同**（筛选是客户端的，这个参数名我猜的，无效）；有效的筛选参数是 `?weights=open`（见第二节），本条仅留底 | b-aa-open.* |
| 同上 | aa-methodology（…/methodology/intelligence-benchmarking） | 200 ok；未细读，本期未用 | b-aa-methodology.* |
| 同上 | ds-updates、ds-updates-en（api-docs.deepseek.com 更新日志中/英） | 200 ok | b-ds-updates.*、b-ds-updates-en.* |
| 同上 | openrouter（https://openrouter.ai/deepseek/deepseek-v4.1-flash-1017，清单里写成 deepseek-v4.1-flash-20261017 后被重定向取到 1017 页） | 200 ok | b-openrouter.* |
| 同上 | livebench（https://livebench.ai/） | **blocked**：正文过短（JS 渲染）；改用 opencli，见第二节 | 无 .txt |
| 同上 | arena-webdev（https://arena.ai/leaderboard/code/webdev） | **blocked**：403，Cloudflare 质询；改用 opencli | 无 .txt |
| 同上 | aihot（https://aihot.virxact.com/） | **failed**：协议错误（TLS）。随后 17:31 对 aihot.news/story/… 再失败一次；17:4x 用 curl、opencli 各再试一次仍 SSL 协议错误，停止 | 无 |
| 17:13:25 | arena-ranking-method（https://arena.ai/blog/ranking-method）、arena-ranking-method-old（https://news.lmarena.ai/ranking-method/）、arena-help-rankings（help.arena.ai 帮助文章） | 3 条均 200 ok（这三页没有 Cloudflare 质询） | b-arena-ranking-method.*、b-arena-ranking-method-old.*、b-arena-help-rankings.* |
| 17:31:26 | cryptobriefing、tokenpost、aifront、chinaz、decoder（the-decoder.com）：对应 URL 见 evidence-b.md 第三节与 manifest-3 | 5 条 200 ok | b-cryptobriefing.*、b-tokenpost.*、b-aifront.*、b-chinaz.*、b-decoder.* |
| 同上 | scmp（https://www.scmp.com/…/deepseek-says-new-flash-ai-model-beats-kimi-k3-on-cyber-coding-benchmarks）；aihot-story | scmp **blocked** 403 Cloudflare，不重试、不绕过；aihot-story failed | 无 |
| 约 17:31 | curl（`--max-time 60`）https://api.fxtwitter.com/status/2111894091168682086 | 200，7.7 KB。公开 JSON，不含抓取者信息 | b-arena-x-post.json |
| 约 17:31 | curl 同上 …/status/2098148674203488422（AA 9 月 10 日原帖） | 200 | b-aa-x-sep10.json |
| 约 17:36 | curl https://hn.algolia.com/api/v1/items/50148452（HN 评论树） | 200 | b-hn-50148452.json |

## 二、浏览器（opencli，会话 b-arena、b-aa、b-x、b-lb、b-hot，均在结束时 close）

| 时间 | 动作 | 结果 |
|---|---|---|
| 17:09–17:10 | b-arena：打开 https://arena.ai/leaderboard/code/webdev，`eval` 取 DOM 表格（脚本 `b-arena-extract.js`，用 `textContent`，因后台窗口里 `innerText` 为空） | 成功。输出 `b-arena-webdev-table.txt`（读取时刻 2026-10-19T09:10:43Z = 17:10:43 北京）。页面内嵌 JSON 含 rating/ratingUpper/ratingLower、license 等字段，我只在命令输出里用，未另存 |
| 17:12–17:14 | b-arena：点击 “Show Filters” → License Type “Open Source”（页面 UI 控件，纯筛选） | 成功，URL 变为 `?license=open-source`；表格存 `b-arena-webdev-opensource-table.txt`（17:14:03 北京）。随后重新打开原页，用“Hide Filters” |
| 17:12–17:17 | b-arena：悬停 Rank Spread 的 (i) 图标 | 未读出提示文字（弹窗未进入 DOM）；改取 Arena 博客与帮助中心页（见第一节）作定义 |
| 17:15–17:19 | b-arena 截图 | `opencli … screenshot --width 1400 --height 1550`。试过 `document.documentElement.style.zoom='2'`（700 CSS px）：Arena 表格在 700 px 布局下列被截断（侧栏 + 内容区挤占），改用 1.1 倍缩放并关闭侧栏与过滤栏，使全部列在 1400 px 里完整可见；同时把表格容器的 `height/maxHeight` 设为 1000px，以便一屏看到第 1–17 名（只改布局高度，未改任何数据）。原始截图放在 scratchpad（`arena-raw1.png`、`arena-raw2.png`），入库的是 `barking shot cropbox` 裁切后的 13 与 17 |
| 17:20 | `barking shot render` 渲染 AA 模型页（headless Chrome，700 CSS px，DPR 2，高 4000） | 成功；页面图表区域（JS 懒加载）在 headless 里是空白，只用了顶部 Model summary → 14 |
| 17:21–17:25 | b-aa：AA 排行页，点开 “Weights” 弹窗，取消 Proprietary 勾选、勾选 Open（页面控件），读 DOM 表 | 成功；筛选后 URL 为 `?weights=open`；表存 `b-aa-lb-openweights-table.txt`（09:25:31Z = 17:25:31 北京）。随后在同一页对 Weights:All 状态重读：`b-aa-lb-all-table.txt`（17:34:34 北京，脚本 `b-aa-extract.js`） |
| 17:2x | b-aa：AA 排行页截图；对 AA 模型页尝试 `zoom='2'` | **失败/放弃**：zoom=2 令 AA 页面出现重复平铺、版面错乱（截图中页面内容被重复）；改为 1 倍 + 1400 px 视口；Model summary 单独用 headless 渲染补成 2 倍（14） |
| 17:27–17:29 | b-aa：AA 模型页滚动取 “Cost per Intelligence Index Task”“Output Tokens per Intelligence Index Task” 图 | 成功但滚动位置不好控制（`scrollTo` 被页面夹到某个位置），逐次截图后取到可用的两张（16、18）。对 AA 旧版页同法截到 token 图（21）与顶部卡片（20）；旧版页的成本图未截到（滚动夹位，放弃） |
| 17:30 | b-x：打开 https://platform.twitter.com/embed/Tweet.html?id=2111894091168682086&theme=light，`zoom='2'` 后截图 | 成功（`barking shot render` 同一 URL 先试过，返回空白图，已不用）；嵌入页为匿名，不含抓取者账号 → 19 |
| 17:33–17:36 | b-aa 全部权重表截图 | `--height 3000` 与 `--height 2600` 各报 `cdp_timeout`（失败），改为滚动到表格后 1900 px 高度成功 → 22。尝试再滚表内容器后截图又 `cdp_timeout`，放弃（GLM-5.3、Kimi K3 行不在图内，文本表有） |
| 17:40 | b-lb：打开 https://livebench.ai/，`eval` 取渲染后文本（`b-livebench-rendered.txt`），截图 | 成功 → 23。读取 2026-10-19T09:40:16Z = 17:40:16 北京 |
| 17:4x | b-hot：打开 https://aihot.news/、https://aihot.virxact.com/ | 均 `ERR_SSL_PROTOCOL_ERROR`，AIHOT 取不到 |

## 三、热度（barking heat，`b-heat.json`，2026-10-19 17:33:38）

HN（Algolia，自 10-17 03:46 北京起）：“DeepSeek V4.1 Flash 1017” 8 条、“DeepSeek-V4.1-Flash” 7 条、“Arena WebDev DeepSeek” 0 条。主要条：50148452（AA 页，138 分/46 评，10-19 12:41）、50129045（HF 页，944 分/707 评，10-17 16:21）、50129121（OpenRouter 页，519 分/5 评）。Reddit：两次搜索均被重定向到登录页（old.reddit.com），记为失败，未重试、未登录。

## 四、截图（`docs/2610/1018/images/13–23`，日志 `b-shots-log.jsonl`）

目视确认：13、14、15、16、17、18、20、21、22、23 的成图逐张打开查看（文字清楚、边界在文字行之间）；19（X 嵌入卡片）只在裁切前的原图上看过，成图的框 [0,0,1102,2384] 覆盖整张卡片，未再单独打开复看。已知小瑕疵：20 的底边切在卡片下沿，数字与说明文字完整、卡片圆角下沿被截；原始顶部截图已被后续截图覆盖，无法重裁。

像素比：14 为 2 倍（DPR 2）；19 为嵌入页 CSS zoom 2；其余（13、15、16、17、18、20、21、22、23）为 1 倍，1400 px 视口（13、17 另有 1.1 倍页面缩放）。原因：opencli 没有设置 DPR 的参数，对这些页面用 CSS zoom 2 会使版面错乱或表格列被挤出视口（见上）；1 倍的 1400 px 宽图缩到 1080 宽时不会发虚，但文字相对偏小。Arena 在 headless 下有 Cloudflare 质询，无法用 `barking shot render` 取 2 倍图。

扩展图标：Arena、AA、LiveBench 页面上都有浏览器翻译扩展在页面右侧的小图标；13、15、17、19、22 已裁掉；23 右缘残留一小条边；16 右上有一小块灰色残留（AA 固定导航的边缘）。

## 五、本组新增文件清单

- `docs/2610/1018/sources/`：`evidence-b.md`、`capture-log-b.md`、`b-manifest.tsv`、`b-manifest-2.tsv`、`b-manifest-3.tsv`、`b-fetch-log.jsonl`、`b-heat.json`、`b-arena-extract.js`、`b-aa-extract.js`、`b-arena-webdev-table.txt`、`b-arena-webdev-opensource-table.txt`、`b-aa-lb-openweights-table.txt`、`b-aa-lb-all-table.txt`、`b-livebench-rendered.txt`、`b-arena-x-post.json`、`b-aa-x-sep10.json`、`b-hn-50148452.json`，以及 barking fetch 生成的 `b-<name>.html/.txt/.headers`；`b-shots-log.jsonl`。
- `docs/2610/1018/images/`：13–23（11 张）。
- HTML 原件（`b-*.html`）不入库，由 Claude 事后传网盘；单个 1 MB 以上的文本文件：无（最大的 `b-aa-lb.txt` 等约几十 KB；HTML 最大 `b-aa-model.html` 4.3 MB，不入库）。
- 删除：自己生成的试验图 `docs/2610/1018/images/_b-test.png`（普通 `rm` 精确路径，已删）。
- 另：`/tmp` 与 scratchpad 里有中间文件（未入库）。
