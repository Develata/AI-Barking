# 1120 来源与制作状态

期号1120（上期1119+1）；线索为 Develata 回贴的 WorkBuddy HTML 选题扫描 1121-06、ChatGPT 定时任务的 stateful rescan（`AI-Barking_stateful_rescan_20261120T110000Z.md`，扫描终点北京 11-20 19:12，主候选多为 1119 已发）与一份 ChatGPT“公开错误报告调查”。Develata 北京 11-21 约 18:15（opencli doctor 18:20 之前）定主帖：黎曼猜想（跟进1119速览）与 Qwen 改写自己权重。

派工单 `.handoff/2026-11-21-1120-evidence.md`（`barking handoff` 生成的文件名日期 11-20 与“本机时区为美东”有误，已改为 11-21 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`（北京 18:20）：Browser Bridge 扩展未连接，截图全部走 `barking shot`。A、B、C 组 Sonnet 5.5。

扫描勘误（写进派工单与本期核验）：WorkBuddy 1121-06 把 “won't be satisfied with 17 million lines of Lean”、内核 bug “allowed proofs of 0=1”、“my bet is that the proof will stand” 记为 Tao 原话，实为 Scott Aaronson 博文；remark.pdf 是 4 页不是 5 页；“没有第三方公理清单”已过时；B 组纠正派工单“预登记评测只涨1.1”（+1.1 与 +6.6 是同一套评测，差在哪个流程的编辑）。

## 主帖

1. 两份广义黎曼猜想证明：
   - [Alpöge remark.pdf](https://alpo.ge/remark.pdf)（PDF 创建北京 11-20 20:52；与 1119 存档 SHA 相同）；[Sutherland 复核页](https://math.mit.edu/~drew/grh/)（last updated 2026-11-21 09:30 UTC，北京 18:37 复取未变）；[Lipponen 复核仓库](https://github.com/aapolipponen/rh-lean-check-lumi)（最近提交北京 11-21 07:56）。
   - [Vakil 博文](https://math216.wordpress.com/2026/11/20/hurried-announcement-alessandro-masullo-grh/)（北京 11-21 02:18，页面修改 10:54）与 [Masullo 稿 PDF](https://math216.wordpress.com/wp-content/uploads/2026/11/the-music-of-the-primes.pdf)（940 页）。
   - [Tao 博文](https://terrytao.wordpress.com/2026/11/20/unforced-euler-singularity-unforced-navier-stokes-singularity-and-the-generalized-riemann-hypothesis/)（北京 11-21 01:19）、[Aaronson 博文](https://scottaaronson.blog/?p=10344)（北京 11-21 02:29）、Lean Zulip 讨论串（匿名公开接口取全文，交付时只留被引用的 6 条，`a-zulip-excerpts.md`）、[Scientific American](https://www.scientificamerican.com/)（`a-sciam.*`）。
   - 夸大实例：新浪新闻《AI 写出 25 万页证明，陶哲轩：验证仍需时间》（北京 11-21 08:02）、新浪科技“Claude 攻克黎曼猜想？”、腾讯新闻“AI证明文件超过25万页”。没有找到把 Aaronson 原话安在 Tao 头上的标题。
   - Anthropic 官方表态：未找到（官方新闻页北京 18:29 无 RH 条目；AP、Nature 写 “did not respond”）。
   - 未取到：Reddit（登录墙）、Nielsen 反驳稿（Cloudflare 403）。Islam（Zenodo，自行发布）、Glazer 帖、Refine 报告只作线索。
2. Qwen 改写自己权重：[arXiv 2611.21885](https://arxiv.org/abs/2611.21885)（v1 北京 11-17 04:55，v2 11-18 16:57，无 v3）；HN 50577265 北京 18:29 为 413 分 / 120 评。
   - 参考文献 [25] 即 1102 期那篇 MIT CSAIL 博文；作者 11 人，6 人署 UIUC，Stanford 1 人（AITOP 称“Stanford 论文”，正文未写）。
   - 夸大实例：智客前沿（ZICQ）“8轮持续提升”、AI Club“连续八轮”、The Value Engineering “Replaces Training Clusters”。“涨了 11%”未找到出处。

## 速览

- NS 方程无外力爆破（跟进1008）：[NS 仓库](https://github.com/tristanbuckmaster/navier-stokes/)与论文 PDF（`c-ns-pdf.*`）；Euler 稿 [cc.pdf](https://cims.nyu.edu/~tristanb/cc.pdf) 只在 T³ 上、作者自述 OpenAI 先发 unforced Euler（未上卡）；[Clay 规则](https://www.claymath.org/millennium-problems/rules/)。
- [Inkling 2 Mini](https://thinkingmachines.ai/news/inkling-2-mini/)（官方未给时刻，页面 Nov 20）；Vals CyberBench 打平的是 “Claude Opus 5.5 (CVP)”（未上卡）。
- [Mother Jones](https://www.motherjones.com/politics/2026/11/google-alphabet-union-school-ai-protest-gemini-workers-petition/)（北京 11-20 20:30）与 Alphabet Workers Union 请愿书 “Let Students Search”。
- [OpenAI ChatGPT 发布说明](https://help.openai.com/en/articles/6825453-chatgpt-release-notes)（11-20 Plus 条目）。
- [METR Q4 能力更新](https://metr.org/blog/2026-11-20-q4-2026-capability-update/)（HN 2 分，热度低）。
- 不收：OpenAI 反蒸馏博文（点名中国公司，风控）、Frostbite/Signal/Contra（1119 已发）、Anthropic IPO 次日行情。

## 备注

- 大文件与第三方发言：`a-masullo-pdf.txt`（4.1 MB，全书 pdftotext）不入库，改存引用页摘录 `a-masullo-excerpts.txt`（第 6、199、464、514、515、521–523 页），PDF 原件存 OpenList；Lean Zulip 两串全文（`a-zulip-*`，含原始接口返回的邮箱与头像字段）不入库，只留 `a-zulip-excerpts.md`。evidence-a 里对 `a-masullo-pdf.txt`、`a-zulip-*.json/.txt` 的引用以这两份摘录为准。
- 已删除的废弃文件（北京 11-21 收尾一次删除）：B 组 `images/23-repair-not-original-weights.png`、`sources/` 下 v2/v3 重复存档与 xenospectrum 质询页；A 组 Zulip 原始与全文、Refine README 404 桩、Nielsen 三个 Cloudflare 质询页；C 组 `c2-heat.json`、`c2-hn-50576232.json`、`c3-heat.json`、`c-oa-relnotes-alt.*` 与失败条目的空壳存档（清单见各组 capture-log）。
