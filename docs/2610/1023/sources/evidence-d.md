# 1023 期 D 组（速览）取证表

执行方：Claude Haiku 5.5（claude-haiku-5-5），2026-10-24 取证。北京时间 = UTC+8。官方页只有日期的，写“官方未给时刻”。本组不截图（派工单规定），只存档与摘句。

## 一、逐条事实

| 条目 | 一句事实（照原文措辞） | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 存档文件 | 发布时刻 | 限定词 | 状态 |
|---|---|---|---|---|---|---|---|---|
| D1 Claude Duet 安全说明【跟进1021】 | Anthropic 称，断开应用后，新访问即刻停止，但 Claude Duet 已从该应用学到的内容保留，直到用户重置其记忆。 | L1 | https://claude.com/resources/articles/claude-duet-safety-privacy-and-security | “Disconnecting an app stops new access from that moment, but what Claude Duet already learned from it stays until you reset its memory, which you can do anytime from the app’s settings.” | `d-duet.html`、`d-duet.txt`（`d-duet.headers`） | 页面标注 October 23, 2026（官方未给时刻） | 原文同段：“Claude Duet is still in research preview”；记忆重置须用户在应用设置中自行操作；删除账户“wipes everything Claude Duet has stored”。 | 已找到 |
| D2 Google 结核筛查部署 | Google Research 博客称，合作方 Nexus AI 把 Google 的胸片算法转为认证临床诊断工具，目前约 40 家医疗机构在用，逾 90 名临床人员评估过逾 2.5 万张胸片。 | L1 | https://research.google/blog/bringing-foundational-models-to-the-front-lines-of-tb-screening/ | “Nexus AI’s tool operates in approximately 40 healthcare facilities. To date, over 90 clinicians have evaluated more than 25,000 chest radiographs, reducing turnaround times from weeks to under 45 seconds, accelerating rapid triage while empowering healthcare professionals to make final clinical decisions.” | `d-google-tb.html`、`d-google-tb.txt`（`d-google-tb.headers`） | October 23, 2026（官方未给时刻） | 1）“rapid triage”，最终诊断由临床人员决定；2）筛查阳性者“can be immediately referred for confirmatory molecular testing”，即需分子检测确认；3）胸片数是评估数，不是确诊数。 | 已找到 |
| D2 工具归属 | 工具归 Nexus AI（南非 Pretoria 医疗科技公司）；Google 的贡献是其胸片算法（Google Research 与 Google for Health 开发），并开源 HAI-DEF 胸片 embedding、MedSigLIP、MedGemma（4B/27B）。 | L1 | 同上 | “we partnered with Nexus AI, a Pretoria-based medical technology company, and they translated our chest X-ray algorithms into a certified clinical diagnostic tool.” | 同上 | 同上 | Google 自身不运营该工具（原文主语为 Nexus AI）。 | 已找到 |
| D2 监管进展 | 工具已获欧盟 MDR IIb 类认证；与 FIND 合作申请 WHO 预认证（PQ），并有通往美国 FDA 许可的“ongoing pathway”。 | L1 | 同上 | “Nexus AI achieved Class IIb certification”；“signed an agreement with FIND to pursue the World Health Organization Pre-qualification (WHO PQ)…alongside an ongoing pathway toward the U.S. FDA clearance” | 同上 | 同上 | WHO PQ 与 FDA 均为“推进中”，原文未称已获得。 | 已找到 |
| D2 国家数（原文内部不一致） | 原文称“Across six countries”，随后列出三国在用（南非、塞拉利昂、科特迪瓦）与三国扩展中（刚果民主共和国、赞比亚、喀麦隆）。 | L1 | 同上 | “Across six countries — South Africa, Sierra Leone, and Côte d'Ivoire, with expansions underway in the Democratic Republic of the Congo, Zambia, and Cameroon” | 同上 | 同上 | 原文“six countries”与“expansions underway”的分列不能完全对上；正文若写国数，应照原文列法，写“三国在用，三国扩展中”，不要写“六国落地”。需 Claude 决定措辞。 | 部分支持（国数口径需注明） |
| D2 矿井部署 | Nexus AI 在 Mponeng 金矿地下约四公里处部署了便携 X 光套件与离线 AI。 | L1 | 同上 | “deployed ultra-portable digital X-ray kits and offline AI four kilometers underground.” | 同上 | 同上 | 原文未给矿工筛查人数。 | 已找到 |
| D3 Brotli CVSS（扫描称 v4.0 9.2） | CVE 记录的 CVSS v4.0 基础分为 9.2（CRITICAL），由 Google（CNA）给出；记录内无 CVSS v3.x 评分。 | L1 | https://cveawg.mitre.org/api/cve/CVE-2026-111497（cve.org 同源 JSON，cve.org 页面需 JS，未取） | `"baseScore":9.2`，`"baseSeverity":"CRITICAL"`，`"vectorString":"CVSS:4.0/AV:N/AC:L/AT:P/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N"` | `d-cve.json`、`d-cve.headers` | datePublished 2026-10-22T13:35:41Z（北京 10/22 21:35:41）；dateUpdated 2026-10-24T03:55:16Z（北京 10/24 11:55:16） | CISA ADP 的 SSVC 评估：Exploitation none，Automatable no，Technical Impact total。 | 已找到 |
| D3 CVE 描述与受影响版本 | 谷歌 Brotli 解码器的越界写，影响 1.0.3–1.2.0；远程未认证攻击者可构造 Brotli 压缩流，破坏内存、泄露内存、拒绝服务或“可能”执行任意代码。 | L1 | 同上 | “allows a remote unauthenticated attacker to corrupt adjacent decoder state memory, disclose sensitive memory contents, cause a denial of service, or potentially execute arbitrary code using a crafted Brotli-compressed stream.” | 同上 | 同上 | 受影响：Google Brotli 1.0.3 至 <1.0.10、1.1.0 至 <1.1.1、1.2.0 至 <1.2.1；修复 1.0.10 / 1.1.1 / 1.2.1。“potentially”保留。 | 已找到 |
| D3 Node.js 公告（扫描称 high） | Node.js 公告将该 Brotli 问题列为 high，致谢 OpenAI Codex Security 报告人。 | L1 | https://nodejs.org/en/blog/vulnerability/october-2026-security-releases | “Node.js Brotli decoder out-of-bounds write permits remote code execution (CVE-2026-111497) - (high)”；“Thank you, to OpenAI Codex Security - Khai Tran and Sylvie Mayer for reporting this vulnerability…” | `d-node.html`、`d-node.txt`、`d-node.headers` | Wednesday, October 21, 2026（官方未给时刻） | 公告称 “This vulnerability affects Node.js 26.x, 24.x, and 22.x.” 修复版本见其发布链接：22.23.4、24.22.1、26.12.1。 | 已找到 |
| D3 Istio 评分（扫描称 3.1 7.7） | Istio 公告 ISTIO-SECURITY-2026-007 在 “Disclosure Details” 的 CVE 栏下标注 CVSS 3.1 7.7，向量仅含可用性影响（A:H）。同一公告另一条 GHSA-7p2q-4xvg-vxc6（istiod 拒绝服务）也标 7.7，两者对应关系原文不清。 | L1 | https://istio.io/latest/news/security/istio-security-2026-007/ | “CVSS Impact Score7.7 CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:C/C:N/I:N/A:H”（CVE(s) 为 CVE-2026-111497）；Brotli 条目原文：“Fixed a vulnerability in the Brotli compression library used by Envoy.” | `d-istio.html`、`d-istio.txt`、`d-istio.headers` | Oct 15, 2026（官方未给时刻） | 3.1 与 CVE 记录的 4.0 评分体系不同，向量也不同（4.0 为 VC/VI/VA 三项 High）。Istio 的 7.7 不能与 CVE 的 9.2 直接比较。 | 部分支持（分值存在，归属不清） |
| D3 patchtheplanet 页面 | OpenAI 运营的 Patch the Planet 页面称，Node.js、Bun、Envoy Proxy、Istio（配置 Brotli 解压）及 Google Brotli 上游均有修复版本；“Other software to check”列出 Electron 内嵌运行时、curl/libcurl、Python 与 Java 绑定、字体解码器等。 | L1（OpenAI 官方页） | https://patchtheplanet.org/brotli | “Crafted Brotli-compressed data can corrupt memory and allow remote code execution in affected software, including Node.js, Bun, and deployments of Envoy Proxy or Istio configured for Brotli decompression.” | `d-brotli-ptp-opencli.txt`（opencli 渲染后全文）；`d-brotli-ptp.html`、`d-brotli-ptp.headers`（fetch 结果 blocked，仅 51 个非空白字符的壳页） | Updated October 23, 2026（官方未给时刻） | 页面口径为 “allow remote code execution”；“Other software”一节须用 DOM 取（innerText 漏掉折叠内容）。 | 已找到（修复版本见下表） |
| D3 “二月源码已修” | 扫描称 OpenAI X 帖说 Brotli 问题“二月源码已修”。 | — | 未找到 OpenAI X 帖（WebSearch 无结果；帖子 ID 未知，未取 fxtwitter） | 无 | 无 | CVE 记录 datePublic 2026-02-27T07:50:00Z（北京 2026-02-27 15:50）见 `d-cve.json`，但这是 CNA 的“公开日期”字段，不是修复日期，不能当作“二月已修”的证据。 | — | 未找到一手来源 |
| D4 Railway 初步报告 | Railway 初步报告称：“at most 1.85% of all workspaces had some environment variables potentially compromised”，约 10 万工作区。 | L1 | https://blog.railway.com/p/preliminary-report-october-security-incident | “we can validate that at most 1.85% of all workspaces had some environment variables potentially compromised. This represents, numerically, about 100,000 workspaces.” | `d-railway.html`、`d-railway.txt`、`d-railway.headers` | Oct 22, 2026（官方未给时刻）；攻击时间线 UTC 2026-10-20 13:58 至 10-22 03:32（北京 10/20 21:58 至 10/22 11:32） | 1）“potentially compromised”保留“可能”；2）Railway 另称因密钥被上游重置的用户“larger than 100k”；3）因无法确定哪些变量被解密，高风险名单多于 1.85%。 | 已找到 |

