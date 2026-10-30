# 1029 期 A 组证据：Cursor 扩展市场 Claude Code 扩展投毒

执行方：claude-sonnet-5-5；取证时间 2026-10-30 23:08–23:55（北京）。时间一律北京时间，括号内保留原时区。状态只用：已找到 / 部分支持 / 与说法不符 / 未找到一手来源。截图文件均在 `docs/2610/1029/images/`。抓取细节见 `capture-log-a.md`。

事件一句话（全部来自 Cursor 博文，一手 L1）：Cursor 称其 VS Code 扩展市场里的 `Anthropic.claude-code` 2.1.313 是被恶意修改的版本，“not developed by Anthropic”，“At most 5,000 users were affected”，在 2026-10-29（太平洋时间）约 2:27am–1:35pm 可下载，即北京 10-29 17:27 至 10-30 04:35。

## 一、证据表

### A1 受影响版本、非 Anthropic 开发、最多 5,000 用户、曝露时段

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 版本是 Anthropic.claude-code 2.1.313，出现在 Cursor VS Code 扩展市场 | L1 | https://cursor.com/blog/claude-code-extension-security-notice （存档 a-cursor-blog.txt；官方中文版 a-cursor-blog-cn.txt） | “Today we identified a security incident involving a specific extension package in the Cursor VS Code extension Marketplace: Anthropic.claude-code version 2.1.313. This malicious version of the extension was not developed by Anthropic.” | 01-cursor-notice-head.png | 标题写 “identified in the Cursor Extension Marketplace”。博文日期 “Oct 29, 2026”，页面没有发布时刻，官方未给时刻 | 已找到 |
| “At most 5,000 users”（上限） | L1 | 同上 | “At most 5,000 users were affected, and we will reach out directly.” | 01-cursor-notice-head.png、08-cursor-x-card.png | **口径没有定义**：“users” 不说是安装数、下载数还是活跃用户；整页没有出现 “install”“download” 计数，也没有给实际执行 payload 的主机数。是上限，不是确数。博文还说会 “reach out directly” | 部分支持（上限已找到；口径与执行数官方未给） |
| 曝露时段 2:27am–1:35pm PT，10 月 29 日 | L1 | 同上 | “Between about 2:27am PT and 1:35pm PT today, a maliciously modified version of Anthropic’s Claude Code extension was made available in the Cursor VS Code extension Marketplace.”；处置段：“between 2:27am PT and 1:35pm PT on October 29, 2026” | 02-cursor-time-window.png、06-cursor-what-you-should-do.png | “today” 由同一博文处置段的 “on October 29, 2026” 确定为 10-29。PT 在 10 月 29 日是 PDT（UTC-7，夏令时 11-01 才结束）→ 09:27–20:35 UTC → **北京 10-29 17:27 至 10-30 04:35**，约 11 小时 8 分。“about” 是官方原词 | 已找到 |
| 博文发布日期与更新记录 | L1 | 同上；web.archive.org 快照（只用于查看“某页面以前写了什么”）a-wb1–a-wb4 | 页面标注 “Oct 29, 2026 · Company”，“5 min read”，“Author: Cursor Team” | 01-cursor-notice-head.png | 页面没有“更新于”字样。Wayback CDX 在 2026-10-30 08:43:21Z（北京 16:43）、10:32:26Z（18:32）、11:44:30Z（19:44）、12:48:31Z（20:48）有 4 个快照，**文本与我 15:08Z（23:08 北京）抓到的实时页逐字相同**。最早快照晚于 Cursor 的 X 帖（北京 15:46），所以 15:46 之前博文有没有改过，没有证据，不能断言 | 部分支持 |

### A2 4,100 与 5,000

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 官方数字是 5,000（Cursor 官方 X 帖，北京 10-30 15:46） | L1 | https://x.com/cursor_ai/status/2116074289053175808 （fxtwitter JSON a-cursor-x-fx.json；X 嵌入卡片截图） | “We identified a security incident affecting a modified Claude Code VS Code extension accessed through our Cursor IDE extension marketplace, impacting at most 5,000 users.” | 08-cursor-x-card.png | 发帖 2026-10-30 07:46:39 UTC（fxtwitter `created_at`；与帖子 ID 推算一致）= 北京 15:46:39。fxtwitter 取数时（北京 23:09）：5,173 赞、817 转、395 回复、520 引用、1,326 收藏。帖子是 “impacting at most”，与博文的 “At most 5,000 users were affected” 同数 | 已找到 |
| 官方给过 4,100 | — | 无 | 全文检索我存档的 Cursor 博文（英文、中文）、官方 X 帖 JSON、BleepingComputer、The Hacker News、Help Net Security、OX Security、CellCog、StepSecurity、Phorion、DoublePulsar、OSM 全部 txt，没有 “4,100”“4100”“4.1k”。所有来源一致写 5,000 | — | AIHOT 页面本身（https://aihot.news）curl 与 Chrome 都连不上（见日志），没法看到它的原条目。能看到的二手转引：GitHub 上 geekjourneyx/agendum 的 issue #94（2026-10-30 11:15 UTC）转引 AIHOT 标题 “Cursor 披露 Claude Code VS Code 扩展被植入恶意软件事件，最多影响 5000 用户”（北京 15:46）——写的是 5000。博文在 Wayback 4 个快照里均为 5,000。**4,100 找不到出处**（不排除是 AIHOT 某个摘要或 WorkBuddy 的误记，未证实） | 未找到一手来源 |

