# 1008 C 组抓取日志（速览）

仅 C 组；基线 commit `35910be`。未 commit、未 push、未删除他人文件；未触碰 `docs/2610/1005/sources/c-repo-tree.json` 与其他组文件；不需要截图，未做。
时间一律北京时间（UTC+8）；本机时钟为美东，取证起点 `date -u` = 2026-10-09 01:37Z = 北京 09:37。逐条时刻按“约”记（命令连续执行，未逐条记秒）。
工具：curl（普通浏览器 User-Agent）、firecrawl（scrape / search）、Scoop 的 poppler `pdftotext -enc UTF-8 -layout`（PDF 文本）、Python 标准库（`c-heat.py`、`c-c1-numcheck.py`；HTML 转文本沿用 1007 期 `a-html2txt.py`）。未使用浏览器会话、未读取本机浏览器 Cookie、未登录、未发帖或点赞、未绕过付费墙。

## 抓取登记

| 约北京时间（10/9） | URL | 工具 | 结果 |
|---|---|---|---|
| 09:37 | https://arxiv.org/abs/2609.24927 、https://arxiv.org/abs/2609.24927v2 | curl | 200；`c-arxiv-abs.html`（43 KB）、`c-arxiv-listing.html`（v2 摘要页，同内容） |
| 09:37 | https://arxiv.org/pdf/2609.24927v2 、…v1 | curl | 200；`c-arxiv-2609.24927v2.pdf`（1,819,037 字节）、`…v1.pdf`（1,818,062 字节，本地）；`pdftotext -layout` → `.txt`（v1 1208 行、v2 1219 行，20 页，以 \f 分页） |
| 09:37 | http://export.arxiv.org/api/query?id_list=2609.24927 | curl | 200；`c-arxiv-api.xml`（published 2026-09-21T17:22:44Z；updated 2026-09-25T18:15:11Z；评论“20 pages, 10 tables, 4 figures”） |
| 09:38 | https://raw.githubusercontent.com/pingdotgg/ts-rust/main/README.md | curl | 200，14,316 字节；`c-ts-rust-readme.md` |
| 09:38 | https://api.github.com/repos/pingdotgg/ts-rust （及 /commits?per_page=100、/git/trees/HEAD?recursive=1、/contributors、/releases、/commits?path=README.md） | curl | 200；`c-ts-rust-repo.json`、`c-ts-rust-commits-latest100.json`（精简）、`c-ts-rust-tree.json`（精简）、`c-ts-rust-contributors.json`、`c-ts-rust-releases.json`、`c-ts-rust-readme-commits.json`（38 次 README 提交） |
| 09:39 | https://raw.githubusercontent.com/pingdotgg/ts-rust/{adb41aac,db8f30ba,934345c4,c1e5e5cd,26f69b80,e8993f51,9b9c101c}/README.md | curl | 全部 200；`c-ts-rust-readme-<sha>.md`（对比 README 措辞演进） |
| 09:39 | https://registry.npmjs.org/tsc-rs | curl | 200；`c-npm-tsc-rs.json`（版本时间戳） |
| 09:38 | https://raw.githubusercontent.com/Queuingtheorydotcom/11SquaresFormalized/main/{README.md, docs/VERIFICATION_20261006.md} 与 https://api.github.com/repos/Queuingtheorydotcom/11SquaresFormalized（及 /commits、/git/trees/HEAD?recursive=1） | curl | 200；`c-11sq-readme.md`、`c-11sq-verif.md`、`c-11sq-repo.json`、`c-11sq-commits.json`（4 次提交）、`c-11sq-tree.json`（精简，原 2.9 MB） |
| 09:43 | 同仓库 ACKNOWLEDGEMENTS.md、PROVENANCE.md、MISSING.md、AGENTS.md、PC_RESUME.md、SIMPLIFICATION_HANDOFF.md、docs/PUBLICATION.md；verification/completed-run-20261006/{summary,independent-review,provenance}.json | curl | 200；`c-11sq-*.md`、`c-11sq-run-*.json`；`final-audit.json.gz`（965 KB）未下载 |
| 09:44 | 同仓库 commit `b237948f` 的 README.md、ACKNOWLEDGEMENTS.md、PROVENANCE.md | curl | README 200（`c-11sq-old-b237948f-README.md`）；另两个 404（当时尚无） |
| 09:44 | https://github.com/EvolvingPrograms/11SquaresEvolving 及其 Actions 运行页 37414883750；api.github.com 同路径 | curl | 404（私有） |
| 09:41 | https://news.ycombinator.com / hn.algolia.com：search?query=11SquaresFormalized；items/49993121 | curl | 200；`c-hn-11sq.json`、`c-hn-49993121.json` |
| 09:42 | https://raw.githubusercontent.com/Queuingtheorydotcom/11SquaresOptimal/main/{README.md, PROOF.md, docs/PUBLICATION.md, docs/REPRODUCING.md, paper/README.md}；api.github.com 仓库与目录树 | curl | 200；`c-11sqopt-*.md`、`c-11sqopt-repo.json`、`c-11sqopt-tree.json`（精简）；grep 无任何模型名 |
| 09:44 | https://jlevy.github.io/squares/cases/11.html | curl | 200，417,654 字节；`c-jlevy-11.html`（本地）、`c-jlevy-11.txt` |
| 09:45 | https://www.cnbc.com/2026/10/08/open-ai-revenue-nvidia-oracle-coreweave.html | curl | 200，838,816 字节；`c-cnbc.html`（本地）；正文从 `<p>` 抽出 → `c-cnbc.txt` |
| 09:45 | https://claude.com/resources/articles/dashboards-and-motion | curl | 200，508,855 字节；`c-claude-dash.html`（本地）、`c-claude-dash.txt` |
| 09:45 | https://www.anthropic.com/news/anthropic-cyber-mission | curl | 200，216,975 字节；`c-anthropic-cyber.html`（本地）、`.txt` |
| 09:46 | https://www.anthropic.com/research/launching-opt-in-vuln-finding-service-for-open-source 、https://red.anthropic.com/oss-scanner | curl | 200；`c-oss-scanner-research.*`、`c-oss-scanner-red.*` |
| 09:45 | https://cactuscompute.com/blog/whistle | curl | 200，124,152 字节；`c-whistle.html`（本地）、`c-whistle.txt` |
| 09:46 | https://huggingface.co/api/models/Cactus-Compute/whistle 、…/raw/main/README.md | curl | 200；`c-whistle-hf.json`、`c-whistle-hf-readme.md` |
| 09:45 | https://www.reuters.com/legal/legalindustry/usa-today-sues-openai-copyright-infringement-over-ai-training-2026-10-08/ | curl → firecrawl_scrape | curl 401（774 字节，已删除）；firecrawl 200，返回完整正文，无付费墙提示；正文手工存入 `c-reuters-usatoday.txt` |
| 09:47 | https://www.documentcloud.org/documents/28731453-usa-today-v-openai/ 、https://s3.documentcloud.org/documents/28731453/usa-today-v-openai.pdf 、https://tmsnrt.rs/4hMudzE | curl | 200；`c-usatoday-complaint-dc.html`、`c-usatoday-complaint.pdf`（4,208,852 字节，本地，79 页）→ `c-usatoday-complaint.txt`；tmsnrt.rs 重定向到 fingfx.thomsonreuters.com 的 PDF（头部 200，未另存） |
| 09:48 | firecrawl_search：USA Today Gannett sues OpenAI … | firecrawl | 摘要含 The Verge 跟进、DocumentCloud 链接；见 `c-search-snippets.md` |
| 09:50 | firecrawl_search：Et Tu Brute … Quartz / Fast Company；Bloomberg … OpenAI differs …；FT OpenAI annualised revenue $50bn；OpenAI spokesperson … | firecrawl | 摘要见 `c-search-snippets.md`；Fast Company 无对应结果；FT 摘要含 $70bn 字样；未找到 OpenAI 公开声明 |
| 09:51 | https://qz.com/ai-chatbots-claude-chatgpt-wealth-pricing-study-100726 | curl | 200，353,428 字节；`c-quartz.html`（本地）、`c-quartz.txt` |
| 09:52 | https://digiko.io/ai-money/ai-agents-pick-198-pricier-flights-for-the-rich-set-a-dollar-cap | firecrawl_scrape | 200；已读，仅在 `c-search-snippets.md` 摘要（L5，未存全文） |
| 09:52 | https://finance.yahoo.com/technology/article/openais-annualized-revenue-20-billion-lower-than-prior-investor-estimates-174058048.html | firecrawl_scrape | 200（cacheState hit）；正文存 `c-yahoo-ft-revenue.txt` |
| 09:49 | https://www.ft.com/content/b66a9858-f8fb-46cb-b506-44bfe26fca2a | curl | **403**（付费墙/拦截页，270 KB）；未绕过；该文件已删除 |
| 09:53 | https://www.reuters.com/technology/openais-annual-recurring-revenue-nears-70-billion-axios-reports-2026-09-29/ | curl | **401**；已删除；仅标题级线索（来自搜索摘要） |
| 09:55 | hn.algolia.com 搜索（`c-heat.py`）与 items/{50000676,50008187,50008427} | curl（Python urllib） | 200；`c-heat.json`、`c-hn-50000676.json`、`c-hn-50008187.json`、`c-hn-50008427.json` |
| 09:58 | https://www.reddit.com/r/mathematics/comments/1wzf3ra/ | curl（.json）→ firecrawl | curl **403**（已删除返回的 HTML）；firecrawl 回“不支持该站点”；未取得 |
| 10:00 | https://pingyou.com/papers/eleven-squares.pdf | curl | 200，1.77 MB。这是 HN 评论里被引的另一篇论文（Fibonacci 几何），与 C3 的 Lean 仓库无关；pdftotext 后确认无关，**已删除 PDF 与 txt**（自己的试验文件） |
| 10:02 | https://erich-friedman.github.io/packing/squinsqu/ | curl | 仅 154 字节，不含正文；已删除 |

