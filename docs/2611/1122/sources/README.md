# 1122 来源与制作状态

期号1122（上期1121+1）；线索为 Develata 回贴的 WorkBuddy HTML 选题扫描 1123-18（生成于北京 11-23 18:02）、11-19 的 ChatGPT rescan 邮件（`AI吠点-邮件-2026-11-19-1316.md`，其 C1–C3 已在 1118、1121 发过，本期未用）与一段 ChatGPT 关于黎曼猜想进展的摘要（粘贴）。Claude 北京 11-24 00:05–00:25 现查 HN（Algolia）、Lean 社区博客、MIT Sutherland 页、Tao 博客、MiniMax 博客与 GitHub、Anthropic 与 OpenAI 状态页后提候选；Develata 北京约 00:28 定主帖：①黎曼猜想证明获 Lean FRO 与 Mathlib Initiative 认证（跟进 1120）；②MiniMax 内部模型 MIM 的密码分析与中文夸大稿。风控（Develata 同时拍板）：第二条标题与封面不写“比 Claude 快”，正文可交代对比对象。

派工单 `.handoff/2026-11-24-1122-evidence.md`（`barking handoff` 生成的文件名日期 11-22 与“本机时区为美东”有误，已改为 11-24 与北京时间；本机 macOS、UTC+8）。派工前 `opencli doctor`（北京 00:20）：Browser Bridge 已连接。A、B 组 Sonnet 5.5，C 组（速览）Haiku 5.5，北京约 00:30 派出。

扫描勘误：
- WorkBuddy 1123-18 称 OpenAI 11-21 有 211 分钟全站故障（13:48–17:21 美西、峰值 75%）：所据页面 `status.openai.com/incidents/n254wyd7nml7` 是 2023-11-21 的事故（页面时间戳 Nov 21, 2023），2026 年状态 JSON 无对应事故。C 组发现，Claude 核对页面内嵌时间戳确认。Claude 初读时只看了正文没看年份，险些采信。
- WorkBuddy 称“央广网 11-23 发稿”：未找到；中文夸大稿源头是新智元（新浪财经 11-23 07:02）。
- WorkBuddy 称 MiniMax“第三方复现为零”：ePrint 2026/2993（Machinefi Labs，11-23）基于 MiniMax 代码复核了 4 个 TII 密钥（L6 处理，未进正文）。
- WorkBuddy 称 pequalnp 站点写“no result here is a proof unless it says checked”：存档中未找到。
- ChatGPT 摘要称 Clay 官网把 RH“标记为 Unsolved”：页面状态位实为“Active”（已解决的庞加莱猜想为“Solved”）。
- ChatGPT 摘要称 Lean 声明“由 Lean 和 Mathlib 社区发表、得到 Lean FRO 与 Mathlib Initiative 背书”：发布时（北京 11-21 22:26）确是此署名；11-23 北京 17:38 改为两家机构署名（PR #154）。

## 主帖

1. 黎曼猜想 Lean 认证（A 组，`evidence-A.md`、`capture-log-A.md`）：
   - [Lean 社区博客联合声明](https://leanprover-community.github.io/blog/posts/riemann-hypothesis/)（页面 2026-11-21 13:55 UTC）；署名改动见 [leanprover-community/blog](https://github.com/leanprover-community/blog) PR #143、#149、#150、#154。
   - [MIT Sutherland 复核页](https://math.mit.edu/~drew/grh/)（last updated 2026-11-23 14:35 UTC）。
   - [Xena 2026-11-12](https://xenaproject.wordpress.com/2026/11/12/and-the-proof-is-checked-in-lean/)；[Tao 11-20 博文](https://terrytao.wordpress.com/)（`a-tao.txt`）；[Clay 规则](https://www.claymath.org/millennium-problems/rules/)与 [RH 页](https://www.claymath.org/millennium/riemann-hypothesis/)。
   - 夸大实例：[投资界转载新周刊稿](https://news.pedaily.cn/202611/572261.shtml)（更新于 11-23 17:33）；另存新浪科技、腾讯、爱范儿、SciAm 稿（evidence-A 第四节）。
   - 热度（HN，北京 00:40 前后）：Lean 博客帖 1233 分 / 1036 评；原 PDF 帖 2919 / 2096；“Yes, 17M lines of Lean is a lot” 205 / 247。
2. MiniMax 密码分析（B 组，`evidence-B.md`、`capture-log-B.md`）：
   - [MiniMax 博客](https://www.minimax.io/blog/practical-cryptanalysis-with-minimaxs-latest-internal-model)（datePublished 2026-11-20T06:41:45Z）；[GitHub MiniMax-AI/Research](https://github.com/MiniMax-AI/Research)（论文 PDF、README；提交 `640514fd` 与 issue #1、#2 回复见 `b-gh-issue-replies.md`）。
   - NIST 状态：`b-nist-*.txt`（IR 7620、8309、8545，FIPS 203/204/205）。
   - 夸大实例：[新智元稿（新浪财经转载）](https://finance.sina.com.cn/wm/2026-11-23/doc-inizskhq8466652.shtml)（11-23 07:02）；网易号同稿（07:12:21）。对照：IT之家（11-22 21:23，较克制）。
   - 热度：HN 两帖 3 分 / 0 评、8 / 1；中文侧为主。

## 速览（C 组，`evidence-C.md`、`capture-log-C.md`）

- 智谱 GLM-5.4：[智谱公众号](https://mp.weixin.qq.com/s/D-IZr-QaT1or_chminK8IQ)（北京 11-23 17:40:42）与 [Z.ai 博客](https://z.ai/blog/glm-5.4)。
- OpenAI 困惑度提案：[openai.com/index/perplexity-and-intelligence](https://openai.com/index/perplexity-and-intelligence/)（November 22, 2026）。
- Claude 11-23 报错：[status.claude.com](https://status.claude.com/)（API `incidents.json`）。
- ChatGPT GPT-6 对话报错：[status.openai.com](https://status.openai.com/incidents/01M84HKHX73RF7HTVZQ2A9GFXP)（API 北京 00:42 取，`c-c6b-openai-incidents-0042.json`）。
- pequalnp：[pequalnp.com](https://pequalnp.com/)（opencli 北京 00:12–00:14 快照）。

## 待删除与不入库

提交前已一次删除：作废裁切与低清旧图（`images/` 下 06、08、09-…-baidu、11 旧版，B 组 19–21 四张截断图）；重复或拦截页壳（`a-quanta-169564/169569.*`、`a-clay-qualifying.*`、`a-science.*`、`a-phys.*`、`a-nielsen.*`、`b-tii-gh.*`、`c-c6-ms-mai-code2.*`、`c-c6-openai-gpt6-incident.json`、`c-c5-pnp-dashboard-opencli.json`）；含致谢名单个人账号的 `c-c6-stanford-ai100.txt`（保留去掉致谢段的 `c-c6-stanford-ai100-redacted.txt`）；超过 1 MB 的 HN 全量评论 JSON `a-hn-50574968.json`（只引了分数与评论数，见 `a-heat.json`）；含回复者账号信息的 issue 评论原始 JSON（回复原文见 `b-gh-issue-replies.md`）。文本存档中的个人邮箱已替换为“[邮箱已隐去]”（22 个文件、167 处；机构与角色邮箱保留）。