### A3 恶意版本怎么进的市场

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Cursor 自家扩展市场（而非 Open VSX 镜像） | L1 + L3 | Cursor 博文；OSM 博文 https://opensourcemalware.com/blog/trojanized-claude-code-extension-plants-persistent-ssh-backdoor | Cursor：“a maliciously modified version of Anthropic’s Claude Code extension was made available in the Cursor VS Code extension Marketplace.” OSM（UPDATE 段）：“OpenSourceMalware has been contacted by Cursor and they've let us know that the Claude Code extension in the Cursor Marketplace experienced some form of compromise today and this is the origin of the malicious Claude Code extension, not Open-VSX” | 02-cursor-time-window.png、09-osm-update-and-scope.png | **Cursor 博文没有说“怎么进的”**：没写账号被盗、发布流程被绕过还是市场被改；只写动作 “We rotated our keys, rebuilt the marketplace, and brought it back online”，“We audited access and confirmed no other extensions were tampered with”。OSM 引述 Cursor 的话是 “some form of compromise”，是 OSM 转述，不是 Cursor 原文 | 部分支持（来源是 Cursor 市场：已找到；入侵方式：官方未说） |
| Anthropic 是不是该扩展在 Cursor 市场的发布者 | — | 无 | Cursor 只写 “was not developed by Anthropic”。没有说谁是 Cursor 市场里这个条目的发布者，没有说有无发布者验证、签名；Cursor 页面里没有这类措辞。Cursor 的 X 帖与博文都没提 “verified”“signed”“publisher” | — | 反例/二手：Kevin Beaumont（DoublePulsar）写 “it was, in fact, uploaded as Anthropic’s verified account”（他的说法，见 A9，未见 Cursor 或 Anthropic 证实）。Anthropic 员工自称 “felixrieseberg” 的 HN 评论（见 A7）写 “A malicious actor used access to Cursor’s marketplace to publish a tampered copy of the Claude Code extension, which we didn’t build or publish” | 未找到一手来源 |
| 已加什么防护 | L1 | Cursor 博文 | “Removed the malicious version from the marketplace immediately upon detection and blocked further distribution. We followed with a full shut down of the VSCode Marketplace out of an abundance of caution.”；“We rotated our keys, rebuilt the marketplace, and brought it back online”。X 帖：“reinstated the marketplace after confirming there is no further risk” | 05-cursor-what-we-did.png、08-cursor-x-card.png（X 卡被截断，后半句见 a-cursor-x-fx.json） | 没有提签名、发布者验证、审核流程等新防护；“rotated our keys” 没说是哪些 key | 部分支持 |

### A4 payload 行为与成败

Cursor 博文原句（截图 03、04）：

| 步骤 | Cursor 原文（逐字） | OSM 对应（逐字或 txt 位置） | 一致性 |
|---|---|---|---|
| 触发条件 | 博文未单列触发条件，只写 “The extension runs malicious code that attempts to install a persistent SSH backdoor.” | “The extension activates after VS Code startup and when its webview opens and runs the extension.js file.”；“the user doesn't need to begin a Claude conversation. Loading the extension module is enough.” | Cursor 未写，OSM 补充（OSM 自述 “The provenance of the analyzed directory is not established.”） |
| 收集项 | “Collects host and user information: hostname, username, OS, architecture, home directory path, environment variable names, working directories, configured Git email, and public IP.” | 同类项；另写 “Names of up to 40 environment variables containing KEY, TOKEN, SECRET, PASS, AWS, or GCP”，“Environment-variable values are not placed directly in the initial beacon.” | 基本一致；40 个上限与关键词筛选只有 OSM 写 |
| SSH 公钥、sshd、反向隧道 | “Attempts to add an attacker-controlled SSH key to authorized_keys, enable the local SSH service, and open a reverse SSH tunnel. Outside of highly unusual circumstances where Cursor was started with administrator privileges, due to errors in the malicious code, this step would fail.” | 公钥：“appends an Ed25519 key to ~/.ssh/authorized_keys. The key is labeled svc-telemetry”；开 sshd：“These operations may fail without sufficient privileges, but failure does not stop the remaining attack chain.”；隧道：“It repeatedly connects to: tunnel@134.209.82[.]225”，Attack Chain 表里 Tunnel 一行 Outcome 写 “Confirmed code path” | **不一致处**：Cursor 把“加公钥、开 sshd、开隧道”三件事打包成 “this step would fail”，而同一篇博文后面的 “Additionally” 段又说 “The extension still adds the attacker’s SSH key”。OSM 的表对隧道写 “Confirmed code path”，对开 SSH 服务写 “Conditional on OS and privileges” |
| 持久化 | “Attempts to make the reverse SSH tunnel persistent via a LaunchAgent (macOS) or a scheduled task that attempts to run as SYSTEM with a Startup-script fallback (Windows). Background scripts retry the tunnel roughly every 60 seconds.” | macOS LaunchAgent `com.apple.sysmond.plist`；Windows 任务 `SysMon`（SYSTEM）、回退 `sysmon.cmd`；**另有 Linux**：“On Linux, the same files are written under ~/.config, and the script is added to the user’s crontab with an @reboot entry.” | Cursor 只提 macOS 与 Windows；OSM（及 Phorion、StepSecurity）还写了 Linux 分支。Cursor 的 IOC 里 VSIX 只列 darwin-arm64 与 win32-x64 |
| 外发 | “Reports host details and backdoor status via HTTP POST to /beacon on port 8099.” | “It sends the collected JSON to: http://127.0.0.1:8099/beacon http://134.209.82.225:8099/beacon”；“The response is discarded.” | 一致；OSM 另写“没有下载第二阶段”（“No additional executable or script is downloaded by the injected loader.”） |
| “bug / 大多失败”与仍成功的步骤 | “Additionally, turning on the local SSH service requires admin rights, which the malicious extension does not have on most macOS and Windows hosts. On those hosts the step fails and the attacker could not have logged in through the tunnel. The extension still adds the attacker’s SSH key, installs persistence, and sends host details to the attacker. Follow the remediation steps below regardless.” | OSM 无“失败率”表述；Phorion 对 macOS 写了具体 bug：“systemsetup -getremotelogin prints an administrator-access error and exits with status 0 instead of returning Off. … the outbound tunnel can still connect to the attacker’s server, but any traffic forwarded back through the tunnel will hit a closed port 22 on localhost”（摘自 a-phorion.txt） | Cursor 官方原话**没有出现 “bug”“mostly failed”**，是 “due to errors in the malicious code, this step would fail” 与 “the step fails”；“most macOS and Windows hosts” 是官方原词，没给比例。仍成功的三项官方明写：加公钥、装持久化、外发主机信息（这三项与 “At most 5,000” 一起看，口径是“可能受影响”，不是“已被入侵”） |
| 攻击者服务器 | “Note: The attacker server no longer functions. We worked with the cloud provider that was hosting it to take it offline, so the reverse shell and host detail POST are not functional.” | 无 | — |

