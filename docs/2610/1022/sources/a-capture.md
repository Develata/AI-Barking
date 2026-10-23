# A 组官网截图记录

截图于 2026-10-23（北京时间）。截图由 `barking shot` 从官方页面渲染；入选图片只做像素裁切，没有重绘、改字或压缩。渲染与裁切记录在本目录的 `a-shots-log.jsonl`；下列高亮候选坐标相对最终 PNG，格式为半开像素框 `[x0,y0,x1,y1)`。

## Anthropic：formal verification

- 来源：https://www.anthropic.com/research/formal-verification
- 成品：`images/01-formal-raw.png`，1400 × 1002 px。
- 状态：可用。700 CSS px、DPR 2 根页渲染 `formal-long` 后，取框 `[0,9168,1400,10170)`。截图保留 Figure 2 的说明，以及紧接着的结果段落、证明规模和假设。段落和已存文本 `preflight-formal.txt` 一致。
- 未采用的尝试：带 `#turning-tests-into-proofs` 的渲染只有大面积空白和章节导航，未作为证据图。
- 可供高亮的逐字短句：
  - “239 entry points across 123 different OSS-Fuzz libraries.”：`[1075,419,1216,452)`、`[64,467,746,498)`。
  - “about 180 million lines of Rocq”：`[1003,515,1251,538)`、`[65,563,252,593)`。
  - “single-threaded static binary with a fixed memory budget on a RISC-V CPU using Linux OS.”：`[800,751,1286,782)`、`[65,798,884,829)`。

## SoftBank：紧急速報メール维护公告

- 来源：https://www.softbank.jp/corp/news/info/2026/20261023_01/
- 成品：`images/02-softbank-raw.png`，1400 × 1395 px。
- 状态：可用。700 CSS px、DPR 2，使用 iPhone User-Agent 渲染官方页面的移动响应式版 `softbank-mobile`，取框 `[0,420,1400,1815)`。截图含完整公告标题、日期、PHP 维护说明、三品牌、紧急地震/海啸等类别、维护起始时间和预计完成日期；与 `preflight-softbank.txt` 一致。
- 未采用的尝试：桌面 User-Agent 的 700 CSS px 渲染横向截断了公告标题；该图未用作成品。
- 可供高亮的逐字短句：
  - “PHPの脆弱性に対する対応”：标题两行，`[1153,28,1343,70)`、`[42,92,400,148)`。
  - “現在、PHPの脆弱性に対応するために緊急速報メールの緊急メンテナンスを実施しており、“ソフトバンク”、“ワイモバイル”および“LINEMO”のお客さまへの緊急速報メールの配信を停止しています。”：`[42,399,1331,436)`、`[45,440,1205,479)`。
  - “緊急地震速報や津波警報、災害・避難情報を受信できません”：`[573,516,1324,556)`。
  - “2026年10月23日（金）午前6時47分から”：`[41,936,548,976)`。
  - “メンテナンスは10月26日（月）に完了する予定です。”：`[113,1086,690,1121)`。
