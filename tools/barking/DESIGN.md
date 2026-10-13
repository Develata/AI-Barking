# barking 新子命令设计（2026-10-10）

仓库：`E:\gitclone\AI-Barking`（主工作树，只读参考；你的工作树路径见派工消息）。crate：`tools/barking`（Rust 2024，`cargo build --release`）。
背景：每期制作流程见 `AGENTS.md`、`EDITORIAL.md`、`docs/workflow/publish.md`、`templates/`。这些命令把其中**纯机械**的步骤收进 `barking`；编辑判断（已发过摘要、封面挑选、取证内容）不自动化。

## 共享约定（所有模块必须遵守）

- 每个子命令一个模块 `src/<name>.rs`，入口 `pub fn run(args: &[String]) -> ExitCode`；用法错误退出码 2 并打印用法；失败 1；成功 0。内部函数返回 `Result<T, String>`，错误信息中文、可直接读；代码注释英文，风格对齐现有 `card.rs` / `offsite.rs` / `lint.rs`（doc comment 写 why，不写 what）。
- 在 `src/main.rs` 的 `match` 里加一条分支，在 `USAGE` 里加一行（中文，与现有行对齐）。整合时主会话会重新合并 main.rs，所以 main.rs 的改动只允许这两处。
- 共享辅助（已存在，直接用，不要重复实现）：
  - `crate::repo::Issue::resolve(&Path)` → `Issue { root, dir, yymm, mmdd }`，方法 `rel()`（`docs/2610/1008`）、`key()`、`date()`（`Date`）、`images()`、`sources()`、`publish_txt()`、`title()`、`Issue::all(root)`。
  - `crate::repo::Date`（`new`、`from_mmdd(year, "1008")`、`days()`、`from_days()`、`plus_days(n)`、`mmdd()`、`yymm()`、`iso()`）；`crate::repo::beijing_now()`（`2026-10-10T12:34:56+08:00`）、`beijing_today()`；`crate::repo::git(root, &[..]) -> Result<String>`；`crate::repo::git_status(root) -> Vec<(XY, path)>`；`crate::repo::human_size(bytes)`。
  - `crate::card::{repo_root(dir), find_chrome(), chrome_cmd(chrome, work, (w,h)), file_url(path), png_size(path), run_ocr(raw, width, lang, work) -> Vec<Word>, Word{text,left,top,right,bottom,conf,line}, load_spec(images) -> Option<Spec>, Spec{kind, date, tldr, roundup, annot}, Spec::outputs(), Spec::main_entries(), Spec::other_entries(), Kind}`。
  - `crate::offsite::{process(root, dir, upload, &mut client) -> Result<usize>, ignored_files(root, yymm, mmdd) -> Vec<Entry{rel,abs,size,sha256}>, git_ls_files(root, flags, pathspec), load_env(root) -> HashMap, write_if_changed(path, text, what), parse_manifest(text) -> Vec<(path, size, sha)>, MANIFEST}`。
  - `crate::lint::{lint_issue(dir) -> Vec<Finding{level,path,line,msg}>, Level, normalize(raw), char_count, blocked_platforms(line)}`。
