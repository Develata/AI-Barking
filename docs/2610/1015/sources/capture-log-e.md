# 1015 期 E 组抓取日志

时区：北京时间（UTC+8）。工具：`barking fetch`（网页与 PDF 存档，限时并发）、`barking heat`（HN / Reddit 热度）。未截图，未使用 opencli、firecrawl、WebFetch 或 curl；未登录，未访问付费墙与镜像站。

## 一、网页存档（barking fetch）

清单：`sources/e-manifest.tsv`（本组自写的输入清单，`name<TAB>url`）。命令：`barking.exe fetch docs/2610/1015 e docs/2610/1015/sources/e-manifest.tsv`，开始于 2026-10-16 19:40:06，用时 4 秒，并发 6，同一域名至多 2。结果：ok 9，blocked 0，failed 0。明细见 `sources/e-fetch-log.jsonl`。

| name | URL（已去查询参数） | HTTP | 结果 | 字节 | 存档 |
|---|---|---|---|---|---|
| goodfire | https://already-clean.goodfire.com/ | 200 | ok | 2176329 | e-goodfire.html / .txt / .headers |
| trolley | https://zachlandes.com/trolley-problem/ | 200 | ok | 473672 | e-trolley.* |
| krea-blog | https://www.krea.ai/blog/introducing-krea-3 | 200 | ok | 117429 | e-krea-blog.* |
| krea-page | https://www.krea.ai/krea-3 | 200 | ok | 414507 | e-krea-page.* |
| flux-bfl | https://bfl.ai/blog/flux-3-image-klein | 200 | ok | 251498 | e-flux-bfl.* |
| flux-hf | https://huggingface.co/blog/flux-3 | 200 | ok | 270904 | e-flux-hf.* |
| simons | https://www.simonsfoundation.org/news/simons-foundation-launches-15m-effort-to-help-mathematicians-navigate-ai-era/ | 200 | ok | 256638 | e-simons.* |
| nvidia | https://blogs.nvidia.com/blog/nemotron-3-5-super-vl/ | 200 | ok | 128710 | e-nvidia.* |
| metr | https://www.anthropic.com/news/an-update-on-independent-evaluators | 200 | ok | 130334 | e-metr.* |

注：各页 HTTP 200 后已逐页看过 .txt 开头与关键段落，确认为正文而非验证页。页面日期见 evidence-e.md。`.headers` 为 barking 按公开规则重写后的字段，未含抓取地点或 IP。

## 二、补充存档（第二批）

清单：`sources/e-manifest-2.tsv`。命令同上，开始于 2026-10-16 19:46:07，用时 2 秒，结果 ok 4，blocked 0，failed 0。

| name | URL | 结果 | 存档 |
|---|---|---|---|
| nv-hf-nvfp4 | https://huggingface.co/nvidia/NVIDIA-Nemotron-3.5-Super-VL-120B-A12B-NVFP4 | ok | e-nv-hf-nvfp4.html / .txt（卡片含 License: openmdw-1.1） |
| nv-dev-blog | https://developer.nvidia.com/blog/nvidia-nemotron-3-5-super-vl-powers-specialized-agents-with-multimodal-reasoning/ | ok | e-nv-dev-blog.html / .txt |
| goodfire-essay-md | https://already-clean.goodfire.com/essay.md | ok | e-goodfire-essay-md.md（纯文本版，无 .txt） |
| goodfire-research | https://www.goodfire.com/research | ok | e-goodfire-research.html / .txt（列出 “October 15, 2026”） |

未取：E2 的 Jev 开发方（TypeSafe）文档（派工未要求）。E6 的许可名由 NVFP4 卡片元数据给出，未另取 NVIDIA 许可全文页。

## 三、热度（barking heat）

命令形式：`barking.exe heat docs/2610/1015 e --hn "…" --reddit "…"`。

- 第一次运行：19:40:15，HN 查询 “Neural Networks Are Already Clean”、“trolley problem AI”、“Krea 3”、“FLUX 3 Image Klein”、“Simons Foundation mathematicians AI”、“Nemotron 3.5”、“independent evaluators METR Anthropic”；Reddit 查询 “Neural Networks Are Already Clean”、“Krea 3”。该次写入的 `e-heat.json` 因工具拒绝覆盖而未能写入（工具报错：已有 e-heat.json，拒绝覆盖）。该次输出已见，但有一条查询（trolley problem AI）为 0 命中，故改用更精确的查询重跑。
- 第一次生成的 `e-heat.json` 是本组试验文件，已移到会话暂存目录（仓库外），不在仓库内。
- 第二次运行：19:42:15，HN 查询 “Neural Networks Are Already Clean”、“Jev trolley”、“Jev”、“trolley problem”、“Krea 3”、“FLUX 3 Image Klein”、“Simons Foundation mathematicians AI”、“Nemotron 3.5”、“Anthropic employee-like access METR”；Reddit 查询 “Neural Networks Are Already Clean”、“Krea 3”。HN 9 组正常返回，写入 `sources/e-heat.json`。
- Reddit 两次运行共 4 次查询全部失败：old.reddit.com 的 search.json 被重定向到登录页（最终 URL 为 `old.reddit.com/login/...`，Content-Type text/html），解析不了 JSON。同一来源已失败两次，按派工规则未再重试，Reddit 热度记为失败。

## 四、时间线

| 北京时间 | 动作 | 结果 |
|---|---|---|
| 19:39 | 读派工单、模板、上期日志 | — |
| 19:40:06 | barking fetch（9 个 URL） | ok 9 |
| 19:40:15 | barking heat 第一次 | HN 命中，Reddit 失败；写入被拒 |
| 19:42:15 | barking heat 第二次（移走试验文件后） | HN 9 组，Reddit 失败 |
| 19:46:07 | barking fetch 第二批（4 个 URL） | ok 4 |
| 之后 | 读存档文本，摘句，写 evidence-e.md 与本日志 | — |

## 五、假设与说明

- 官方页面未给时刻的，只写日期，不编造时刻；HN 时刻为 HN 投稿时间（北京时间）。
- E1 的 Last-Modified（2026-10-16 06:36 GMT）是页面修改头，不当作发布时刻；发布日期取自 Goodfire 研究索引（October 15, 2026）。
- E2 为个人博客，级别未定（作者与 Jev 开发方的关系未核），只按原文摘句，不按 L1/L2 署名。
- 社区热度按派工只用 barking heat（HN 与 Reddit）；Reddit 失败后未补其他社区。
- 派工单说明 D/E 组不需要截图、不需要“可能的吠点”与夸大实例，本组据此未做。
