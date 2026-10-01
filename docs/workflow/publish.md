# 发布后：提交、推送与归档

Develata 在对话中说“已发布”，即授权 Claude 对当期完成下面全部步骤，包括推送到 `origin/main` 和上传 OpenList。授权只覆盖当期这一轮；发布后的标题、文字或配图修订各自单独提交，推送前另行确认。

## 为什么要及时推送

选题检索由 ChatGPT 定时任务完成：每天 0、6、12、18 点各一次，读取 GitHub 上 `main` 分支的 `daily-scan.md` 作为 prompt，检索此前 36 小时的新闻。各次结果彼此可见，但不是增量检索。定时任务只能从远端看到“已发过”，所以发布后要尽快推送，最好赶在下一个整点前；GitHub 的 raw 文件可能有几分钟缓存。

检索结果由 Develata 汇总回贴给 Claude。Claude 以本地 `daily-scan.md` 的“已发过”为准去重，因为本地可能比远端新。

## 步骤

1. **已发过**：在 `daily-scan.md` 末尾“已发过”追加本期一行，按最终发布版的标题和口径写。只在发布后写：未发布的选题一旦写入并推送，就会被定时检索排除。
2. **检查**（任一不过就停下报告）：
   - `cargo run --release --manifest-path tools/barking/Cargo.toml -- lint <期次目录>`，0 个错误；
   - 配图无损压缩：`oxipng -o 4 --strip safe <期次目录>/images/*.png`，像素不变；
   - 大文件：`cargo run --release --manifest-path tools/barking/Cargo.toml -- offsite <期次目录>` 生成 `sources/offsite.tsv`；待提交文件中没有单个超过 1 MB 的，有就先问；
   - 身份：按 `templates/codex-evidence.md` 第 7 条，grep 抓取者的头像链接、显示名与 handle，确认未入库；
   - 凭据与会话状态（`.env`、cookies、storage-state）未入库。
3. **提交**：按 `AGENTS.md` 的提交约定分三批、依序提交。
4. **推送**：`git push origin main`，再 `git fetch` 核对 `origin/main` 上的 `daily-scan.md` 与本地一致。
5. **上传**：`cargo run --release --manifest-path tools/barking/Cargo.toml -- offsite <期次目录> --upload`，逐个核对远端大小一致。
6. **回报**：各批提交哈希、推送结果、上传数量与失败项。

## 存储分工

| 存放处 | 内容 |
|---|---|
| GitHub（本仓库） | 发布正文、配图说明、`images/` 中封面以外的 PNG（无损压缩后）、`sources/` 中的文本（md、txt、json、jsonl、py、ps1 等）、`sources/offsite.tsv` |
| OpenList（`openlist.develata.me`） | 封面 `images/00-cover*`，`sources/` 中的 PDF、HTML、图片与音视频原件 |

- 分界写在根目录 `.gitignore`，只对未跟踪文件生效。2609/0928 及更早期次的原件已在 Git 历史中，不搬也不改写。
- OpenList 目标路径：`<OPENLIST_ROOT>/<YYMM>/<MMDD>/<相对期次目录的路径>`，与仓库目录一一对应。
- `sources/offsite.tsv` 每行一个不入库文件：相对期次目录的路径、字节数、SHA-256。它入库，用来证明网盘上的原件就是当时取证的那一份。
- 单文件 1 MB 为提交上限。超过的文本文件（如大 JSON 采样）先问是否必须入库；入库的历史无法靠 `.gitignore` 撤回。

## 凭据

上传读取仓库根目录的 `.env`（已被 `.gitignore` 排除），键名见 `.env.example`：

| 键 | 含义 |
|---|---|
| `OPENLIST_URL` | 站点地址，如 `https://openlist.develata.me` |
| `OPENLIST_USERNAME` | 上传专用用户名 |
| `OPENLIST_PASSWORD` | 该用户的密码 |
| `OPENLIST_ROOT` | 该用户可见路径下的归档根目录，如 `/AI-Barking` |

建议在 OpenList 后台为上传单独建一个用户：基本路径限定到归档所在的网盘目录；权限只勾“创建目录或上传”（源码 `internal/model/user.go` 权限位 3，覆盖同名文件也只需这一项），不勾删除、重命名、移动、WebDAV 等；不开二步验证（脚本无法输入验证码）。凭据由 Develata 直接写进 `.env`，不在对话中发送。