- 依赖：不新增 crate（`png`、`serde`、`serde_json`、`sha2`、`toml`、`ureq` 已在 Cargo.toml）。外部程序通过 `std::process::Command` 调用：`git`、`oxipng`、`tesseract`、`ffmpeg`、Chrome（`find_chrome`）、poppler（`pdftoppm`/`pdftotext`，优先环境变量 `BARKING_POPPLER` 指定的目录，其次 `~/scoop/apps/poppler/current/bin`，最后 PATH；Git Bash 的 PATH 上有 xpdf 旧版 `pdftotext`，没有 `-bbox-layout`，所以必须按这个顺序找）、`wsl`。
- 文件写入：仓库里的 markdown 都是 LF 结尾；写回时保持 LF，不要引入 CRLF。写文件前比较内容，未变就不写（可用 `offsite::write_if_changed`）。
- 时间戳一律 `repo::beijing_now()`。
- 不 commit、不 push、不改 `docs/` 下已有期次文件（除非该子命令的职责就是改它，且在 `--dry-run` 下不写）。你的工作树里没有图片文件（`.gitignore` 排除）；测试需要图片时，从主工作树 `E:\gitclone\AI-Barking\docs\2610\1009\images\`、`E:\gitclone\AI-Barking\docs\2610\1007\images\` 只读取用，输出写到自己的临时目录（`std::env::temp_dir()/barking-<name>/…`）。
- 质量门槛：`cargo fmt`、`cargo build --release` 无 warning、`cargo clippy --release --all-targets -- -D warnings` 通过、`cargo test --release` 通过；纯逻辑（解析、分区、日期、表格改写）写单元测试；对真实期次目录跑一遍并把实际命令与输出记在回报里。
- 退出前：`git status --short` 只应有你的新模块、main.rs 的两处改动、以及你新增的模板文件。

## 1. `publish-check`（模块 `publish.rs`，与 `stage` 同一文件）

```
barking publish-check <期次目录>
```
对应 `docs/workflow/publish.md` 第 2 步，固定顺序执行，**任一步失败即停**（退出 1），每步打印 `[n/7] 名称 … ok/失败` 与要点：

1. lint：`lint::lint_issue(dir)`，打印全部 finding；有 error 即失败。
2. 发布后标记（规则见 `publish.md` 第 1 步）：
   - `README.md` 内容目录表有 `| 2026 / <MMDD> |` 行且链接到本期 `doc_<MMDD>_publish.txt`；
   - `daily-scan.md`“## 已发过”表有 `| <MMDD> |` 行（`cards.toml` 的 `kind = "roundup"` 速览专帖不要求），且该行不含 `待填`；
   - `daily-scan.md`“## 近 3 天速览已收”表含本期 `cards.toml` 全部非主帖 `[[roundup]]` 条目（`| <MMDD> | tag | text |`，逐字）。
   缺任何一项即失败，提示 `barking scan-rows <期次目录>`。
   实现上只做**按节定位的逐行检查**：找到 `## 近 3 天速览已收` / `## 已发过` / `## 内容目录` 标题后，在到下一个 `## ` 之前的行里找是否有一行（trim 后）等于期望的 `| MMDD | tag | text |`。不要在 publish.rs 里实现表格解析与改写——表格的真正操作只属于 `scan.rs`（整合时主会话可能把这里改成调用 scan.rs 的辅助函数）。
3. oxipng：对 `images/*.png`（不含子目录）：若 `sources/offsite.tsv` 已存在且记录了该文件、SHA-256 与当前一致，则跳过（已登记、可能已上传，压了会让清单失效）；否则运行 `oxipng -o 4 --strip safe <文件…>`（一次传多个文件）。找不到 oxipng → 失败并提示 `scoop install oxipng`。打印压缩前后总字节数。
4. offsite 清单：先读旧 `sources/offsite.tsv`，凡旧清单有而本次找到的不入库原件里没有的路径（本地未取回或不再匹配规则），列出并失败，不重写清单（先从 OpenList 取回，确属删除的手动改清单）；否则 `offsite::process(&root, dir, false, &mut None)`，打印文件数。
5. 大文件：候选入库集合 = 本期目录与 `.handoff/` 下未跟踪（`git ls-files --others --exclude-standard`）与已修改（`git diff --name-only HEAD --`）的文件，加上 `README.md`、`daily-scan.md`；任一文件 > 1 MiB（1_048_576 字节）即失败并列出（规则：先问 Develata）。
6. 身份：从 `.env`（`offsite::load_env`，环境变量优先）读 `BARKING_IDENTITY_PATTERNS`（`;` 分隔的子串，大小写不敏感）；对第 5 步的候选集合逐个文件搜索内容（按 UTF-8 lossy 读取，按行报 `文件:行号`）与文件名；命中即失败。未配置 → 打印 warn“身份检查跳过：.env 未设置 BARKING_IDENTITY_PATTERNS”，不失败。
7. 凭据与会话状态：`git check-ignore -q` 确认 `.env`、`cookies.txt`、`cookies.json`、`storage-state.json`、`.playwright-cli/` 仍被忽略；候选集合中不得出现文件名匹配 `.env*`（`.env.example` 除外）、`cookies.*`、`storage-state*.json`；否则失败。

