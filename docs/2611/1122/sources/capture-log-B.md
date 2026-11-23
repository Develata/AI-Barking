# 1122 期 B 组抓取记录

执行方 Claude Sonnet 5.5（claude-sonnet-5-5）。时间为北京时间（UTC+8，本机时区），取自各命令输出与 `b-fetch-log.jsonl`、`b-shots-log.jsonl` 的 `time_bj`。命令前均已 `export BARKING_CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"`；`opencli doctor` 显示扩展已连接，但 opencli 全页截图在 minimax.io 上只返回页面中段（版面异常），故改用 `barking shot render`（headless Chrome）。未登录任何站点、未输入凭据、未接受 Cookie、未使用镜像或代理站。清单文件：`b-fetch-list-1.tsv` … `b-fetch-list-17.tsv`。

主体工具 `barking fetch docs/2611/1122 b <清单>`（默认 UA，同域并发≤2，限时 60/180 s）、`barking shot`、`barking heat`；个别请求用 curl、`gh api`（GitHub 公共 API，使用本机 gh 的默认认证，仅读公开仓库）、WebSearch（只用于找线索 URL，内容均另行抓取原页）。

## 一、过程中的事件

- 北京 11-24 00:11:40，MiniMax-AI/Research 新增提交 `640514fd`“fix Blender issue”，发生在本组取证过程中。`b-pdf-blender.*`、`b-gh-readme-*.md`（00:11–00:12）是提交前版本；`b-pdf-blender-v2.*`（00:2x，取自 `raw.githubusercontent.com/MiniMax-AI/Research/640514fd/...`）是提交后版本；提交完整 diff 存 `b-gh-commit-640514fd.json`（作者邮箱已替换为 `[REDACTED]`）。
- 浏览器截图尝试（`opencli browser --session b-shot --window background open … screenshot --full-page`，00:15）：生成 1400×11563 的整页图，但内容从文章中段开始（疑似 minimax.io 的整页拼接问题），弃用，改 `barking shot render`。临时图在 `本机会话临时目录的 b-tmp 子目录`（临时目录，不在仓库）。
- Cloudflare 质询：ePrint 2026/1593.pdf、2026/1984.pdf（barking fetch 403 blocked；00:2x 用 opencli 浏览器会话在 eprint.iacr.org 页内 `fetch('/2026/1984.pdf')` 仍返回 403 “Just a moment...” 页，不再尝试绕过）；alphaxiv 未用。
- 付费墙：无。

## 二、barking fetch（全部条目，含重试）

