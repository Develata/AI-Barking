# 1012 期 C 组取证：ArtCraft 用 AI 复刻 Adobe 套件

取证方：Claude Sonnet 5.5（claude-sonnet-5-5）。时间一律北京时间（UTC+8），括号内为原时区；本机美东 EDT（UTC-4）。取数时刻 2026-10-12 北京 13:10–14:00（UTC 05:10–05:55）。抓取过程见 `capture-log-c.md`，脚本与存档均在本目录（`c-` 前缀）。HTML 原件、图片只存本地，不入库；`c-bsky-thread.json` 约 0.96 MB，`c-techradar.html`（1.9 MB）不应入库。

分级说明：对本组而言，GitHub 仓库、README、commit、release、官网、开发者本人的 HN 评论属 L1/L2（官方账号与署名发言）；TechRadar、Ars Technica 等属 L4；AIHOT、卡兹克等属 L5/L6。HN 评论、Bluesky 回复只作线索（L6）。

## 一、核心事实清单（C1–C11）

### C1 storytold 仿制仓库清单、“7 款”出处

GitHub API（`gh api repos/storytold/<repo>`，取数 2026-10-12 北京 13:54，UTC 05:54:37；原始见 `c-heat.json`、`c-gh-repos-detail.jsonl`、`c-gh-storytold-repos.json`）。许可证列是 GitHub 检测值（两份许可证文件并存时显示 Apache-2.0）；各 README 自述均为 “MIT OR Apache-2.0”。CLAUDE.md / AGENTS.md 指仓库根目录是否有该文件（`gh api .../contents`，同刻）。

| 仓库 | README 自述对标产品 | stars | forks | created_at（UTC） | pushed_at（UTC） | 许可证（检测） | CLAUDE.md / AGENTS.md |
|---|---|---:|---:|---|---|---|---|
| photocraft | Adobe Photoshop | 48,967 | 7,898 | 2026-09-30T13:37:48Z | 2026-10-12T05:43:06Z | Apache-2.0 | 无 / 有 |
| vectorcraft | Adobe Illustrator | 7,250 | 3,263 | 2026-09-30T13:43:03Z | 2026-10-12T04:29:03Z | Apache-2.0 | 有 / 有 |
| filmcraft | Adobe Premiere Pro | 10,419 | 4,117 | 2026-09-30T13:41:20Z | 2026-10-12T03:26:26Z | Apache-2.0 | 有 / 有 |
| lightcraft | Adobe Lightroom | 10,392 | 3,578 | 2026-09-30T14:09:27Z | 2026-10-11T15:02:30Z | Apache-2.0 | 有 / 有 |
| pdfcraft | Adobe Acrobat | 9,940 | 3,772 | 2026-09-30T13:39:37Z | 2026-10-12T05:38:32Z | Apache-2.0 | 有 / 有 |
| effectcraft | Adobe After Effects | 5,275 | 2,594 | 2026-10-01T14:24:46Z | 2026-10-11T22:02:58Z | Apache-2.0 | 有 / 有 |
| designcraft | Adobe InDesign | 3,090 | 2,002 | 2026-10-01T13:13:14Z | 2026-10-12T03:58:49Z | Apache-2.0 | 有 / 有 |
| wordcraft | Microsoft Word | 6,671 | 1,877 | 2026-10-07T08:34:24Z | 2026-10-12T05:48:02Z | Apache-2.0 | 有 / 有 |
| soundcraft | Avid Pro Tools | 1,904 | 1,099 | 2026-10-07T08:31:31Z | 2026-10-12T01:06:08Z | Apache-2.0 | 有 / 有 |
| deckcraft | Microsoft PowerPoint | 2,118 | 1,104 | 2026-10-07T05:27:21Z | 2026-10-11T22:02:50Z | Apache-2.0 | 有 / 有 |
| cadcraft | Autodesk AutoCAD | 2,841 | 1,308 | 2026-10-07T05:22:31Z | 2026-10-12T00:42:45Z | Apache-2.0 | 有 / 有 |
| gridcraft | Microsoft Excel | 3,130 | 1,306 | 2026-10-07T05:18:16Z | 2026-10-12T01:25:48Z | Apache-2.0 | 有 / 有 |

另有 craft-launcher（安装器，1,334 stars）、craft-libs、craft-fonts、photocraft-corpus、artcraft（主产品 18,790 stars）、artcraft-services（后端，783 stars，许可证 NOASSERTION）。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 与 Adobe 对标的仓库数是 7（Photoshop、Illustrator、Premiere Pro、Lightroom、Acrobat、After Effects、InDesign）；官网总数为 12 款 Craft 应用（另含 Word、Pro Tools、PowerPoint、AutoCAD、Excel 五款非 Adobe 对标） | L1 | https://getartcraft.com/apps ；各仓库 README 首段（如 https://github.com/storytold/photocraft ） | 官网：“Twelve apps. One craft.”；“Image editing, vector illustration, video, photography, PDFs, motion graphics, page layout, word processing, audio, presentations, CAD and spreadsheets.” | 无 | “7 款”最早见于开发者在 Reddit 的发帖标题（经 HN 49938443 转录标题 “I've created an open source 100% Rust clean-room clone of 7 of Adobe's apps”，Reddit 本站 403 未读）与 HN 评论 49934844（10/02 北京 23:47）的七行链接；官网 10/07 的 Bluesky 卡片描述写 “seven native, open-source apps”，现版本（取数时）写 “twelve”。PDF 仓库曾名 PrintCraft，10/07 起改名 PdfCraft（vectorcraft 提交 1f78732，photocraft PR #766）；VectorCraft 曾名 DrawCraft（10/01 改名，据 vectorcraft CLAUDE.md） | 已找到 |
| PhotoCraft 仓库：创建 2026-09-30、星标 4.9 万 | L1 | https://github.com/storytold/photocraft | GitHub API：stargazers_count 48,787（05:11Z）→ 48,967（05:54Z）；forks 7,861 → 7,898；created_at 2026-09-30T13:37:48Z；description “PhotoCraft: An open-source, photo editor written by scratch in pure Rust” | 25 | 星标含取数时刻差；git 首个提交 “init repo” 2026-09-30T02:12:04-04:00（= UTC 06:12:04，北京 14:12:04），第二个提交 “one-shot” 2026-09-30T05:02:08-04:00（北京 17:02:08；API：241 个文件，+42,241 行）；GitHub 仓库创建晚于首个提交 | 已找到 |
| README 称“clean-room reimplementation of Adobe Photoshop” | L1 | https://github.com/storytold/photocraft （README，`c-readme-photocraft.md`，取 2026-10-12 北京 13:12） | “Image editing; an open-source, clean-room reimplementation of Adobe Photoshop, rebuilt in pure Rust.”；“**Clean-room:** implemented from public specs and observed behaviour only. No proprietary code, shaders or assets.” | 25 | 其余 11 个 Craft 仓库 README 首段有同类句式（均为 “an open-source, clean-room reimplementation of …”）。商标声明原句：“PhotoCraft is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Adobe Inc.” | 已找到 |
| 项目有 AI 开发痕迹（CLAUDE.md / AGENTS.md / commit 署名） | L1 | https://github.com/storytold/photocraft/blob/main/AGENTS.md ；git 历史（`c-gh-commit-stats.txt`） | AGENTS.md 首行：“# AGENTS.md: guide for AI agents and contributors”；README：“New contributors and AI agents: start with [`AGENTS.md`](AGENTS.md)”；commit 尾注 “Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>” | 无 | blobless bare clone 统计：photocraft 共 1,614 个提交，其中 1,045 个带 “Co-Authored-By: Claude …” 尾注（Opus 5.5 共 881、Opus 5.5 (1M context) 93、Opus 4.8 45、无版本 “Claude” 32、Fable 5.1 16、Sonnet 5.5 6、Mythos 5.1 4、其他 2；同一提交可带多个模型尾注，故各模型计数之和 1,079 大于 1,045）；git 作者名 “Brandon Thomas” 256 个提交（其中 181 个有 Claude 尾注）；最早带尾注提交 2026-10-01T13:25:17-04:00（北京 10/02 01:25）。尾注只表示该提交声明 Claude 共同署名，不等于代码占比。photocraft 贡献者 225 人（GitHub contributors API，合计 1,613 次提交），echelon 253 次 | 已找到 |