## 二、修复版本（只记录原文已见）

| 项目 | 修复版本 | 来源 |
|---|---|---|
| Node.js | 22.23.4 · 24.22.1 · 26.12.1 | `d-brotli-ptp-opencli.txt`；`d-node.txt` 发布链接 |
| Bun | 1.4.4 | patchtheplanet |
| Envoy Proxy | 1.40.0 · 1.39.3 · 1.38.6 · 1.37.8（1.36.12 含修复但不支持） | patchtheplanet |
| Istio | 1.29.9 · 1.30.6 · 1.31.2（1.29 已 EOL，1.29.9 为最后一版） | patchtheplanet；`d-istio.txt` |
| Google Brotli | 1.0.10 · 1.1.1 · 1.2.1 | `d-cve.json`；patchtheplanet |

## 三、热度（`barking heat`，HN；Reddit 未采样）

| 条目 | HN 条目 | 分数 | 嵌套评论 | 发帖时刻（北京） | 来源 |
|---|---|---|---|---|---|
| D1 Duet 安全说明 | 50218815 “Claude Duet Safety, Privacy, and Security” | 1 | 1 | 2026-10-24 14:42:22 | `d-hn-50218815.json` |
| D1 参考：1021 Duet 主帖 | 50186037 “Claude Duet” | 565 | 529（Algolia 报 532） | 2026-10-22 03:14:01 | `d-hn-50186037.json` |
| D3 Node.js Brotli RCE | 50189292 “Node.js Brotli RCE (CVE-2026-111497)” | 23 | 0 | 2026-10-22 07:58:43 | `d-hn-50189292.json` |
| D3 Node.js 安全公告 | 50192733 “Node.js security advisory: Brotli decoder RCE (CVE-2026-111497)” | 17 | 3 | 2026-10-22 16:27:26 | `d-hn-50192733.json` |
| D4 Railway 初步报告 | 50211898 “Preliminary Report Oct 21st Security Incident” | 36 | 8 | 2026-10-24 00:48:50 | `d-hn-50211898.json` |
| D2 Google 结核筛查 | 无（Algolia 搜索 “TB screening chest X-ray”“tuberculosis AI chest radiograph”“Nexus AI tuberculosis” 均无结果） | — | — | — | — |

