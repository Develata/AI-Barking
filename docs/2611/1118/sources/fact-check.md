# 1118 期事实清单

结果只用 ✅ 已对照一手来源 / ⚠️ 仅二手或部分支持 / ❌ 与来源不符。时间为北京时间。取证：A、B 组 Sonnet 5.5，C 组 Haiku 5.5（`claude-haiku-5-5`）；A、B 两组子代理在写最终回报前因额度/超时中断，`evidence-*.md` 与 `capture-log-*.md` 已写盘，Claude 直接读存档验收。Claude 抽查：A 组 15 处、B 组 6 处、C 组 11 处原句在存档中逐字命中。

## 一、NGINX 漏洞（A 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 开源网站服务器软件 NGINX | L1 | ✅ | `a-wiz.txt` L43：“As the world's most widely used open-source web server , it powers more than 225 million websites .” | Wiz 自述，引 Netcraft（`a-netcraft.txt` L149：nginx 225,366,509 sites）；第 1 轮反向核验后删去“最常用的”以省字 |
| NGINX 11月18日修了两个漏洞（标题、省流：AI 挖出11年老漏洞；NGINX 修了两个要开 HTTP/2 等条件才能利用的漏洞，Wiz 称用 AI 找到；“2.25亿网站”只是在用数） | L1 | ✅ | `a-nginx-home.txt` L68–71：“2026-11-18 … nginx-1.30.7 … nginx-1.31.9 mainline versions have been released”；`a-nginx-sec.txt` L57 起：CVE-2026-123007、CVE-2026-123633 “Not vulnerable: 1.31.9+, 1.30.7+” | 官网只标日期“2026-11-18”，未给时刻与时区；Rapid7 披露文 datePublished 2026-11-18T14:00Z（北京 11-18 22:00）标题已写 “Fixed”，故修复版本不晚于北京 11-18 22:00 公开，北京日期为 18 日；同批共修多个 CVE，正文只讲 HTTP/2 两个；nginx.org 评级 123007 major、123633 medium |
| 都要开着 HTTP/2（一种网页传输协议） | L1 | ✅ | `a-wiz.txt`：“Both vulnerabilities require HTTP/2 to be enabled on the affected NGINX server.”；`a-certeu.txt`：123007 “The only precondition is HTTP/2 enabled on a listening port” | “网页传输协议”为通俗释义：HTTP/2 是 HTTP 协议第 2 版（RFC 9113），本号注 |
| 其中一个还要转发请求且连接数吃紧 | L1 | ✅ | `a-f5-123633.txt` L34：“When NGINX Plus or NGINX Open Source is configured to use HTTP/2 for client connections and proxy client requests to a backend, a specially crafted request can result in use-after-free in the worker process, if this process is running low on worker_connections.” | CVE-2026-123633；“连接数吃紧”对应 running low on worker_connections |
| 内存随机化防护关闭或被绕过时，未登录者可在服务器上执行代码 | L1 | ✅ | `a-f5-123007.txt`、`a-f5-123633.txt` L34：“attackers can execute code on systems with Address Space Layout Randomization (ASLR) disabled or when the attacker can bypass ASLR”；`a-certeu.txt`：“allow an unauthenticated remote attacker to execute code with the privileges of the NGINX worker process” | Wiz 写已演示 ASLR 绕过（`a-wiz.txt`），正文未写“难” |
| 安全公司 Wiz 称用 AI 工具找到（省流：Wiz 称用 AI 找到） | L1 | ✅ | `a-wiz.txt` L71：“Wiz Research used Wiz Atlas , the harness powering Wiz’s AI Code scanning solution , to discover both vulnerabilities. Our researchers used Gemini 4 Carbon and ChatGPT 5.6 Sol to validate the impact” | “找到”对应 discover；两模型是 validate，正文未写模型 |
| 其中一个2015年就在（标题“11年老漏洞”） | L1 | ✅ | `a-wiz.txt` L73：“The integer underflow vulnerability dates back to NGINX 1.9.5 in 2015, meaning it had been hiding in the code for more than a decade.”；`a-nginx-sec.txt`：123007 Vulnerable 1.9.5-1.31.8 | 2015→2026 为 11 年；另一个“nearly six years” |
| 另有四个团队也独立发现（承前“Wiz 称”） | L1 | ✅ | `a-wiz.txt` L41：“Wiz Research, Ikotas Labs, Metnir, Rapid7, and Xint independently uncovered two critical remote code execution vulnerabilities” | Wiz 的说法；F5 致谢逐 CVE 各四方、名单不同（123007：Rapid7、Wiz、Xint、Ikotas；123633：Metnir、Wiz、Xint、Unit 42），见 `a-f5-123007.txt` L150、`a-f5-123633.txt` L151 |
| 对你：受影响的是网站服务器 | L1 | ✅ | `a-wiz.txt`：“Successful exploitation could let attackers take control of affected NGINX servers”；F5：“this is a data plane issue only” | 漏洞在服务器端软件 |
| 对你：开源版 NGINX 升到1.30.7或1.31.9及以上（1.31.8仍有漏洞） | L1 | ✅ | `a-nginx-sec.txt` L57：“Not vulnerable: 1.31.9+, 1.30.7+”；`a-censys.txt` L448：“1.31.8 is affected while 1.30.7 is not, even though 1.31.8 is the higher number” | 发行版回移补丁可能不改版本号（Xint），正文未展开 |
| 对你：并重启 | L2 | ✅ | `a-xint.txt` L317：“A configuration reload alone does not replace the running executable.”；Metnir 博客：Restart NGINX after installing the patched executable | 研究方建议，非 F5 原文 |
| 对你：暂时升不了，Wiz 等建议先关 HTTP/2 | L1 | ✅ | `a-wiz.txt` L77：“If upgrading is not immediately possible, temporarily disable HTTP/2.”；`a-certeu.txt` L128：“disabling HTTP/2 on exposed listeners … mitigates the vulnerabilities CVE-2026-123007 and CVE-2026-123633” | F5 公告 Mitigation 栏写 “None”，所以写“Wiz 等”而非“官方”；只挡这两个 CVE |
| 吠点①：“超过2.25亿网站”是使用量 | L1/L3 | ✅ | `a-wiz.txt` L43；`a-netcraft.txt` L149：225,366,509 | Netcraft 九月调查 |
| 吠点①：Censys 按版本和 HTTP/2 数，一个漏洞暴露约882万个服务，另约1亿个看不出版本 | L3 | ✅ | `a-censys-dash-live.txt` L10：“8,817,296 nginx services on an affected version with HTTP/2 on at their most recent scans”；L34：“Another 103,799,113 nginx services (92% of all detected) do not reveal their version and are not shown.” | 看板只追踪 CVE-2026-123007；单位是 services（端点），不是网站；北京 11-19 21:07 数据，会变；Censys：“The real number of exposed services is higher than the exposed count” |
| 吠点②：有中文稿写“从公网直取宿主机根权限” | L5（被纠正说法的出处） | ✅ | `a-m-steinslab.txt` L39：“连环攻破六个关卡，攻击者从公网直取宿主机根权限” | 团子技术日报（聚合站）；只证明它这样写过 |
| 吠点②：漏洞本身只到 NGINX 进程权限 | L1 | ✅ | `a-certeu.txt`：“execute code with the privileges of the NGINX worker process”；`a-xint.txt` L1202：“The nginx vulnerabilities give an attacker code execution with worker privileges.” | |
| 吠点②：Xint 另串了一个截至19日无补丁的 Linux 内核漏洞才拿到 | L2/L3 | ✅ | `a-xint.txt` L86：“We chained this vulnerability with a separate flaw in the Linux kernel's SCTP subsystem to gain root privileges on the host.”；L1204：“Until a kernel patch is available, we are withholding most of the details”；`a-censys.txt` L428：“Reaching root on the host requires chaining one of these with a separate Linux kernel flaw, which has no patch yet.” | Censys 未验证该链 |