### C2 开发者自述：AI 参与、模型、成本、时长；身份

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 开发者本人称正在用 Opus 5.5 做 Adobe 全套 | L2（HN 署名 echelon；与 GitHub 同名账号的关联为推断，见下） | https://news.ycombinator.com/item?id=49934844 | “I'm currently using Opus 5.5 to create 100% pure-Rust, immediate mode rendering, open source implementations of Adobe's entire suite:” 后列 photocraft（Photoshop）、vectorcraft（Illustrator）、filmcraft（Premiere）、lightcraft（Lightroom）、printcraft（Acrobat Pro）、effectcraft（After Effects）、designcraft（InDesign）；“But now you can just create 1:1 functional equivalents, clean room, that are highly performant and cross-platform.” | 26 | 发于 2026-10-02T15:47:03Z（北京 10/02 23:47:03）；父帖为 HN 49934228（Home Assistant 博文帖）。同一评论树里 dofm（HN 50006830，10/08 北京 23:14）反驳：“You're not doing this clean-room, if Claude is running Photoshop and studying it while writing code. You do not have a clean room.” | 已找到 |
| 开发者称不再自己写代码 | L2 | https://news.ycombinator.com/item?id=49879369 | “I haven't written a single line of code since February, and I don't think I ever will again.” | 无 | 发于 2026-09-28T15:13:59Z（北京 9/28 23:13:59），上下文是谈支付系统，非专指本项目；Ars Technica 转述为 “After coding exclusively with AI tools since February” | 已找到 |
| 开发者在 HN 称“一次性写出整个 Photoshop” | L2 | https://news.ycombinator.com/item?id=49928088 | “Software is over. But for real. I just one-shotted Photoshop. The whole thing.” | 无 | 发于 2026-10-01T23:05:21Z（北京 10/02 07:05:21），在 “Ask HN: Who is hiring? (October 2026)” 帖下。10/08 15:17:08Z（北京 23:17）他本人回复（HN 50006851）：“To clarify from reporter mischaracterizations, this comment wasn't "gloating".”。git 里确有名为 “one-shot” 的提交（见 C1），但此后提交达 1,600 余个 | 已找到 |
| 开发者公开身份 | L1/L2 | https://github.com/echelon （GitHub API：name “Brandon Thomas”；bio “Founder: ArtCraft, Storyteller.ai, FakeYou.” “Engineer, filmmaker. Rust.”）；PetaPixel、Ars Technica、Gizmodo 均称 Brandon Thomas | 同上 | 无 | 只记职业身份与公开账号。HN 用户 echelon 与 GitHub 用户 echelon 为同一人是推断（同名、同谈这些仓库；PR 作者 @echelon；HN 评论 49982081 的网友称“The author is on HN too”）。Ars 链到其 LinkedIn（未读）。Futurism 称联合创始人 “Michael Caffrey” 向其表示项目为 “reverse-engineered … using Claude 5.5”（见吠点与夸大节） | 部分支持（姓名与模型有一手；HN↔GitHub 关联为推断） |
| 所用模型、花费、时长 | L2 | HN 49934844（模型）；git 尾注（模型）；PhotoCraft ROADMAP.md（工时口径） | ROADMAP：“Hours are Opus 5.5 agent wall-clock hours for one agent working sequentially, calibrated from this repository's 1,004 merged PRs”；“Remaining to **full parity** ~2,300–4,100 h” | 无 | 开发者公开的是“剩余工时估算”，而非已花费的 token / 金额 / 开发时长：**未见**他公布过花费或总时长。HN 网友 4b11b4（49959653）问 “At what cost?”，echelon 回 “Only have to pay the price once.”（49959674） | 部分支持（模型已找到；花费、时长未找到一手来源） |
| AIHOT 条目“Brandon Thomas 用 Claude Opus 5.5 逆向重写 Adobe 七款软件” | L5 | AIHOT 公开 API（`https://aihot.news/api/public/items?q=Adobe`，存 `c-aihot-api-adobe.json`）；原始来源为卡兹克 X 长文 https://x.com/Khazix0918/status/2109501787736813663 与公众号 | AIHOT 标题：“从Brandon Thomas用Claude Opus 5.5开源重写Adobe全家桶看AI逆向一切趋势”（X，publishedAt 2026-10-12T04:29:53Z = 北京 12:29:53）；“AI逆向一切：Brandon Thomas用Claude Opus 5.5重写Adobe七款软件开源，REA项目逼近10万Star”（公众号，2026-10-12T00:26:00Z = 北京 08:26:00）；摘要“用Claude Opus 5.5以Rust逆向重写了Photoshop等7款Adobe软件” | 无（AIHOT 网页被 bot 拦截，见 C9） | 姓名与“Opus 5.5”有一手（上两行）；“逆向”是卡兹克的措辞，仓库自述为 clean-room，并明写“Inspected without launching it”式检查（见吠点） | 部分支持 |

