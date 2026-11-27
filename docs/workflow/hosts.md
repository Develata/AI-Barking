# 主机差异：Windows、Linux 与 macOS

2026-11-01 起（Develata 决定）主力主机是 Debian 13，Windows 与 macOS 检出照旧可用。选题、取证、核验、出图、发布的流程与 `barking` 子命令三端相同，只有下表这些地方不同。模板与派工单里的命令按 bash 写，`<仓库根>`、`<本机绝对路径>` 按本表替换；跨模型复核与多 agent 编排细则见 `docs/workflow/harness.md`。

## 对照表

| 项目 | Windows | Linux（Debian 13） | macOS |
|---|---|---|---|
| 仓库根 | `E:\gitclone\AI-Barking`（命令里写 `E:/gitclone/AI-Barking`） | `/home/develata/gitclone/AI-Barking` | 如 `/Users/<user>/gitclone/AI-Barking`（以实际检出路径为准） |
| Shell | Git Bash；PowerShell 不支持 `<`，改用 `Get-Content -Raw <提示词> \| …` | bash | zsh / bash（`barking review` 生成 `bash -c` 命令，两者均可直接执行） |
| 截图、渲页的输出路径 | Windows 绝对路径（`E:\…`） | 普通绝对路径 | 普通绝对路径 |
| 系统临时目录（`barking` 的 review、card、shot 中间文件） | `%TEMP%` | `$TMPDIR`，未设则 `/tmp` | `$TMPDIR`（系统默认 `/var/folders/…`） |
| 跨模型复核：`barking review --reviewer codex` | WSL（Debian）里的 Codex，`-s read-only`；Windows 原生 Codex 沙箱不可靠，不用。环境变量 `BARKING_WSL_DISTRO`（默认 `Debian`）、`BARKING_WSL_CODEX`（默认 `/home/deve/.bun/bin/codex`） | 本机 Codex，`-s read-only`，`bash -c`。环境变量 `BARKING_CODEX`（默认 PATH 上的 `codex`）。2026-11-01 验证：只读沙箱能读仓库、写文件被拒 | 本机 Codex，`-s read-only`（Seatbelt 沙箱，2026-11-18 验证可读仓库、写文件报 `Operation not permitted`）。限时命令优先找 `timeout`，否则找 `gtimeout`（`brew install coreutils`）；未安装时 `--effort medium` 会提示不限时运行 |
| 跨模型复核：`barking review --reviewer claude` / `print-only` | 本机 `claude -p`（Git Bash）或 `print-only` 仅生成提示词 | 本机 `claude -p` 或 `print-only` | 本机 `claude -p` 或 `print-only`（限时同上，自动识别 `timeout`/`gtimeout`） |
| 读者稿检查（`templates/style-review.md`） | Codex 走 `-s danger-full-access`；Claude 走本机 `claude -p` | Codex 走 `-s read-only`；Claude 走本机 `claude -p` | 同 Linux |
| 封面生成（`templates/codex-cover.md`） | `-s danger-full-access -C <仓库根>`；被拦时 `-s workspace-write -C {OUT}`（1024 期如此） | `-s workspace-write -C {OUT}`（尚未在 Linux 实跑过；失败时退回 `danger-full-access` 并在 `images/README.md` 记一笔） | 同 Linux（也可按 `docs/workflow/harness.md` 第 5 节改用支持参考图的其他生图工具并如实记录） |
| Codex 当取证执行方 | `danger-full-access`（要联网、跑 cargo 与 opencli） | 同左 | 同左 |
| 包管理 | Scoop | apt（系统库）、`cargo binstall`（Rust CLI）、Bun（JS CLI） | Homebrew（系统库与原生依赖）、`cargo binstall`（Rust CLI）、Bun（JS CLI） |
| oxipng（`publish-check`） | `scoop install oxipng` | `cargo binstall oxipng`（已装） | `cargo binstall oxipng`（或 `brew install oxipng`） |
| Tesseract（`barking card` 批注图） | `scoop install tesseract tesseract-languages`（设了 `TESSDATA_PREFIX`，装前打开的终端要重开） | `sudo apt install tesseract-ocr tesseract-ocr-chi-sim`（2026-11-01 已装 5.5.0，语言 `chi_sim`、`eng`；需 sudo，由 Develata 装） | `brew install tesseract tesseract-lang` |
| poppler（`barking shot pdf`、`fetch` 的 PDF 转文本） | Scoop 的 poppler；barking 先找 `~/scoop/apps/poppler/current/bin`，因为 Git Bash PATH 上的 xpdf 版 `pdftotext` 没有 `-bbox-layout` | apt 的 `poppler-utils`（已装），走 PATH | `brew install poppler`，走 PATH（或设 `BARKING_POPPLER`） |
| Chrome（headless 渲染） | `C:\Program Files\…` 自动找到 | `/usr/bin/google-chrome`、`/usr/bin/chromium` 自动找到 | `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`（及 Chromium / Edge）自动找到，或设 `BARKING_CHROME` |
| opencli | 照旧 | Bun 全局，已装 | Bun 全局（`bun add -g …`） |
| agent-reach | 照旧 | 未装；取证默认用 opencli，确需时按 agent-reach skill 安装 | 同 Linux |
| 文件名大小写 | 不区分（barking 已按此处理） | 区分；同名只差大小写的图照样被 `barking card` 拒绝 | APFS 默认不区分；同名只差大小写的图照样被 `barking card` 拒绝 |

找不到上述程序时，`barking` 的报错会按当前主机给出安装命令。

## 换机检出缺的东西

这些不入库，换主机（Linux、macOS 或新 Windows 检出）时要单独准备：

1. `.env`：照 `.env.example` 建，值（OpenList 账号、`BARKING_IDENTITY_HANDLES` 等）从现有主机拷过来，不入库、不贴进对话。没有它，`barking offsite` 上传与 `publish-check` 的身份检查都做不了（后者只警告）。
2. 往期图片：`images/` 与 `sources/` 里的二进制原件只在 OpenList 与旧主机检出里。要用往期图（封面比对“前 3 期”、重跑旧期的视觉复核、取用批注底图）时，从旧主机拷或从 OpenList 下载，拷完按 `sources/offsite.tsv` 核对 SHA-256。新一期从新主机开工则无此问题。
