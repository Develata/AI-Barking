# 多 Agent 编排与复核 Harness 指南

本文件把仓库原本散落在外部用户级配置中的编排、派工、封面生图与跨模型复核细节收进仓库，保证任何支持本地 shell 与文件读写的 agent（Claude Code、Codex、Antigravity、Qoder、WorkBuddy 等）在读完根目录 `AGENTS.md` 与 `EDITORIAL.md` 后，都能独立跑通整套制作与发布流程。

主机与系统依赖差异见 `docs/workflow/hosts.md`；内容规范见 `EDITORIAL.md`；提交、上传与推送见 `docs/workflow/publish.md`。

## 1. 角色与模型家族互斥表

流程只按**角色**划分，不绑定某一款 agent 工具；历史期次记录与旧派工单中出现的“Claude”，均指当时默认的主执行方。

| 角色 | 职责 | 对模型/环境的要求 |
|---|---|---|
| **主执行方（Executor）** | 与 Develata 对接、定新期号与选题、生成并派发取证任务、盯进度与验收、抽查一手材料、起草正文与标题候选、抽签并验收封面、填 `cards.toml` 并渲染卡片、发起复核并逐条判断采纳、执行 `lint` / `scan-rows` / `publish-check` / `stage` / `offsite` / `git push` | 具备本机读写文件、运行 shell 命令与看图能力；记住自己的实际模型家族 |
| **取证子代理（Evidence Worker）** | 按派工单分组回溯一手来源、批量存档、截图、采样热度、交 `evidence-<组>.md` 与 `capture-log-<组>.md`；不写正文、不下结论 | 可由主执行方派生子代理、后台会话或独立 CLI 执行；是否与主执行方同家族不限 |
| **独立复核方（Reviewer）** | 读者稿检查（无工具纯文本）、反向核验（只读 + 联网）、视觉复核（只读 + 看图） | **必须与主执行方分属不同模型家族，且不带本期上下文（全新只读会话）** |

### 四家模型家族互斥矩阵（GPT / Claude / Gemini / DeepSeek）

核心约束：**主执行方家族 ≠ 复核方家族**。此外，若取证子代理与复核方分属不同家族，反向核验同时承担取证环节的跨模型逐行核对（见 `templates/reverse-review.md` 第 2 条）。

| 主执行方模型家族 | 首选复核方与 `barking review` 后端 | 备选跨家族复核方 | 读者稿检查（`templates/style-review.md`） |
|---|---|---|---|
| **Claude**（Anthropic） | **GPT**（`--reviewer codex`，默认最新 Sol：`gpt-6.2-sol`） | **Gemini** / **DeepSeek**（`--reviewer print-only`） | `codex exec -m gpt-6.2-sol`（或 Gemini / DeepSeek 独立无工具会话） |
| **GPT**（OpenAI / Codex） | **Claude**（`--reviewer claude`，默认最新 Opus：`opus`） | **Gemini** / **DeepSeek**（`--reviewer print-only`） | `claude -p --model opus`（或 Gemini / DeepSeek 独立无工具会话） |
| **Gemini**（Google / Antigravity 等） | **GPT**（`--reviewer codex`）或 **Claude**（`--reviewer claude`） | 另一方或 **DeepSeek**（`--reviewer print-only`） | `codex exec -m gpt-6.2-sol` 或 `claude -p --model opus` |
| **DeepSeek** | **GPT**（`--reviewer codex`）或 **Claude**（`--reviewer claude`） | 另一方或 **Gemini**（`--reviewer print-only`） | `codex exec -m gpt-6.2-sol` 或 `claude -p --model opus` |

- **多模型自动路由型 agent**（如 Qoder、WorkBuddy 等自动选模型的环境）：动工前先确认主执行方实际使用的模型家族，再按上表选不同家族的复核方；若无法确认当前会话底层模型家族，不得假定跨家族成立，应显式切换固定模型或按降级处理并告知 Develata。
- **降级规则**：首选跨家族 CLI 不可用或超限时，先尝试表内另一跨家族复核方（如另一 CLI 或 `--reviewer print-only` 交其他家族独立只读会话）；若所有跨家族渠道均不可用且时间来不及等待恢复，才允许用同一份提示词另开一个**不带本期上下文的同家族独立会话**复核，在 `sources/fact-check.md` 写明“复核方：<家族>（同家族，降级）”，并在交付终审时明确告诉 Develata 本期未做跨模型复核，由 Develata 决定是否发布。不得跳过复核，也不得把降级复核记成跨模型复核。

## 2. 独立复核信使纪律（Courier Protocol）

无论由哪个工具发起复核，都遵守同一套隔离与验收规则：