## 脚本

- `c-heat.py`：HN Algolia 查询（关键词表在脚本内），输出 `c-heat.json`。
- `c-c1-numcheck.py` → `c-c1-numcheck.txt`：在论文 v2 文本中定位各数字所在 PDF 页，并用 Table 7 复算“封锁非金融属性后保险差距的百分比变化”。需先有 `c-arxiv-2609.24927v2.txt`。
- 文本转换：`python docs/2610/1007/sources/a-html2txt.py in.html out.txt`（沿用，未复制）。

## 自己删除的试验/无效文件（均为本组刚生成）

`c-ft-openai-revenue.html`（FT 403 页）、`c-reuters-axios-070.html`（401 页）、`c-reddit-11sq-mathematics.json`（403 页）、`c-11sq-contributors.json`（空数组）、`c-11sq-paper.pdf` / `c-11sq-paper.txt`（无关论文）、`c-friedman-squinsqu.html` / `.txt`（154 字节）、`c-11sq-old-b237948f-ACKNOWLEDGEMENTS.md` / `-PROVENANCE.md`（404 响应）。
精简（覆盖写回同名文件）：`c-11sq-tree.json`（2.9 MB → 约 4 KB）、`c-ts-rust-tree.json`（636 KB → 约 2 KB）、`c-ts-rust-commits-first.json` 精简后改名 `c-ts-rust-commits-latest100.json`（501 KB → 约 25 KB）、`c-11sqopt-tree.json`。

## 假设与偏离

- 页码按 pdftotext 以 \f 分页的顺序，与论文页脚印刷页码一致。
- Reuters 页面用 firecrawl 取到完整正文，且返回内容里没有付费墙提示，故视为公开可读；FT 与 Bloomberg 未试图绕过。
- 隐私检查：本组存档为公开网页/API 文本，未使用登录会话，无抓取者头像、显示名或 handle；交付前已对 `docs/2610/1008/` 做 grep 自查（见回报）。
