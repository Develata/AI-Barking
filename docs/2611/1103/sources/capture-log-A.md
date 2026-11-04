# 1103 期 A 组抓取日志

执行：Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8）。批量抓取的逐条结果在 `a-fetch-log.jsonl`，截图动作在 `a-shots-log.jsonl`，热度在 `a-heat.json`。清单文件：`a-fetch-list-1.tsv` … `a-fetch-list-4.tsv`。

## 一、barking fetch（批次与结果）

| 批次 | 时刻 | 条目 | 结果 |
|---|---|---|---|
| 1 | 20:46 | 腾讯新闻版 `tx-news`；tencent.com preview 稿；Kimi 博客；新京报；路透；什么值得买；HF API（tencent、Tencent-Hunyuan、moonshotai、Hy4-preview）；HF Hy4-preview 模型卡；GitHub API（Tencent-Hunyuan、MoonshotAI）；GitHub Hy4-preview | 13 条 ok；路透 401 blocked（改用 opencli，见下） |
| 2 | 20:47 | tencent.com 英文 Hy4 稿；tencent.com 新闻室列表；混元 Blog 首页 | 前两条 ok；Blog 首页是 JS 壳，blocked（改用 opencli） |
| 3 | 20:56 | IT之家；澎湃；21 财经；网易科技；网易订阅；新浪；hy.tencent.com/research/hy4；什么值得买 Hy4 稿 | 7 条 ok；hy.tencent.com 同样 JS 壳 blocked |
| 4 | 21:22 | tencent.com 简体、繁体中文 Hy4 稿 | 2 条 ok（简体稿含“将于近期开源模型权重”） |

## 二、opencli 与其他单次请求

| 时刻 | 目标 | 工具 | 结果 |
|---|---|---|---|
| 20:50 | https://hunyuan.tencent.com/blog 、详情 `/research/hy4?langVersion=zh` | opencli browser（extract 与 eval outerHTML） | 成功，存 `a-hy-blog-hy4.md`、`a-hy-blog-hy4.html`（图片签名参数已去除）。列表页日期 2026-11-03，详情页 2026年11月4日 |
| 20:52 | 路透正文 | opencli browser（innerText） | 首次渲染含正文，存 `a-reuters-body.txt`；稍后弹订阅框；未登录，未绕过 |
| 20:55 | 腾讯官网 `/newsroom/all-news/` 找到 Hy4 英文稿地址 | curl | 成功 |
| 20:57 | AA https://artificialanalysis.ai/leaderboards/models 、`/models/hy4`、`/models/tencent-hy4` | curl | 列表页 200，Hy4 0 处（Hy3 有）；两个 /models/ 地址 404。存 `a-aa-leaderboard.html`（仅本地） |
| 20:58 | LMArena leaderboard/text | curl 403；opencli 停在 Cloudflare“请稍候”页 | **失败**，未绕过 |
| 20:59 | Kimi 官方 X 公开时间线 | `opencli twitter tweets Kimi_Moonshot --limit 20`；`api.fxtwitter.com/status/2113680701770068067` | 成功，存 `a-kimi-x-timeline.json`、`a-fx-2113680701770068067.json`（均为公开内容，不含抓取者信息）。最新帖 2026-10-30 06:00 UTC |
| 21:00 | Kimi 博客索引 | opencli extract | 最新文章 2026-10-24 |
| 21:02 | 微信公众号“腾讯技术工程”“腾讯混元”原文 | `opencli weixin search`（搜狗微信）→ opencli 打开 10 条结果 | 搜索成功，存标题与摘要 `a-weixin-search-titles.txt`（去掉含 token 的跳转链接）；10 条结果点开全部遇微信验证码页，**失败**，未绕过 |
| 21:04 | Hy4-preview LICENSE 与 README；ModelScope Hy4-preview API | curl | 成功，存 `a-hf-hy4-preview-LICENSE.txt`、`a-hf-hy4-preview-README.md`、`a-modelscope-hy4-preview-api.json` |
| 21:05 | 权重探测：ModelScope Hy4 / Hy4-FP8、HF tencent/Hy4 与 Hy4-FP8、HF moonshotai/Kimi-K3.1-Pro 与 K3.1、GitHub MoonshotAI/Kimi-K3.1-Pro 与 Tencent-Hunyuan/Hy4、cnb.cool、ai.gitcode.com | curl | 全记在 `a-probe-weights-2026-11-04.txt`：ModelScope 404，HF 401（不存在或私有），GitHub 404，cnb 404；gitcode 返回 200 的 SPA 壳，不能判定 |
| 21:10 | HF 按 lastModified 排序（tencent、moonshotai） | curl | 存 `a-hf-tencent-lastmod.json`、`a-hf-moonshot-lastmod.json` |
| 21:12 | Reddit 搜索 | `barking heat`（old.reddit）、`opencli reddit search` | **失败**：登录墙 / 403，未绕过 |
| 21:13 | hy.tencent.ai 英文 Blog（HN 提交所指） | opencli extract，分两个 chunk | 成功，存 `a-hy-blog-hy4-en.md` |
| 21:19 | `barking heat`：HN “Tencent Hy4”、“Kimi K3.1 Pro”，条目 50362291 | barking | 成功，`a-heat.json`、`a-hn-50362291.json` |
| 21:24 | 权重复查：HF API（tencent、moonshotai）、直接探测、GitHub API、X 时间线 | curl、opencli | 无变化；存 `a-hf-moonshot-api-recheck-2124.json`、`a-hf-tencent-api-recheck-2124.json` |
| 21:28 | 交回前最后一次复查：HF moonshotai 与 tencent 的最新仓库、`Kimi-K3.1-Pro` 与 `Hy4` 直接探测、GitHub MoonshotAI | curl | 无变化（HF 仍无 K3.1 系列与 Hy4；GitHub MoonshotAI 最新 TCSExam 2026-11-03） |

