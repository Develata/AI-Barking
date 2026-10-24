# 1023 期 A 组抓取日志

时间均为北京时间（UTC+8），2026-10-24。机器本地为美东，本地时间 + 12 小时 = 北京时间。批量抓取明细另见 `a-fetch-log.jsonl`，截图明细见 `a-shots-log.jsonl`，热度见 `a-heat.json`。

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 19:52:46 | https://www.kimi.ai/blog/kimi-k3-1-pro | barking fetch | ok（200，正文 .txt 291 行；benchmark 表由脚本渲染，.txt 无表，数据读自 HTML 内嵌 JSON） |
| 19:52:46 | https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks | barking fetch | ok |
| 19:52:46 | https://artificialanalysis.ai/models/kimi-k3-1-pro | barking fetch | ok（HTML 4.4 MB，只存本地） |
| 19:52:46 | https://artificialanalysis.ai/evaluations/cyber-index | barking fetch | failed：HTTP 404（A6） |
| 19:52:46 | https://huggingface.co/moonshotai | barking fetch | ok |
| 19:52:46 | https://huggingface.co/api/models?author=moonshotai&sort=lastModified&direction=-1&limit=30 | barking fetch | ok（JSON，无 K3.1） |
| 19:52:46 | https://www.reuters.com/technology/moonshot-curbs-hacking-ability-new-kimi-model-fears-grow-over-ai-powered-2026-10-24/ | barking fetch | blocked：HTTP 401（无重试，转载见下） |
| ~19:55 | https://huggingface.co/api/models/moonshotai/Kimi-K3.1-Pro、Kimi-K3.1、Kimi-K3.1-Pro-Highspeed | curl（各 1 次） | 均 401（不存在的仓库） |
| 20:18:07 | platform.kimi.ai/docs/pricing/chat-k3-1（猜测路径）、/docs/pricing、artificialanalysis.ai/leaderboards/models、/evaluations/terminalbench-4-0、/evaluations、www.kimi.ai/blog、/blog/kimi-k3 | barking fetch（清单 a-urls2.tsv） | 7 条 ok（terminalbench 首次连接失败后自动重试成功；`chat-k3-1` 路径实际落到 K3.1 Pro 概览页，不是价格页） |
| ~20:22 | https://platform.kimi.ai/docs/pricing/chat.md、/docs/guide/kimi-k3-1-pro-quickstart | barking fetch（a-urls3.tsv） | ok（价格表在 .md 中为空表，取自 `a-kimi-pricing.html` 内嵌数据） |
| 20:24:16 | Hacker News（Algolia 搜索 “Kimi K3.1”“Kimi K3.1 Pro”） | barking heat | HN 2 组命中；Reddit 搜索被重定向到登录页，失败（响应非 JSON） |
| ~20:25 | https://hn.algolia.com/api/v1/items/50212065 | curl | ok（a-hn-50212065.json，553 条评论树，Algolia 无评论得分） |
| ~20:26 | https://statics.kimi.ai/kimi-blogs/kimi-k3-1/20261023/xdev-bench-hard-results.webp | curl | ok（官方图表原图 2400×1238，转 PNG 缩至 1400 宽 → images/02-kimi-xdev-chart-raw.png） |
| 20:28:06 | thenextweb.com、implicator.ai、rappler.com（Reuters 转载）、stratnewsglobal.com、sina.cn（5 条） | barking fetch（a-urls4.tsv） | 全部 ok |
| 20:03–20:50 | https://www.kimi.ai/blog/kimi-k3-1-pro | opencli browser（会话 a-shot…a-shot4；前几次因视口高度改动导致页面重排、滚动位置错乱，作废；最后用 1264×719 真实视口逐屏截图） | 页面“Cybersecurity”段 8 屏拼接成功；“Cybersecurity benchmarks”表所在位置截图为白屏（canvas 未渲染），未能截取 |
| ~20:50 | 同上 | barking shot stack | images/01-kimi-cyber-raw.png（1190×5752，8 屏，间隔 0，窗口缩放 zoom=2，已隐藏页面固定顶栏、浮动按钮与浏览器翻译插件图标；页面正文像素未改） |
| ~21:00 | https://artificialanalysis.ai/articles/kimi-k3-1-pro-launch-benchmarks | barking shot render（headless Chrome，700 CSS px、DPR 2，高 2600） | aa-article 中间图 1400×5200；cropbox 出 03-aa-index-raw.png（1400×1320）与 04-aa-tbench-raw.png（1400×860，第一次裁切底部 x 轴标签被切掉，已用 `rm` 删除自己刚生成的 04-aa-tbench-raw.png 后重裁）；AA 页面为浅色底 |

## 说明

- 未登录、未输入凭据、未接受 Cookie 之外操作；opencli 会话中页面显示匿名头像（未登录 Kimi）。
- 存档中未保存抓取者信息（`barking fetch` 的 .headers 只存白名单头）。交付前对 `docs/2610/1023/` 中 a- 文件 grep 了 avatar 与 @handle：命中的都是页面公开文字里的账号提及与导航链接，没有抓取者标识。
- 已知失败：Reuters 原站 401；Reddit 搜索需登录；AA Cyber Index 404；Cybersecurity benchmarks 表无截图。
- 超过 1 MB 的文件均为 HTML 原件（a-aa-model.html 4.4 MB、a-aa-leaderboard.html 1.9 MB、a-aa-tbench.html 1.1 MB），只存本地；文本存档（.txt/.json/.md）均小于 1 MB。
- a-urls*.tsv 为本组抓取清单。
