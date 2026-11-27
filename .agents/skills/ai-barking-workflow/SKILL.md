---
name: ai-barking-workflow
description: >-
  End-to-end editorial, fact-checking, card/cover rendering, cross-family review,
  and publishing workflow for an AI 吠点 (AI-Barking) issue. Activate when creating,
  researching, drafting, reviewing, rendering cards/covers for, or publishing an
  AI-Barking news issue in this repository.
---

# AI 吠点制作与发布入口

本技能是薄入口，不重复正文规则；全部细节以仓库内规范文件为准。

## 必读规范（按顺序）

1. [AGENTS.md](../../../AGENTS.md) — 交付格式、目录结构、期号规则与三批提交/推送约定。
2. [EDITORIAL.md](../../../EDITORIAL.md) — 选题、正文结构、标题、对比、文风、封面、卡片、速览、事实分级与六步核验流程。
3. [docs/workflow/harness.md](../../../docs/workflow/harness.md) — 角色划分、GPT / Claude / Gemini / DeepSeek 四家模型家族互斥表、复核运行时、派工盯进度与封面生图替代方案。
4. [docs/workflow/hosts.md](../../../docs/workflow/hosts.md) — Windows / Linux / macOS 依赖、沙箱与路径差异。
5. [docs/workflow/publish.md](../../../docs/workflow/publish.md) — 终审交付、七步检查、三批提交、OpenList 上传与推送。

## `barking` 命令速查

记 `BARKING="cargo run --release --manifest-path tools/barking/Cargo.toml --"`，本期目录记为 `ISSUE=docs/<YYMM>/<MMDD>`（期号 = 最新一期 `+1`）。

| 阶段 | 命令 | 模板 / 说明 |
|---|---|---|
| 1. 生成取证派工单 | `$BARKING handoff $ISSUE --groups "A=主题1,B=速览"` | 先跑 `opencli doctor`；模板 [handoff-evidence.md](../../../templates/handoff-evidence.md) |
| 2. 批量取证与截图 | `$BARKING fetch` / `shot` / `html2txt` / `heat` | 细则见 [codex-evidence.md](../../../templates/codex-evidence.md) |
| 3. 读者稿检查（异家族） | 按模板交异家族无工具会话 | 模板 [style-review.md](../../../templates/style-review.md)，结果存 `sources/style-review.md` |
| 4. 封面抽签与生图 | `$BARKING cover-draw $ISSUE` | 模板 [codex-cover.md](../../../templates/codex-cover.md) |
| 5. 渲染配图卡片 | `$BARKING card $ISSUE`（预检加 `--check`） | 说明 [templates/cards/README.md](../../../templates/cards/README.md) |
| 6. 跨家族反向 / 视觉复核 | `$BARKING review $ISSUE --reverse\|--visual --reviewer codex\|claude\|print-only [--run]` | 须选与主执行方不同模型家族；模板 [reverse-review.md](../../../templates/reverse-review.md)、[visual-review.md](../../../templates/cards/visual-review.md) |
| 7. 机械格式检查 | `$BARKING lint $ISSUE` | 须 `0 个错误` 才进入发布检查 |
| 8. 发布标记与七步检查 | `$BARKING scan-rows $ISSUE` → 补写 `daily-scan.md` 摘要 → `$BARKING publish-check $ISSUE` | 流程见 [docs/workflow/publish.md](../../../docs/workflow/publish.md) |
| 9. 三批暂存、上传与推送 | `$BARKING stage $ISSUE --batch 1\|2\|3 --add` + `git commit` → `$BARKING offsite $ISSUE --upload` → `git push` | 推送前核对 `git log --oneline origin/main..HEAD` |

## 最小自检红线

- **模型家族互斥**：主执行方与独立复核方不得同家族（详见 `docs/workflow/harness.md` 第 1 节）；实在降级须在 `sources/fact-check.md` 注明并告知 Develata。
- **验收**：自述回报不算验收；不调用 `rm` 删临时复核/取证文件；交付终审前必须通过 `barking lint`（0 错误）与 `barking publish-check`。