## 三、截图（`docs/2611/1103/images/`，2 倍像素比，宽度均 ≤ 1400 px）

| 文件 | 来源 | 工具 | 备注 |
|---|---|---|---|
| `01-tx-news-top.png` | 腾讯新闻版 | `barking shot render` + `cropbox` | 标题、署名与 12:52、首两段；左侧悬浮工具栏已裁掉 |
| `02-tx-news-bench-chart.png` | 同上 | 同上 | 官方对比图整张含脚注。最初裁的一版少了脚注，已移到 `sources/a-tmp-02-bench-chart-partial.png` 并把完整版改名为 `02-…`（日志 `a-shots-log.jsonl` 里仍是旧名 `02-tx-news-bench-chart-full.png`） |
| `03-tx-news-responsible.png` | 同上 | 同上 | “负责任的开源”“现在就体验”、价格表、“Hy4 flash 将于近期发布”；可见此版无权重句 |
| `03a-hy-blog-weights-limits.png` | 混元 Blog 中文 | opencli（html 缩放 2 倍，700 CSS px 版式）+ `cropbox` | 负责任的开源和发布、价格表、权重与系统卡句、局限性；隐去了浏览器翻译插件浮标；页面为浅色；日志里原名 `15-hy-blog-weights-limits.png` |
| `04-tx-news-finance.png` | 腾讯新闻版 | `barking shot` | 金融众测段；底部带出一小截演示图，边界未完全落在文字行间 |
| `04a-hy-blog-finance.png` | 混元 Blog 中文 | opencli | 金融与办公场景整段（含分数、幻觉率）；原名 `14-hy-blog-finance.png` |
| `05-tencentcom-en-lead.png` | tencent.com 英文稿 | `barking shot` | 标题、日期、首段 |
| `05a-tencentcom-zhcn-lead.png` | tencent.com 简体稿 | opencli | 标题、日期、含权重句的首段 |
| `06-tencentcom-en-finance.png` | tencent.com 英文稿 | `barking shot` | 众测、评分、幻觉率 12.95%→7.2% |
| `07-tencentcom-en-cyber.png` | tencent.com 英文稿 | `barking shot` | 网络安全与生物安全一节 |
| `08-hf-hy4-preview-head.png` | HF Hy4-preview 模型卡 | `barking shot`（匿名 headless） | 许可证标签、点赞、下载 |
| `09-hf-tencent-models-recent.png` | HF `models?sort=created&author=tencent` | 同上 | 页面显示 Models 176、Sort: Recently created；无 Hy4 正式版 |
| `10-hf-moonshotai-models-recent.png` | HF `models?sort=created&author=moonshotai` | 同上 | Models 19，最新 Kimi-K3 |
| `11-kimi-blog-availability.png` | Kimi 博客 | opencli | Availability 节“by Nov 4, 2026”；正文在滚动渐显动画里，文字颜色偏淡 |
| `12-smzdm-kimi-today.png` | 什么值得买桌面版 | `barking shot stack` | 标题与时间、小标题、正文三段拼接；作者账号名不在图中 |

三批过程中的 `a-tmp-*.png` 为中间截图，不入库，列在回报的待删除清单里。

没有截图的项：路透（headless 被设备验证拦，opencli 版有订阅弹窗）、LMArena（Cloudflare）、微信公众号（验证码）、Kimi 官方 X 帖（只存 JSON，若要上图须另截帖子卡片）、AA（只存页面，缺席无法截图）。

## 四、使用到的假设与限制

- 北京时间 = 本机美东 EST + 13 小时；X/路透的 UTC 时间换算为 UTC+8。
- “人民币价格”等数字均照官方原文，没有换算。
- 路透正文读取：见“二”表。
- 对比图读数为目视读取，未用 OCR，需复核。
