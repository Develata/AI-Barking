# 1118 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区 UTC+8），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json` 与命令输出时间。文件前缀 `a-`。未登录任何账户，未接受任何 Cookie 横幅，未提交表单，未发帖、点赞、评论、关注。`.headers` 由 `barking fetch` 去掉抓取地点与标识。浏览器：`barking shot` 的 headless Chrome（匿名，`BARKING_CHROME` 指向 Chrome 应用二进制）与 opencli 浏览器扩展（Browser Bridge，已连接；只读取公开页面）。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 21:30:09–21:30:15 | https://www.wiz.io/blog/nginx-http2-rce-vulnerabilities ；https://nginx.org/en/security_advisories.html ；https://nginx.org/ ；https://nginx.org/en/CHANGES ；https://www.cert.europa.eu/publications/security-advisories/2026-022/ ；https://censys.com/advisory/nginx2root/ ；https://nginx2root.com/ ；https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json ；HN 页与 Algolia（50547374、50547423） | `barking fetch`（`a-fetch-list-1.tsv`） | 全部 ok：`a-wiz.*`、`a-nginx-sec.*`、`a-nginx-home.*`、`a-nginx-news.*`、`a-certeu.*`、`a-censys.*`、`a-nginx2root.*`（壳页，约 32 行）、`a-kev-json.json`（1.8 MB）、`a-hn-page.*`、`a-hn-algolia.json`、`a-hn2-algolia.json`；已打开 txt 看过是正文 |
| 21:31:02–21:31:05 | https://xint.io/blog/nginx2root-remote-code-execution ；https://www.rapid7.com/blog/post/ve-cve-2026-123007-critical-nginx-http-2-integer-underflow-leads-to-rce-fixed/ ；https://nginx2root.censys.com/ ；https://www.wiz.io/blog/atlas-ai-vulnerability-researcher ；my.f5.com K000164058、K000163926、K000163928；https://nvd.nist.gov/vuln/detail/CVE-2026-123007、…123633 ；https://www.cve.org/CVERecord?id=CVE-2026-123007 | `barking fetch`（`a-fetch-list-2.tsv`） | xint、rapid7、wiz-atlas、censys-dash ok；**blocked**：my.f5.com 三篇（正文 49 字符，需 JS 渲染）、NVD 两篇（8 字符）、cve.org（38 字符）。F5 三篇改用 opencli 渲染取得（见下）；NVD、cve.org 改用 CVE 官方 JSON API；`a-censys-dash.*` 是未渲染壳页，实时看板改用 opencli |
| 21:31–21:36 | my.f5.com/manage/s/article/K000163926、K000163928、K000164058 | opencli browser（`a-f5` 会话，`eval document.body.innerText`） | 成功，`a-f5-123007.txt`、`a-f5-123633.txt`、`a-f5-main.txt`；页头显示 “SIGN IN”，未登录，页面公开 |
| 21:3x | https://cveawg.mitre.org/api/cve/CVE-2026-123007 与 …-123633 | curl（`--max-time 60`） | 200，`a-cvejson-CVE-2026-123007.json`、`a-cvejson-CVE-2026-123633.json` |
| 21:3x | https://api.fxtwitter.com/status/2123071157083754677 | curl（`--max-time 60`） | 200，`a-x-ikotas.json`（Ikotas Labs 公开帖子；账号是被引的发现方，非抓取者；抓取者信息未出现） |
| 21:3x | Exa 检索（`web_search_exa`，共 4 次：Ikotas、Metnir、中文二手稿、英文二手稿） | Exa MCP | 仅作线索与二手稿清单；其中被引用的页面均另行 `barking fetch` 存档后再核对原文 |
| 21:33:23–21:33:32 | https://metnir.com/research/drainbreak-nginx-http2 ；https://ubuntu.com/security/notices/USN-9159-1 ；https://www.csoonline.com/article/4250460/…html ；https://www.asahi.com/ajw/articles/17064605 ；https://www.precursorsecurity.com/blog/nginx2root-critical-nginx-rce-vulnerabilities ；https://patchmon.net/blog/nginx2root-cve-2026-123007-nginx-rce-patch ；https://www.netcraft.com/blog/september-2026-web-server-survey ；https://w3techs.com/technologies/overview/web_server ；https://security-tracker.debian.org/tracker/CVE-2026-123007 ；https://cryptobriefing.com/openai-ikotas-labs-nginx2shell-vulnerability/ | `barking fetch`（`a-fetch-list-3.tsv`） | 全部 ok：`a-metnir.*`、`a-usn.*`、`a-cso.*`、`a-asahi.*`、`a-precursor.*`、`a-patchmon.*`、`a-netcraft.*`、`a-w3techs.*`、`a-deb-tracker.*`、`a-cryptobrief.*`；precursor、patchmon、cryptobrief 仅作线索未引用进 evidence |
| 21:3x | https://metnir.com/research ；GitHub API https://api.github.com/repos/nginx/nginx/commits/b89c4100bf6762a45e4a6463eaf4b70368277c94 | curl（`--max-time 40`） | 200，`a-metnir-research.html`、`a-gh-commit-123007.json`（公开提交的作者/提交日期与信息） |
| 21:48:12–21:48:18 | https://daily.steinslab.io/events/2026-11-19-nginx-http2-rce/ ；https://www.theregister.com/security/2026/11/18/…/5310887 ；https://www.securityweek.com/f5-patches-critical-nginx2root-flaws-as-poc-exploits-go-public/ ；https://tech.ifeng.com/c/8yAhYxVIgH0 ；https://securityaffairs.com/203440/…html ；https://www.infosecurity-magazine.com/news/five-critical-vulnerabilities/ ；https://www.secrss.com/articles/95792 ；businesswire PSA 稿；https://prtimes.jp/main/html/rd/p/000000002.000193448.html | `barking fetch`（`a-fetch-list-4.tsv`） | 除 businesswire 外全部 ok（`a-m-*`）；**businesswire：blocked，HTTP 403**，未绕过，只有 Exa 检索摘录（未引用） |
| 21:48:25 | https://censys.com/blog/nginx2root-simple-test/ | `barking fetch`（`a-fetch-list-5.tsv`） | ok，`a-censys-test.*` |
| 21:48:34 | HN `--item 50547374 --item 50547423`；HN 搜索 “nginx2root”、“nginx RCE”；Reddit 搜索 “nginx HTTP/2 remote code execution”、“nginx2root” | `barking heat docs/2611/1118 a` | HN 成功（`a-heat.json`、`a-hn-50547374.json`、`a-hn-50547423.json`）；**Reddit 两次均失败**：重定向到 old.reddit.com 登录页（响应非 JSON），不登录、不重试 |
| 21:30 与 21:50 | https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json | `barking fetch`、curl | 两次均取到目录版本 2026.11.18（dateReleased 2026-11-18T19:17:41Z），五个 CVE 均不在内；记录见 `a-kev-check.txt`；第二次文件 `a-kev-json-2150.json`（1.8 MB） |
| 21:3x–21:49 | https://nginx2root.censys.com/ （实时看板） | opencli browser（`a-censys` 会话，渲染后 `eval innerText` + 全页截图） | 成功，`a-censys-dash-live.txt`；页面标 “Updated 13:07 UTC”（北京 21:07），之后每小时更新，数字会变 |