### C3 “decompiled（反编译）”说法的源头

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| Bluesky 上 James O'Malley 的帖子写成“反编译” | L6（社交转述；作者为公开记者/播客主） | https://bsky.app/profile/jamesomalley.co.uk/post/3mxbl36nkms2q （公开 API 取 JSON：`c-bsky-thread.json`） | “Holy shit. Someone has decompiled the Adobe suite, rebuilt it in Rust and released it as open source. Either all software is dead or the US copyright laws are about to receive an update.” | 31 | createdAt 2026-10-07T08:54:11.813Z（北京 16:54:11）；取数时 4,420 赞、1,244 转、200 回复、650 引用；链接卡描述写 “seven native, open-source apps”。他本人当日后续回复（09:57Z，北京 17:57）仍写 “If software can be easily decompiled, it reinforces my argument that security through obscurity is no longer an option.”；11:37Z 回复承认 “the open source rebuilds that were made in days are buggy and a bit broken at present.”。**未见更正**：作者主页至取数时（10/12 05:22Z）已拉取的帖子里，10/07 之后无关于 decompiled 的更正（`c-bsky-authorfeed.json`） | 已找到 |
| 该说法被指出不准确 | L6 | 同帖回复：boxobark.ing（184 赞）；HN 49993989 | boxobark.ing：“This isn't a decompilation, it's a vibe coded reimplementation.”；HN fotcorn（10/07 15:15:10Z，北京 23:15）：“This has been posted a few times already, and no, it's not decompiled from real Photoshop.” | 无 | HN 49993722 的帖子标题照用 “Someone has decompiled the Adobe suite, rebuilt in Rust and released it as OSS”（提交者 jbredeche，10/07 14:55:13Z，北京 22:55:13；取数 36 分 / 31 评）。注：同为反编译字样的另一类说法（“reverse-engineered”）见吠点与夸大节 | 已找到 |

### C4 TechRadar 全文

存档 `c-techradar.txt` / `.html` / `.headers`（curl，取数 2026-10-12T05:16:15Z）。页面 JSON-LD：datePublished 2026-10-10T13:00:00+00:00（北京 21:00），HTTP last-modified 2026-10-10 13:01:22 GMT；作者 Steve Paris（署名栏）。**标题两处不同**：页面 headline 与 URL 为 “One dev is recreating 7 Adobe apps in Rust for free — so I tested it out”，但 `<title>` 与 og:title 为 “One dev is recreating 7 Adobe apps with AI — so I tested it out | TechRadar”（疑似后改，未见更正说明）。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| PhotoCraft 打不开 HEIC | L4 | https://www.techradar.com/pro/one-dev-is-recreating-7-adobe-apps-in-rust-for-free-so-i-tested-it-out | “PhotoCraft failed for me at the very first hurdle: it can’t read the HEIC format.”；“To be fair to PhotoCraft, many apps and services still have an issue with HEIC… but Photoshop doesn’t.” | 27 | 作者 iPhone 照片；转 JPG 后继续测试 | 已找到 |
| 保存时只能存 JPG | L4 | 同上 | “when I decided to save my project, PhotoCraft saved it… as a JPG. There was no file format option and the only one available destroys any possibility of going back to your project and continuing your work.”；“If PhotoCraft doesn’t yet have its own file format, why is there no option to save your work as a PSD document instead?” | 无（文字在 27 图之后的段落） | 注意与扫描摘要“工程保存受限”相比，原文是“只存成 JPG、无格式选项、无 PSD 选项”；而 README 称 PhotoCraft 有原生 `.pcraft` 格式并能存 PSD，见 C5 | 已找到 |
| FilmCraft：不能拖片段边缘裁切 | L4 | 同上 | “Like I couldn’t manually trim a clip in my sequence. I could use the blade tool for sure, but mousing to the end of the clip and dragging its edge inwards? Sorry, no can do.” | 28 | 同段另有 “just like PhotoCraft, it’s crazy slow.”（旧 Intel 机型）。作者另称 “the latest versions of Photoshop and Premiere Pro run rings around these apps on the same hardware.” | 已找到 |
| FilmCraft：音频只有噪声 | L4 | 同上 | “the audio from the clips I was using were completely unusable. All I got when I tried playing the footage back was static. Very loud static. It’s bound to be a lack of a specific codec again” | 28 | “lack of a specific codec” 是作者自己的猜测，未经证实 | 已找到 |
| 测试硬件与版本 | L4 | 同上 | “I’ll be using my trusty 16” Intel-based MacBook Pro”；“PhotoCraft is at 0.20, while FilmCraft’s on 0.2.1.”；“PhotoCraft is 80MB, while FilmCraft, 90MB.” | 无 | 原文未给测试日期；“0.20”应为笔误或指 v0.2.0（该版 2026-10-05T04:42:21Z 发布，北京 12:42）；FilmCraft v0.2.1 发布于 2026-10-06T12:18:26Z（北京 20:18）。TechRadar 发布时（10/10 北京 21:00）PhotoCraft 最新已是 v0.6.0（10/10 02:51Z），FilmCraft 最新为 v0.5.0（10/10 02:51Z）。作者未说明下载自何处 | 部分支持（版本为作者自述，未能核对其实际安装版） |
| 合法性 | L4 | 同上 | “is it even legal? I’m not a lawyer, so I haven’t got a clue, but the developer claims it’s all clean code and nothing copyrighted was copied.” | 无 | 作者原话；并非 TechRadar 的核实 | 已找到 |
| 结论 | L4 | 同上 | “there’s a heck of a lot of overpromising and a huge amount of underdelivering”；“If you’re looking for free alternatives to the Adobe hegemony, these are not the apps you’re looking for. At least not yet.” | 无 | | 已找到 |

### C5 版本差异：HEIC、保存、FilmCraft 裁切

数据来自 GitHub API（releases、pulls、issues、commits），`c-gh-photocraft-releases.json`、`c-gh-filmcraft-releases.json`；时刻为 UTC / 北京。

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| PhotoCraft HEIC 读取：10/10 TechRadar 发布时已有 | L1 | https://github.com/storytold/photocraft/pull/372 ；https://github.com/storytold/photocraft/releases/tag/v0.5.0 | PR #372 “Open HEIC/HEIF photos (#355)”：“iPhones save HEIC by default and Photoshop opens it; PhotoCraft had no HEIF code”（提交信息）；“Read-only (no pure-Rust HEVC encoder)”；v0.5.0 发布说明：“Open HEIC/HEIF photos (#355) by @ljkx in …/pull/372” | 33 | 用户报告 issue #355 “HEIC Support missing”（“If I try to open HEIC images it cannot be opened in photocraft 0.2”）开于 2026-10-06T18:17:24Z（北京 10/07 02:17）；PR #372 由外部贡献者 ljkx 提交（10/06 19:42:02Z），合并于 2026-10-07T14:20:41Z（北京 22:20:41）；v0.5.0 发布 2026-10-08T14:52:50Z（北京 22:52:50）。所以 **TechRadar 10/10 发布前 2 天，官方版已含 HEIC 读取**；其测试版（“0.20”≈v0.2.0）早于该修复。v0.5.0 标签下三个打包脚本均以 `--features heif` 构建：`packaging/linux/package.sh`、`packaging/windows/package.ps1`、`packaging/macos/package.sh`（`cargo build --release --locked -p photocraft -p photocraft-cli --features heif`）；apps/photocraft/Cargo.toml 注释 “official builds (CI, packaging/, release) build with `--features heif`”。**未对发布的二进制实测打开 HEIC** | 已找到（修复时间与打包开启，均看源码；未实测二进制） |
| “只能存成 JPG”已改：平面图的分层编辑 Save 默认 PSD | L1 | https://github.com/storytold/photocraft/pull/2234 | v0.7.0 发布说明：“Default layered edits of flat images to PSD on Save by @harrisonoconnorhover in …/pull/2234”；另 “Save As to a flat format keeps the layered original (#2550, #2579)”（PR #2611） | 无 | PR #2234 合并 2026-10-10T06:23:58Z（北京 14:23），**早于 TechRadar 发布 7 小时，但 v0.7.0 发布于 2026-10-11T02:12:31Z（北京 10:12）**，晚于 TechRadar；#2611 合并 10/10 17:12:42Z；另有 open PR #3244 “Save in place for every writable format a document was opened from”。README 另称有原生 `.pcraft` 格式 | 已找到 |
| FilmCraft 拖边裁切已修 | L1 | https://github.com/storytold/filmcraft/pull/399 ；issue #259 | issue #259 “Dragging Video After Trim”（“when you drag it it just drags the whole video file instead of making the video shorter or longer”）；PR #399 “#259: the edge you press is the edge you trim”；FilmCraft v0.5.0 发布说明含 “#259: the edge you press is the edge you trim by @ItzBloxy in …/pull/399” | 无 | issue 开于 2026-10-08T11:53:09Z，PR 合并 2026-10-09T12:17:17Z（北京 20:17），v0.5.0 发布 2026-10-10T02:51:29Z（北京 10:51）。症状与 TechRadar 所述相符，但 TechRadar 未写具体操作，**是否为同一缺陷是推断** | 已找到（修复）；同一缺陷为推断 |
| FilmCraft 音频噪声 | L1 | https://github.com/storytold/filmcraft/pull/359 ；issue #236 | issue #236 “Audio crackling when reviewing footage and also after exporting”；v0.5.0 发布说明：“#236: audio in MKV and OBS-remuxed MP4 plays without crackling by @IppSec in …/pull/359”；另有 issue #76 “Weird clipping noise in MAC Mini M1”、#108 “HE-AAC source … the AAC track is corrupted (samples up to 8x full scale)”（均已关闭，10/07 11:08Z 前后；PR #129） | 无 | 修复限于 MKV / OBS 转封装 MP4 等具体情形，TechRadar 作者的素材格式未知，**不能断言已修复其所遇问题** | 部分支持 |