状态：已找到（A4 全部有一手原句，逐项对照见上；无法判断的是各步骤实际成功比例：官方只给定性描述）。截图：03-cursor-payload-steps.png、04-cursor-fail-vs-still-works.png、09-osm-update-and-scope.png。

OSM 文章信息：发布戳 2026-10-30T09:42:00Z（北京 10-30 17:42）；作者字段是 UUID（页面显示署名 “6mile (Paul McCarty)”）；但正文写 “A malicious version, 2.1.313 was published to the Cursor Marketplace, today, October 29th, 2026” —— 发布戳与 “today” 不一致，原样记录，没有自行取舍。

### A5 用户该怎么办

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 处置建议（五步） | L1 | Cursor 博文 | “If you installed or updated to Anthropic.claude-code 2.1.313 from Cursor VS Code extension marketplace between 2:27am PT and 1:35pm PT on October 29, 2026:” 1. “Run the check below to validate if you are impacted.” 2. “Search across endpoint, network, and extension-installation telemetry for the indicators below.” 3. “Isolate hosts showing evidence of payload execution or backdoor installation.” 4. “Remove the unauthorized SSH key, persistence mechanisms, and running tunnel processes using the remediation scripts below, and assess whether the host should be rebuilt. Uninstalling the extension may not be sufficient — the payload drops standalone persistence scripts that keep running.” 5. “Rotate credentials and sessions accessible from affected hosts, from a trusted system. The limited env-var collection does not rule out later credential access over the backdoor.” | 06-cursor-what-you-should-do.png | 是否轮换凭据：官方第 5 步要求轮换（对象是 “affected hosts”）；卸载是否足够：官方原话 “may not be sufficient”。X 帖另写 “We are asking all customers update to Claude Code for VS Code version 2.1.314”，博文无这句（我在英文与中文博文里都 grep 过 2.1.314，没有） | 已找到 |
| 自查命令（原样摘录，未在本机执行） | L1 | Cursor 博文 | macOS：`grep -qs svc-telemetry ~/.ssh/authorized_keys && echo "svc-telemetry key found" || echo "No key found; machine not impacted"`。Windows（PowerShell）：`$files = "$HOME\.ssh\authorized_keys", "C:\ProgramData\ssh\administrators_authorized_keys" | Where-Object { Test-Path $_ }` 换行 `if ($files -and (Select-String svc-telemetry $files)) { "svc-telemetry key found" } else { "No key found; machine not impacted" }`。官方说明：“If the key is found, your machine is impacted. Follow steps 3 to 5 above, and run the remediation script for your platform below.” | 07-cursor-check-commands.png（页面内代码块横向滚动，截图右侧被截；完整命令以 a-cursor-blog.txt 为准） | 命令只在 macOS 与 Windows 给出，**没有 Linux 命令**（Linux 有无受影响博文没说）。官方还写 “No key found; machine not impacted”：该命令只检查公钥，其他持久化文件（如 LaunchAgent）未检查。另：官方还给了两个修复脚本（remediate-macos.sh、remediate-windows.ps1，博文里是下载链接）——**未下载、未运行、未评估其内容**。macOS 脚本 “run it as the affected user, not as root”；Windows 脚本 “run it from PowerShell opened with Run as Administrator”，命令含 `-ExecutionPolicy Bypass` | 已找到 |

