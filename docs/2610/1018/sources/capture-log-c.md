# 1018 期 C 组抓取日志

时区：北京时间（UTC+8）；本机为美东（EDT），北京时间 = 本机时间 + 12 小时。执行模型 Sonnet 5.5（claude-sonnet-5-5）。未登录、未接受 Cookie、未使用镜像或代理域名、未读取浏览器 Cookie、未使用 firecrawl。2026-10-19。

## 一、网页与 PDF 存档（barking fetch）

| 清单 | 条目 | 开始 | 结果 | 存档 |
|---|---|---|---|---|
| `c-list.tsv`（在系统临时目录，未入库） | abs（https://arxiv.org/abs/2610.18861） | 17:08:27 | 200 ok，43,236 字节 | c-abs.html/.txt/.headers |
| 同上 | html（https://arxiv.org/html/2610.18861v1） | 同上 | 200 ok，654,756 字节，3,347 行文本（未用于引用，以 PDF 文本为准） | c-html.* |
| 同上 | pdf（https://arxiv.org/pdf/2610.18861v1） | 17:10:22 完成 | 200 ok，9,578,485 字节（9.6 MB，**不入库**），2,515 行文本（pdftotext -layout），共 45 页 | c-pdf.pdf/.txt/.headers |
| 同上 | gh（https://github.com/togethercomputer/HybridAgent） | 17:08:29 | 200 ok，320,100 字节 | c-gh.* |
| `c-list2.tsv` | x-cryptobriefing、x-aiweekly、x-explainx、x-mindverse（媒体报道）；hf-8b（https://huggingface.co/togethercomputer/HybridAgent-8B-Agentic） | 17:18:07 | 全部 200 ok；已打开 .txt 看过，cryptobriefing/aiweekly/explainx 为正文；mindverse 仅存档未引用 | c-x-*、c-hf-8b.* |

## 二、其他请求

| 时刻 | URL / 命令 | 工具 | 结果 |
|---|---|---|---|
| 17:15 | https://api.github.com/repos/togethercomputer/HybridAgent 与 /contents | curl --max-time 60 | 成功：c-gh-api.json、c-gh-contents.json（星标 213、forks 28、MIT、created 2026-10-09T02:59:46Z、pushed 2026-10-18T17:37:03Z） |
| 17:15 | https://raw.githubusercontent.com/togethercomputer/HybridAgent/main/README.md | curl | 成功：c-gh-readme.md（8,079 字节） |
| 17:15 | https://hn.algolia.com/api/v1/items/50145337 | curl | 成功，另存 c-hn-50145337-algolia-raw.json（我自己的试验文件，因与 `barking heat` 的输出同名而改名保留，没有删除任何文件） |
| 17:16:37 | `barking heat docs/2610/1018 c --item 50145337 --hn "Hybrid Reasoning Agents" --hn "HybridAgent latent" --reddit "…"` | barking | HN 成功：c-heat.json、c-hn-50145337.json（384 分 / 187 评，item 接口 383 分）；Reddit 搜索失败：响应被重定向到 old.reddit.com 登录页，非 JSON（已记入 c-heat.json 的 errors） |
| 17:17 | `opencli reddit search "Hybrid Reasoning Agents latent"` | opencli（匿名搜索接口，无登录动作） | 成功：c-reddit-search-opencli.yaml（r/tech_x 215 分 / 84 评；r/LocalLLaMA 30 分 / 12 评；r/hackernews 转贴 1 分 / 1 评） |
| 17:19 | https://api.fxtwitter.com/status/2110400256647340476 | curl | 成功：c-fx-2110400256647340476.raw.json（项目负责人公开帖，2026-10-14 16:00:05 UTC）；公开 JSON，无抓取者信息 |
| 17:08–17:20 | WebSearch × 4（中英文关键词） | WebSearch | 找到 aiweekly、cryptobriefing、explainx、mind-verse、papers.cool、digg、deeplearn.org 等英文页；**未找到中文媒体报道**（机器之心、量子位、新智元、IT之家、36氪均无命中） |

## 三、截图与裁切（barking shot；均为 2 倍像素，宽度 ≤1400 px，只裁切不改像素）

PDF 渲页 `shot pdf --dpi 170`：第 1、4、8、9、10、11、35、36 页（中间图在系统临时目录 `barking-shot/26101018-c/`，不入库）。abs 页用 headless Chrome `shot render`（700 CSS px、DPR 2；opencli 截图未用，抓取走 headless 以免带入浏览器状态），页面为白底，未登录；裁掉顶部 arXiv 导航栏（含 Log in）与下方工具栏。全部记录在 c-shots-log.jsonl（17:11–17:15）。

| 文件 | 来源 | 内容 | 目视确认 |
|---|---|---|---|
| images/25-abs-page.png（1400×1530） | arXiv abs 页渲染 | 标题、作者、完整摘要（含 10.4%、35.5%–54.7%、1.8x–2.6x、2.7x–3.7x）、评论栏、提交历史 | 是，文字清楚，无账号信息 |
| images/26-table1-main-results.png（1130×1042） | PDF p9 | 表 1 全表与完整说明（含 5 个 frontier 行、3 个框架行、8B/14B 块与 HybridAgent 行） | 是，行列完整 |
| images/27-baselines-training-setup.png（1130×705） | PDF p8 | Baselines（frontier 五个模型名）、Models and Training Setups（Qwen3-Base、54K、12K、1K）、Implementation Details（H100、自扩展推理引擎） | 是 |
| images/28-fig6-fig7-efficiency.png（1130×879） | PDF p10 图 6 + p11 5.2 与图 7（两段竖向拼接，段间白色 24 px） | 图 6（Token/Step Usage）与图 7（Wall-Clock Time，134 s / 63 s） | 是，图例、图注、5.2 正文完整 |
| images/29-table9-cost-estimate.png（1130×884） | PDF p35 表 9 + p36 G.2 正文续（两段拼接，段落在页脚处断开） | 表 9 与说明、成本估算前提、排除项 | 是 |
| images/30-claim-10p4-sentence.png（1130×375） | PDF p9 5.1 起首 + p10 续（两段拼接，句子在页面分界处断开） | “17.4%”“18.5%”“10.4% at the RL stage”的原句 | 是 |
| images/31-fig2-latent-readout.png（1130×558） | PDF p4 图 2 | 可读性散点图与问答示例（Latent · 56 steps 等）及图注 | 是 |

共 7 张（派工单建议 3–6 张；多出的一张是图 6/7 与表 9 拆开，便于分别对应 token/延迟/成本）。深色底：无。

## 四、失败与未完成

- Reddit：`barking heat` 的 Reddit 搜索被登录页拦截，改用 opencli 取到帖级数字；未逐条读取 Reddit 评论。
- HN：评论分数不公开，无法给“得分”。
- 中文媒体夸大标题：未找到，未编造。
- 论文附录 J（案例研究）与图 9、图 10 未截图未精读；未运行仓库代码或权重（未验证论文数字能否复现）。
- 未核对 Hugging Face 14B 检查点页与数据集页（只存了 8B 页）。

## 五、自查

- `docs/2610/1018/` 下本组文件 grep 本机用户名、浏览器登录信息：无命中；fxtwitter JSON 与 Reddit/HN 数据均为公开内容，无抓取者头像、昵称或 handle。
- 未改动仓库其他文件，未 commit、未 push，未删除任何文件。b 组、d 组文件不是本组产物，未触碰。