成功时打印摘要并提示下一步（`barking stage`）。

## 2. `stage`（模块 `publish.rs`）

```
barking stage <期次目录> [--batch 1|2|3] [--add]
```
按 `AGENTS.md`“提交约定”把本期待提交文件分三批（仓库相对路径，`/` 分隔），只看 `git status` 中未跟踪与已修改的文件（忽略被 .gitignore 排除的）：

- ①仓库层：`README.md`、`daily-scan.md`、`.handoff/*-<MMDD>-*.md`。
- ③本期成品：`docs/<YYMM>/<MMDD>/doc_<MMDD>_publish.txt`、`images/README.md`、`images/cards.toml`、`sources/README.md`、`sources/fact-check.md`、`sources/evidence.md`、`sources/capture-log.md`（存在且有改动的）。
- ②取证存档：本期目录下其余全部待提交文件（含 `sources/evidence-<组>.md`、`capture-log-<组>.md`、`offsite.tsv`、脚本、文本存档）。`CORRECTION.md` 不属于三批（发布后修订单独提交），若出现，单独列出并提示。

默认打印三批的文件清单（计数、> 1 MiB 的标出）和建议提交信息：
- ① `<MMDD>：仓库层——README 期次索引、daily-scan 已发过与近 3 天速览已收、取证派工单`（按实际存在的部分裁剪）；
- ② `<MMDD>：取证存档——<待填：按组描述存档内容>、截图脚本与 offsite.tsv`，并附各组前缀统计（如 `a- 37 个，b- 21 个`）供填写；
- ③ `<MMDD>：本期成品——发布正文、配图说明与 cards.toml、来源入口、事实核验、取证清单与抓取日志`。

`--batch N --add`：先确认暂存区干净（`git diff --cached --name-only` 为空，否则失败并列出）；`--batch 1` 时若 `daily-scan.md` 含 `待填` 则拒绝（占位行推送后会污染定时检索；这一步独立于 publish-check，防止跳过检查直接暂存）；再 `git add -- <精确路径…>`（AGENTS.md：只暂存精确路径）；然后打印建议的提交信息。顺序提醒：`--batch 3` 时若 ② 仍有未提交文件，打印 warn“取证存档先于成品提交”。② 为空时打印“当期没有新增取证存档，第②批省略”。**不执行 `git commit`、不 push**（提交信息需要编辑内容，由 Claude 执行）。

单元测试：给定一组 `(XY, path)` 与期次信息，分区结果正确（含 .handoff 匹配、CORRECTION.md 排除、忽略其他期次的文件）。

## 3. `scan-rows`（模块 `scan.rs`）

```
barking scan-rows <期次目录> [--dry-run]
```
生成发布后 `README.md` 与 `daily-scan.md` 里纯机械的行（`publish.md` 第 1 步；**只能在 Develata 说“已发布”后运行**，命令说明里写明）。幂等：已存在的行不重复加。

- 输入：`Issue::title()`（正文首行）、`card::load_spec(images)`（`kind`、`roundup`）。
- `README.md`“## 内容目录”表：行格式与现有行完全一致：
  `| 2026 / <MMDD> | [<标题>](docs/<YYMM>/<MMDD>/doc_<MMDD>_publish.txt) | [配图说明](docs/<YYMM>/<MMDD>/images/README.md) | [来源入口](docs/<YYMM>/<MMDD>/sources/README.md) |`
  年份 = `20` + YYMM 前两位。按期次升序插入（放在最后一个期次小于本期的行之后）；已有本期行则跳过并打印。
