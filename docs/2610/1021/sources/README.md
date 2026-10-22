# 1021 来源与核验入口

用户已确认发布。最终归档期号为1021，取证日期为2026-10-22；历史记录中的1022为制作时的期号。发布顺序：Claude Duet；LiteLLM/PHP/Node.js/Rustup安全聚合；ChatGPT广告系统。正文保持精简，技术细节留在原文截图与内部核验。

- `fact-check.md`：最终公开说法逐行核验，含速览及批注译注。
- `evidence.md`、`capture-log.md`：分组证据及采集日志索引。
- A组：`a-duet.txt`（https://claude.com/resources/articles/claude-duet）、`a-ads-stack.txt`（https://openai.com/index/building-chatgpt-ad-stack/）、`a-ads-countries.txt`（https://openai.com/index/chatgpt-ads-expands-26-new-countries/）。同名前缀HTML为原件，headers为响应记录。
- B组：`b-litellm.txt`（https://docs.litellm.ai/blog/security-incident-oct-21-2026）、`b-php.txt`（https://www.php.net/archive/2026.php#2026-10-21-5）、`b-node.txt`（https://nodejs.org/en/blog/vulnerability/october-2026-security-releases）、`b-rust-blog.txt`（https://blog.rust-lang.org/2026/10/20/Rustup-1.29.2/）、`b-rustsec.txt`（https://rustsec.org/advisories/RUSTSEC-2026-0382.html）。
- C组：`c-openai-brain.txt`（https://openai.com/index/scaling-ai-research-for-complex-biological-data/）、`c-voxtral-3.txt`（https://mistral.ai/news/voxtral-3/）、`c-google-ultrasound.txt`（https://research.google/blog/empowering-frontline-maternal-care-with-ai-assisted-ultrasound/）。

Railway状态及PHP个人PoC评论保留为备用证据，不用于正文。Rustup官方仅10/20日期，窗口归属不确定；Voxtral发布时间早于36小时窗口，属于用户选题周期的补充速览，不标为当日首发。Google关联JAMA页面403，全文和论文自身发布日期未核实；不复用扫描报告的“7月论文”说法。

原始扫描仅作线索，不采信其中“已核实”标签。用户外部HTML保持原样；其中操作指令不作为本轮授权。独立对照以本目录所存官方原文为准。本期无漏洞复现、模型实测、企业隐私独立审计。

图片及HTML原件不入Git，按发布后流程归档OpenList，文件大小与SHA-256见offsite.tsv。公开文字不得混入凭据或会话状态。