未入正文：nginx.org 评级 major/medium 对 Wiz “Critical 9.2”（F5 CVSS 3.1 为 8.1 High、4.0 为 9.2）；Rapid7 博客写明由研究员 Stephen Fewer 发现（`a-rapid7.txt` L831），全文除站点导航外无 AI 字样，Ikotas 用 GPT-6 Astra（F5 致谢）；Rapid7 PoC 需约 128 GB HTTP/2 流量、低于约 300 Mbit/s 网络不成；CERT-EU、Censys 称未见在野利用，CISA KEV 未收；Ubuntu 已发 USN-9159-1，Debian 稳定版截至 11-19 21:33 仍标 vulnerable；修复提交 11-11 已在 GitHub 公开。

## 二、Gemini 4 Neon（B 组）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 谷歌发布 Gemini-4-Neon | L1 | ✅ | `b-blog.txt`；datePublished 2026-11-18T21:00:00Z | = 北京 11-19 05:00 |
| 称默认档每任务成本比 Carbon 低32%（省流：谷歌称 Gemini-4-Neon 比 Carbon 便宜） | L1 | ✅ | `b-blog.txt` L741：“At its default high thinking level, it pairs a high Artificial Analysis Intelligence Index score with a 32% lower cost per task than Carbon.” | “没写按哪个价”限于该博客段落与全文（B 组全文检索无脚注）；方法学 PDF B 组称已逐页检索、无成本口径，反向核验未独立核 PDF；第 1 轮后删去“上一款”（谷歌 11-12 另发 Hydrogen） |
| 评测机构 AA 测得 Neon 最高档每任务6.44美元，比 Carbon（high 档，原价7.76美元）便宜约17%（省流：AA 的比价按 Carbon 原价，而它正打五折） | L3 | ✅ | `b-aa.txt` L37：“It costs $6.44 per task, ~17% less than Carbon ($7.76 at non-discounted pricing)”；同段 “Neon (xhigh) … Carbon (high)”；L53：“50% above Gemini 4 Carbon’s currently discounted price of $2/$10 (which ends on November 30), and 25% below its standard $4/$20” | Neon 最高档为 xhigh |
| API 标准价每百万 token 输入、输出：Neon 收3、15美元；Carbon 原价4、20美元，半价促销到11月30日 | L1 | ✅ | `b-pricing-rendered.txt` L196、L259 “Standard”；Carbon L201 起：“$2.00 through November 30, 2026. $4.00 starting December 1, 2026.”，输出 “$10.00 through November 30, 2026. $20.00 starting December 1, 2026.”；Neon “$3.00”、“$15.00” | 另有 Batch、Flex、Priority 档，正文限定“标准价” |
| 对你：Neon 免费层不能用 | L1 | ✅ | `b-pricing-rendered.txt` L257：“Gemini 4 Neon is not available on the Free tier.”；`b-blog.txt` L713：“rolling out today in the Gemini API … for paid Google AI Pro and Ultra subscribers in Google Antigravity, and for Pro and Ultra subscribers in the Gemini app” | 第 1 轮指出原引 L194 是 Carbon，已改 L257；另有 Gemini for business |
| 对你：截至11月30日，Carbon 每 token 单价比 Neon 低三分之一 | L1 | ✅ | 同上：$2/$10 对 $3/$15 | 本号核算：2/3、10/15 均为 2/3，即低三分之一；缓存 $0.10 对 $0.15 同比例 |
| 吠点①：谷歌的32%没写按哪个价 | L1 | ✅ | `b-blog.txt` L741 | 限于博客正文（B 组全文检索无脚注、方法学 PDF 无成本口径，后者复核未独立核）；第 2 轮后从事实段移入吠点 |
| 吠点①：AA 按原价测，同为 high 档 Neon 3.74美元、Carbon 7.76美元 | L3 | ✅ | `b-aa.txt` L33：“At high effort, Neon matches Carbon (high) at less than half the Cost per Task using Carbon’s standard (non-discounted) pricing.”；`b-aa-neon.html` 内嵌 Neon (high) 3.7427499；`b-aa-fig1.png` 柱标签 $3.74、$7.76 | AA 智能指数这套测试 |
| 吠点①：按现在半价、同样用量折算，Carbon 约3.88美元，与 Neon 相近，比 Neon 最高档6.44美元便宜约四成（本号推算） | 推算 | ✅（推算已标注） | $7.76÷2 = $3.88；3.88/3.74 = 1.037；1 − 3.88/6.44 = 39.8% | 第 2 轮 BLOCKER：原稿只比 xhigh，属挑基线，补上同为 high 档的结果 | 前提写进正文：同样 token 用量；Carbon 输入、缓存、输出促销价恰为标准价一半（价格页 2/4、10/20、0.10/0.20）；AA 无折扣价成本 |
| 吠点②：36氪等写的“成本只有 Carbon 的60%”（第 2 轮前为吠点③） | L4（被纠正说法的出处） | ✅ | `b-m-36kr.txt` L151：“Neon 速度是 Carbon 的 3 倍，成本只有 Carbon 的 60%。”；`b-m-chaincatcher.txt` 标题“速度 3 倍、成本 60%” | 只证明它们这样写过；第 1 轮指出 mashdigi 正文署了 Glean、ETtoday 写的是“降低60%”，均不计入 |
| 吠点②：出自 Glean 公司的早期评估，谷歌博客只是引用 | L1/L2 | ✅ | `b-blog.html` L5395：“In our early evaluations, it matched or exceeded Gemini 4 Carbon on overall quality at 3x the speed and 60% of the cost.”——Thai Tran，AI Product Lead, Glean | 截图 `images/25-google-glean-quote-60pct.png` |