- `daily-scan.md`：
  - “## 近 3 天速览已收”表：为每条非主帖 `[[roundup]]` 追加 `| <MMDD> | <tag> | <text> |`（同期次同 text 已存在则跳过）；按期次升序放置；然后删除期次日期 ≤ 本期日期 − 3 天的行（现有行只有 MMDD，年份按本期年份；跨年用 `Date` 算，不要用字符串比较；实际做法见 git 历史：发 1008 时删 1005 行、留 1006/1007）。
  - “## 已发过”表：`kind` 为主帖时，若无 `| <MMDD> |` 行，追加 `| <MMDD> | <标题>：待填（选题与口径摘要，由 Claude 按最终发布版填写） |`；速览专帖不加。这一行是编辑内容，工具只放占位；`publish-check` 会因 `待填` 失败，所以不会带着占位推送。
- 只改这两张表内的行，文件其余内容逐字节不动；LF 结尾。`--dry-run` 打印将要加/删的行，不写文件。
- 单元测试：用内嵌的表格文本测插入位置、去重、三天修剪（含跨月）、速览专帖不加已发过行。
- 验收：在你的工作树对 `docs/2610/1009` 跑 `--dry-run`，应报告“已存在、无改动”（1009 的行已在文件里）；对一个临时构造的期次（复制 1009 到 `docs/2610/1010` 并改名，测试后删除）跑一次，检查生成的行与 1009 现有行格式逐字一致。

## 4. `handoff`（模块 `handoff.rs` + 模板 `templates/handoff-evidence.md`）

```
barking handoff <期次目录> --groups "A=OpenAI 数学撤稿,B=Anthropic 使用政策,C=速览" [--force]
barking handoff --index
```
- 先从 `.handoff/2026-10-09-1009-evidence.md` 与 `2026-10-08-1008-evidence.md` 提炼模板 `templates/handoff-evidence.md`：两份派工单的“目标／先读／通用规则／授权与范围／验收／回报”六节措辞几乎逐字相同，只差期次、基线 commit、未跟踪清单、组号、截图编号区间。模板保留**最新（1009）**的措辞（含“不要运行会读取本机浏览器 Cookie 的工具”等踩坑经验），把可变处改成占位符：`{MMDD}`、`{YYMM}`、`{TITLE_GROUPS}`（`A OpenAI 数学撤稿｜B Anthropic 使用政策｜C 速览`）、`{GROUP_COUNT_ZH}`（三/四）、`{GROUP_LETTERS}`（`A/B/C`）、`{BASELINE}`（`git rev-parse --short HEAD`）、`{UNTRACKED}`（`git status --short` 中除本期目录与本派工单外的条目，写成“工作区已知的未提交改动，一律不要碰：…”；没有就写“工作区干净”）、`{PREV_ISSUES}`（最近两期目录，用于“参考上期脚本”一句——改为指向 `barking shot / html2txt / heat` 命令，并仍可参考上期 `capture-log-*.md`）、`{UNPUBLISHED}`（已有目录但 `daily-scan.md`“已发过”里还没有的期次：列“另加尚未写进 daily-scan 的 X 期：主帖 = <标题>；速览 = <非主帖条目 text 用“、”连接>”，从各期 `publish_txt` 与 `cards.toml` 读；没有就留空）、`{IMAGE_RANGES}`（按组在派工单中的位置分配，每组 12 张：第 1 组 01–12，第 2 组 13–24，第 3 组 25–36，第 4 组 37–48（2026-10-13 起按 1011/1012 实际用法）；主题为“速览”的组写“不需要截图”，但仍占所在位置的区间，所以截图组最多 4 个且须在前 4 位；全部为速览组时（没有主帖的速览专帖）图片一行改写为“本期没有截图组，只存档与摘句”，编号区间句与“截图在本组区间内”的验收项省略）、`{ISSUE_NOTES}`（通用规则一节里每期不同的隐私/风控要点，如 1009 的受害人姓名脱敏、“政府/议员”字样提示；命令行参数 `--note "<一条>"` 可重复，每条成一个 `- ` 项；没有则留一行 `- 本期无额外隐私/风控要点。`）、`{GROUP_SECTIONS}`（每组一节骨架：`# X 组：<主题>`、`主帖第 n 条。`/`速览`、`## 已知情况（Claude {日期} 已读，须独立复核）`、`## 取证清单` 表头 `| # | 说法 | 要求 |`、`候选配图（X 组 NN–NN）：`；速览组按 1009 的 D 组格式给四列表头 `| # | 条目 | 已知线索（须核） | 要求 |`）。
- 生成 `.handoff/20<YY>-<MM>-<DD>-<MMDD>-evidence.md`（已存在则失败，`--force` 覆盖）；若本期 `images/`、`sources/` 目录不存在则创建空目录（派工单写“已新建空目录”）。
- 每次运行后重写 `.handoff/README.md`：保留现有开头两段与结尾一段说明文字，中间列表改为全部 `.handoff/*.md`（README 除外）按文件名倒序，链接文字取各文件第一行标题（去掉 `# `）。`--index` 只做这一步。
- 单元测试：占位符全部被替换（生成文本里不残留 `{`+大写字母）；编号区间分配；全部为速览组时的改写；UNPUBLISHED 判断。
- 验收：在你的工作树用 `docs/2610/1009` 和 `--groups "A=Anthropic 越界报告,B=Google 拟购 Spirit 数据,C=紫外线全天图,D=速览"` 生成到临时文件名（用 `--force` 前先把生成结果写到 temp 对比），与真实的 `2026-10-09-1009-evidence.md` 前六节 diff，差异应只在占位符处与本期特有的隐私/风控条目（`{ISSUE_NOTES}`）处；回报里贴 diff 摘要。