### C6 开发者对成熟度的自述；README 的成熟度标注

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| “super early alpha” | L2 | https://news.ycombinator.com/item?id=49959605 | “Haha, I wasn't ready to post this to HN. It's not anywhere close to ready. … (I want to post this again when we clear Beta phase.) These are still super early alpha. All MIT/Apache licensed. No telemetry. I'm going to try to drive these to 100% parity but it's going to take a minute. And I'll need lots of help. (Anyone want to join me? I'll pay stipends and bug bounties.)” | 30 | 发于 2026-10-05T01:04:03Z（北京 10/05 09:04:03）；帖由 aghuang 提交（HN 49958850，10/04 23:02:43Z = 北京 10/05 07:02:43，取数 164 分 / 232 评） | 已找到 |
| “99% parity … months and not years” | L2 | https://news.ycombinator.com/item?id=49961661 | “99% parity will take a while, but I'm sure it'll be measured in months and not years. Feel free to check in on this claim. It's measurable.” | 无 | 发于 2026-10-05T07:17:09Z（北京 15:17:09），是对网友 slopinthebag（49961583）的回复。slopinthebag 引他此前所说 “you posted this on multiple subreddits and claimed "We're going to reach 100% parity within a month."”（49961705）；Ars Technica 引同一句并标出处为他的 Reddit 帖 r/SaaS 1wwnhbt（Reddit 本站 403，未读，**仅见转述**）。他回：“Reddit isn't HN. I needed to seed a community that could help contribute bug reports and PRs.”（49962334） | 已找到（HN）；Reddit 原帖仅见转述 |
| README 的成熟度标注 | L1 | https://github.com/storytold/photocraft | 徽章 “status: early alpha”；“> [!IMPORTANT] **Status:** PhotoCraft is in early alpha, and we want to be straight about where it stands: much of Photoshop's feature surface exists in some form, but **it is not yet a Photoshop replacement for daily professional work**. The biggest gaps are AI/generative features, 15 missing tools, depth in typography and pro workflows, and plug-in compatibility. Every Photoshop menu item is wired to a command (`docs/parity-checklist.md`), but that measures wiring, not behaviour.” | 25（徽章）、29（Status 段） | 取数 2026-10-12 北京 13:34；“15 missing tools” 在 ROADMAP.md 已写 “toolbox count 56 of 68, 12 missing”（README 与 ROADMAP 当日不一致） | 已找到 |
| 项目自评数字 | L1 | https://github.com/storytold/photocraft/blob/main/ROADMAP.md （`c-photocraft-ROADMAP.md`） | “**Stage: alpha** · next: beta”；“Feature breadth ~76% … partly measured: menus 628/628, tools 56/68”；“Ready for real work ~45% (40–50%) … estimated”；“22 of 25 performance budgets are missed, and PSDs we write are not yet reliably accepted by Photoshop (#1281, #2469)”；“Last reviewed: 2026-10-11” | 无 | 自评口径，“estimated”；比例随 commit 更新。其他仓库自评：LightCraft README “As a day-to-day Lightroom replacement we're nearer 61% for the whole product, ~49% for a typical working photographer”；FilmCraft README “Feature breadth: ~86%”“Ready for real work: ~56%”（“re-measured 2026-10-10 against Premiere Pro 26.5.2”）；EffectCraft README “It isn't yet a replacement for After Effects on client work.” | 已找到 |
| 官网的成熟度标注 | L1 | https://getartcraft.com/apps | PhotoCraft、PdfCraft 标 “Early alpha”；其余 10 款标 “In development” | 无 | 取数 05:15Z；官网 PhotoCraft 页文案写 “The image editor you already know how to use.” | 已找到 |

### C7 ArtCraft 主产品与 Craft 应用仓库的关系；许可证

| 说法 | 级 | 一手来源 URL | 原文摘句 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| 主产品是 AI 图像/视频工作室；Craft 应用以 AI 生图为可选项并作为收入来源 | L1 / L4 | https://getartcraft.com/ ；https://github.com/storytold/artcraft ；Ars Technica（见 C10 表） | 官网页尾：“The open-source studio for controllable AI image and video.”；artcraft README：“ArtCraft is the IDE for interactive AI image and video creation.”；PhotoCraft README：“And ArtCraft itself, our AI image and video studio for artists who want real control.”；Ars（引开发者 Reddit 评论）：开发由 “selling token-based access to the pre-existing Artcraft visual AI model and IDE” 资助，“We have a team building popular software for filmmakers, so supporting this is fully funded.”（Reddit 评论 pe799hc，**仅见 Ars 转述**） | 无 | 官网定价页：Basic $10 / Pro $35 等订阅，“Free & open source · Subscriptions optional†”（取数 05:15Z，20% 折扣活动中） | 已找到 |
| 主仓库许可证：10/10 前为 “fair source”，10/10 改为 MIT OR Apache-2.0 | L1 | https://github.com/storytold/artcraft ；提交 8abde75 | 旧 LICENSE.md：“ArtCraft is a form of "fair source" software. … You cannot sell the ArtCraft software commercially … You cannot use the ArtCraft code to develop a competing business or product (eg. selling AI image and video tools).”；改许可提交信息：“Relicense ArtCraft under MIT OR Apache-2.0 (#2043) … Removes the WIP fair-source `LICENSE.md` and adds `LICENSE-MIT` and `LICENSE-APACHE`, matching PhotoCraft, LightCraft, and the other craft apps.”；“Co-authored-by: Claude Opus 5.5” | 无 | 提交时间 2026-10-10T08:14:38Z（北京 16:14:38）。HN 10/05 02:35:44Z 评论 kmeisthax（49960110）所指 “fair source” 是当时主仓库的旧许可证，**该评论当时属实，现已过时**。后端仓库 artcraft-services 仍为 LICENSE.md（“ArtCraft License (WIP)”，同样 “fair source”，GitHub 检测 NOASSERTION）。改许可提交信息自称 “Follow-up needed”：NOTICE 排除的 Nintendo 音效、meme 图片等素材尚待替换或移除 | 已找到 |