未入正文：AA 按原价、两者都用 high 档，Neon $3.74、Carbon $7.76，低一半多，与谷歌的32%对不上（第 1 轮后为省字删去）；谷歌博客的“#3、61分”是 xhigh 档（AA 模型页 high 60 分）；xhigh 输出 283M token 对 high 123M，Verbosity #93/95；GDP.pdf xhigh 28.0% 低于 high 33.4%；AA 文写 “~15% more output tokens” 与模型页总量（283M 对 205M）口径不一；Coding Agent 59 对谷歌 58.6 为取整；扫描称 Carbon high $4.90、“AA 未建 Neon 页”均不符。

## 速览

（主帖条目逐字取自小标题；其余条目见下表）

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Anthropic IPO 定价每股83美元，发售9.8亿股，含老股东出售部分 | L1 | ✅ | `c-anthropic-ipo.txt`：“priced its initial public offering of its Class A common stock at $83.00 per share”；“The offering consists of 980,000,000 shares of Class A common stock, 956,351,354 of which are being sold by Anthropic and 23,648,646 of which are being sold by certain selling stockholders.” | 页面 Nov 18, 2026，官方未给时刻；预计11/19纳斯达克交易，未核实际开盘，图上不写；HN 50553843 295分/550评 |
| 谷歌 Genie 4 向符合条件的 Ultra 订阅用户开放，Pro 用户可排队 | L1 | ✅ | `c-genie-4.txt`：“we are rolling out a new Project Genie experience to eligible Google AI Ultra subscribers”；“Google AI Pro members can join the waitlist on the same page” | 北京 11-19 02:00；“世界模型”为谷歌自述的类别；HN 50550415 278分/174评 |
| DeepSeek：调用模型时触发的服务端搜索，11月18日12时起不再按次收费 | L1 | ✅ | `c-pricing-en.txt`：“Starting 12:00 on November 18, 2026 (Beijing Time, UTC+8), the server-side search and web fetch tool calls triggered when calling the model are no longer billed; the results they return are appended to the model context and billed as model token usage.” | 跟进1016：按次收费自11/1 00:00（北京）起（`c-news-261017-en.txt`“Billing starts Oct 31, 2026, at 16:00 UTC”），实收17天12小时；独立 Search/Fetch API 仍每千次成功请求 $0.75/5元；HN 无相关帖 |
| 研究者称接管了语音模型 Kokoro 的发布账号，在本地改过权重但未上传 | L2 | ✅ | `c-kokoro-medium.txt`：“One domain registration. One password reset.”；“in 87 seconds on a gaming GPU, we rewrote the model’s weights”；“Nothing was leaked, changed, or uploaded, and the account has been secured.”；BleepingComputer（L4）`c-kokoro-bleeping.txt`：“never uploaded the modified weights to Hugging Face” | 研究者自述（签名原文），按 L2 写“研究者称”；下载量两源口径不一，未上图；HF 截至 BC 撰写未回应 |
| OpenAI 自报：估计人做2–4小时的内部研发任务，无人干预估计成功率达46% | L1 | ✅ | `c-openai-jagged.txt`：“From March to October, estimated success rose from 23% to 46% for tasks estimated to require two to four hours of human work”；L184/L188：“estimated success rates without human intervention” | 46% 为10月；任务来自 OpenAI 内部研发流程，方法细节未公开，任务构成可能逐月变化；第 1 轮后补“内部研发”“无人干预”；HN 50556229 3分 |