### A6 攻击者与处置

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 攻击者身份 | — | Cursor 博文 | 官方**没说**攻击者是谁，也没说归因。博文里只有 “attacker-controlled SSH key”“attacker server” 等泛称 | — | OSM 结论段：“Distribution source, attacker identity, infrastructure ownership, and the status of any official release remain unverified and should not be inferred from package branding.”（a-osm-blog.txt 末段） | 未找到一手来源 |
| 端点下线、与云商合作 | L1 | Cursor 博文 | “Note: The attacker server no longer functions. We worked with the cloud provider that was hosting it to take it offline”；“We worked with the hosting provider to take the malicious server offline, disrupting the attack by blocking the connection to that server.” | 04-cursor-fail-vs-still-works.png、05-cursor-what-we-did.png | 没点名云商；没说下线时刻 | 已找到 |
| 恶意版本移除与市场关闭 | L1 | Cursor 博文 + X 帖 | 博文：“We followed with a full shut down of the VSCode Marketplace”；X 帖：“we took down our extension marketplace this afternoon” | 05、08 | “this afternoon” 是 X 帖原词（太平洋/美东时间，官方未给时刻，也未给重新上线时刻） | 部分支持 |

### A7 Anthropic 与 Open VSX 方面的回应；版本史

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Anthropic 官方声明 | — | anthropic.com 新闻页未查到；status.claude.com 无相关事故（a-anthropic-status.txt：“No incidents reported today.”）；github.com/anthropics/claude-code 搜 “2.1.313”“backdoor”“cursor malicious” 无相关 issue（a-gh-issues-*.json）；GitHub release 说明（v2.1.314、v2.1.312 的 body，在 a-gh-releases.json 里）按 cursor/marketplace/malicious/VS Code extension 检索，没有提到本事件（命中的是无关的 vim 光标、managed settings 条目） | 没找到 | — | 多家媒体同步写没有：CellCog “We found no statement from Anthropic, Open VSX or the Eclipse Foundation.”；BleepingComputer 写 Cursor “has not explained how attackers distributed the compromised package under Anthropic's name”；上面是媒体陈述，非一手 | 未找到一手来源 |
| Anthropic 员工公开发言（HN 评论，北京 10-30 22:08） | L2（身份未核，暂按线索） | https://news.ycombinator.com/item?id=50304129 （HN 帖 https://news.ycombinator.com/item?id=50300683 下，存档 a-hn-hn-50300683.json） | “Felix from Anthropic here. We’re working with Cursor on this incident, but it wasn’t a compromise of Anthropic’s systems or employee laptops. A malicious actor used access to Cursor’s marketplace to publish a tampered copy of the Claude Code extension, which we didn’t build or publish. The versions on our official channels are not affected and this only impacts Cursor users.” | — | 评论者 HN 用户名 felixrieseberg，时间 2026-10-30T14:08:52Z = 北京 22:08。**这是 HN 用户名的自我介绍**，我没有拿到可链接的 Anthropic 官方账号、官网或公司邮箱来核对此人身份；按 EDITORIAL 的 L2 要求（署名、可链接）还差一步。采用前需 Claude 或 Develata 核身份，或找到 Anthropic 官方帖/声明 | 部分支持 |
| Open VSX / Eclipse 声明 | — | open-vsx.org、Eclipse 基金会博客未做深查（只查了 API） | 没找到 | — | 同上，CellCog 称没找到；Open VSX 的 `Anthropic/claude-code` 命名空间显示 `verified: true`（a-openvsx-ext.json） | 未找到一手来源 |
| npm 版本史 | L1（registry 原始数据） | https://registry.npmjs.org/@anthropic-ai/claude-code （a-npm-times.json，从 1.5 MB packument 抽 `time` 与 `dist-tags`） | `2.1.312`: 2026-10-27T18:36:10.160Z；`2.1.314`: 2026-10-30T00:12:56.911Z；`2.1.313` 不在 `time` 与 `versions` 里；`dist-tags`: stable 2.1.308，latest 2.1.314，next 2.1.314 | — | 北京：2.1.312 = 10-28 02:36；2.1.314 = 10-30 08:12。取数时刻北京 10-30 23:10。**跳号原因没有任何证据，不推断**（可能没发布过，也可能被撤下；我无法区分） | 已找到 |
| Open VSX 版本史 | L1（registry） | https://open-vsx.org/api/Anthropic/claude-code （a-openvsx-ext.json、a-openvsx-versions.json） | 当前 `version`: 2.1.314，`timestamp`: 2026-10-30T01:07:38.165143Z（北京 09:07）；`/versions` 列表：2.1.314、2.1.312、2.1.311 …，**没有 2.1.313**；`totalSize` 324；命名空间 `Anthropic`，`verified: true`，`downloadCount` 63,055,731（全历史累计，非本次） | — | `publishedBy` 是一个 GitHub 登录名（Open VSX 元数据；不写入正文，也未核其身份）。63,055,731 是累计下载，不能拿去换算本事件影响人数 | 已找到 |
| GitHub releases 版本史 | L1 | https://github.com/anthropics/claude-code/releases （API 存档 a-gh-releases.json） | v2.1.314：published_at 2026-10-30T01:06:26Z（北京 09:06）；v2.1.312：2026-10-27T20:24:20Z（北京 10-28 04:24）；列表里同样没有 v2.1.313 | — | 三套时间不同（npm 00:12Z、GitHub release 01:06Z、Open VSX 01:07Z），分列，未合并 | 已找到 |
| 更新到 2.1.314 | L1（Cursor X 帖）+ L4 | Cursor X 帖；The Hacker News | X 帖：“We are asking all customers update to Claude Code for VS Code version 2.1.314”；THN：“Anthropic has published version 2.1.314 to both Open VSX and Microsoft VS Code Marketplace.” | 08-cursor-x-card.png（卡片被截断，不含这句，见 a-cursor-x-fx.json） | THN 是媒体（L4）；Open VSX 与 npm 的 2.1.314 时间点已由 registry 数据证实，微软市场的 2.1.314 我没有一手数据 | 部分支持 |