## 5. `shot`（模块 `shot.rs`）

把各期 `sources/<组>-shot.py`、`a-pdf-shots.py`、`m-raw-crops.py`（见主工作树 `docs/2610/1007/sources/a-shot.py`、`docs/2610/1008/sources/a-pdf-shots.py`、`docs/2610/1009/sources/m-raw-crops.py`）的功能收进一个子命令。原则：**只裁切、拼接，不改像素**；输出已存在就拒绝覆盖；每次动作追加一行 JSON 到 `sources/<组>-shots-log.jsonl`（字段仿 a-shot.py：`time_bj`、`action`、`url`、`name`、`file`、`box`、`full_size`、`tool`、`anchors`…）。

```
barking shot <期次目录> <组前缀> render  <url> <name> [--width 700] [--height 1600] [--scale 2] [--chrome-arg <arg>]...
barking shot <期次目录> <组前缀> pdf     <pdf> <页码> <name> [--dpi 144]
barking shot <期次目录> <组前缀> lines   (<name> | --from <png>) [--lang eng]
barking shot <期次目录> <组前缀> crop    (<name> | --from <png>) <out.png> --start "<锚点>" --end "<锚点>" [--pad-top 24] [--pad-bottom 24] [--url <url>]
barking shot <期次目录> <组前缀> cropbox (<name> | --from <png>) <out.png> <x0> <y0> <x1> <y1> [--url <url>]
barking shot <期次目录> <组前缀> stack   <out.png> --part <png>:<x0>,<y0>,<x1>,<y1> [--part ...]... [--gap 24] [--bg auto|#rrggbb] [--url <url>]
```
- 中间渲染放 `std::env::temp_dir()/barking-shot/<YYMM><MMDD>-<组>/<name>.png`；`<name>` 只允许 `[a-z0-9-]`。
- `render`：无头 Chrome（`card::find_chrome` 找可执行文件；**不要复用 `card::chrome_cmd`**，它写死了 scale 1 与 `--allow-file-access-from-files`，自己构造 `Command`），参数：`--headless=new`、临时 `--user-data-dir`、`--disable-gpu`、`--hide-scrollbars`、`--force-device-scale-factor=<scale>`、`--window-size=<width>,<height>`、`--timeout=30000`、`--lang=en-US`、`--screenshot=<绝对路径>`；不用 `--virtual-time-budget`。打印输出路径与像素尺寸（`card::png_size`）。同名重渲会替换中间图（成功后才替换；中间图是草稿不是证据，日志已逐次记录渲染），正式输出 `images/` 仍拒绝覆盖。日志里的 URL 与 `--chrome-arg` 先净化：`file://` 只记仓库内相对路径或文件名，http(s) 去掉 `user:pass@`，键名含 token/key/secret/auth/sig/session/password/cookie/access 的查询参数值记 `[REDACTED]`。
- `pdf`：用 poppler `pdftoppm -r <dpi> -f p -l p -png <pdf> <base>` 渲到中间目录（找到生成的文件改名为 `<name>.png`），同时用 `pdftotext -enc UTF-8 -bbox-layout -f p -l p <pdf> -` 解析 `<line xMin yMin xMax yMax>…<word>…</line>`，把各行文本与 y 范围（pt × dpi/72 → 像素）写到 sidecar `<name>.lines.tsv`（`y0\ty1\ttext`，HTML 实体解码）。
- `lines`：若 sidecar 存在则打印它；否则对图跑 Tesseract（`card::run_ocr` 按 `line` 字段聚合成行）打印 `y0 y1 文本`，用于找锚点。
- `crop`：锚点在行文本里做大小写不敏感子串匹配（起始取第一个命中；结束锚点从起始行起（含起始行）找第一个命中），裁切框 `(0, start.top - pad_top, 全宽, end.bottom + pad_bottom)`，sidecar 存在用 sidecar 行，否则用 OCR 行；找不到锚点即报错。
- `cropbox` / `stack`：像素框裁切；`stack` 纵向拼接各段，段间留 `gap` 像素、填 `bg`（`auto` = 第一段来源图 (5,5) 像素色；默认 auto），宽取各段最大，不足处填 bg。
- PNG 读写用 `png` crate：解码时 `Transformations::EXPAND`（调色板→RGB），16 位图报错“不支持 16 位 PNG”；EXPAND 后灰度图仍是 Gray/GrayAlpha：裁切时按原 color type 原样写出；`stack` 遇到灰度段且 `bg` 为彩色时把灰度提升为 RGB（每通道取同一灰度值，像素值不变）或各段 color type 不一致时统一提升到 RGBA，不能悄悄丢通道；带 ICC 的灰度段不提升为彩色（灰度 ICC 用在 RGB 图上不合规），报错让各段分别出图。含 cICP / mDCV / cLLI 块的 PNG（`png` crate 无法带到输出）解码前即拒绝。写出时 bit depth 8、color type 与上面的规则一致。像素值不变（单元测试：构造小图，裁切后逐像素比对）。
- 工具字符串：`headless Chrome <width> CSS px, DPR <scale>; Tesseract anchor crop` / `pdftoppm -r <dpi> + pdftotext -bbox-layout crop` / `pixel-box crop` / `vertical stack`。
- 验收：对主工作树 `docs/2610/1009/images/02-report-forms-philly.png` 做 `cropbox --from` 到临时输出（把期次目录指到你工作树里的 `docs/2610/1009`，输出会写到你工作树的 `images/`，测完删除并把 jsonl 复原）；对一个真实 PDF（主工作树 `docs/2610/1008/sources/a-oai-ns-paper.pdf` 若存在，否则任一 PDF）跑 `pdf` + `lines`；`render` 用 `https://example.com`。

