# Codex 交接：0925 期配图截图

## 目标

为“AI 吠点”0925 期截取 10 张一手页面截图（5 张正式图、5 张备用图），供小红书、抖音、微博、公众号配图。截图是证据，读者要能凭图核对正文说法，所以只能截原站实时页面，不能重绘、改字或用转载图。

Develata 的原话：“写一个 Codex 交接任务去截”。

## 背景与先读文件

- 仓库：`E:\gitclone\AI-Barking`，分支 `main`，基线 commit `8568147`。
- 先读：
  1. `AGENTS.md`：交付格式。
  2. `EDITORIAL.md`：“事实分级”一节。
  3. `0925/images/README.md`：截图清单，含文件名、截取位置、对应正文段落。**以该文件为准。**
  4. `0925/doc_0925_publish.txt`：正文，用来理解每张图要证明哪句话。
- 本期两个选题：Anthropic 恢复对三类“输出前拒答”收费；澳大利亚政府披露 OpenAI 的 agent 未授权访问 Medicare 统计报告门户。

## 起始基线

- `git status`：只有 `?? 0924/images/00-cover.png` 与 `?? common_images/`（Develata 的未跟踪文件）。保留，不动、不提交。
- `0925/images/` 目前只有 `README.md`，没有图片。

## 环境

- 用 Codex 应用自带的浏览器打开页面并截图。如 X 帖子需要登录才能看全，用浏览器里已登录的会话；不要自行登录或输入任何凭据。某个页面打不开时，记为失败并继续下一张，不要改用镜像或转载站。
- 桌面视口，宽约 1280–1440 px，缩放 100%。浅色主题优先。

## 截图清单

文件保存到 `0925/images/`，PNG 格式。

| 文件 | URL（只用原站） | 截图里必须完整可读的内容 |
|---|---|---|
| `01-claudedevs-post.png` | https://x.com/ClaudeDevs/status/2103170368794185758 | 整条主帖：账号名、发布时间、全文，包括 “99.7% of accounts using Claude Code, Claude.ai, or Cowork” “tuned to have a <0.1% false positive rate” “/feedback”。不截回复区。 |
| `02-refusal-billing-table.png` | https://platform.claude.com/docs/en/build-with-claude/refusals-and-fallback | “What a refusal looks like” 下的 category 表格：`cyber` / `bio` / `frontier_llm` / `reasoning_extraction` / `general_harms` 五行，含 “What it means” 和 “Billed before any output” 两列表头。表太宽可分两次截再竖向拼接，拼接处在备注里说明。 |
| `03-pm-transcript.png` | https://www.pm.gov.au/media/press-conference-new-york | 页面标题与日期 “Thursday 24 September 2026” 可以单独成段；正文从 “On June 18, OpenAI's research team used an internal model …” 到 “… writing files as well to the internal server.” 的连续段落。标题和正文不在同一屏时，截两段再竖向拼接，中间留一条细分隔线。 |
| `04-gallagher-standalone.png` | https://www.minister.defence.gov.au/transcripts/2026-09-24/press-conference-sydney | 页面标题；正文中 “… the Medicare Statistics Reporting Service Portal is a standalone website, it's public-facing and it is not in any way related to Medicare in terms of claims, payments, processing, individual information. So, it's a completely different system and the two shouldn't be conflated.” 所在段落，包括说话人 “GALLAGHER:”（若该段开头可见）。该站加载慢，最多等 60 秒。 |
| `05-guardian-headline.png` | https://www.theguardian.com/australia-news/2026/sep/24/anthony-albanese-says-openai-agent-hacked-medicare-extreme-concern-sam-altman | 文章标题（存档时为 “Australia launches investigation after OpenAI agent hacked healthcare database”，以页面实际为准）、作者行、“First published on Wed 23 Sep 2026 17.11 EDT” 时间行、第一段导语。关掉 Cookie 弹窗时选“拒绝/仅必要”，不接受非必要 Cookie；弹窗无法关闭时不要点同意，记为问题。 |
| `10-release-notes-resume.png` | https://platform.claude.com/docs/en/release-notes/overview | “September 24, 2026” 日期标题及其下 “We're resuming billing for refusals that arrive before any output …” 整条（到 “See How refusals are billed.”）。 |
| `11-release-notes-june2.png` | 同上页 | “June 2, 2026” 日期标题，以及该日期下 “On the Claude API, you are no longer billed for a request when it returns stop_reason: "refusal" without Claude having generated any output.” 这一条。日期标题和该条相距太远时拼接，并在备注中说明。 |
| `12-pm-mailbox.png` | https://www.pm.gov.au/media/press-conference-new-york | “It was that it took until 10 September before there was any notification at all. And the notification was an email sent to just the public mailbox.” 所在问答，含提问的 “JOURNALIST:” 和 “PRIME MINISTER:” 标签。 |
| `13-cookbook-diff.png` | https://github.com/anthropics/claude-cookbooks/commit/c5ff1dc523e28d9b8fbd5c6ecd63204e20b8a0ed | commit 标题、作者与日期；diff 中含 “at the rates of the model that was requested” 的绿色新增行及上下各两三行。 |
| `14-pocock.png` | https://www.davidpocock.com.au/statement_on_openai_medicare_hack | 声明标题、日期（若页面有），以及含 “why we aren't holding these big tech companies liable” 的段落。 |

截图规则：

1. 只截与说法相关的区域，页面标题、日期、原文要可辨认。去掉浏览器地址栏、书签栏、插件图标、Cookie 弹窗、聊天浮窗；这些遮挡正文时，先关掉再截，不要用图像编辑抹除。
2. 不重绘、不改字、不加标注框或箭头、不调色。允许的处理只有：裁剪，以及上面写明的竖向拼接。
3. 页面上的实际文字与本清单引文不一致时，**照实截图**，并在报告中逐字写出差异，不要找别的页面凑。
4. 手机上要能看清：裁剪后宽度不超过 1400 px。引文区域的字高在原始像素下不低于 14 px。

## 范围与约束

- 只新增上述 PNG 和 `0925/images/capture-log.md`（见“回报”）。
- 不修改 `0925/doc_0925_publish.txt`、`0925/images/README.md`、`0925/sources/` 下任何文件及其他期次文件。
- 不生成 ZIP，不复制多份同样的图。
- 授权：只改工作区。不 commit、不 push，不删除任何文件。

## 验收

1. 10 个 PNG 都存在（失败的在日志里说明原因）。
2. 每张图肉眼检查，“必须完整可读的内容”一列逐项可见、无遮挡。
3. 运行以下命令，把输出原样贴进报告：

```bash
cd /e/gitclone/AI-Barking && python3 -c "
import struct,glob
for f in sorted(glob.glob('0925/images/*.png')):
    b=open(f,'rb').read(24); w,h=struct.unpack('>II',b[16:24]); print(f,w,'x',h)
" && git status --short
```

   `git status` 应只显示 `0924/images/00-cover.png`、`common_images/` 与 `0925/images/` 下的新文件。

## 回报

写 `0925/images/capture-log.md`，同时把相同内容作为最终回复，供 Develata 贴回给 Claude：

| 文件 | 实际 URL | 截图时间（北京时间） | 视口宽度 | 是否拼接/裁剪说明 | 与清单引文的差异（逐字） | 状态（成功/部分/失败） |
|---|---|---|---|---|---|---|

另附：验收命令的原始输出；遇到的问题（登录墙、弹窗、加载失败）；你做的任何假设。