### A8 仅影响 Cursor 吗

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Cursor 的范围表述 | L1 | Cursor 博文、X 帖 | 博文：“These findings concern the identified package. We have no evidence that other versions of the extension or the official distribution are affected.” X 帖：“It does not impact other extensions downloaded from the cursor.com/marketplace, Grok Bot, Grok or our other SpaceXSI products.” | 04-cursor-fail-vs-still-works.png、08-cursor-x-card.png | **Cursor 原文没有点名** VS Code 官方市场、Open VSX、Windsurf、VSCodium、npm CLI；“the official distribution” 没有定义。没说就写没说 | 已找到（官方没说其余渠道） |
| OSM 的范围表述 | L3（安全研究者自述；其结论只针对其分析的样本） | OSM 博文 | “What is not malicious: The @anthropic-ai/claude-code NPM package is NOT malicious / The OpenVSX extension is NOT malicious / The Microsoft VSCode extension for Claude is NOT malicious”；同文：“The provenance of the analyzed directory is not established. These findings show that this specific artifact is malicious; they do not, by themselves, demonstrate that Anthropic’s official Marketplace release or distribution infrastructure was compromised.” | 09-osm-update-and-scope.png | 这些 NOT malicious 是 OSM 的断言，文中不见逐渠道的证据；OSM 版本在更新前后立场有变（UPDATE 段改称来源是 Cursor 市场“not Open-VSX”）。**早期报道曾写 Open VSX 受影响，后来被更正**（见 A9） | 已找到 |
| 媒体口径 | L4 | The Hacker News；BleepingComputer | THN：“The malicious update was distributed only through Cursor's marketplace, with no evidence that it reached Open VSX or Microsoft's official VS Code Marketplace.”；BleepingComputer：“While initial reports suggested that npm, Open VSX, and Microsoft's Visual Studio Code Marketplace were also impacted, it was later confirmed that Cursor was the only source distributing the malicious extension.” | 11-thn-correction-note-v2.png | BleepingComputer 的 “it was later confirmed” 没写由谁确认。我的 registry 数据：npm、Open VSX、GitHub 均无 2.1.313（见 A7），与“这些渠道没有该版本”相容，但不能证明它们从未有过 | 部分支持 |

### A9 热度与转述

热度（`barking heat`、HN Algolia；Reddit 失败）：

| 帖 | 链接 | 分/评论 | 帖子时刻 | 取数时刻 |
|---|---|---|---|---|
| HN “Backdoor in Claude Code for VS Code extension on the Cursor Marketplace”（链 phorion.io） | https://news.ycombinator.com/item?id=50300683 | 188 分（item API）/195 分（搜索），含嵌套 298 条评论 | 2026-10-30T07:09:12Z = 北京 15:09 | 北京 23:33（item）、23:28（搜索） |
| HN “Claude Code VS Code extension with backdoor released on Cursor marketplace”（链 doublepulsar.com） | https://news.ycombinator.com/item?id=50301983 | 393 分 / 1 评论 | 2026-10-30T10:22:51Z = 北京 18:22 | 同上 |
| HN “Malicious Claude Code Extension Found in Cursor Marketplace”（链 Cursor 博文） | https://news.ycombinator.com/item?id=50301004 | 12 分 / 1 评论 | 2026-10-30T08:04:26Z = 北京 16:04 | 同上 |
| Reddit | — | 搜索被重定向到登录页，失败，未登录、未换镜像 | — | — |

HN 评论里的质疑（仅作线索 L6；Algolia 的评论数据没有每条评论的分数，所以只给评论链接与回复数，不编分数）：见下面“可能的吠点”。

3–5 篇报道（media 首发/权威/夸大）：

