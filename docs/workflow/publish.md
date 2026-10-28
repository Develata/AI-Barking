# 发布后：提交、推送与归档

Develata 在对话中说“已发布”，即授权 Claude 对当期完成下面全部步骤，包括推送到 `origin/main` 和上传 OpenList。授权只覆盖当期这一轮；发布后的标题、文字或配图修订各自单独提交，推送前另行确认。

## 为什么要及时推送

选题检索由 ChatGPT 定时任务完成：每天 0、6、12、18 点各一次，读取 GitHub 上 `main` 分支的 `daily-scan.md` 作为 prompt，检索此前 36 小时的新闻。各次结果彼此可见，但不是增量检索。定时任务只能从远端看到“已发过”，所以发布后要尽快推送，最好赶在下一个整点前；GitHub 的 raw 文件可能有几分钟缓存。

检索结果由 Develata 汇总回贴给 Claude。Claude 以本地 `daily-scan.md` 的“已发过”为准去重，因为本地可能比远端新。

## 步骤

下面的 `barking` 即 `cargo run --release --manifest-path tools/barking/Cargo.toml --`。

1. **已发过与索引**：`barking scan-rows <期次目录>` 生成机械部分：`README.md` 的期次索引行；`daily-scan.md`“近 3 天速览已收”的本期条目（`cards.toml` 中没有 `main = true` 的 `[[roundup]]`，每条一行：期次、标签、图上那句事实），并删掉期次日期早于或等于本期减三天的行（如发 1008 时删 1005）；“已发过”的占位行（速览专帖没有主帖，不写）。然后把占位行改成本期摘要：按最终发布版的标题和口径写选题、关键数字与吠点——这是编辑内容，工具只放占位，`待填` 没改掉时 `publish-check` 与 `stage --batch 1` 都会拒绝。只在发布后写：未发布的选题一旦写入并推送，就会被定时检索排除。
2. **检查**：`barking publish-check <期次目录>`，按固定顺序做下面七项，任一不过就停下报告：
   - lint，0 个错误；
   - 发布后标记齐全：README 索引行、已发过行（无占位）、速览已收条目与 `cards.toml` 逐字一致；
   - 配图无损压缩（默认执行，2026-10-02 起）：`oxipng -o 4 --strip safe`，只压 `images/` 下的 PNG。像素不变，但文件字节和 SHA-256 会变、文本类元数据会被删，所以必须在 offsite 清单和上传**之前**做；已记入 `offsite.tsv` 且 SHA-256 未变的文件不再压（已上传的期次因此不会被重压）；不加 `-a`（它会改全透明像素的颜色值，不是严格无损）；JPEG、WebP 不动；需要逐字节保留的原件放进 `sources/`；
   - 生成 `sources/offsite.tsv`（全部图片与原件都在清单里）；
   - 大文件：待提交文件中没有单个超过 1 MB 的，有就先问；
   - 身份：按 `templates/codex-evidence.md` 第 7 条，用 `.env` 里 `BARKING_IDENTITY_HANDLES` 列出的抓取账号 X handle，经 fxtwitter 公开接口解析出显示名、头像 ID 与数字用户 ID（缓存在 `~/.config/barking/identity.json`，一天后自动刷新，改名后旧名仍保留在缓存里），加上 `BARKING_IDENTITY_PATTERNS` 手填的片段，搜索待提交文件与文件名，确认未入库（两者都未配置时只警告）；
   - 凭据与会话状态（`.env`、cookies、storage-state）仍被 `.gitignore` 排除，且没有进入待提交集合。