### C8 Adobe 公开回应

| 说法 | 级 | 来源 | 原文摘句 | 截图 | 条件 | 状态 |
|---|---|---|---|---|---|---|
| Adobe 官方声明或发言人引语 | — | 已查：PetaPixel、Ars、Futurism、Vice、Gizmodo、TechSpot、HotHardware、Phoronix、OMG Ubuntu、AppleInsider、Tedium 全文存档（`c-media-*.txt`）；WebSearch 三轮；Adobe 官方新闻页未单独检索 | PetaPixel：“PetaPixel has asked Adobe for comment and will update this story in the event the company has anything to say about ArtCraft.”（页面 updated 2026-10-08T11:24:18Z；取数 05:47Z 时仍是此句） | 无 | 其余媒体全文无 “Adobe said / declined / did not respond” 之类句子（grep 核对）。Adobe 新闻室首页（https://news.adobe.com/ ，curl 200，存 `c-adobe-news.txt`，取数 13:5x）无 ArtCraft/PhotoCraft 字样（仅首页最新条目，未做站内搜索）。Vice 的 “Adobe's legal team is probably performing some stretches and warm-up exercises …” 是 Vice 的调侃，不是 Adobe 的话 | 未找到一手来源 |

### C9 热度

| 说法 | 级 | 一手来源 URL | 数字 | 截图 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| HN 帖子积分与评论（HN Algolia，取数 2026-10-12T05:54:37Z，北京 13:54；`c-heat.json`） | L5 | https://news.ycombinator.com/item?id=49958850 等 | 49958850 “ArtCraft Apps – open-source Adobe compatible suite written in Rust”：164 分 / 232 评（提交 10/04 23:02:43Z = 北京 10/05 07:02:43）；49981449 “Adobe Creative Suite Cleanroom Port to Rust”：134 / 79（10/06 17:20:00Z）；49993722 “Someone has decompiled the Adobe suite, rebuilt in Rust and released it as OSS”：36 / 31（10/07 14:55:13Z）；50010170 “What ArtCraft's Vibe-Coded Apps Say About Adobe”（tedium.co）：32 / 70（10/08 18:44:29Z）；50004284 “Open-Source Rust Alternatives to Adobe Apps”：33 / 31（10/08 10:55:10Z）；49988881 “Clean room implementation of Adobe products”：26 / 31；49986072 “Photocraft: An open-source, clean-room reimplementation of Adobe Photoshop”：15 / 17；49938443 “I've created an open source 100% Rust clean-room clone of 7 of Adobe's apps”（链到 r/Adobe，已被版主删除）：18 / 1；49982597 “Someone recreated Adobe suite with Opus and open sourced them”：4 / 1；49989811 “Opus 5.5 Built Seven Adobe Clones in Rust”（orcarouter）：2 / 0。**WordCraft（微软 Word 对标）10/12 北京 09:57 提交的 HN 50050370：91 / 82**（“An open-source, clean-room reimplementation of Microsoft Word in pure Rust”） | 无 | 与扫描摘要一致（164/232，49993722 未给数字）。评论数为 Algolia 展开的评论总数。HN 评论本身的得分网站不公开，**评论得分一律未取到** | 已找到 |
| Bluesky 帖子热度 | L6 | https://bsky.app/profile/jamesomalley.co.uk/post/3mxbl36nkms2q | 4,420 赞、1,244 转、200 回复、650 引用（公开 API，05:54Z；取数时 05:22Z 同值） | 31 | | 已找到 |
| 卡兹克 X 长文 | L6 | https://x.com/Khazix0918/status/2109501787736813663 （fxtwitter：`c-fx-khazix.json`，正文 `c-x-khazix-article.txt`） | 浏览 8,564、点赞 86、转发 5、收藏 76（05:50Z）；created_at “Mon Oct 12 04:29:53 +0000 2026”（北京 12:29:53） | 无 | | 已找到 |
| AIHOT 10/12 条目 | L5 | https://aihot.news/items/hvxwaxeyj6i82xv44ef4ce6fz 、https://aihot.news/items/u9f1ozdbs5nobwugj3yss3z0e （经公开 API 取得，`c-aihot-api-adobe.json`） | 两条均 selected=true；API 字段 score 为 67（X 条）与 66（公众号条）；category=opinion；discoveredAt 10/12 04:35:34Z、02:12:04Z。热点榜（`/hot`，10/12 13:20 更新）与日报页（10/12 第 174 期，头条为 GPT-6.1 Sol 上下文条）里**没有** ArtCraft 条目 | 无 | AIHOT 网页（首页、/all）对无头 Chrome 与 curl 返回 JS 挑战或 `{"error":"blocked","code":567}`，未绕过；热点榜与日报页可读（`c-aihot-hot.txt`、`c-aihot-daily.txt`）。API 的 score 含义未核对，**不等同于页面热度数字**；页面“热度”与“排名”未取到 | 部分支持（条目与数字有，页面热度与排名未取到） |
| 中文媒体 | L5 | 80aj：https://www.80aj.com/2026/10/09/ai-artcraft-adobe-challenge/ （发布 2026-10-09T03:52:53+08:00）；techgogogo：https://www.techgogogo.com/2026/10/09/artcraft-open-source-free-adobe-alternative-claude/ （发布 2026-10-09T10:30:00Z，北京 18:30） | 见 C10 | 无 | 新浪微博转载页（weibo.com/2/detail/…）搜索可见，未抓取；未找到站长之家、腾讯新闻、华军软件园对应文章（WorkBuddy 扫描未提及这些，不予核） | 部分支持 |
| Reddit | — | r/SaaS 1wwnhbt、r/Bard 1wxmqpt、r/Adobe 1wvyxjd（已删）、r/FuckAdobe 1wwlvq7 | 无 | 无 | reddit.com 与 old.reddit.com 对 curl 返回 403，未硬绕；仅能经 PetaPixel、Ars、Futurism、TechSpot 转述。Futurism 引 r/SaaS 评论 “Briefly tried it. Absolutely amazing”“About time we leave adobe for good” | 未找到一手来源 |

### C10 夸大实例（把事件写歪的标题或文章）

以下均已存档并逐字核对。