## 批注译注

译注逐字对应 `images/cards.toml` 的 gloss；原文见各 quote 与上表来源。

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| nginx-wiz：五家独立发现；为超过2.25亿个网站提供服务；两个漏洞都要求开 HTTP/2 | L1 | ✅ | `a-wiz.txt` L41、L43 及摘要第4条 | 第 2 轮视觉复核后删去译注里的本号评论（“使用量不是受影响数”“F5 致谢各四方”） |
| censys-exposed：8,817,296 个 nginx 服务处于受影响版本且开着 HTTP/2（按最近一次扫描） | L3 | ✅ | `a-censys-dash-live.txt` L10 | 截图 UTC 13:07 |
| f5-aslr：ASLR 关闭或被绕过时可执行代码 | L1 | ✅ | `a-f5-123007.txt` L34 | |
| google-32pct：默认 high 档在 AA 智能指数上得分高，每任务成本比 Carbon 低32% | L1 | ✅ | `b-blog.txt` L741（“pairs a high Artificial Analysis Intelligence Index score with a 32% lower cost per task”） | “得分高”是谷歌自述 |
| aa-17pct：Neon（xhigh）6.44美元比 Carbon（high）便宜约17%，Carbon 按非折扣价 | L3 | ✅ | `b-aa.txt` L37 | 档位取自同文 “Neon (xhigh) … Carbon (high)” |
| glean-60pct：Glean AI 产品负责人的早期评估，整体质量追平或超过 Carbon，速度3倍、成本60% | L2 | ✅ | `b-blog.html` L5395 | Glean 的说法，非独立评测 |