## 6. `html2txt` 与 `heat`（模块 `fetch.rs`）

```
barking html2txt <in.html> <out.txt> [--force]
barking heat <期次目录> <组前缀> [--hn "<查询>"]... [--reddit "<查询>"]... [--item <HN id>]... [--since <unix 秒>]
```
- `html2txt`：复现 `docs/2610/1007/sources/a-html2txt.py` 的行为（纯 Rust，无依赖）：去掉 `<script|style|noscript|svg>…</…>` 与注释（找这些元素时整段跳过注释与其他标签，所以注释里或属性值里的 `<script>` 不算元素；找注释时同样整段跳过其他标签，所以属性值里的 `<!--`（如 `<p title="<!--">`）不算注释起点、不吞正文；未闭合的元素与注释不吞后文）；`<a href="…">文字</a>` → `文字 [href]`；块级标签（p、div、br、li、h1–h6、tr、section、article、ul、ol、table、header、footer、main、blockquote、figure、figcaption，以及 pre、hr、nav、aside、dt、dd、title）换行；去其余标签；解码 HTML 实体（命名实体：amp/lt/gt/quot/apos、Latin-1 全块 U+00A0–U+00FF 共 96 个（nbsp…yuml，含 eacute/euml/ntilde 等）、mdash/ndash/hellip/ldquo/rdquo/lsquo/rsquo/trade/bull/euro/dagger/Dagger/larr/rarr/uarr/darr/minus/equiv，区分大小写，其余原样保留；数字实体 `&#NNN;`/`&#xHH;`，C1 段按 Windows-1252）；输入与解码结果中的 `\r\n`、`\r` 一律变 `\n`；空白折叠（空格、`\t`、U+00A0），多个空行折成一个；末尾加换行。输入开头的 BOM 去掉，非法 UTF-8 变 U+FFFD。每一遍只向前扫描，对“从某处起找不到”的 `>`、`-->`、闭合标签做记忆，缺终止符的输入也是线性时间。输出已存在且无 `--force` 则拒绝；`--force` 用 `offsite::write_if_changed`（内容未变不写）。单元测试覆盖每条规则。
- `heat`：HN Algolia `https://hn.algolia.com/api/v1/search?query=…&tags=story&numericFilters=created_at_i>SINCE&hitsPerPage=20`（`--since` 缺省为 7 天前）、条目 `https://hn.algolia.com/api/v1/items/<id>`（原样存为 `sources/<组>-hn-<id>.json`）、Reddit `https://old.reddit.com/search.json?q=…&sort=relevance&t=week&limit=25`（UA 用桌面 Chrome 字符串，仿 a-heat.py）。用 `ureq`（已在依赖；用法参考 `offsite.rs` 的 Agent 构造）。汇总写 `sources/<组>-heat.json`：`{"fetched_bj", "hn": {查询: [{id, points, comments, time_bj, title, url}]}, "reddit": {查询: [{sub, score, comments, time_bj, title, permalink, url}]}, "items": [id…], "errors": [...]}`；API 缺的计数与时间（`points`、`comments`、`score`、`time_bj`）写 `null`，不补 0；单个请求失败记入 `errors` 不中断，错误信息带 HTTP 状态与最终 URL，响应不是 JSON 时再带 Content-Type（Reddit 被拦时常重定向到 HTML 页）；终端打印摘要（每条一行）。条目响应正文以 `{` 开头（忽略前导空白）即原样保存，解析只用于摘要行，解析失败仍保留文件并打印 warn。全部请求失败时仍写出汇总文件（hn/reddit 为空、errors 保留），退出码 1；重跑须改组前缀或先删除该文件。文件已存在则拒绝（提示改组前缀或删除）。
- 验收：`html2txt` 对主工作树任一 `sources/*.html` 跑一遍，与同名 `.txt` 比对（允许有差异，但要在回报里说明差异性质）；`heat` 对 `--hn "Claude Haiku 5.5"` 跑一次（联网）。

