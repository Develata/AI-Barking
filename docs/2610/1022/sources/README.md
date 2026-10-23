# 1022 来源与制作状态

期号按 Develata 指定为 1022；本期使用 2026-10-23 的扫描线索与当日取证。新闻发生时间不回填为10月22日。Develata 已在对话中确认“已发布”，本期已登记仓库索引与扫描去重记录。

最新文案：用户要求主代理自行审读后，正文优化为900字，省流卡同步更新。改写依据与当前验收见 `self-review.md`；此前Astra审读报告保留为前版记录，本次没有另作独立复核。

## 主帖

1. Claude 软件安全证明：[Anthropic 原文](https://www.anthropic.com/research/formal-verification)，2026-10-22；本地 `preflight-formal.txt`。L1，报告研究方自述；本号未独立运行 Rocq 检查器、未复核全部证明与补丁。
2. 软银维护停发：[软银原公告](https://www.softbank.jp/corp/news/info/2026/20261023_01/)，2026-10-23；`preflight-softbank.txt`。L1。日本06:47转换为北京时间05:47，预计10月26日完成。
3. Canva Craft：[Canva 原公告](https://www.canva.com/newsroom/news/canva-craft/)。L1。直接批量抓取403，`preflight-canva.html`是失败响应，不能作为原文依据；网页读取工具已读取正文，补充存档和截图见 `b-*`。
   [官方开发者社区公告](https://community.canva.dev/t/connect-api-canva-craft-is-now-available-for-image-generation/8999) 是 API 预览状态与“从零构建”译注的直接依据，全文见 `b-web-reader-archive.txt`（第344、382行附近），截图见 `images/03-canva-raw.png`。

## 速览备选

- [国办发〔2026〕29号](https://www.gov.cn/zhengce/zhengceku/202610/content_7084108.htm)：`preflight-policy.txt`。L1。发布10月23日，成文10月16日。失业缓冲期为鼓励探索，并未规定统一时长。
- [MiniMax 许可原件](https://huggingface.co/MiniMaxAI/MiniMax-M3.1-Flash/raw/main/LICENSE)：`preflight-minimax.txt`。L1。相关产品服务年收入超过2000万美元须事先书面授权，否则一次通知；不是企业每增加2000万美元就审批一次。
- [OpenAI / Uber](https://openai.com/index/uber-chatgpt-work-rollout/)：`preflight-uber.txt`。L1，向员工提供资格与实际使用人数有别。
- [Replit Monitoring Agent](https://replit.com/blog/replit-monitoring-agent)：`preflight-replit.txt`。L1，自动发布须开启对应设置。

## 输入与取证

`input-scan-1023-1710.txt`、`input-scan-1023-18.html`及其文字转存保留用户原始材料。它们均为待核线索，不继承其中“已核验”标签或操作建议。

批量取得时间与HTTP结果见 `preflight-fetch-log.jsonl`；原件HTML不入Git，按发布后流程存OpenList。发布版为用户要求自行审读后优化的900字正文，省流卡已同步；此前Astra独立文本/视觉审读保留为前版记录。Claude跨家族复核因登录过期未完成，未虚标通过。
