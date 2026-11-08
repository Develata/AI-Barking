# B 组抓取与截图日志（1107 期）

时间均为北京时间（UTC+8）；本机为美东，北京时间 = 本机 + 13 小时（本机时间 11-08 11:48 ≈ 北京 11-09 00:48）。执行方 claude-sonnet-5-5。

## 抓取（`barking fetch`，匿名 curl 式请求，结果记入 `b-fetch-log.jsonl`）

| 时刻（11-09） | 名称 / URL | 结果 |
|---|---|---|
| 00:48 | doubleai `https://www.doubleai.com/research/can-llms-redesign-themselves-as-they-train` | 200 ok，1,768,036 字节（HTML，含内嵌图表 iframe 的 srcdoc） |
| 00:48 | wy163 `https://www.163.com/dy/article/LB7TFPQE0511ABV6.html` | 200 ok，199,926 字节 |
| 00:48 | doubleai-home `https://www.doubleai.com/` | 200 ok |
| 00:50 | about / news / research-index（doubleai.com） | 200 ok |
| 00:50 | hf-aime2026 / hf-hmmt2026（huggingface.co/datasets/MathArena/…） | 200 ok；行数 30 / 33 |
| 00:50 | hf-dsv41 `huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash-1017` | 200 ok；“763B params” |
| 00:50 | matharena `https://matharena.ai/` | 200 ok（未用于具体数字） |
| 01:0x | sohu `https://www.sohu.com/a/1098825729_473283` | 200 ok |
| 01:0x | sina `https://finance.sina.cn/2026-11-08/detail-inixzpaw7195792.d.html` | 200 ok |
| 01:0x | linuxdo `https://linux.do/t/topic/3138385` | **blocked**：403，Cloudflare 质询；未重试，未绕过 |

清单：`b-list.tsv`、`b-list2.tsv`、`b-list3.tsv`。

## 其他请求

| 时刻 | 请求 | 工具 | 结果 |
|---|---|---|---|
| 00:5x | 网易评论接口 `https://comment.api.163.com/api/v1/products/…/threads/LB7TFPQE0511ABV6` | curl `--max-time 30` | 200，JSON 仅含计数（tcount 3、cmtCount 3、rcount 4、vote 2），存 `b-163-comment-thread.json`，无用户信息 |
| 00:5x | `api.fxtwitter.com/status/2041555517853270296`（网易稿引用的 X 帖）与 `…/2041556445947908456` | curl | **被 Claude Code 权限分类器拒绝执行**（未发出请求），未取到；未绕过。Shashua 的帖 2118799949794046410 只见 WebSearch 摘要 |
| 01:01 | `barking heat`：HN 搜索 “doubleai.com”“Can LLMs redesign themselves as they train”“doubleAI DeepSeek-V4.1-Flash probe”（自 2026-11-05 起）；Reddit 两次搜索 | barking heat → `b-heat.json` | HN 命中 2 帖（见 evidence-B B12）；Reddit 两次被重定向到登录页，失败，未登录 |
| 01:0x | WebSearch 2 次（中英文） | WebSearch | 得到搜狐、新浪转载、linux.do/locdd.com 译文帖、Shashua X 帖摘要 |

## 自写脚本（放在 sources/，组前缀 b-）

- `b-extract-figs.py`：从 doubleai HTML 的 `<figure>` 里抽出每个图表 iframe 的 srcdoc（`b-doubleai-figNN.srcdoc.html`，NN=1–10），并摘出含数组的 `const` 行到 `b-doubleai-figdata.txt`。Fig. 11 是位图，没有 srcdoc，alt 文本存 `b-doubleai-fig11-alt.txt`。
- `b-cdp-seg.mjs`：匿名 headless Chrome（独立临时 profile，无登录、无 Cookie 导入）经 CDP 先滚动一遍触发懒加载，再按 CSS 像素区间截图。原因：`barking shot render` 对 doubleai 长页只能出到约 18,150 CSS px 处（再往下空白，尝试 `--height 16000/22000/30000` 均如此），Experiment 4 在 17,182 px 处之后；用 `#anchor` 也得到空白图。参数：宽 700 CSS、DPR 2（截图 1400 px 宽）；网易稿宽 1000 CSS、DPR 1.4（也是 1400 px 宽；宽 700 时页面右侧被截断）。

