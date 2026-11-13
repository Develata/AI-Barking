# 1112 期 C 组抓取日志（Edge0-145B“笔记本能跑”）

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（UTC+8，本机时区），取自 `c-fetch-log.jsonl`、`c-shots-log.jsonl`、`c-heat.json` 与终端时间。文件前缀 `c-`。未登录任何账户，未提交表单、未发帖；未下载模型权重或推理包，只存文本（HTML/JSON/MD/TXT）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 22:29:05 | edge0.ai 博客主文、首页、/aboutus、/blog、/research；docs.edge0.ai；huggingface.co/Edge0（组织页）、Edge0-145B-NVFP4 页 + raw README + API；Edge0-120B-NVFP4 页 + API；github.com/Edge0-AI/Edge0 页 + raw README；v0.2.0 release 页；GitHub 组织页（`c-list1.tsv`） | `barking fetch`（17 条） | 16 条 ok；**`hf-145b-config`（raw config.json）404 failed**（该仓库无 config.json，未再试）。`.txt` 已逐一打开确认是正文 |
| 22:29:40 | GitHub：模型文档、edge0-strata/EDGE0.md、edge0-infer 145B/120B 文档、模型许可证与 NOTICE、README_zh、commits / repo / releases / issues API；HF：145B-q3k 页与 API、NOTICE、release-scope.json、model.json、commits API；Qwen3.8-Flash-Next 卡页与 API（`c-list2.tsv`） | `barking fetch`（19 条） | 19 条 ok |
| 22:30 | raw.githubusercontent.com/Edge0-AI/Edge0/b7900cbb/README.md 与 docs/models/edge0-145b.md；huggingface.co/Edge0/Edge0-145B-NVFP4/raw/6aab5f43/README.md | curl（`--max-time 60`） | 200，存 `c-gh-readme-b7900cbb.md`、`c-gh-model-doc-b7900cbb.md`、`c-hf-145b-readme-6aab5f43.md`（博客发布前修订） |
| 22:30 | docs.edge0.ai/llms.txt | curl | 200，存 `c-docs-llms.txt`（目录只含 8B/35B，无 145B 页） |
| 22:30:45 | HN item 50475713 与 `barking heat`（HN 查询 “Edge0-145B”“edge0.ai”；Reddit 查询 “Edge0-145B”“Edge0 145B”） | `barking heat` | HN ok（`c-heat.json`、`c-hn-50475713.json`）；**Reddit 两个查询均失败**（响应为 old.reddit.com 登录重定向的 HTML，非 JSON），未重试 |
| 约 22:31 | `opencli reddit search "Edge0-145B" --subreddit LocalLLaMA` | opencli（超时 90 s） | **失败**：SECURITY_BLOCK，Reddit 对该端点返回 403，提示不是登录失效；按规则不重试、不绕过 |
| 约 22:31 | Reddit 域内 WebSearch | WebSearch | 工具拒绝（该域名不可访问） |
| 22:31:40 | 第三方报道：dev.to、emergent.sh、byteiota、cctest.ai、activepieces、news.lavx.hu、ai-tldr.dev（`c-list3.tsv`） | `barking fetch`（7 条） | 7 条 ok |
| 约 22:32 | hn.algolia.com search_by_date（query=Edge0，近一周） | curl | 200，存 `c-hn-algolia-edge0.json`；Edge0 相关帖仅 50475713 |
| 22:30–22:40 | WebSearch 5 次（英文 “Edge0-145B laptop 29GB”、中文 “Edge0-145B 笔记本 本地运行 1450亿参数”、`"Edge0-145B"`、中文 “… 苹果 Qwen3.8-Flash-Next 衍生”、中文 “… 29GB 报道”） | WebSearch | 仅作线索与定位；引文都回到本地存档核对。第 1、2 次检索未收录 11-12 后的页面；后三次得到报道列表（共 5 次，含 1 次被拒的 Reddit 域内检索另列）。**未找到中文媒体报道** |

被拒或失败：`hf-145b-config` 404；Reddit（`barking heat` 两查询、opencli 403、WebSearch 域名限制）；daily.dev 两帖（未存档，仅搜索结果列出）；ifanr 那篇（RTX Spark 笔记本，与 Edge0 无关，未存档）。

## 截图

浏览器：headless Chrome（`BARKING_CHROME` 指向 `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`，临时 user-data-dir，匿名，未登录），CSS 宽 700、DPR 2，输出宽 1400 px。`barking shot render` 出中间图（在会话临时目录，不入库；博客页 60000 px 高一张 `blog2`），`cropbox` / `stack` 裁切、拼接（不改像素）。中间图预览切片与辅助脚本 `slice.py`（Pillow）在会话 scratchpad，不入库。深色底页面：博客（深色）、HF（深色主题 + 白底卡片）、GitHub（深色主题）；HN、emergent、DEV 为浅色底。

| 文件 | 来源页 | 渲染 / 裁切时刻 |
|---|---|---|
| 25–31 | edge0.ai 博客（`blog2` 整页 60000 px 高渲染，22:34:03） | 裁切 22:35:07–22:36:49 |
| 32–33 | huggingface.co/Edge0/Edge0-145B-NVFP4（`hf145`） | 渲染 22:36:59；裁切 / 拼接 22:37:44 |
| 34 | github.com/Edge0-AI/Edge0（`ghrepo`） | 渲染 22:37:08；裁切 22:38:02 |
| 35 | news.ycombinator.com/item?id=50475713（`hn`） | 渲染 22:38:11；拼接 22:39:40 |
| 36 | emergent.sh 报道页 + dev.to 报道页（`emergent`、`devto`） | 渲染 22:39:03 与 22:39:20；拼接约 22:39:50 |

记录缺漏：36 号图的 `stack` 命令因输出管道被 `tail` 截断触发 “Broken pipe”，图已生成，但 `c-shots-log.jsonl` 里**没有**对应记录。该图的来源与框为：`emergent` 的 [0,200,1400,2440] 与 `devto` 的 [0,715,1400,1860] 两段，间隔 24 px。

自查：目视逐张打开 25–36（25 末行字下缘略贴边；33、35、36 为拼接图，间隔色为页面主色）。对 `docs/2611/1112/` 下 `c-` 文件与图片检索抓取者标识（本机用户名、邮箱、浏览器配置路径）：无命中（检索 “/Users/” 时有对 GitHub / HF 页面里 `/users/…` 路径的误报，均为站点自身链接）。HF 与 GitHub 页面均为匿名抓取，页面上的 “Log In” 为站点自身按钮；HF 组织页 HTML 含站点成员头像链接（`bupalinyu`），是 HF 公开页面内容，非抓取者。单个文件超过 1 MB 的：无文本文件超过 1 MB（最大 `c-qwen-card.html` 532 KB，HTML 原件不入库）；`images/36-media-headlines-laptop.png` 约 1.4 MB（图片不入库）。

## 命令与脚本

- 清单文件：`c-list1.tsv`、`c-list2.tsv`、`c-list3.tsv`（`barking fetch` 输入）。
- 无其他自写脚本入库。我曾误用一次 `rm -f` 被权限拦截，已改用不删文件的命名方式；无文件被删除。