| 说法 | 级 | 来源（站点 / URL） | 原句逐字 | 发布时刻（北京） | 与事实的差 |
|---|---|---|---|---|---|
| “反编译” | L6 | Bluesky 帖，https://bsky.app/profile/jamesomalley.co.uk/post/3mxbl36nkms2q | “Someone has decompiled the Adobe suite, rebuilt it in Rust and released it as open source.” | 2026-10-07 16:54:11 | 项目自述为 clean-room；回复与 HN 均指出并非反编译；作者未更正（见 C3） |
| 反编译（HN 标题） | L6 | HN 49993722 | “Someone has decompiled the Adobe suite, rebuilt in Rust and released it as OSS” | 2026-10-07 22:55:13 | 照抄 Bluesky 帖 |
| “逆向工程” | L4 | Futurism，https://futurism.com/future-society/adobe-creative-suite-software-open-source-artcraft | 标题：“Adobe in Trouble After Someone Reverse Engineered Adobe Creative Suite and Released It as Open Source”；正文：“ArtCraft is, at its core, an effort to reverse engineer Adobe’s software”；“As ArtCraft co-founder Michael Caffrey told Futurism, the whole project came about after the team reverse-engineered the top seven Adobe tools using Claude 5.5.” | 2026-10-09 01:54:54（发布 2026-10-08T13:54:54-04:00） | 标题对 “Adobe in Trouble” 无依据（全文无 Adobe 回应）。文中引的联合创始人 “reverse-engineered” 一句只见 Futurism 转述，**未在项目一手来源中找到同样措辞**；仓库自述是 clean-room（但见吠点中的 “installed bundle inspected”）。Futurism 另称 “a four-person team of vibecoders” |
| “AI 驱动的逆向工程” | L4 | Ars Technica，https://arstechnica.com/ai/2026/10/software-is-over-bold-ai-developer-takes-aim-at-adobe-with-open-source-clones/ | “Now, one developer is using AI-powered reverse engineering to mimic the look, feel, and functionality of Adobe’s well-known software with what he hopes will soon be perfect fidelity.” | 2026-10-08 05:56:42（2026-10-07T21:56:42Z） | 同文也写他说用 Opus 5.5 做 “clean-room replacements”；“reverse engineering” 是作者概括。该文原文也报道了 “100% parity within a month” 与他后来的修正，并提 “trade dress” 法律风险 |
| “逆向” | L4 | Vice，https://www.vice.com/en/article/everything-we-know-about-the-free-adobe-clones-built-with-ai-do-they-actually-work/ | “According to Ars Technica, developer Brandon Thomas used Anthropic’s Claude Opus 5.5 and Rust to reverse-engineer Adobe’s core software suite into an open-source clone called ArtCraft.” | 2026-10-10 01:19:43（2026-10-09T17:19:43Z） | 转述 Ars 的措辞，并称 “legally dubious software” |
| “逆向”“利用 AI 逆向” | L4 | TechSpot，https://www.techspot.com/news/114136-software-over-developer-uses-ai-build-free-adobe.html | “One developer has even used AI-powered reverse engineering to create a suite of tools intended to replicate Adobe's apps, leading him to declare that "software is over."” | 2026-10-08 19:15（2026-10-08T06:15:00-05:00） | 同上；“declare that software is over” 来自 HN 一条与此项目无关的招聘帖下的评论（见 C2） |
| “逆向” | L4 | HotHardware，https://hothardware.com/news/dev-uses-ai-to-build-clones-of-photoshop-and-entire-adobe-suite-for-free | “Thomas leveraged AI to reverse-engineer Adobe's suite, and even went as far as promising to match 100% of Adobe’s core features within a month through "clean-room replacements."” | 2026-10-08 22:22 | 同上 |
| “一位开发者”“一人” | L4 | HotHardware | “A lone developer has released an open-source, AI-generated alternative to Adobe's entire Creative Cloud suite” | 2026-10-08 22:22 | git 贡献者 225 人（C1），有联合创始人与团队；开发者自称 “We have a team”（Ars 转述）；TechRadar 标题也写 “One dev” |
| “像素级复刻”“没碰 Adobe 一行代码，没有逆向工程” | L5 | techgogogo，https://www.techgogogo.com/2026/10/09/artcraft-open-source-free-adobe-alternative-claude/ | 标题：“ArtCraft！开源免费版Adobe全家桶！由Claude像素级复刻！”；正文：“最令人震惊的是开发者的声明：他没碰 Adobe 一行代码，没有逆向工程，没有扒取源码，完全是让 AI 照着功能和界面从零写出，愣是实现了像素级复刻。” | 2026-10-09 18:30 | “像素级”无依据；项目自评为早期 alpha，photocraft ROADMAP 自估 “Ready for real work ~45%”；另一页签写“完成度约 85%/94%/35%”（该站自写）。“没碰 Adobe 一行代码”的“开发者的声明”未能在一手来源找到 |
| “逆向重写 7 款” | L5/L6 | 数字生命卡兹克，X 长文 https://x.com/Khazix0918/status/2109501787736813663 ；AIHOT 摘要 | 正文：“他最近做了一件很搞的事，直接用Claude Opus 5.5给Adobe逆向了，用Rust重写了Adobe的软件，然后，免费开源”；“第一条就是上面Thomas逆向Adobe的路子，一般称为净室设计”；“不到10天，单PS的这个项目，现在已经有4万6的Star了。” | 2026-10-12 12:29:53 | 把 clean-room 当作逆向的一种；“4万6的Star” 与 GitHub 取数（48,787→48,967）大致相符 |
| “用 AI 复刻 7 款 Adobe 软件” | L5 | 80aj，https://www.80aj.com/2026/10/09/ai-artcraft-adobe-challenge/ | “一个名为ArtCraft的开源项目正以净室开发结合AI编程（vibe coding）的方式，逐一对Adobe Creative Cloud套件进行重构复刻。” | 2026-10-09 03:52:53 | 措辞较克制，写“净室开发”；可作对照 |
| “七款全部完成” | — | 未找到明确写“七款全部完成”的标题或文章；多家（如 Gizmodo、PetaPixel）写 “free”“recreated … seven Adobe apps” | Gizmodo：“it feels like using glitchy, but real, Photoshop.”（原句来自作者自述试用） | | 项目 README 自评为 alpha，见 C6 |

### C11 有理有据的质疑（线索 L6；不对法律问题下结论）

HN 评论得分不公开；下列只给评论链接与作者、时刻（UTC）。Bluesky 回复给赞数。