被拒或失败：my.f5.com / NVD / cve.org（JS 壳页，已改用 opencli 或官方 JSON）、businesswire（403）、Reddit（登录重定向）。Xint 与 Rapid7 的 PoC（GitHub）与 Censys PoC（Censys-Research/nginx2root）仓库未抓取（不需要，也不运行任何第三方代码）。LWN 文章 https://lwn.net/Articles/1108362/（HN 50557703 指向）未抓取。

## 截图

均匿名，已逐张打开确认文字清楚、范围正确（`08c` 例外，见下）。像素比：`barking shot render` 的 700 CSS px 页为 DPR 2（1400 px 宽）；Censys 公告页与实时看板、F5 公告页、CERT-EU 通告用 opencli 浏览器 + `zoom` 取得（Censys 公告 2400 px 宽整页后裁到内容列约 1220 px 宽，真 2×；F5 与 CERT-EU 为 700 CSS px、1400 px 宽，2×；Censys 看板同 700 CSS px，2×）。F5 页、CERT-EU 页、Censys 页的浏览器翻译插件图标与 Cookie 横幅：均未点击“接受”，用 DOM 隐藏横幅后截图，并按像素框裁掉插件图标所在区域（`08c` 因含图标应删除，已另出 `08e`）。

| 文件 | 来源页 | 工具与说明 |
|---|---|---|
| `01-wiz-summary.png`（1400×2544） | Wiz 博客 | render 高 12000 CSS px，stack 两段：标题/作者/日期 + Executive Summary 全部要点；浅色底；顶部 “Blog” 返回链接被裁掉半行（不影响正文） |
| `02-wiz-atlas-validate.png`（1400×2770） | Wiz 博客 Overview | cropbox，含两段 HTTP/2 前提与 Atlas/validate、2015/近六年整段 |
| `03-wiz-timeline.png`（1400×1090） | Wiz 博客 | “Responsible disclosure timeline” 全节 |
| `04-wiz-vuln-table.png`（1400×1470） | Wiz 博客 | “The Vulnerabilities” 小节（两漏洞表 + HTTP/2 前提与 ASLR 句） |
| `04b-wiz-all-cves-table.png`（1400×2510） | Wiz 博客 | “Vulnerabilities Addressed in the Release” 含引言与 10 行全表 |
| `05-wiz-mitigation.png`（1400×1690） | Wiz 博客 | “Mitigation and detection” 全节（修复版本表、关 HTTP/2 步骤） |
| `05b-wiz-exploitation.png`（1400×1770） | Wiz 博客（第二次 render，高 16500 CSS px） | “Exploitation” 全节 |
| `06-nginx-advisories-severity.png`（1400×3450） | nginx.org 公告页 | 页首到 ssl_preread 条目（本批 10 条全含 Severity 栏）；**深色底页面**（页面自身配色） |
| `07-nginx-home-news.png`（1400×700） | nginx.org 首页 | Latest News 的 11-18 与 10-21 条；深色底 |
| `07b-nginx-changes-credits.png`（1400×810） | nginx.org/en/CHANGES | 1.31.9 变更日志前几条（含 123007 与 123633 的措辞与致谢）；深色底 |
| `08-f5-k123007-description.png`（1400×1270） | my.f5.com K000163926 | opencli，Published/Updated 日期、Description、Impact 与 Status；页头标题 “K000163926 …” 被裁掉大半 |
| `08b-f5-k123007-ack-mitigation.png`（1400×510） | 同上 | Mitigation（None）与 Acknowledgments（含 Ikotas 用 GPT-6 Astra） |
| `08c-f5-k123633-description.png`（1400×1260） | my.f5.com K000163928 | **含浏览器翻译插件图标（右侧粉色圆标），不要使用，列入待删除**；已由 `08e` 替代 |
| `08d-f5-k123633-ack-mitigation.png`（1400×520） | 同上 | Mitigation（None）与 Acknowledgments（Metnir、Wiz、Xint、Unit 42） |
| `08e-f5-k123633-desc-impact.png`（1400×1050） | 同上 | Description 与 Impact（从 “Published Date” 之下起，避开插件图标） |
| `09-certeu-no-exploitation.png`（1400×3330） | CERT-EU 2026-022 | opencli，隐藏 Cookie 横幅（未点接受），裁掉插件图标；Release Date、Summary（含 “no exploitation in the wild”）、Technical Details、Affected Products、Recommendations 首句；底部 “Compromise assessment” 之后与 Mitigation 节在存档 txt |
| `10-censys-premise.png`（1220×820） | Censys 公告 | 隐藏 Cookiebot 横幅（未点 “Allow all”）；Vulnerability Description：ASLR、root 需串内核漏洞 |
| `10b-censys-five-flaws-table.png`（1220×3420） | Censys 公告 | 五漏洞表（含 Severity (nginx.org) 列）；表在窄栏里列宽很窄、较长 |
| `10c-censys-poc-version-trap.png`（1220×840） | Censys 公告 | “not plug-and-play”（128 GB、单一测试环境）、Xint 串内核、版本号陷阱 |
| `11-censys-hosts-map.png`（1220×810） | Censys 公告 | “Countries by host count” 地图（All 19,671,971 hosts）与 host 口径图注；深色图 |
| `11b-censys-details-glance.png`（1220×2240） | Censys 公告 | “Details at a Glance” 全表（PoC、Exploitation、Patch、Interim Mitigation） |
| `11c-censys-web-properties.png`（1220×1460） | Censys 公告 | “Censys Queries”：331,242,037 / 74,183,698 web properties 与查询式 |
| `12-rapid7-impact.png`（1400×1534） | Rapid7 博客 | **拼接边界落在图注文字中间，不要使用，列入待删除**；已由 `12b` 替代 |
| `12b-rapid7-limits-credit.png`（1400×1424） | Rapid7 博客 | Impact 首段 + 128 GB/300 Mbit/s/AVX-512 段 + Credit；中间略去 Figure 1 视频区（视频占位，略去） |
| `12c-xint-affected-how-found.png`（1400×2580） | Xint 博客 | Affected versions 表、What to do、How we found them、at a glance |
| `12d-xint-internet-exposure.png`（1400×570） | Xint 博客 | 2.2 Internet Exposure（Shodan 5670 万 / 1194 万 / 98.9% 及“不确认易受攻击”的限定） |
| `12e-metnir-pipeline-found.png`（1400×664） | Metnir 博客 | **标题下副题第二行被切半行，不要使用，列入待删除**；已由 `12f` 替代 |
| `12f-metnir-pipeline-found.png`（1400×704） | Metnir 博客 | 标题/副题 + 首两段（8/29 pipeline 发现、公开模型、ASLR 开启下执行） |
| `12g-cso-headline.png`（1400×680） | CSO Online | 夸大实例：标题、日期、副题；底部露出头图顶端 |
| `12h-censys-tracker-top.png`（1100×600） | nginx2root.censys.com | opencli，隐藏无；WERE/ARE EXPOSED 数字，页头 “Updated 13:07 UTC”；右侧第三张卡被裁；深色底 |
| `12i-censys-tracker-exposure.png`（690×1850） | 同上 | Exposure 环图（9.3m with HTTP/2）与“103,799,113 … do not reveal their version”等文字；只取左栏以避开右侧插件图标；深色底，宽度小于 1400 |
| `12j-steinslab-headline.png`（1400×920） | 团子技术日报 | 夸大实例：标题与首两段；深色底 |

其余过程文件（`a-hn-algolia.json` 等）列于抓取表。未截的内容：A1 “1.31.7/1.31.8 builds”句（在存档）、Ikotas 的 X 帖卡片（F5 公告已是更强的一手来源，且 X 帖不入正文）、Xint 页的发现时间线表与 “AI-assisted research is finding bugs faster than ever” 段（该页高度超过渲染上限，OCR 定位失败，原文在 `a-xint.txt` 第 504–508 行附近）。

## 隐私自查

对 `docs/2611/1118/sources/a-*` grep “/Users/”“develata”：仅在 `a-heat.json` / `a-shots-log.jsonl` 的路径字段（均为仓库内相对路径或不含用户名的临时名）之外无命中；所有页面均未登录，截图内无登录态头像/昵称。`a-x-ikotas.json` 只含被引账号（Ikotas Labs）的公开信息。