汇总文件：`d-heat.json`（条目 5 个，HN 0 组、Reddit 0 组，无请求失败）。HN 数字取自各条目 JSON 的顶层 `points` 与 `created_at`；评论数取 `barking heat` 的嵌套计数。注意：早先 Algolia 搜索结果的字段与条目易错配，本表以逐条 JSON 为准。

## 四、扫描说法勘误

1. Brotli CVSS “v4.0 9.2”：**核实为真**（`d-cve.json`，Google CNA 给出，CRITICAL）。
2. Brotli CVSS “Istio 3.1 7.7”：Istio 页面确有该数字，但其在公告中的归属不清（同页另一条 istiod 拒绝服务的 GHSA 也标 7.7），且向量只含 A:H，与 CVE 4.0 的 9.2 不是同一评分体系。正文若引用，须写明“Istio 公告 CVSS 3.1 7.7”，不能写成该 CVE 的统一分数。
3. Brotli “二月源码已修”：**未找到一手来源**。CVE 记录的 datePublic 为 2026-02-27，但这是公开日期字段，不是修复日期；OpenAI X 帖未取得。
4. Brotli 受影响方：扫描称“列出 Python urllib3、Electron、curl、字体解码等”——**核实为真**，但在 patchtheplanet 页面的 “Other software to check” 折叠区（innerText 漏掉，DOM 中有）；列出的是 Electron 内嵌运行时、curl/libcurl、Python 与 Java 绑定、字体解码器，urllib3 只出现在代码示例中。
5. Railway “约 10 万工作区可能涉及环境变量访问”：**基本属实**，但原文是 “potentially compromised”（环境变量“可能”被泄露），且 Railway 另说因密钥被上游重置的用户“larger than 100k”。正文保留“可能”，并注明“约 10 万”是按 1.85% 换算的估计。
6. Google 结核筛查：扫描中的“约 40 家机构、90 余名临床人员看过 2.5 万余张胸片”“欧盟 MDR IIb”“WHO 预认证与 FDA 在推进”**均与原文一致**。新增须注意：原文 “six countries” 与分列的国数不一致（见 D2 国家数一行）；“推进中”不是已获批。
7. Duet 安全说明：扫描与 1021 已发的 Duet 研究预览是两篇不同文章。1021 所依据的 `claude-duet` 页（10/21）只写了记忆继承（“Claude Duet carries over your memory…”），未见“断开应用后已学内容保留”的句子；本条为新增事实，不是 1021 的重复。

## 五、可能的吠点

本组按派工单只需“扫描说法勘误”与“一句事实”，未展开“可能的吠点”与“夸大说法实例”。已见线索（仅供参考，L6）：Brotli 的 HN 讨论（`d-hn-50192733.json`）中有评论问 Express 5 的 body-parser 是否默认启用 Brotli 解码，另一条称 curl 与 git 也带该解码器（均为 `d-hn-50192733.json` 的嵌套评论）。这些未核，不进正文。
