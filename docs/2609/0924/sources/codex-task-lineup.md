# Codex 取证任务：GPT-5.6 与 GPT-6 产品线档位对比

按 `templates/codex-evidence.md` 的分级与规则执行，期次目录 `0924/`。只取证，不写结论。已有存档先用：`openai-pricing-expanded-rows.json`、`aa-sol-luna-article.json`、`aa-sol-luna-current.json`、`aa-opus-article.json`、`openai-launch-browser.json`。新截图从 `20-` 开始编号，最多 2 张。

待验证的编辑假设（不要预设它成立）：GPT-5.6 有 Sol / Terra / Luna 三档；GPT-6 有 Astra / Sol / Luna 三档；按能力和价格，GPT-6 Astra ≈ 旧 Sol 档、GPT-6 Sol ≈ 旧 Terra 档、GPT-6 Luna ≈ 旧 Luna 档，因此除 Luna 外属于“换名变相涨价”；GPT-6 Sol 比 GPT-6 Luna 强得不多。

需要的数据（每项给一手来源 URL 与原文或原始数值）：

1. GPT-5.6 系列的全部型号名称，是否存在 Terra；各型号 API 标准价（输入/输出，短上下文与长上下文），以及 OpenAI 对各型号的定位原话。若 5.6 当时有促销价，分别记录原价与促销价及促销时间。
2. GPT-6 Astra 的 API 标准价（输入/输出，短/长上下文）与 OpenAI 定位原话；发布日期。
3. Artificial Analysis 当前 Intelligence Index（注明版本 v4.3 / v4.3.2）与 Cost per Task：GPT-5.6 Sol、GPT-5.6 Terra（若有）、GPT-5.6 Luna、GPT-6 Astra、GPT-6 Sol、GPT-6 Luna，均注明 effort（优先 max，同时记录 AA 页面实际展示的 effort）。同一张页面同一版本的数值优先。
4. AA 文章中“GPT-6 Sol 与 Luna 分别对应哪个前代”的原话（例如 “level with GPT-5.6” 中比较对象是 5.6 Sol 还是其他型号）。
5. OpenAI 官方是否说明 GPT-6 各档与 GPT-5.6 各档的对应关系或迁移建议（如“Sol 用户迁移到…”）。

输出 `0924/sources/evidence-lineup.md`：一张型号 × {价格, AA 指数, 单任务成本, effort, 版本, 来源} 表；一张“假设逐条检验所需事实”表（只列事实与来源，不下结论）；数据缺失如实写“未找到”。最后一条消息 ≤250 字摘要。不修改任何其他文件。
