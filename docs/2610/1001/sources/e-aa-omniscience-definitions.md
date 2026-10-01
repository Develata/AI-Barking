# AA-Omniscience 指标定义（Claude 独立抓取，供 1001 核验）

来源：https://artificialanalysis.ai/evaluations/omniscience （curl，HTTP 200；抓取约北京时间 2026-10-02 01:4x，本摘录写入于 01:47，UTC+8）。只摘定义句，页面全文未入库。

| 指标 | 原文（逐字） |
|---|---|
| 基准简介 | AA-Omniscience, a benchmark designed to measure both factual recall and knowledge calibration across 6,000 questions. Questions are derived from authoritative academic and industry sources, and cover 42 economically relevant topics within six different domains. |
| Accuracy | AA-Omniscience Accuracy (higher is better) measures the proportion of correctly answered questions out of all questions, regardless of whether the model chooses to answer |
| Hallucination Rate | AA-Omniscience Hallucination Rate (lower is better) measures how often the model answers incorrectly when it should have refused or admitted to not knowing the answer. It is defined as the proportion of incorrect answers out of all non-correct responses, i.e. incorrect / (incorrect + partial answers + not attempted) |
| Index | AA-Omniscience Index (higher is better) measures knowledge reliability and hallucination. It rewards correct answers, penalizes hallucinations, and has no penalty for refusing to answer. |

用途：正文“没答对的题里错答占15%”“答对率仅50%”的口径出处。Argon 的 45 分门槛来自另一个指标 Artificial Analysis Intelligence Index，不是 AA-Omniscience Index。
