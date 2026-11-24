# 1123 来源与制作状态

期号1123（上期1122+1）；线索为 Develata 回贴的 WorkBuddy HTML 选题扫描 1124-18（生成于北京 11-24 18:00）与 ChatGPT 定时 rescan（`AI-Barking_stateful_rescan_20261124T110000Z.md`）。两份扫描的推荐前三完全不重合；Claude 北京 11-24 22:30 前后用 HN Algolia 重查热度、对 `daily-scan.md` 已发过去重后提候选，Develata 北京约 22:50 定主帖：①Anthropic 公开 Claude 署名的 VP≠VNP 论文与 Lean 证明；②NariQ 把 Qwen3.8-27B 量化进约 10GB。

派工单 `.handoff/2026-11-24-1123-evidence.md`（`barking handoff` 生成的文件名日期 11-23 与“本机时区为美东”有误，已改为 11-24 与北京时间）。派工前 `opencli doctor`（北京 22:55）：Browser Bridge 已连接。A、B 组 Sonnet 5.5，C 组（速览）Haiku 5.5，北京约 23:05 派出；Claude 因用量将尽于北京约 23:20 催交 A、B 组，A 组截图在用量恢复后补做。

扫描勘误：
- WorkBuddy 称 Strands Decider v2“成本降 98%、响应快 78%”：98% 出自 11-10 的 v1 替换 Sonnet 4.6 一帖（HN 736 分），不是 v2（Claude 查 HN）。
- WorkBuddy 称 BioEmu2 为新发布：其 GitHub 仓库 10-21 已上过 HN（Claude 查 HN）。
- WorkBuddy 称 NariQ 中文稿把代码与权重许可混写：B 组读到的中文稿都没写许可，未核到。
- WorkBuddy 称“BF16 级质量属实”：掘金原文是条件句“若BF16级质量属实”。
- ChatGPT 扫描称 AISI 原站未显示绝对日期：页面写“Nov 23, 2026”（C 组）。
- ChatGPT 扫描称 a16z 81% 为结账口径：原文是“走到结账表单末尾”的运行占比，分母为允许游客结账的网店，不是付款完成率（C 组）。
- 扫描未提 OpenAI 同题论文：A 组发现 openai/math 北京 11-24 09:20:59 提交“VP and VNP are different”。

## 主帖

1. VP≠VNP（A 组，`evidence-A.md`、`capture-log-A.md`）：
   - [anthropics/formal-math vp-vnp](https://github.com/anthropics/formal-math/tree/main/vp-vnp)（提交 918c8e27，北京 11-24 04:04:44；[PR #51](https://github.com/anthropics/formal-math/pull/51) 合并 05:43:50）；论文 `paper.pdf`（70 页，`a-paper-pdf.txt`）。
   - [openai/math VP and VNP are different](https://github.com/openai/math/blob/main/preprints/VP-and-VNP-are-different-November-23-2026/paper.pdf)（提交 e07100d3，北京 09:20:59）；history.md、根 README。
   - 背景：Valiant 1979（Crossref）；[Bürgisser 综述 arXiv:2406.06217](https://arxiv.org/abs/2406.06217)；Mahajan arXiv:1307.3863。
   - 夸大实例：[新智元（网易号）](https://www.163.com/dy/article/LCGA4F6V0511ABV6.html)（10:21:50）；[华尔街见闻（网易号）](https://www.163.com/dy/article/LCGC2BG805198NMR.html)（10:55:40）；凤凰网转载；36Kr 英文站；emergent.sh；Korea Daily。对照：机器之心、什么值得买较克制。
   - 热度（HN，北京 23:03）：主帖 343 分 / 311 评；另两帖 26 / 2、23 / 8；OpenAI 帖 5 / 1。
2. NariQ（B 组，`evidence-B.md`、`capture-log-B.md`）：
   - [Hugging Face 模型卡](https://huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF)（仓库创建北京 11-20 09:27:15；11-22 改三轮平均）；[nari-labs/nari-llama.cpp](https://github.com/nari-labs/nari-llama.cpp)（README、release、Metal 与 CUDA 资格文档）；[arXiv:2611.26157](https://arxiv.org/abs/2611.26157)（v1，北京 11-19 22:13:27）。
   - 夸大实例：[掘金“衍辉AI速递 11.24”](https://juejin.cn/post/7711225699185410102)（08:31:20）；mobdome（疑似虚构实测）；zmj.net 两篇；TechNewsReel。对照：OpenAI Hub、nextfor.ai 较准确。
   - 热度：HN 181 分 / 35 评（北京 22:59）；Reddit 被登录墙拦下，未取到。

## 速览（C 组，`evidence-C.md`、`capture-log-C.md`）

- Shirudo：[OpenAI](https://openai.com/index/introducing-shirudo/)、[思科新闻稿](https://newsroom.cisco.com/c/r/newsroom/en/us/a/y2026/m11/cisco-and-openai-launch-shirudo-a-new-project-to-safeguard-agentic-ai.html)、[GitHub](https://github.com/shirudo-dev/shirudo)、Isovalent 博文。
- Claude 插件页：[帮助中心 17740565](https://support.claude.com/en/articles/17740565-manage-plugins-skills-and-connectors-from-one-plugins-page)。
- Claude Ship：[claude.com](https://claude.com/resources/articles/introducing-claude-ship)。
- Anthropic 企业访谈：[anthropic.com/research/working-with-ai](https://www.anthropic.com/research/working-with-ai)。
- AISI：[aisi.gov.uk](https://www.aisi.gov.uk/blog/attack-policy-selection-can-make-monitor-evaluations-misleading)。
- a16z：[a16z.com](https://a16z.com/welcome-to-the-b2a-economy-how-agents-buy/)；方法页 promptingcompany.com。
- DeepMind：[deepmind.google](https://deepmind.google/blog/finding-faster-quantum-algorithms-with-agents-and-formal-verification/)。

## 待删除与不入库

提交前一次删除：失败抓取的残留（`c-cisco-shirudo-news.*`、`c-cisco-investor-shirudo.*` 为 404 或 Cloudflare 质询页，`c-aihot-home.headers`、`c-c-develata-news-1124.*` 为 SSL 失败与 404，`b-juejin-user-list.json` 为“must login”响应）。`c-c-prompting-methodology.*` 是方法页的跳转页，保留作记录。

文本存档中的个人邮箱（论文作者、新闻稿联络人共 4 个，在 4 个文件里）已替换为“[邮箱已隐去]”；机构与角色邮箱保留。超过 1 MB 的 `a-gh-commit.html`、`b-unsloth-q38.html`、`c-c-figma-agent.html` 是 HTML 原件，按规则不入库，存 OpenList。

读者稿检查后删去的吠点③所用批注图：成图 `publish/12-score-table.png` 已移出 `publish/`（Claude 移到草稿目录，不上传）；底图 `score-table-raw.png` 留作备用图。