3. **提交**：`barking stage <期次目录>` 列出三批文件与建议提交信息；按 `AGENTS.md` 的提交约定依序 `barking stage <期次目录> --batch 1 --add` 后 `git commit`，再 `--batch 2`、`--batch 3`。第②批的提交信息要按组描述存档内容后再提交；工具只暂存精确路径，不提交。
4. **推送**：先 `git log --oneline origin/main..HEAD`，除当期三批外还列出别的提交，就先问 Develata：“已发布”只授权推送当期三批（1025 期曾把三个此前未推送的规范改动一并推了出去）。确认后 `git push origin main`，再 `git fetch` 核对 `origin/main` 上的 `daily-scan.md` 与本地一致。
5. **上传**：`cargo run --release --manifest-path tools/barking/Cargo.toml -- offsite <期次目录> --upload`：每个远端目录先列一次，已有且大小一致的文件跳过；其余默认 4 路并发上传（单个文件失败自动重试，共 3 次；远端缺目录时，该目录的第一个文件单独先传），传完再列一次远端，逐个核对大小一致。网盘报请求过频时加 `--jobs 1` 串行。
6. **回报**：各批提交哈希、推送结果、上传数量与失败项。

## 存储分工

| 存放处 | 内容 |
|---|---|
| GitHub（本仓库） | 只入库文字：发布正文、配图说明 `images/README.md`、`images/cards.toml`、`sources/` 中的文本（md、txt、json、jsonl、py、ps1 等）、`sources/offsite.tsv` |
| OpenList（`openlist.develata.me`） | `images/` 下的全部图片（封面、省流卡、速览聚合图、吠点卡、批注截图、备用图），以及 `sources/` 中的 PDF、WPS/DOC 文档、HTML、图片与音视频原件；逐个文件直传，不打压缩包 |

- 分界写在根目录 `.gitignore`：`images/` 下所有 png、jpg、jpeg、webp、gif，和 `sources/` 下的 PDF、WPS/DOC 文档（2026-10-16 起）、HTML、图片与音视频。0924–1001 期原先已入库的图片与原件，已于 2026-10-02 全部补传 OpenList（417 个文件逐个核对远端大小一致）、记入各期 `offsite.tsv`，并用 `git filter-repo` 从 Git 历史中移出。
- 没有图片的克隆里，`barking lint` 把 `offsite.tsv` 列出的图片视为存在；重渲卡片前，先从 OpenList 的同一路径取回 `-raw` 底图。
- OpenList 目标路径：`<OPENLIST_ROOT>/<YYMM>/<MMDD>/<相对期次目录的路径>`，与仓库目录一一对应。
- `sources/offsite.tsv` 每行一个存于 OpenList 的文件：相对期次目录的路径、字节数、SHA-256。它入库，用来证明网盘上的原件就是当时取证的那一份。
- 单文件 1 MB 为提交上限。超过的文本文件（如大 JSON 采样）先问是否必须入库；入库的历史无法靠 `.gitignore` 撤回。

## 凭据

上传读取仓库根目录的 `.env`（已被 `.gitignore` 排除），键名见 `.env.example`：

| 键 | 含义 |
|---|---|
| `OPENLIST_URL` | 站点地址，如 `https://openlist.develata.me` |
| `OPENLIST_USERNAME` | 上传专用用户名 |
| `OPENLIST_PASSWORD` | 该用户的密码 |
| `OPENLIST_ROOT` | 归档根目录，相对该用户的基本路径。WebDAV 地址 `https://openlist.develata.me/dav/AI-Barking` 对应的 OpenList 路径是 `/AI-Barking`：用户基本路径为 `/` 时填 `/AI-Barking`；基本路径已限定为 `/AI-Barking` 时填 `/` |
| `BARKING_IDENTITY_HANDLES` | `publish-check` 身份检查用：抓取时登录的 X 账号 handle（不带 `@`，多个用 `;` 分隔）。显示名、头像 ID、数字用户 ID 由工具经 fxtwitter 公开接口解析并缓存在仓库外，改名后自动跟上且保留旧名 |
| `BARKING_IDENTITY_PATTERNS` | 可选，身份检查的手填补充片段（如其他平台的用户名），`;` 分隔，大小写不敏感，至少 4 个字符。这些字符串只能写在 `.env` 里，不要出现在仓库任何文件中 |

建议在 OpenList 后台为上传单独建一个用户：基本路径限定到归档所在的网盘目录；权限只勾“创建目录或上传”（源码 `internal/model/user.go` 权限位 3，覆盖同名文件也只需这一项），不勾删除、重命名、移动、WebDAV 等；不开二步验证（脚本无法输入验证码）。凭据由 Develata 直接写进 `.env`，不在对话中发送。
