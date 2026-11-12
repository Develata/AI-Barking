# 1111 来源与制作状态

期号1111（上期1110+1）；线索为 Develata 回贴的两份扫描：ChatGPT stateful rescan（北京 11-12 01:12，`AI吠点-邮件-2026-11-12-0129.md`）与 HTML 选题扫描 1112-06（北京 11-12 06:00）。Claude 对照“已发过”与“近 3 天速览已收”去重、现查 HN 热度后列候选，Develata 选定玛雅 + AWS + Grok 三条主帖。

派工单 `.handoff/2026-11-11-1111-evidence.md`（文件名日期是 `barking handoff` 按美东算的；派工实际在北京 11-12 约 17:52，派工单正文的时区行已改为北京时间）。四组执行方：A、B、C 为 Claude Sonnet 5.5（`claude-sonnet-5-5`，harness 计约 25、20、19 分钟），D 为 Claude Haiku 5.5（`claude-haiku-5-5`，14 分钟）。取证全由 Claude 系代理完成，反向核验由 Codex（gpt-6-astra）逐行核对事实清单与存档。

## 主帖

1. 玛雅遗址：[Miotti 博文](https://pietromiotti.com/latest/maya-site/)、[更正前 Wayback 快照](https://web.archive.org/web/20261110201337/https://pietromiotti.com/latest/maya-site/)、[HN 50452455](https://news.ycombinator.com/item?id=50452455)、[作者原帖](https://x.com/PietroMiotti/status/2120219901814043087) 与 [更正帖](https://x.com/PietroMiotti/status/2120557109410775430)（fxtwitter JSON）、[论文 DOI](https://doi.org/10.1016/j.jasrep.2025.105288)（Crossref、OpenAlex 元数据）、[TechRadar](https://www.techradar.com/ai-platforms-assistants/google-researcher-discovers-unmapped-maya-site-in-mexico-using-48-ai-agents-project-takes-less-than-72-hours-after-agents-scour-nasa-lidar-data)、The Verge。
   - 未核：论文页被 ScienceDirect 验证页拦，“遗址在论文第 14 页”只有作者自述与 HN 评论；正文只写作者的更正。
   - 未核：Parcak 的 Bluesky 帖设了不在外部显示，未存档、未引用（`a-bsky-parcak.json` 不提交）。
   - 更正上线时刻是推断：北京 11-11 23:56 的快照还没有更正段；作者的更正帖发于 11-12 00:39。
2. AWS Emulator：[官方文](https://www.aboutamazon.com/news/aws/how-autonomous-ai-agents-built-aws-emulator)、[GitHub 仓库](https://github.com/aws/aws-emulator)、[保真度文档](https://docs.aws.amazon.com/emulator/latest/userguide/fidelity.html)、[v1.0.322 保真度分数文件](https://github.com/aws/aws-emulator/releases/download/v1.0.322/scores.json.gz)（1.1 MB，不入库；sha256 eca6d69b…08cd）、[Vogels 博客](https://www.allthingsdistributed.com/2026/11/aws-emulator-and-the-engineers-job-when-agents-write-the-code.html)、[HN 50456631](https://news.ycombinator.com/item?id=50456631)。
   - “约4.3万项通过约56%”是本号按官方分数文件合计（B 组与 Claude 各自下载重算，一致）；官方只按服务给“k of n”。
   - 官方文导语写“数千个智能体”，正文写“数百个”，并列记录。`vendor/` 框架（Apache-2.0）由 “Scaffolder” 提交，官方未说明怎么写成，未进正文。
3. Grok 应用：[Claroc 报告](https://blog.claroc.io/ghostshares-grok-build-apps/)、[Ars Technica](https://arstechnica.com/security/2026/11/grok-build-coding-agent-leaks-oodles-of-confidential-info-some-belonging-to-spacexsi/)、[The Decoder](https://the-decoder.com/grok-built-apps-exposed-patient-data-plaintext-passwords-and-spacexsis-own-sales-pipeline/)、THN、Cybernews。
   - C 组没有打开任何 grok.me 应用，也没有存泄露数据。Ars 文里唯一一个真实应用地址，已在 `c-ars.txt/.html` 里替换为占位符。
   - 报告里尝试的地址数有两种：43,551 与 43,601，正文不写这个数。“报告自称未发现第三方取数”是扫描的说法，报告里没有这句。
   - 厂商回应只有 Claroc 的转述（正文已删）。HN 热度很弱（三帖 5–11 分），靠媒体报道。

## 速览

- Gemini Decision 1：[谷歌博客](https://blog.google/innovation-and-ai/technology/developers-tools/gemini-decisions-api/)、[AI Studio 文档](https://aistudio.google.com/docs/decisions-api)。中文稿的“9 项第一 / 92.2%”在四个谷歌一手页均未找到。
- Ring Drone Cam：[Amazon 公告](https://www.aboutamazon.com/news/devices/ring-drone-cam-rover-cam-security-robot)、[产品页](https://ring.com/products/drone-cam)。
- AlphaGenome 2：[DeepMind 博客](https://deepmind.google/blog/alphagenome-2-deciphering-how-cells-read-dna-to-better-understand-disease/)、[Nature 新闻](https://www.nature.com/articles/d41586-026-03673-0)（注册墙截断）。
- 台积电：[SEC Form 6-K](https://www.sec.gov/Archives/edgar/data/0001046179/000104617926000797/tsm-boardx202611xx.htm)（路透 401、台积电新闻稿 403）。
- 未收：OpenAI × Upwork（“非因果”压不进 40 字，HN 0 命中）、AFT/UFT × Anthropic（新闻稿“automatically”与协议“If requested”有张力）、Anthropic Worse Together（HN 3 分）、a16z B2A（观点文）、Anthropic IPO 超额认购、OpenAI 三季度营收、Grok CSAM 诉讼、Claude Meetings、Arena Agent 榜、Atuin。

## 备注

- B 组截图 13–24 用尽后溢出到 37–44。已删废图 22、41 与 A 组三张作废的 07 图，以及各组回报列出的残留（重复抓取、质询页、404/429 页、手打摘录；`a-bsky-parcak.json` 因原帖设了不在外部显示而删除）；fetch-log 里仍留有这些条目的记录。
- `c-ars.*`、`d-aft-*`、`d-worse-together*` 的存档做过脱敏替换，它们的 sha256 和 fetch-log 里记的不一致，记 `offsite.tsv` 时按现文件重算。