## 反向核验

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-19 约 23:35；审的是读者稿检查前的正文与卡片，配图说明尚未写）

结论：不通过，3 项 BLOCKER、8 项 SHOULD_FIX、1 项 NEEDS_VERIFICATION。逐条判断：

1. BLOCKER 省流与小标题把 AA 的“按原价”归给谷歌——采纳。省流改为“谷歌称 Gemini-4-Neon 比 Carbon 便宜；AA 的比价按 Carbon 原价，而它正打五折”；小标题改为“新模型称更省，测评按原价比”；事实段给谷歌的32%加“没写按哪个价”。
2. BLOCKER NGINX 事件段丢了第二个漏洞的额外条件与 ASLR 条件——采纳。事实段改为“都要开着 HTTP/2，其中一个还要在转发请求且连接数吃紧时；内存随机化防护关闭或被绕过时，未登录的攻击者可在服务器上执行代码”；原吠点② 的 ASLR 半句移入事实段，吠点② 改为纠正“从公网直取宿主机根权限”。
3. BLOCKER 谷歌事件段把非折扣价的17%写成无条件比较——采纳。事实段补“Carbon（high 档，按原价7.76美元）”。
4. SHOULD_FIX DaisyRose 丢基础模型与“约”——采纳，加上后超 40 字，删去该条速览（速览仍 7 条）。
5. SHOULD_FIX OpenAI 速览缺“估计人做”“内部研发”“无人干预”——采纳，改为“OpenAI 自报：估计人做2–4小时的内部研发任务，无人干预估计成功率达46%”。
6. SHOULD_FIX 译注“五家各自发现两个”——采纳，删“各自”，加“（Wiz 的说法；F5 致谢名单逐个漏洞各列四方）”；正文改“另有四个团队也独立发现”，承前“Wiz 称”。
7. SHOULD_FIX Carbon 不是“上一款”——采纳，删去（谷歌 11-12 另发 Hydrogen）。
8. SHOULD_FIX 价格未限定 API 标准档、吠点①缺 Neon 绝对数——采纳，事实段写“API 标准价”，吠点①补“比 Neon 的6.44美元”与“同样用量”前提；Carbon 原价4、20美元写进事实段。
9. SHOULD_FIX “中文稿常写”样本混杂、L4/L5 标 ✅——采纳，改为“36氪等写的”，清单剔除 mashdigi（正文已署 Glean）与 ETtoday（“降低60%”是另一说法），级别改记“L4（被纠正说法的出处）”，备注“只证明它这样写过”。
10. SHOULD_FIX 配图说明缺失——已补 `images/README.md`。
11. SHOULD_FIX 清单覆盖缺口与行号——采纳：补“网页传输协议”释义行、google-32pct 的“得分高”、glean-60pct 的“整体质量追平或超过”；Neon 免费层引文由 L194（Carbon）改为 L257。
12. NEEDS_VERIFICATION NGINX 日期时区与“方法学 PDF 无脚注”——采纳为限定：清单写明官网只标日期“2026-11-18”、正文照写日期不换算；“没写按哪个价”限于博客，PDF 一项注明是 B 组检索、复核未独立核。

