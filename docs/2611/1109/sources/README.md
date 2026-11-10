# 1109 来源与制作状态

期号1109（上期1108+1）；线索为 Develata 回贴的 ChatGPT 选题扫描（邮件版，北京 11-09 19:11）与 HTML 扫描 1110-06，Claude 现查 HN 与一手页面。Develata 拍板：主帖 Fable 5.2 发布（价格、费马、黑洞合为一帖）与“AI 泡沫不存在”长文；PSI 量子猜想只进速览。

派工单 `.handoff/2026-11-09-1109-evidence.md`（文件名日期是 `barking handoff` 按美东算的，派工实际在北京 11-10 15:35；派工单正文的时区行已改为北京时间）。四组执行方：A、B、C 为 Claude Sonnet 5.5，D 为 Claude Haiku 5.5（速览组试用，约 40 分钟交回）。取证与起草同属 Claude 家族，事实清单与存档的跨模型核对由反向核验（Codex）完成。

## 主帖

1. Fable 5.2 发布：[发布页](https://www.anthropic.com/claude-fable-and-mythos-5-2)、[价格页](https://platform.claude.com/docs/en/about-claude/pricing)、[模型总览](https://platform.claude.com/docs/en/models/overview)、[系统卡](https://www.anthropic.com/document/claude-fable-5-2-mythos-5-2-system-card)（PDF 只存网盘）、[Artificial Analysis 模型页](https://artificialanalysis.ai/models/claude-fable-5-2)、[Vals](https://www.vals.ai/models/anthropic_claude-fable-5-2)；费马：[claude.dev 技术博客](https://claude.dev/blog/safely-scaling-multi-agent-teams-with-fable-5-2/)、[9 月原文](https://www.anthropic.com/research/formalizing-fermats-last-theorem)；黑洞：[Hexagon 175](https://hexagonmath.org/2611.00175v1)、[Hexagon 174](https://hexagonmath.org/2611.00174v1)、[Hexagon 首页](https://hexagonmath.org/)；夸大实例：[智通财经](https://www.zhitongcaijing.com/content/detail/1520763.html)、[凤凰网转载新智元](https://tech.ifeng.com/c/8xugZ5rMQG3)。
   - 待核：本次 1,000 智能体的 Lean 产物未公开；黑洞结果无第三方复算；“内部 Claude 研究模型”是否即 Fable 5.2 早期版本，原文未说；路透社稿未找到。
   - AA 数字为北京 11-10 15:29 快照，会漂移。
2. “AI 泡沫不存在”长文：[博文](https://matteoraso.github.io/im-not-convinced-the-ai-bubble-exists.html)、[HN](https://news.ycombinator.com/item?id=50418975)、[首版 S-1](https://www.sec.gov/Archives/edgar/data/2130357/000162828026067168/anthropicpbc-sx1.htm)、[S-1/A](https://www.sec.gov/Archives/edgar/data/2130357/000162828026068676/anthropicpbc-sx1ax1.htm)。S-1 与 S-1/A 的全文 txt 各约 1.6 MB，已删去不入库，可由离线存的 `c-s1-first.html`、`c-s1a.html` 经 `barking html2txt` 重建；正文所用段落与季度利润表见 `c-s1-excerpts.txt`（取证与核验记录里的 `c-s1a.txt` 行号即重建后的行号）。
   - 作者文末的竞选与募捐段不截、不摘、不存外链（`c-blog.*` 已替换为占位）；正文不点作者名。
   - 待核：作者引的“3 万亿承诺”原文章（archive.li 被验证页挡住）；Meta 年报 PDF 链接 403，用 SEC 同年 10-K 代替。

Claude 与第一条的当事模型同属 Anthropic，第二条核对的也是 Anthropic 招股书，两条靠跨模型复核把关。

## 速览

- Meta Muse Ember：[官方博客](https://research.meta.ai/blog/muse-ember)。
- 谷歌 TPU 8i Max / 8tx：[官方博客](https://blog.google/innovation-and-ai/infrastructure-and-cloud/global-network/tpu-8i-max-and-8tx/)。
- PSI 量子猜想：[PSI 研究页](https://psi.inc/research/aaronson-ambainis/)、[GitHub](https://github.com/physical-superintelligence/aaronson-ambainis)、[Aaronson 博客](https://scottaaronson.blog/?p=10295)。
- Asahi Linux：[进展报告](https://asahilinux.org/2026/11/progress-report-7-3/)、[LWN](https://lwn.net/Articles/1105936/)。
- DeepX：[EE Times](https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/)（L4；DeepX 官方新闻稿未找到）。
- 未收：英伟达 CEO 员工信（1108 速览已收董事会调查，新增内容是私生活）、高盛（1108 主帖，只有热度增长）、Anthropic 模型选型指南（普通读者门槛低）、Fable 5.2 渠道接入、Mythos 5.2 衣壳与电路板（发布页自述）。

## 备注

- 批注底图 `s1a-ratio-raw.png` 由 `barking shot` 以 `s1a-172-raw.png` 之名裁出后改名（`ed-shots-log.jsonl` 记的是旧名），像素未改。
- `blackhole-provenance-raw.png` 由 Hexagon 论文页（174）的标题区、来源声明区与 Hexagon 首页说明段三段拼接，图题与译注分别标“论文页”“站点首页”。