1. **中性输入**：提示词只给规则、文件路径/图片与审查清单，**不写作者对稿件的自评、预期结论或怀疑的错点**。反向核验的固定 adversarial charter 维护在 `templates/reverse-review.md` 正文开头。
2. **只读隔离与不删旧件**：复核方只读不改文件；提示词与报告由 `barking review` 自动按序号新建在系统临时目录 `barking-review/<MMDD>/`，从不覆盖已完成报告，也**不调用 `rm` 删除旧提示词或旧报告**。
3. **原样保留与逐条核实**：复核报告必须原样保留（不软化、不替复核方筛除意见）；主执行方逐条对照原文与工作区文件判断采纳与否，结论和理由记入 `sources/fact-check.md` 对应一节。
4. **失败判定**：复核 CLI 不存在、WSL 不可用（Windows）、非零退出、超时（退出码 `124`）或输出报告为空，均记为**“未完成独立复核”**，不得静默放行，不得把主执行方自查当成独立复核。超时或格式异常时，最多用更精简的中性提示词重试一次。

## 3. 三类复核运行时（`barking review --reviewer`）

`barking` 简写均指 `cargo run --release --manifest-path tools/barking/Cargo.toml --`。

### 3.1 `--reviewer codex`（GPT 家族，默认）

- **适用**：主执行方为 Claude、Gemini 或 DeepSeek 时。
- **宿主路由**（详见 `docs/workflow/hosts.md`）：
  - **Linux / macOS**：直接在本机原生调用 `codex`（Linux 只读沙箱与 macOS Seatbelt 只读沙箱均已验证可读仓库、拒绝写入）；路径直接传本机绝对路径。
  - **Windows**：经 WSL（Debian）调用 WSL 内的 `codex`，路径经 `wslpath -a` 转换并设 `MSYS_NO_PATHCONV=1`；Windows 原生 Codex 沙箱不可靠，**复核一律不用**。
- **关键参数**：
  - 反向核验：`--search`（顶层参数，允许联网核查一手来源）+ `exec -m <model> -c 'model_reasoning_effort="<effort>"' -s read-only --skip-git-repo-check --ephemeral -C <仓库根> -o <报告> - < <提示词>`。
  - 视觉复核：不加 `--search`，`-C <期次目录>`，每张图附一个 `--image=<相对路径>`。
  - 限时：`--effort medium` 默认限时 1200 秒（Linux 用 `timeout`，macOS 用 `gtimeout`，见 `docs/workflow/hosts.md`）；`high` / `xhigh` / `max` 不限时；退出码 `124` 表示超时。
- **环境变量**：`BARKING_CODEX`（Linux/macOS 可执行文件名或路径，默认 `codex`）、`BARKING_CODEX_MODEL`（默认最新 Sol，当前 `gpt-6.2-sol`）、`BARKING_WSL_DISTRO`（Windows 默认 `Debian`）、`BARKING_WSL_CODEX`（Windows 默认 `/home/deve/.bun/bin/codex`）。

### 3.2 `--reviewer claude`（Claude 家族）

- **适用**：主执行方为 GPT、Gemini 或 DeepSeek 时。
- **宿主路由**：Linux、macOS、Windows（Git Bash）均直接本机调用 `claude -p`，无需 WSL 路径转换。
- **关键参数**：
  - 反向核验（工作目录为仓库根）：`claude -p --model <model> --effort <effort> --tools "Read,Grep,Glob,WebSearch,WebFetch" --allowedTools "Read,Grep,Glob,WebSearch,WebFetch" --strict-mcp-config --no-session-persistence < <提示词> > <报告>`。
  - 视觉复核（工作目录为期次目录，由复核方用 `Read` 逐张看图）：`--tools "Read,Grep,Glob" --allowedTools "Read,Grep,Glob"`，其余同上。
  - 读者稿检查（见 `templates/style-review.md`）：`--model opus --effort high --tools "" --strict-mcp-config --no-session-persistence`。
  - 默认 `--effort` 为 `high`（支持 `medium` / `high` / `max`，不接受 `xhigh`）；`medium` 时若本机有 `timeout`/`gtimeout` 同样限时 1200 秒。
- **环境变量**：`BARKING_CLAUDE`（默认 `claude`）、`BARKING_CLAUDE_MODEL`（默认最新 Opus：`opus`）。

### 3.3 `--reviewer print-only`（Gemini / DeepSeek / 任意其他独立只读会话）

- 运行 `barking review <期次目录> --reverse|--visual --reviewer print-only`：只填充并写出提示词文件、打印建议报告路径、审查文件范围或附图清单，不要求本机装有 `codex`、`claude` 或 WSL，也不能与 `--run` 同用。
- 主执行方将提示词文件内容（以及 `--visual` 列出的图片路径）交给任意**不同模型家族、无本期上下文、只读**的独立会话或子代理执行，将完整输出原样保存至报告路径后再逐条核实。

## 4. 取证派工、沙箱与盯进度协议

