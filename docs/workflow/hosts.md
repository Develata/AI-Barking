# 主机差异：Windows 与 Linux

2026-11-01 起（Develata 决定）主力主机是 Debian 13，Codex 直接在本机调用，不再经 WSL；Windows 检出照旧可用。选题、取证、核验、出图、发布的流程与 `barking` 子命令两边相同，只有下表这些地方不同。模板与派工单里的命令按 bash 写，`<仓库根>`、`<本机绝对路径>` 按本表替换。

## 对照表

| 项目 | Windows | Linux（Debian 13） |
|---|---|---|
| 仓库根 | `E:\gitclone\AI-Barking`（命令里写 `E:/gitclone/AI-Barking`） | `/home/develata/gitclone/AI-Barking` |
| Shell | Git Bash；PowerShell 不支持 `<`，改用 `Get-Content -Raw <提示词> \| …` | bash |
| 截图、渲页的输出路径 | Windows 绝对路径（`E:\…`） | 普通绝对路径 |
| 系统临时目录（`barking` 的 review、card、shot 中间文件） | `%TEMP%` | `$TMPDIR`，未设则 `/tmp` |
| 跨模型复核：`barking review`、`codex-reviewer` | WSL（Debian）里的 Codex，`-s read-only`；Windows 原生 Codex 沙箱不可靠，不用。环境变量 `BARKING_WSL_DISTRO`（默认 `Debian`）、`BARKING_WSL_CODEX`（默认 `/home/deve/.bun/bin/codex`） | 本机 Codex，`-s read-only`，命令形如 `bash -c '…' codex …`。环境变量 `BARKING_CODEX`（默认 PATH 上的 `codex`，即 Bun 全局的 `~/.bun/bin/codex`）。2026-11-01 验证：只读沙箱能读仓库、写文件被拒 |
| 读者稿检查（`templates/style-review.md`） | `-s danger-full-access` | `-s read-only`（只读 stdin、`-o` 由 CLI 自己写） |
| 封面生成（`templates/codex-cover.md`） | `-s danger-full-access -C <仓库根>`；被拦时 `-s workspace-write -C {OUT}`（1024 期如此） | `-s workspace-write -C {OUT}`（尚未在 Linux 实跑过；失败时退回 `danger-full-access` 并在 `images/README.md` 记一笔） |
| Codex 当取证执行方 | `danger-full-access`（要联网、跑 cargo 与 opencli） | 同左 |
| 包管理 | Scoop | apt（系统库）、`cargo binstall`（Rust CLI）、Bun（JS CLI） |
| oxipng（`publish-check`） | `scoop install oxipng` | `cargo binstall oxipng`（已装） |
| Tesseract（`barking card` 批注图） | `scoop install tesseract tesseract-languages`（设了 `TESSDATA_PREFIX`，装前打开的终端要重开） | `sudo apt install tesseract-ocr tesseract-ocr-chi-sim`（2026-11-01 尚未装，需 Develata 输密码） |
| poppler（`barking shot pdf`、`fetch` 的 PDF 转文本） | Scoop 的 poppler；barking 先找 `~/scoop/apps/poppler/current/bin`，因为 Git Bash PATH 上的 xpdf 版 `pdftotext` 没有 `-bbox-layout` | apt 的 `poppler-utils`（已装），走 PATH |
| Chrome（headless 渲染） | `C:\Program Files\…` 自动找到 | `/usr/bin/google-chrome` 自动找到 |
| opencli | 照旧 | Bun 全局，已装 |
| agent-reach | 照旧 | 未装；取证默认用 opencli，确需时按 agent-reach skill 安装 |
| 文件名大小写 | 不区分（barking 已按此处理） | 区分；同名只差大小写的图照样被 `barking card` 拒绝 |

找不到上述程序时，`barking` 的报错会按当前主机给出安装命令。

## Linux 检出缺的东西

这些不入库，换主机时要单独准备：

1. `.env`：照 `.env.example` 建，值（OpenList 账号、`BARKING_IDENTITY_HANDLES` 等）从 Windows 检出拷过来，不入库、不贴进对话。没有它，`barking offsite` 上传与 `publish-check` 的身份检查都做不了（后者只警告）。
2. 往期图片：`images/` 与 `sources/` 里的二进制原件只在 OpenList 与 Windows 检出里。要用往期图（封面比对“前 3 期”、重跑旧期的视觉复核、取用批注底图）时，从 Windows 检出拷或从 OpenList 下载，拷完按 `sources/offsite.tsv` 核对 SHA-256。新一期从 Linux 开工则无此问题。
3. Tesseract：见上表。