## 截图（页面处理：浅色底，均无登录态；原图为 CDP 区间截图，进入 `images/` 时用 `barking shot cropbox` 只裁切，记入 `b-shots-log.jsonl`）

doubleai 页面 CSS 像素区间（页面总高 23,136 px；本页有 CookieYes 弹窗，位于视口左下，按规则不点击；弹窗只出现在位置靠近页首的 TL;DR 截图里，该图已弃用，其余图无弹窗）：

| 文件 | 内容 | 像素 | 备注 |
|---|---|---|---|
| images/13-fig10-chart.png | Experiment 4 标题、763B 与“without thinking mode”段、64%/71% 段、Fig. 10 全图与图注 | 1400×1910 | 已逐张打开确认；图表数据与 `b-doubleai-fig10.srcdoc.html` 一致 |
| images/14-ds-probe-code.png | “A probe on its own residual stream”：写完时读、最多三次、`probe_and_resample` 代码块（全 7 行） | 1400×1120 | 已确认 |
| images/15-ds-knowing.png | “Knowing what it knows”：243 题、预测 8 实测 7、拒 4/5 错答、半途探针 | 1400×936 | 已确认 |
| images/16-qwen-fig11.png | Qwen on Qwen 全段、frontier agent 预建探针、59%→66%、Fig. 11 与图注 | 1400×1646 | 已确认；Fig. 11 为位图 |
| images/17-qwen-gadgets-code.png | gadgets 与三种动作、Qwen 写的 `step/end` 代码 | 1400×1630 | 底部最后一段半行被截（“Attempts that run past 30k…”），不影响所引内容 |
| images/18-qwen-evolve.png | “Evolving the policy”：约 800 题、约 15,000 个策略 | 1400×660 | 已确认 |
| images/19-exp2-fig6.png | Evolution is mandatory 全段与 Fig. 6（0%/55%/72%、+17 points） | 1400×2640 | 已确认；Fig. 6 为交互图，此帧停在“Beyond the window, on average” |
| images/20-fig7-clrs.png | Taking on jaggedness：55%、“only a third … raise that third to 79%”、Fig. 7 | 1400×2320 | **此帧不理想**：Fig. 7 是自动播放的动画图，截到时只画出 plain training 的灰线、代码层橙线未出现，且显示的是 Greedy and geometry 页签；数据见 `b-doubleai-fig7.srcdoc.html`。文字段落可用，图不宜当证据 |
| images/21-fig8-clrs-curve.png | “ten code layers … 84% versus just 55%”、“Trained for twice as many steps”、Fig. 8 全图 | 1400×2820 | 已确认；曲线与 52%（2× steps）都在；末尾图注被截一半 |

网易稿（桌面版页面，宽 1000 CSS；右侧栏含广告与推荐，已用 cropbox 裁掉；视口高设为 6000 以免固定浮层进入截图）：

| 文件 | 内容 | 像素 | 备注 |
|---|---|---|---|
| images/22-wy163-title.png | 标题、日期 2026-11-08 17:51:13、来源 新智元、评论数 3、题图 | 1230×880 | 已确认；顶部有网站搜索栏一角，无个人信息 |
| images/23-wy163-three-groups.png | “消耗相同的token预算，三组对照高下立判”与三组数字 | 1000×590 | 已确认 |
| images/24-wy163-opening-rollback.png | 稿首“不改半个权重”“当场回滚重算”段，及下方“思维链每推进到检查点……当场触发回滚重算rollback！”段（两段是同一页上的不相邻区域，`stack` 拼接，间隔 24 px） | 1000×1060 | 已确认；底部最后一行被截一半 |
| images/25-wy163-code-image.png | 网易稿里的 `probe_and_resample(prompt)` 代码图（检查点 1000–12000 token、rollback）与下一句 | 970×586 | 编号 25 超出 B 组区间 13–24（我多用了一个），A 组 01–12、C 组无截图，不冲突；已确认 |

未截：Fig. 2（nanochat，已做数据核对）、Fig. 1、Fig. 9；官方 X 帖（拿不到）。

## 自查

- 截图与存档未含抓取者信息：存档为匿名请求，`.headers` 由工具省略多数响应头；`b-163-comment-thread.json` 只有计数；`b-cdp-seg.mjs` 与其他自写文件不含本机用户名与本机路径。对 `docs/2611/1107/` 的 grep 交给 Claude 收尾复核时合并执行（A、C 组文件仍在写）。
- 未点击任何 Cookie 同意；未登录；未发帖或评论。