1. **生成派工单**：`barking handoff <期次目录> --groups "A=主题1,B=主题2,C=速览"` 生成 `.handoff/YYYY-MM-DD-<MMDD>-evidence.md` 并更新 `.handoff/README.md`。派工前先跑 `opencli doctor`：浏览器扩展未连接就在派工单写明截图直接走 `barking shot`，避免各组重复等超时。
2. **按当前 Agent 能力派发**：
   - **支持原生子代理的 agent**（Claude Code、Antigravity、Qoder 等）：按组并行派发，明确只做本组、只写 `docs/<YYMM>/<MMDD>/` 下带本组前缀的文件；不改动其他组文件，不 commit、不 push。
   - **由 Codex CLI 执行取证**：按 `templates/codex-evidence.md`，机械采集可用 `gpt-6-luna`（`max`），需要判断与回溯一手来源用 `gpt-6-astra`（`medium` 或 `max`）；取证需要联网和跑 `cargo` / `opencli`，在 Windows、Linux、macOS 均按需要开 `-s danger-full-access`，由派工单严格限定工作范围与禁令。
   - **不支持并行的环境**：按组串行执行，同样遵守分组文件前缀与单组超时线。
3. **通用执行纪律**：
   - 取证子代理不调用 `rm`（部分 harness 会拦截删除命令卡住流程），废弃文件留在原地并在回报里列“待删除文件”清单，由主执行方收尾统一清理。
   - 目标 60 分钟内交回，超过 90 分钟硬截断：哪组超过 60 分钟未交，主执行方通过当前 agent 的子代理通信或后台任务机制催交已完成部分并列出未完成项。
   - **自述回报不算验收**：每组交回后，主执行方必须检查 `git status`、打开关键存档与截图，对高风险事实（价格、跑分、对比结论、标题所涉说法）亲自回一手原文逐字核对。

## 5. 封面生图运行时

- **默认路径**：按 `templates/codex-cover.md`，先跑 `barking cover-draw <期次目录>` 抽 A、B 两组画风轴，再调用 Codex CLI 内置 `image_generation` 生成 3:4 竖版与 2.35:1 横版候选（看板娘出镜时附 `-i common_images/mascot-reference.png`；抽到“角标”时不附参考图，选定后用 `ffmpeg` 叠 `common_images/profile_picture.png`）。
- **其他生图工具替代**：当主执行方自带支持参考图输入的生图工具（例如 Antigravity 的 `image-generator` 子代理）或本机无 Codex 生图权限时，可直接用该工具按同一份提示词与硬性要求出图，并严格执行 `templates/codex-cover.md` 第 4 步逐字核对、比例检查与前 3 期对比验收；在 `images/README.md` 封面节写明实际使用的生图工具与模型。

## 6. 端到端跑通清单（给任意 Agent 的快速索引）

默认读完 `AGENTS.md` 后，制作并交付一期 AI 资讯的标准命令流：

1. **定新期号与选题**：查 `docs/` 下最新一期目录 `+1` 定本期 `<YYMM>/<MMDD>`（不问 Develata、不按系统时钟推算）；对照 `daily-scan.md`“已发过”与“近 3 天速览已收”去重后列候选及吠点，由 Develata 拍板。
2. **取证派工**：`opencli doctor` → `barking handoff docs/<YYMM>/<MMDD> --groups "..."` → 并行/串行取证（内部使用 `barking fetch`、`barking shot`、`barking html2txt`、`barking heat` 与 `opencli`）。
3. **抽查与起草**：先到先审先写，产出 `sources/titles.md`、`doc_<MMDD>_publish.txt`、`sources/fact-check.md`、`sources/README.md`、`images/README.md`。
4. **并行初审**：
   - 读者稿检查：按 `templates/style-review.md` 交异家族无工具会话，结果存 `sources/style-review.md` 并附“采纳”节。
   - 第一轮反向核验：`barking review docs/<YYMM>/<MMDD> --reverse --reviewer <异家族> [--run]`。
5. **封面与卡片**（可与第 4 步并行）：
   - 封面：`barking cover-draw docs/<YYMM>/<MMDD>` → 生图 A/B 候选 → 逐字与前 3 期对比验收 → 复制至 `images/publish/00-cover.png` 与 `00-cover-wide.png`（速览专帖跳过生图，由 `barking card` 渲模板封面）。
   - 卡片：填 `images/cards.toml` → `barking card docs/<YYMM>/<MMDD>` → 打开成图逐张自查。
6. **二轮复核**：卡片渲染且一轮 BLOCKER 改完后，跑 `barking review docs/<YYMM>/<MMDD> --visual --reviewer <异家族> [--run]`，并视改动情况补一轮 `--reverse` 短审；结论记入 `sources/fact-check.md`。
7. **机械校验**：`barking lint docs/<YYMM>/<MMDD>`，必须 `0 个错误`。
8. **发布标记与七步检查**：`barking scan-rows docs/<YYMM>/<MMDD>` → 手填 `daily-scan.md`“已发过”摘要替换 `待填` → `barking publish-check docs/<YYMM>/<MMDD>` 全部通过。
9. **三批提交**：`barking stage docs/<YYMM>/<MMDD>` 查看批次 → 依序 `--batch 1 --add` + `git commit`、`--batch 2 --add` + `git commit`（无新增取证存档可省）、`--batch 3 --add` + `git commit`。
10. **上传与推送**：`barking offsite docs/<YYMM>/<MMDD> --upload` → 检查 `git log --oneline origin/main..HEAD`（有当期以外的提交先问 Develata）→ `git push origin main` → `git fetch` 核对远端一致并向 Develata 回报终审。