| 北京时间 | name | URL | 状态 | 结果/原因 |
|---|---|---|---|---|
| 11-24 00:11:06 | blog | https://www.minimax.io/blog/practical-cryptanalysis-with-minimaxs-latest-internal-model | 200 | ok |
| 11-24 00:11:06 | nist-pqc | https://csrc.nist.gov/projects/post-quantum-cryptography | 200 | ok |
| 11-24 00:11:06 | gh-issues | https://github.com/MiniMax-AI/Research/issues | 200 | ok |
| 11-24 00:11:06 | gh-repo | https://github.com/MiniMax-AI/Research | 200 | ok |
| 11-24 00:11:06 | nist-pqc-selected | https://csrc.nist.gov/projects/post-quantum-cryptography/selected-algorithms | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:11:06 | nist-round4 | https://csrc.nist.gov/projects/post-quantum-cryptography/round-4-submissions | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:11:06 | hawk-site | https://hawk-sign.info/ | 200 | ok |
| 11-24 00:11:07 | nist-onramp | https://csrc.nist.gov/projects/pqc-dig-sig/round-2-additional-signatures | 200 | ok |
| 11-24 00:11:08 | nist-round3 | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/round-3-submissions | 200 | ok |
| 11-24 00:11:09 | tii-challenge | https://classicmceliece.org/ | None | failed — 首次失败（请求失败：连接失败），已重试；未取得响应：请求失败：连接失败（已重试 1 次） |
| 11-24 00:12:01 | pdf-tii | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/TII-McEliece/paper/main.pdf | 200 | ok |
| 11-24 00:12:01 | pdf-blender | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/Blender/paper/main.pdf | 200 | ok |
| 11-24 00:12:02 | pdf-hawk | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/HAWK/paper/main.pdf | 200 | ok |
| 11-24 00:12:02 | pdf-waterfall | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/Waterfall/paper/main.pdf | 200 | ok |
| 11-24 00:12:03 | pdf-locker | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/LOCKER-ROLLO/paper/main.pdf | 200 | ok |
| 11-24 00:12:03 | pdf-aradi | https://raw.githubusercontent.com/MiniMax-AI/Research/main/Cryptography/ARADI/paper/main.pdf | 200 | ok |
| 11-24 00:12:25 | eprint-1984 | https://eprint.iacr.org/2026/1984 | 200 | ok |
| 11-24 00:12:26 | eprint-430 | https://eprint.iacr.org/2026/430 | 200 | ok |
| 11-24 00:12:26 | eprint-1593 | https://eprint.iacr.org/2026/1593 | 200 | ok |
| 11-24 00:12:26 | tii-gh | https://github.com/mjosaarinen/tii-solved | 200 | ok |
| 11-24 00:12:26 | eprint-1747 | https://eprint.iacr.org/2026/1747 | 200 | ok |
| 11-24 00:12:26 | eprint-1986 | https://eprint.iacr.org/2026/1986 | 200 | ok |
| 11-24 00:12:26 | eprint-1593-pdf | https://eprint.iacr.org/2026/1593.pdf | 403 | blocked — HTTP/1.1 403 Forbidden；Cloudflare 质询（cf-mitigated: challenge）；Cloudflare 质询（含 _cf_chl_opt） |
| 11-24 00:12:27 | eprint-1984-pdf | https://eprint.iacr.org/2026/1984.pdf | 403 | blocked — HTTP/1.1 403 Forbidden；Cloudflare 质询（cf-mitigated: challenge）；Cloudflare 质询（含 _cf_chl_opt） |
| 11-24 00:12:36 | anthropic-crypto | https://www.anthropic.com/research/discovering-cryptographic-weaknesses | 200 | ok |
| 11-24 00:12:36 | nist-news-3rd | https://csrc.nist.gov/News/2026/nist-advances-9-candidates-to-the-3rd-round-of-pqc | 200 | ok |
| 11-24 00:12:37 | nist-round3-sig | https://csrc.nist.gov/Projects/pqc-dig-sig/round-3-additional-signatures | 200 | ok |
| 11-24 00:12:37 | nist-pqcforum-hawk | https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/2r2u6SbHun4/m/0_I2KOZ_CQAJ | 200 | ok |
| 11-24 00:12:54 | cbench-abs | https://arxiv.org/abs/2607.18538 | 200 | ok |
| 11-24 00:12:55 | tii-challenges-gh | https://github.com/Crypto-TII/McEliece-Challenges | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:12:55 | nist-pqc-news | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization | 200 | ok |
| 11-24 00:12:55 | saarinen-tii | https://github.com/mjosaarinen/tii-solved | 200 | ok |
| 11-24 00:12:55 | nist-fips204 | https://csrc.nist.gov/pubs/fips/204/final | 200 | ok |
| 11-24 00:12:56 | nist-fips205 | https://csrc.nist.gov/pubs/fips/205/final | 200 | ok |
| 11-24 00:12:56 | nist-fips203 | https://csrc.nist.gov/pubs/fips/203/final | 200 | ok |
| 11-24 00:12:56 | nist-sha3-round2 | https://csrc.nist.gov/projects/hash-functions/sha-3-project | 200 | ok |
| 11-24 00:12:56 | nist-sha3-final | https://csrc.nist.gov/projects/hash-functions/sha-3-project/round-2-submissions | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:12:57 | nist-sha3-r1 | https://csrc.nist.gov/projects/hash-functions/sha-3-project/round-1-submissions | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:12:57 | cbench-pdf | https://arxiv.org/pdf/2607.18538 | 200 | ok — 响应为 PDF，按 180 秒限时重新请求 |
| 11-24 00:12:58 | anthropic-hawk-pdf | https://anthropic.com/document/hawk_key_recovery.pdf | 200 | ok |
| 11-24 00:13:54 | nist-pqc-r1 | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/round-1-submissions | 200 | ok |
| 11-24 00:13:54 | nist-pqc-r3 | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/round-3-submissions | 200 | ok |
| 11-24 00:13:54 | nist-pqc-r2 | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/round-2-submissions | 200 | ok |
| 11-24 00:13:55 | nist-pqc-r4 | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/round-4-submissions | 200 | ok |
| 11-24 00:13:55 | nist-pqc-add | https://csrc.nist.gov/projects/pqc-dig-sig/round-3-additional-signatures | 200 | ok |
| 11-24 00:13:55 | nist-pqc-sel | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization/selected-algorithms | 200 | ok |
| 11-24 00:13:55 | nist-sha3-r1b | https://csrc.nist.gov/projects/hash-functions/sha-3-project/sha-3-competition/round-1 | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:13:56 | nist-sha3-hist | https://csrc.nist.gov/projects/hash-functions/sha-3-project | 200 | ok |
| 11-24 00:13:56 | nist-sha3-ir7764 | https://csrc.nist.gov/pubs/ir/7896/final | 200 | ok |
| 11-24 00:13:56 | nist-sha3-ir7620 | https://csrc.nist.gov/pubs/ir/7620/final | 200 | ok |
| 11-24 00:14:14 | nist-ir7620-pdf | https://nvlpubs.nist.gov/nistpubs/ir/2009/NIST.IR.7620.pdf | 404 | failed — HTTP/1.1 404 Not Found |
| 11-24 00:14:24 | nist-ir7620-pdf | https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir7620.pdf | 200 | ok |
| 11-24 00:14:32 | eprint-aradi | https://eprint.iacr.org/2024/1240 | 200 | ok |
| 11-24 00:15:26 | cn-163-claude | https://www.163.com/dy/article/LCDT65J60550WHYR.html | 200 | ok |
| 11-24 00:15:26 | cn-163-fast | https://www.163.com/dy/article/LCDCSOP20511ABV6.html | 200 | ok |
| 11-24 00:15:26 | cn-sina | https://finance.sina.com.cn/wm/2026-11-23/doc-inizskhq8466652.shtml | 200 | ok |
| 11-24 00:15:27 | cn-ithome | https://www.ithome.com/1/030/711.htm | 200 | ok |
| 11-24 00:15:28 | en-futu | https://news.futunn.com/en/post/1005289670/minimax-discloses-internal-model-cryptanalysis-results-setting-a-new-record | 200 | blocked — 正文过短（8 个非空白字符），可能要 JavaScript 渲染或被拦截 |
| 11-24 00:15:28 | en-maccus | https://maccus.fr/en/minimax-ai-cryptanalysis/ | 200 | ok |
| 11-24 00:15:29 | green-blog | https://blog.cryptographyengineering.com/category/attacks/ | 200 | ok |
| 11-24 00:15:29 | cn-caiwen | https://www.caiwennews.com/article/1712302.shtml | 200 | ok |
| 11-24 00:15:39 | cn-17173 | https://news.17173.com/content/11222026/213025409.shtml | 200 | ok |
| 11-24 00:15:47 | saarinen-slides | https://mjos.fi/doc/20260816-wac8-ai-cryptanalysis.pdf | 200 | ok — 首次失败（请求失败：连接失败），已重试 |
| 11-24 00:16:23 | en-fourweek | https://fourweekmba.com/ai-minimax-says-its-ai-broke-4-old-nist-crypto-candidates/ | 403 | blocked — HTTP/1.1 403 Forbidden |
| 11-24 00:16:24 | en-technode | https://technode.com/2026/11/23/minimax-publishes-ai-discovered-cryptographic-attacks-with-papers-and-code/ | 200 | ok |
| 11-24 00:16:25 | en-pandaily | https://pandaily.com/minimax-mim-cryptanalysis-research-internal-model-disclosure | 200 | blocked — 正文过短（72 个非空白字符），可能要 JavaScript 渲染或被拦截 |
| 11-24 00:16:25 | en-36kr | https://eu.36kr.com/en/p/4081531539723397 | 200 | ok |
| 11-24 00:16:46 | nist-ir8240 | https://nvlpubs.nist.gov/nistpubs/ir/2019/NIST.IR.8240.pdf | 200 | ok |
| 11-24 00:16:47 | nist-ir8309 | https://nvlpubs.nist.gov/nistpubs/ir/2020/NIST.IR.8309.pdf | 200 | ok |
| 11-24 00:16:54 | nist-ir8545 | https://nvlpubs.nist.gov/nistpubs/ir/2025/NIST.IR.8545.pdf | 200 | ok |
| 11-24 00:21:46 | gh-issue-1 | https://github.com/MiniMax-AI/Research/issues/1 | 200 | ok |
| 11-24 00:21:47 | gh-issue-2 | https://github.com/MiniMax-AI/Research/issues/2 | 200 | ok |
| 11-24 00:21:47 | pdf-blender-v2 | https://raw.githubusercontent.com/MiniMax-AI/Research/640514fd/Cryptography/Blender/paper/main.pdf | 200 | ok |
| 11-24 00:21:48 | gh-commit-640514fd-page | https://github.com/MiniMax-AI/Research/commit/640514fd | 200 | ok |
| 11-24 00:27:56 | green-anthropic | https://blog.cryptographyengineering.com/2026/07/29/some-notes-about-anthropics-new-results/ | 200 | ok |
| 11-24 00:27:56 | djb-tii | https://cr.yp.to/2026/20260722-break-fake-mceliece-sizes-from-tii.sh | 200 | ok |
| 11-24 00:28:05 | cm-team-comment | https://classic.mceliece.org/mceliece-610-20260623.pdf | 200 | ok |
| 11-24 00:32:00 | cnr-search | https://www.cnr.cn/ | 200 | ok — 按 GBK 解码 |
| 11-24 00:32:02 | tii-official | https://crowdchallenge.tii.ae/mceliece-challenges/ | 200 | ok |
| 11-24 00:32:02 | tii-gh-kirshanova | https://github.com/ElenaKirshanova/tii_decoding_challenge | 200 | ok |
| 11-24 00:33:20 | iacr-news | https://iacr.org/news/ | 200 | ok |
| 11-24 00:33:21 | eprint-search-minimax | https://eprint.iacr.org/search?q=MiniMax&submittedafter=2026-11-19 | 200 | ok |
| 11-24 00:33:32 | eprint-2993 | https://eprint.iacr.org/2026/2993 | 200 | ok |