| 发布方 | URL | 发布时间 | 标题（原文） | 写法与问题 |
|---|---|---|---|---|
| BleepingComputer | https://www.bleepingcomputer.com/news/security/malicious-claude-code-extension-on-cursor-planted-ssh-backdoors/ | 2026-10-30 10:36 AM（页面未写时区；站点通常用美东，北京约 22:36，**未证实**） | “Malicious Claude Code extension on Cursor planted SSH backdoors” | 写法稳：“approximately 11 hours”“up to 5,000 developers”；明写 “has not explained how attackers distributed the compromised package under Anthropic's name”；承认早期报道曾写 npm/Open VSX/微软市场受影响，后被更正 |
| The Hacker News | https://thehackernews.com/2026/10/official-claude-code-vs-code-extension.html | 2026-10-30（页面只写日期） | “Official Claude Code Extension on Cursor Marketplace Trojanized with Persistent SSH Backdoor” | 文末更正：“(An earlier version of the story stated that the extension on the Open VSX registry was compromised. The incident has since been traced back to a compromise of the Cursor VS Code Marketplace.)”；同文：“Cursor's statement that the malicious version ‘was not developed by Anthropic’ misses the point.” |
| Help Net Security | https://www.helpnetsecurity.com/2026/10/30/claude-code-extension-ssh-backdoor/ | 2026-10-30（页面只写日期） | “Claude Code extension poisoned with persistent SSH backdoor” | 首段 “researchers at macOS security firm Phorion have found”，次段 “The company spotted the malicious package, version 2.1.313, on Open VSX”——**把 Open VSX 当成发现地**，与 Cursor 说法不同；同页还写 “At most 5,000 users were affected”；有一位 Manifold Security 研究员质疑 Open VSX 版本也被入侵的说法 |
| CellCog（AI 公司博客，二手） | https://cellcog.ai/blog/claude-code-extension-backdoor/ | 2026-10-30 | “Claude Code Extension Backdoored on Open VSX and Cursor” | **标题把 Open VSX 与 Cursor 并列**，与 Cursor、OSM 更新后的说法不同；正文第一段 “A build published as version 2.1.313 of the Claude Code extension, on Open VSX and in Cursor's marketplace”；同时自己查到 Open VSX、VS Code Marketplace、npm 都是 2.1.312→2.1.314，并说 “Anthropic has not commented” |
| StepSecurity | https://www.stepsecurity.io/blog/claude-code-extension-trojanized | 2026-10-30 | “Trojanized Claude Code Extension Delivered Persistent SSH Backdoor to Developer Machines” | 正文：“a trojanized version (2.1.313) … was published to Open-VSX and mirrored to Cursor's marketplace”；称 Cursor 的市场“mirrors” Open VSX——与 Cursor、OSM 更新后的说法不同 |
| OX Security | https://www.ox.security/blog/malicious-claude-code-extension-cursor-ssh-backdoor/ | 2026-10-30 | “Malicious Claude Code Extension Installs SSH Backdoor via Cursor”（页面标题；搜索结果里的另一个标题 “Compromised Claude Code Extension (2.1.313) in Cursor Marketplace Plants Attacker SSH Key and Persistence”） | 引 Cursor 的 5,000 与时间窗，未发现夸大 |

## 二、可能的吠点（均需回到一手来源核实后才能进正文）

1. “At most 5,000 users” 是上限，Cursor 没定义“users”（安装/下载/活跃），也没给实际执行 payload 的数量——来源：Cursor 博文开头段。
2. Cursor 博文自己前后有张力：第二条 bullet 说 “due to errors in the malicious code, this step would fail”（这一条把加 SSH 公钥、开 sshd、开反向隧道打包在一起），后面 “Additionally” 段又写 “The extension still adds the attacker’s SSH key, installs persistence, and sends host details to the attacker.” 逐字并列即可，不替官方理顺——来源：Cursor 博文（截图 03、04）。
3. 官方原文没有 “bug”“mostly failed” 这类词；失败的是“开本地 SSH 服务”这一步（需要管理员权限），范围是 “most macOS and Windows hosts”，没给比例；“highly unusual circumstances where Cursor was started with administrator privileges” 才会成功——来源：Cursor 博文。
4. 卸载扩展不够：Cursor 原话 “Uninstalling the extension may not be sufficient — the payload drops standalone persistence scripts that keep running.”，并要求轮换凭据与会话；自查命令只查 svc-telemetry 公钥，一句 “No key found; machine not impacted” 的结论只来自该公钥——来源：Cursor 博文（截图 06、07）。
5. Cursor 的“怎么进的”没说：博文没写账号被盗还是市场被改，也没写新增了什么防护（只写 “rotated our keys, rebuilt the marketplace”）；BleepingComputer 与 THN 都写“如何以 Anthropic 名义发布仍未解释”——来源：Cursor 博文 05；BleepingComputer（a-bleeping.txt）。
6. “not developed by Anthropic” 的措辞争议：Kevin Beaumont 写 “‘Developed’ is doing some heavy lifting”（他认为它是以 Anthropic 的 verified account 上传的；该点未见 Cursor 或 Anthropic 证实）；THN 写 “misses the point”。另一方面，自称 Anthropic 员工的 HN 评论写 “it wasn’t a compromise of Anthropic’s systems or employee laptops”（身份未核）——来源：a-doublepulsar.txt、a-thehackernews.txt、a-hn-hn-50300683.json。
7. 仅影响 Cursor 市场：Cursor 原文只说 “no evidence that other versions of the extension or the official distribution are affected”，没点名其他渠道；OSM 与 THN 点名 npm/Open VSX/微软市场未受影响，早期报道（THN 早版、StepSecurity、Help Net、CellCog）曾写 Open VSX 受影响，后被更正或仍在——来源：Cursor 博文；OSM；THN 更正说明。
8. 版本号 2.1.313 在 npm、Open VSX、GitHub releases 上都没有（2.1.312 → 2.1.314），这三处是公开 registry 数据，原因没有任何证据；CellCog 与 DoublePulsar 的“2.1.313 被悄悄删除”是他们的推断，Cursor/Anthropic 均未说明（DoublePulsar 写 “silently deleted without telling anybody”，而 Cursor 博文写明了“removed … immediately upon detection”）——来源：a-npm-times.json、a-openvsx-versions.json、a-gh-releases.json、a-doublepulsar.txt。
9. “把 63,055,731 当影响人数”不成立：Open VSX 的 `downloadCount` 是整个扩展的历史累计下载（a-openvsx-ext.json），与本次 “at most 5,000 users” 无关；StepSecurity 与 DoublePulsar 分别写 “tens of millions of lifetime downloads on Open-VSX alone”“used by tens of millions of developers”，容易被误读成受害规模。
10. 技术细节差异（OSM/Phorion 对 Cursor）：OSM 与 Phorion 写了 Linux 分支（crontab @reboot），Cursor 只写 macOS/Windows；Phorion 指 macOS 上 `systemsetup -getremotelogin` 无管理员权限时会输出错误并以 0 退出，使“开 SSH”这步被跳过，但“出站隧道仍可连上攻击者服务器”，只是“转发回来的流量会碰到本机关闭的 22 端口”；Phorion 还写 Linux 的 crontab 检查有 bug，“can wipe existing user cron jobs on repeat run”——来源：a-phorion.txt、a-osm-blog.txt。
11. HN 讨论里的质疑（L6，只作线索）：
    - 评论 https://news.ycombinator.com/item?id=50302743 （回复 2 条）：引用 DoublePulsar 的 “Yesterday, Anthropic were compromised …” 后称 “That's a bit hyperbolic: someone released a version of the Claude Code VS Code extension with a backdoor on the Cursor marketplace”。
    - 评论 https://news.ycombinator.com/item?id=50302525 （回复 4 条）：先写这是 Open VSX 上的扩展，随后两次 EDIT 改口，称 2.1.313 似乎从未发布到微软官方市场（说明社区早期也在这一点上有分歧）。
    - 评论 https://news.ycombinator.com/item?id=50302955 （回复 4 条）：称 VS Code 的“auto update delay”对 Microsoft、GitHub、OpenAI 等 trusted publishers 的扩展不适用，更新仍然立刻生效（评论里附 VS Code 1.123 更新说明链接，**我未核该链接**）——属“自动更新放大风险”的线索。
    - 评论 https://news.ycombinator.com/item?id=50302764 （回复 6 条）：称 Claude Code / Codex 用户其实一直在跑任意代码（评论性观点，不作事实）。