## 7. `review`（模块 `review.rs` + 模板 `templates/reverse-review.md`）

```
barking review <期次目录> (--visual | --reverse) [--effort medium|high|xhigh|max] [--run]
```
拼出跨模型复核的 WSL Codex 命令（规则：`templates/cards/README.md` 第 5 步、`EDITORIAL.md`“核验流程”第 5 条、`~/.claude/agents/codex-reviewer.md`——后者在主工作树外：`C:\Users\QQ\.claude\agents\codex-reviewer.md`，只读）。

- 提示词文件写到 `std::env::temp_dir()/barking-review/<MMDD>/<visual|reverse>-prompt.md`，报告路径 `…/<visual|reverse>-report.md`（运行前若已存在则改名加序号）。
- `--visual`：附图清单 = `Spec::outputs()` 中实际存在的文件 + 每张批注图的 `<stem>-raw.png` + 存在的 `00-cover.png`、`00-cover-wide.png`；用 `templates/cards/visual-review.md`（`---` 之后的正文）填占位符：`{期次目录}` → `docs/<YYMM>/<MMDD>`；`{附图清单…}` → `images/00-tldr.png（省流卡）；images/00-roundup.png（速览聚合图）；images/00-cover.png（封面）；images/41-x.png（批注图）；images/41-x-raw.png（该批注图的原始底图）…`；末尾附上 `EDITORIAL.md` 的“## 省流卡与批注截图”与“## 速览”两节原文。命令形如（`-C` 为期次目录，`--image=` 每图一个，相对期次目录）：
  `MSYS_NO_PATHCONV=1 wsl -d Debian -e bash -lc 'cd "$1" && exec timeout "$5" "$0" exec -m gpt-6-astra -c "model_reasoning_effort=\"$4\"" -s read-only --skip-git-repo-check --ephemeral -C "$1" -o "$2" --image=… - < "$3"' /home/deve/.bun/bin/codex "<期次 WSL 路径>" "<报告 WSL 路径>" "<提示词 WSL 路径>" "<effort>" "<秒数>"`