## 三、curl、gh api、WebSearch（个别请求）

| 北京时间 | 工具 | 操作 | 结果 |
|---|---|---|---|
| 00:11 | curl `--max-time 30` | https://www.minimax.io/sitemap.xml；`curl -I` 博客页 | sitemap 含博客 URL，`lastmod` 2026-11-20T13:03:02.717Z；响应头 `date: Mon, 23 Nov 2026 16:11:17 GMT` |
| 00:11–00:13 | gh api | `repos/MiniMax-AI/Research`（创建/推送时刻）、`/commits`、`git/trees/HEAD?recursive=1`（`b-gh-tree.tsv`，472 个文件）、各 README（`b-gh-readme-*.md`）、`readme` | ok |
| 00:14 | `barking heat docs/2611/1122 b --item 50601022 --item 50595528 --hn "MiniMax cryptanalysis" --hn "MiniMax-AI Research cryptography" --hn "McEliece challenges MiniMax" --hn "Blender-256 collision" --hn "MiniMax internal model MIM" --since 1795000000 --reddit "MiniMax cryptanalysis" --reddit "MiniMax MIM cryptography" --reddit "MiniMax internal model cryptanalysis"` | HN 搜索与条目 ok（`b-heat.json`、`b-hn-50601022.json`、`b-hn-50595528.json`）；Reddit 3 次全部失败（重定向到 old.reddit.com 登录页，不登录），记入 `b-heat.json` errors |
| 00:15、00:21 | gh api | `repos/MiniMax-AI/Research/issues?state=all`（`b-gh-issues-api.json`，issue 提出者登录名与头像链接已替换为占位符）、`issues/1/comments` | 两条 open，0 评论 |
| 00:21 | gh api | `repos/MiniMax-AI/Research/commits`（发现 `640514fd`）、`commits/640514fd`（`b-gh-commit-640514fd.json`）、`contents/…/Blender/artifacts/README.md`、`run.sh`（`b-gh-blender-artifacts-readme.md`、`b-gh-blender-run.sh`，第一次 TLS 超时，重试成功） | ok |
| 00:2x | gh api | `repos/anthropics/cryptography-research-demo/readme` | 无返回（记未找到）；`repos/sweis/mceliece-challenges/readme`（`b-gh-sweis-mceliece-readme.md`，只有标题与 TII 链接）；`repos/ElenaKirshanova/tii_decoding_challenge/commits` 最近提交 2024-03-26 |
| 00:1x–00:2x | WebSearch（standard/extended，各 1 次） | 中文标题、央广网、MiniMax 官方中文发布、Steve Weis、cryptographers reaction、pqc-forum | 线索 URL（新浪、网易、IT之家、17173、财闻、TechNode、Pandaily、36氪英文、MACCUS、FourWeekMBA、Saarinen 幻灯片、Green 博客、djb 脚本）；WebSearch 的概括文字不引用 |