| 质疑 | 链接与原话摘句 | 备注 |
|---|---|---|
| clean-room 的含义：不是 “one team writes specs, another implements” | Ars 评论区（`c-media-ars.txt`；评论总数 370）：“The idea of a clean room implementation relies on multiple human actors. One team writing specs and another totally different team reading them.” | 评论者名未取（Ars 页面未带）；仅为线索 |
| 模型本身训练于开源代码，“clean-room vibe coding” 不成立 | https://news.ycombinator.com/item?id=49960110 （kmeisthax，10/05 02:35:44Z）：“There is no such thing as clean-room vibe coding. Every line of FOSS code has been trained on.” | 同一评论引了 Vectorcraft CLAUDE.md 旧版规则（见吠点第 3 条） |
| 若 Claude “在运行 Photoshop 并研究它”则不是 clean room | https://news.ycombinator.com/item?id=50006830 （dofm，10/08 15:14:54Z）：“You're not doing this clean-room, if Claude is running Photoshop and studying it while writing code. You do not have a clean room.” | 该仓库规则后来改了（见吠点第 3 条） |
| 反编译 vs 行为观察 | https://news.ycombinator.com/item?id=49993802 （RGS1811，10/07 15:01:40Z）：“If this is actually based on decompilation, they're going to get sued into oblivion. If it's a reverse engineering project based on behavioral observation... it's probably relatively thin, and the question becomes whether the maintainer(s) are going to invest enough effort into it to merit user trust.” | |
| 外观、商标与“look and feel” | https://news.ycombinator.com/item?id=49993775 （labrador）：“We're about to revive the Apple vs Microsoft lawsuits of the 90's over "look and feel"”；Ars 正文：“the ArtCraft apps could still face legal trouble if their “trade dress” too closely mimics Adobe’s familiar appearance and interface.” | README 商标声明见 C1 |
| 维护成本与“slop” | https://news.ycombinator.com/item?id=49981899 （mzajc，10/06 18:00:19Z）：“It's slop, and there's no telling if it will still be maintained a year from now.”；回复 49982219（minimaxir）：“If it stops being maintained, it can just be forked.” | |
| 与 Adobe 的功能差距：自评 vs 实际 | TechRadar（C4）；Gizmodo（`c-media-gizmodo.txt`）：“the Free Transform function was an unpredictable mixed bag”；PhotoCraft ROADMAP：“22 of 25 performance budgets are missed”“3/25 budgets met, measured” | |
| 作者是否在盈利 | https://news.ycombinator.com/item?id=49961243 （hypfer，10/05 06:13:43Z）：“you are trying to make money off those recreations”；echelon 回（49961382）：“These are not monetized in any way. They don't have telemetry, paywalls, or anything.” hypfer 再回指向 getartcraft.com/pricing；Ars 转述开发者说由 AI 套餐收入资助 | 官网定价页有订阅与积分，用于 ArtCraft AI 工作室（C7） |
| Bluesky 上的质疑 | boxobark.ing（184 赞）：“This isn't a decompilation, it's a vibe coded reimplementation.”；tomhannen.ft.com（127 赞）：“Maybe still a way to go. Hacker News comment from someone who tried the Photoshop clone - pretty buggy”；proactiveretcon.bsky.social（110 赞）：“Code has never been the bottleneck of development.” | |

## 二、可能的吠点（原文里有、线索摘要没提或说反）