- `--reverse`：新建模板 `templates/reverse-review.md`：先写 codex-reviewer.md 里的 charter 段（“You are an independent adversarial reviewer…”原文照抄），再写本仓库的审查范围（按 EDITORIAL.md 第 5 条：正文 `doc_<MMDD>_publish.txt`、`images/cards.toml` 的省流卡/速览/译注文字、`sources/fact-check.md`、`sources/evidence*.md`、`images/README.md`、`sources/README.md` 与 `sources/` 存档；核对每个数字与判断能否在事实清单与存档中找到、限定词有无丢失、有无夸大、卡片文字是否逐字出自正文、平台名风控；输出格式 `BLOCKER | SHOULD_FIX | NICE_TO_HAVE | NEEDS_VERIFICATION`）；占位符 `{期次目录}`、`{MMDD}`、`{文件清单}`（期次目录下入库文本文件的相对路径清单）。命令同上但 `-C` 为仓库根、加 `--search`、不带 `--image`。
- 路径转换：`wsl -d <distro> -e wslpath -a <Windows 绝对路径>`（传 Windows 形式如 `E:\…`，不要传 `/e/…` 或 `/mnt/…`）。环境变量：`BARKING_WSL_DISTRO`（默认 `Debian`）、`BARKING_WSL_CODEX`（默认 `/home/deve/.bun/bin/codex`）。超时：effort `medium` 为 1200 秒，其余 0（不限）。
- 默认只打印：提示词路径、报告路径、附图数、完整命令（可直接粘到 Git Bash 运行）、以及提醒“结论记入 sources/fact-check.md 的“视觉复核”/“反向核验”节”。`--run` 则同步执行该命令（stdout/stderr 直通），结束后打印退出码（124 = 超时）、用时、报告路径与报告首 40 行。
- 单元测试：占位符替换、附图清单生成（给定 Spec 与存在文件集合）、命令字符串拼装（不实际调用 wsl）。
- 验收：对 `docs/2610/1009` 分别跑 `--visual` 与 `--reverse`（不加 `--run`），回报打印的命令；再用 `wsl -d Debian -e wslpath -a E:\gitclone\AI-Barking` 验证路径转换可用。

## 回报格式（每个代理）

JSON（schema 见派工）：新增/修改的文件列表；实际运行的验证命令与关键输出（`cargo test` 结果、对真实期次目录的运行输出摘录）；偏离本设计的地方与原因；未解决的问题。