### 第 2 轮（Codex `gpt-6-astra` medium，北京 11-19 约 23:55；审第 1 轮修改后的正文与卡片，不看图）

结论：不通过，1 项 BLOCKER、4 项 SHOULD_FIX、1 项 NEEDS_VERIFICATION。

1. BLOCKER 吠点①只拿 Neon xhigh 比，同为 high 档时按半价折算两者相近（3.74 对约3.88），属挑基线——采纳。吠点①改为先给同为 high 档的原价与折算结果，再给 xhigh 对比，并标本号推算。
2. SHOULD_FIX 32%、60% 只有相对数——部分采纳。32% 是谷歌署名说法、在事实段原样引用，吠点①已给同测试的绝对数；60% 是吠点②里署名的被纠正原话，属“对比”一节的例外。批注图译注照原文直译，不补造基线。
3. SHOULD_FIX 译注里“未写 Carbon 的档位与价格口径”是本号判断——采纳，删去；“没写按哪个价”移入吠点①。
4. SHOULD_FIX 配图说明仍写8条、DaisyRose、“各自”——采纳，已改。
5. SHOULD_FIX “11月30日前”有包含歧义——采纳，改为“截至11月30日”；事实段写“半价促销到11月30日”。
6. NEEDS_VERIFICATION NGINX 北京日期——已补证：Rapid7 披露文 datePublished 2026-11-18T14:00Z（北京 22:00）标题写 “Fixed”，修复不晚于北京 11-18 22:00 公开，见本表第一节备注。

## 视觉复核

### 第 1 轮（Codex `gpt-6-astra` medium，北京 11-19 约 23:55；20 张图：两张封面、全部卡片与 6 张批注底图）

结论：2 项 BLOCKER、3 项 SHOULD_FIX、1 项 NICE_TO_HAVE；高亮位置、跨行、编号、事件卡与吠点卡逐字一致性均无问题。

1. BLOCKER 省流卡含纠偏（“是在用数，不是受影响数”“比价按原价、它正打五折”）——部分采纳。省流卡第一条只取“NGINX 修了两个要开 HTTP/2 等条件才能利用的漏洞，Wiz 称用 AI 找到”；第二条保留“AA 的比价按 Carbon 原价，而它正打五折”：它是17%这一数字成立的条件（同 1117 期“按的是五折促销价”），按“省流卡”一条，改变含义的限定词须留在同句。
2. BLOCKER 译注混入本号评论——采纳。删去 nginx-wiz 的“（这是使用量，不是受影响数）”与“F5 致谢”注、google-32pct 的口径注、glean 的“不是谷歌”注；Glean 署名移到批注图标题“谷歌博客 · Glean 产品负责人引语”；censys 译注只留原文含义。
3. SHOULD_FIX 事件卡“没写按哪个价”是质疑——采纳，移入吠点①。
4. SHOULD_FIX “暂无补丁”“现在半价”缺时效——采纳，改“截至19日无补丁”；“现在半价”前一段已写“半价促销到11月30日”，吠点卡单独传播时仍缺日期，受字数余量（6 字）限制未再加，记为已知限制。
5. SHOULD_FIX 英文截图字小——不采纳：批注图保留完整上下文是规范要求，译注区已给关键句中文全文。
6. NICE_TO_HAVE Censys 看板标题右端截断——底图原样，数字与条件完整，不改。