1. **README 与官网不一致的 PSD 保真数字**：README 写 “opening and re-saving a document renders the same for 307 of the 309 files in the psd-tools test set and 169 of 170 in our mixed ag-psd/psd-tools set”，并明写 “A re-saved file is not byte-identical to its source”；官网 PhotoCraft 页写 “134 of 135 real-world test files round-trip byte for byte.”（`c-home-apps-photocraft.txt`）。口径（渲染一致 vs 逐字节一致）与测试集都不同，不能互相印证。ROADMAP 另写 “PSDs we write are not yet reliably accepted by Photoshop (#1281, #2469)”。
2. **“Every Photoshop menu item is wired to a command” 衡量的是接线而非行为**：README 自己写 “that measures wiring, not behaviour”（原句）。README 另写 “If you know Photoshop, you already know PhotoCraft.”
3. **clean-room 规则在 10/08 被收紧**：Vectorcraft 的 CLAUDE.md 在 10/08 之前写 “Behaviour comes from public docs and black-box observation of the running app with synthetic documents only (screenshots by window id, stored under `plan/illustrator/screenshots/`, never committed)”（`c-vectorcraft-CLAUDE-at-1f78732-pre-1008.md`）；2026-10-08T14:39:16Z（北京 22:39）提交 87cf1ff（作者 bflatastic，尾注 Claude Opus 5.5）将其改为 “Never launch, run, script, automate … screenshot or measure Illustrator or any other Adobe application”，提交信息写 “Adobe's Terms of Use forbid reverse engineering, including observing a program's inputs and outputs to recreate it. The agent rules allowed black-box observation of the running app; they now forbid running, automating, screenshotting or measuring any Adobe application, reading an Adobe install …”。这是项目自己的文字，**不作法律判断**。
4. **PhotoCraft 自述检查过 Photoshop 安装包**：`docs/target-app-parity.md`（2026-10-10 re-measure）写 “Adobe Photoshop 2026, version 27.11.0 … installed at `/Applications/Adobe Photoshop 2026` … Inspected without launching it: `Info.plist` document types, the file-format plug-ins' PiPL read/write flags, … `Default Keyboard Shortcuts.kys` … and strings in the main binary (81 "… Tool" titles …)”；并写 “PhotoCraft's own Photoshop menu tree (…menu_catalog.rs, 628 items) was written from Photoshop's menus”。PhotoCraft AGENTS.md 写 “We studied Photoshop and other proprietary editors for *behaviour and look only*.”。（该文件头 “Last updated: 2026-10-10”，晚于 10/08。10/08 的收紧提交 87cf1ff 在 **vectorcraft** 的 CLAUDE.md；photocraft 的 AGENTS.md 取数时仍写 “behaviour and look only”，**未核对 photocraft 是否采用了同样的收紧规则**。）README 的 “implemented from public specs and observed behaviour only” 应与此并读；**未对合法性下结论**。
5. **开源范围**：artcraft 主仓库 10/10 才改为 MIT OR Apache-2.0（此前是限制“竞争性使用”的 “fair source”）；后端 artcraft-services 仍为该 WIP 许可证（C7）。
6. **“开发者一人”**：贡献者 225 人；除 echelon 外 vangel76（115）、Ni-zav（97）、bflatastic（93）、davidmann4（62）；PhotoCraft 提交里 1,614 个中 256 个是 Brandon Thomas 署名。主要新增代码并非他一人手写（他本人称 “I haven't written a single line of code since February”）。
7. **HEIC、保存 JPG、裁切等 TechRadar 所举缺陷在其发布前已有修复提交或版本**：见 C5；但“保存默认 PSD”的版本（v0.7.0）晚于 TechRadar。TechRadar 未注明测试日期，也未注明所装版本的下载来源。
8. **官网应用数已从 7 扩到 12**：10/07 官网卡片写 seven，现写 twelve；其余 5 款对标 Microsoft Office、AutoCAD、Pro Tools（`c-home-apps.txt`）。主帖若写“7 款 Adobe 软件”应限定“对标 Adobe 的 7 款”。
9. **星标**：PhotoCraft 10/07 早晨 6,691（orcarouter，第三方）→ 10/12 约 4.9 万；各媒体口径时点不同。
10. **“PrintCraft”**：媒体与 HN 帖多用旧名，仓库已改为 PdfCraft。
11. **Futurism 的 “reverse-engineered” 联合创始人引语**：在项目官网、README、开发者 HN 评论中没有同样措辞（开发者本人说 “clean room”“black-box observation”）；该引语只此一见。
12. **HEIC 的 “official builds”**：README 写 “included in official builds; optional `--features heif` for source builds”；Cargo 注释写 “Off by default so distributors choose it at build time; official builds and CI enable it”；v0.5.0 的三个打包脚本确实带 `--features heif`（C5）。

## 三、扫描说法勘误

| 扫描 / 初步核对说法 | 取证结果 |
|---|---|
| README：“Re-saving keeps the render of 307 of the 309 psd-tools test files.” | README 原句为 “Re-saving keeps the render of 307 of the 309 psd-tools test files.”（特性卡）与 “opening and re-saving a document renders the same for 307 of the 309 files …”（PSD 节）一致。官网另写 “134 of 135 … round-trip byte for byte”，口径不同（见吠点 1） |
| 许可证 “MIT / Apache” | README 自述 “dual-licensed under MIT or Apache-2.0, at your option”；GitHub 检测栏显示 Apache-2.0（页面“Apache-2.0, MIT licenses found”） |
| HN 评论称“每个仓库都有 CLAUDE.md” | 取数时 12 个 Craft 应用仓库中 11 个有 CLAUDE.md，**photocraft 根目录没有 CLAUDE.md**（有 AGENTS.md）；craft-launcher、craft-libs、artcraft 也有 CLAUDE.md |
| HN 49993722 评论“it's not decompiled from real Photoshop” | 属实：fotcorn，10/07 15:15:10Z；但提交者不是 echelon，且该帖链接到 Bluesky 而非官方页（thesuitonym 评论 49993890 指出） |
| TechRadar：工程保存受限 | 原文是“只存成 JPG，无格式选项，无 PSD 选项”（C4）；并非“工程保存受限”的笼统说法 |
| TechRadar：FilmCraft 无法拖边裁切、音频噪声、Intel MacBook Pro | 均属实（逐字见 C4）；另有 PhotoCraft 选区“不稳”“移动对象时先动轮廓”与 “crazy slow” 等 |
| TechRadar 标题 | 页面 headline 为 “… in Rust for free …”，`<title>` / og:title 为 “… with AI …”，两处不同 |
| TechRadar “Published 10 October 2026” | 页面 JSON-LD 为 2026-10-10T13:00:00Z（北京 21:00）；last-modified 13:01:22 GMT |
| “echelon 自称开发者” | HN 评论里他以 “we” “I'm currently using Opus 5.5 to create …” 的口吻发言，并未写 “I am the developer”；与 GitHub 同名账号关联为推断（姓名 Brandon Thomas、bio Founder）；他人也称 “The author is on HN too”（49982081） |
| “99% parity … months” 在评论 49959605 | 该句在 **49961661**（echelon 对 slopinthebag 的回复，10/05 07:17:09Z）；49959605 是 “super early alpha”“100% parity … take a minute” |
| HN 49993722 链接的 Bluesky 帖 | 属实，且帖文作者 O'Malley 的帖子与链接卡片见 C3 |
| WorkBuddy：AIHOT 10/12 条目“Brandon Thomas 用 Claude Opus 5.5 逆向重写 Adobe 七款软件” | AIHOT 确有此二条（X 与公众号，作者均为数字生命卡兹克），score 67/66；“逆向”是该作者的措辞；开发者自述 clean-room；姓名与 Opus 5.5 有一手（HN 评论、git 尾注）。AIHOT 热点榜 Top10（10/12 13:20）无此条 |
| HN “fair source”评论 | 该评论属实但当时指主仓库旧许可证，10/10 已改（C7） |
| 已知情况：开发者姓名、所用模型、“逆向”说法均未核实 | 姓名与模型已核（C2）；“逆向”见 C10 |

## 四、本组未完成与失败

- reddit.com、old.reddit.com：curl 403（含 r/SaaS 1wwnhbt、r/Adobe 1wvyxjd、r/Bard、r/FuckAdobe），未用镜像站或代理，开发者的 Reddit 原帖及评论只能经媒体转述。
- AIHOT 网页（首页、/all）：JS 挑战与 `code 567` 拦截，未绕过；热点榜、日报页可读；经公开 API（`/api/public/items`）取得相关条目。
- 卡兹克的公众号原文（mp.weixin.qq.com）：只返回空壳，未存档（经 AIHOT API 取元数据）。
- Albawaba：403 “Malformed Browser Request”，未存档。
- Weibo、Notebookcheck、kocpc、jamecling 等：只见搜索摘要，未抓取。
- 开发者 X（@echelon）、getartcraft.com/news：未读。
- HN 评论得分、Reddit 得分：无公开数据，未取。
- Adobe 新闻室：只读了首页（未做站内搜索），C8 结论建立在媒体全文、搜索与新闻室首页之上。
- 开发者花费、时长、token：未找到一手来源。

## 五、截图清单

图片均为 2 倍像素比、宽 1400 px（页面 700 CSS px）；匿名 headless Chrome，全新临时 profile；已逐张打开目视确认。

| 文件 | 内容 |
|---|---|
| `25-photocraft-readme-top.png` | GitHub photocraft 仓库页首屏：About 描述、`48.8k stars` / `7.9k forks`、README 标题、“clean-room reimplementation of Adobe Photoshop” 句、“status early alpha” 徽章（浅色页，窄屏布局，页头显示 Sign in，无登录信息） |
| `26-hn-echelon-opus55.png` | HN 评论 49934844：“I'm currently using Opus 5.5 … Adobe's entire suite”（评论全文；HN 显示 “9 days ago”） |
| `27-techradar-heic.png` | TechRadar “PHOTOCRAFT VS PHOTOSHOP” 小节：HEIC 失败段（含小节标题与截图图片） |
| `28-techradar-filmcraft.png` | TechRadar “FILMCRAFT VS PREMIERE PRO” 小节：不能拖边裁切、慢、音频静噪段 |
| `29-photocraft-readme-status.png` | README 的 Important 状态框（early alpha，“not yet a Photoshop replacement for daily professional work”） |
| `30-hn-echelon-alpha-parity.png` | HN 评论 49959605：“super early alpha … 100% parity … take a minute” |
| `31-bsky-omalley-decompiled.png` | Bluesky 官方嵌入渲染：O'Malley “decompiled” 帖卡片（含计数与时间 “4:54 · Oct 7, 2026”，浏览器时区美东） |
| `32-vectorcraft-commit-cleanroom-rule.png` | GitHub vectorcraft 提交 87cf1ff 信息：“Clean-room: never run, automate or observe Adobe software …”，尾注 Claude Opus 5.5 |
| `33-photocraft-pr372-heic.png` | GitHub PR #372 “Open HEIC/HEIF photos (#355)”：已合并，范围至 “Read-only … Saving still works through every other format.” |

截图 TechRadar 页的两张图中，文章配图是 TechRadar 页面所载（“Image credit: ArtCraft // Future”），属于被截页面内容，未改动。