## 三、夸大说法实例（把事件写歪的标题/文章）

| 发布方 | URL | 原句（逐字） | 发布时间 | 写歪在哪里 | 截图 |
|---|---|---|---|---|---|
| DoublePulsar（Medium，Kevin Beaumont） | https://doublepulsar.com/anthropic-hacked-to-deliver-remote-access-trojan-to-claude-developer-laptops-worldwide-1148a92baae2 | 标题 “Anthropic hacked to deliver remote access trojan to Claude developer laptops worldwide”；正文 “Yesterday, Anthropic were compromised and Cursor VS Code extension used by tens of millions of developers for Claude access was delivering a remote access trojan via an SSH reverse backdoor.” “Anthropic has not informed customers, and the malicious version was silently deleted without telling anybody.” | 页面 `datePublished` 2026-10-30T10:08:04Z = 北京 18:08；`dateModified` 13:31:11Z = 北京 21:31（页面显示 “5 hours ago”，是相对时间）。HN 链接帖 50301983（393 分）北京 18:22 | 标题称 “Anthropic hacked”“worldwide”；正文称 “tens of millions of developers”；与 Cursor 的 “At most 5,000 users”、“not developed by Anthropic” 不合，OSM 称 “do not, by themselves, demonstrate that Anthropic’s official Marketplace release or distribution infrastructure was compromised”；该篇同时引了 Cursor 的 5000（“Cursor say around 5000 developer systems were impacted”），所以“tens of millions”指的是扩展用户基数而不是中招数，读者容易混。作者是公开的安全研究者，这是他个人博客观点；**该稿后来有被修改的迹象（`dateModified` 晚于首发），我抓到的是 23:33 北京的版本** | 12b-doublepulsar-anthropic-hacked-v2.png |
| CellCog 博客 | https://cellcog.ai/blog/claude-code-extension-backdoor/ | 标题 “Claude Code Extension Backdoored on Open VSX and Cursor” | 2026-10-30（页面写 “30 OCTOBER 2026”，无时刻） | 把 Open VSX 和 Cursor 并列为受害渠道，与 Cursor 说法不同；同一文章正文又查到 Open VSX 没有 2.1.313，自相矛盾 | 12-cellcog-title-open-vsx.png |
| Help Net Security | https://www.helpnetsecurity.com/2026/10/30/claude-code-extension-ssh-backdoor/ | “The company spotted the malicious package, version 2.1.313, on Open VSX, an open-source marketplace for VS Code extensions run by the Eclipse Foundation.” | 2026-10-30（页面只有日期） | 报道的发现地是 Open VSX；与 Cursor、OSM 更新后的说法、THN/BleepingComputer 的更正不同；同页引用的另一位研究者（Manifold Security）还在反驳 | 10-helpnet-says-open-vsx-v2.png |
| The Hacker News（早版） | https://thehackernews.com/2026/10/official-claude-code-vs-code-extension.html | 更正说明：“(An earlier version of the story stated that the extension on the Open VSX registry was compromised. The incident has since been traced back to a compromise of the Cursor VS Code Marketplace.)” | 2026-10-30 | 早版写了 Open VSX 被入侵，后更正；我只抓到更正后的版本 | 11-thn-correction-note-v2.png |
| StepSecurity | https://www.stepsecurity.io/blog/claude-code-extension-trojanized | “a trojanized version (2.1.313) of the official Claude Code VS Code extension was published to Open-VSX and mirrored to Cursor's marketplace” | 2026-10-30 | 与 Cursor 自述的来源不同（见 A8） | 无（只存 txt） |