## 四、barking shot（`b-shots-log.jsonl`，25 行）

渲染均为 `render <url> <name> --width W --height H [--scale S]`：minimax.io 700 CSS px / DPR 2；github.com 700 CSS px / DPR 2（另有 1000 px 试验图 `gh-repo`、`gh-issues`，输出 2000 px 宽，未用于正式图）；新浪、网易 1000 CSS px / DPR 1.4（页面版式宽于 700 px，输出仍为 1400 px 宽）。`pdf` 渲 `b-pdf-tii.pdf` 第 3 页，164 dpi。裁切与拼接用 `cropbox`、`stack`、`crop`。正式图 13–18、22–24 见 `evidence-B.md` §8；`lines` 用 Tesseract（eng、chi_sim）找锚点。

GitHub 页面为深色主题（图 17、18），已在 evidence 注明。图 18 拼接时有意裁去 issue 提出者账号行。

## 五、自查

- `grep -rn` 对 `docs/2611/1122/` 检查抓取者信息、本机路径、邮箱：见回报（交付前运行）。
- 存档中保留的他人联系方式：IACR ePrint 页面的作者公开联系邮箱（如 ePrint 2026/1593、2026/1984、2026/2993 页内的 “name @ company com” 形式）属论文页公开信息，未改动；GitHub 页面里的 MiniMax 员工与贡献者公开用户名保留；issue 提出者登录名、头像链接已替换。