中文侧的“Claude Code 被投毒 / 5000 人中招 / Anthropic 被黑”类标题：我只做了 2 次中文搜索（IT之家等），**没有找到具体的中文媒体转载页面**，不能提供原站 URL 与原句，写“未找到”。唯一能提供的中文材料：Cursor 官方中文版博文 https://cursor.com/cn/blog/claude-code-extension-security-notice （存档 a-cursor-blog-cn.txt），原句 “受影响用户最多 5,000 名，我们将直接与他们联系。”；中文博文对 “At most” 译成“最多”，口径同英文版。

## 四、扫描说法勘误

| 扫描说法 | 核对结果 |
|---|---|
| 5,000 / “最多 4100” | 所有一手与媒体来源都是 5,000；**4,100 找不到任何出处**（见 A2） |
| “北京 10-30 约 15:46 官方 X 帖写 5,000” | 属实：帖子 2026-10-30 07:46:39Z = 北京 15:46:39，写 “impacting at most 5,000 users” |
| “Between about 2:27am PT and 1:35pm PT today”，“today” 指 10-29 | 属实：同篇处置段写 “on October 29, 2026”；北京 10-29 17:27 – 10-30 04:35 |
| “恶意 extension.js 前插 66 行加载器，加载扩展即触发、不需开始对话” | 66 行与“不需开始对话”出自 OSM，不是 Cursor；OSM 原文：“a malicious 66-line loader inserted before the legitimate extension code”，“the user doesn't need to begin a Claude conversation”。Cursor 博文没写“66 行”也没写触发条件 |
| “收集主机名/用户名/架构/环境变量名/Git 邮箱/公网 IP” | 属实（Cursor 原文还含 OS、home directory path、working directories） |
| “向 authorized_keys 追加标注 svc-telemetry 的公钥，尝试开 sshd 与反向隧道” | 属实 |
| “LaunchAgent（伪装 com.apple.sysmond）或计划任务 SysMon 持久化” | 属实（来自 Cursor IOC 与 OSM）；Cursor 对 Windows 另写 “attempts to run as SYSTEM with a Startup-script fallback” |
| “向 8099 端口 /beacon 明文 POST” | Cursor 写 “HTTP POST to /beacon on port 8099”；“明文”是 OSM 写的 “unencrypted HTTP beacon”，Cursor 没用这个词 |
| “官方称恶意代码有 bug、开 SSH 那步大多失败，但公钥、持久化、主机信息外发成功” | 部分不符：Cursor 原文没有 “bug”，写的是 “due to errors in the malicious code, this step would fail”；“大多”对应 “most macOS and Windows hosts”；“公钥、持久化、外发成功”原文是 “still adds … installs persistence, and sends host details”，属实。“bug” 一词在 Phorion（“implementation bugs”）与 OSM 的措辞里出现 |
| “卸载扩展不够，需轮换凭据” | 属实：“Uninstalling the extension may not be sufficient”（官方是 “may not”，不是“不够”）；要求 “Rotate credentials and sessions accessible from affected hosts” |
| “Cursor 官方 X 帖北京 10-30 约 15:46 发布” | 属实（见上） |
| Claude 已查：npm 2.1.312 → 2.1.314，无 2.1.313；Open VSX latest 2.1.314（10-30T01:07Z）；GitHub v2.1.312 → v2.1.314 | 全部复核属实：npm 2.1.314 = 10-30T00:12:56.911Z；Open VSX 2.1.314 = 10-30T01:07:38.165Z；GitHub v2.1.314 = 10-30T01:06:26Z；三处都没有 2.1.313 |
| “恶意版本来自 Open VSX”（OSM 初版与若干媒体） | 已被更正：OSM UPDATE 与 Cursor 说法是 Cursor 市场；THN 早版曾写 Open VSX，后更正；但 Help Net、CellCog、StepSecurity 仍写 Open VSX。该点要在正文里选一手来源说话（Cursor 原文），不引这几家 |
| Cursor 博文“修订过吗” | Wayback 4 个快照与实时页文本一致；最早快照晚于 X 帖，之前版本无证据 |
| 派工单未提而我发现的新事实 | X 帖要求升级到 2.1.314（博文没有）；X 帖称范围不含 “Grok Bot, Grok or our other SpaceXSI products”；Cursor 博文页脚显示 SpaceXSI 订阅条款，说明 Cursor 与 SpaceXSI 的关联；自称 Anthropic 员工的 HN 评论（22:08 北京）；DoublePulsar 的 “Anthropic hacked” 夸大标题；OSM 发布戳 10-30 17:42 北京与正文 “today, October 29th” 不一致；Phorion 称 Cursor “additionally pulling NPM and VSCode versions for 2.1.313 from their relevant registries”（Phorion 原文，Cursor 博文无此句，我无法核实） |
| WIRED 五角大楼稿 | 本组没有取证；若遇把两事并写的报道，只记录：我没有遇到 |

## 五、没做到的

- A2：AIHOT 条目原文（站点连不上）、4,100 的出处（找不到）。
- A7：Anthropic 官网新闻页没有逐个查；Open VSX/Eclipse 基金会页面没有查（只查了 API）；Anthropic 员工身份没有核实。
- A9：Reddit 失败（登录墙）；中文媒体具体页面没有找到；HN 评论的分数拿不到（Algolia 不给）。
- 图：没有截 npm/Open VSX 版本列表页与 OSM 的 Attack Chain 表；07 号图的命令被页面横向滚动截断。
