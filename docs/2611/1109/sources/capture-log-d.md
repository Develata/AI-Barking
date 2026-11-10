# 1109 期取证日志：D 组（速览）

时间为北京时间（UTC+8）。工具：`barking fetch`、`barking heat`（均经 `zsh -lc 'cargo run … tools/barking'`）、curl（一次性）、WebSearch（仅找线索，结论以一手页面为准）。本组未截图，`d-shots-log.jsonl` 不存在属正常。

## 批量存档（`barking fetch docs/2611/1109 d`）

| 时间（BJ） | 名称 | URL | 工具 | 结果 | 备注 |
|---|---|---|---|---|---|
| 15:29:24 | d1-meta-muse-ember | https://research.meta.ai/blog/muse-ember | barking fetch | ok（200，680 745 字节，484 行文本） | |
| 15:29:25 | d2-google-tpu-8i-max-8tx | https://blog.google/innovation-and-ai/infrastructure-and-cloud/global-network/tpu-8i-max-and-8tx/ | barking fetch | ok（200，365 237 字节，912 行） | |
| 15:30:24 | d3-deepx-eetimes | https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/ | barking fetch | failed：请求超时（预算不足，未重试） | 只留 `d-d3-deepx-eetimes.headers` |
| 15:29:24 | d4-asahi-progress-7-3 | https://asahilinux.org/2026/11/progress-report-7-3/ | barking fetch | ok（200，25 211 字节，391 行） | |
| 15:29:24 | d4-lwn-1105936 | https://lwn.net/Articles/1105936/ | barking fetch | ok（200，5 877 字节，72 行） | 仅作转载核对 |
| 15:29:25 | d5-psi-aaronson-ambainis | https://psi.inc/research/aaronson-ambainis/ | barking fetch | ok（200，72 819 字节，124 行） | |
| 15:29:27 | d5-github-aaronson-ambainis | https://github.com/physical-superintelligence/aaronson-ambainis | barking fetch | ok（200，347 766 字节，651 行） | 论文 PDF 未取（派工单：PDF 只存本地，本组未取） |
| 15:58:08 | d5-aaronson-blog | https://scottaaronson.blog/ | barking fetch | ok（200，171 513 字节，1160 行） | 博客首页快照，正文含与本条无关的内容（含个人与政治相关段落）；待删，见下方“本组文件与移动记录” |
| 16:01:37 | d5-aaronson-post-10295 | https://scottaaronson.blog/?p=10295 | barking fetch（单篇，替代首页快照） | ok（200，68 758 字节，344 行） | 文中引用的单篇正文与评论（评论为公开发言，已存）|
| 15:58:09 | d6-anthropic-choosing-model | https://platform.claude.com/docs/en/about-claude/models/choosing-a-model | barking fetch | ok（200，345 521 字节，347 行） | 官方文档页，不是扫描所说的博客 |
| 15:58:13 | d3-deepx-newsroom | https://deepx.ai/news/（final_url：https://deepx.ai/company/news/） | barking fetch | ok（200，600 001 字节，499 行） | 体积恰为 600 001 字节，疑被截断，未核；最新条目 09 July, 2026 |
| 16:04:27 | d6-claude-dev-blog | https://claude.dev/blog/ | barking fetch | ok（200，100 674 字节，119 行） | 博客索引；列出 “Which Claude model should you build with? Fable, Opus, Sonnet, or Haiku”（页面日期 Nov 09, 2026） |
| 16:04:27 | d1-meta-eval-report | https://research.meta.ai/static/muse-ember-evaluation-report | barking fetch（PDF 按 180 秒限时重取） | ok（200，238 703 字节，159 行） | Meta 评测报告 PDF，文本存档 `d-d1-meta-eval-report.txt`；未见对 Muse Ember 权重开放的说明 |
| 16:05 前后 | d6-claude-dev-which-model | https://claude.dev/blog/which-claude-model-should-you-build-with/ | barking fetch | ok（200，137 964 字节，277 行） | 开发者博客选型文；页面日期 Nov 09, 2026（官方未给时刻） |

后续单次请求：

| 时间（BJ） | 名称 | URL | 工具 | 结果 | 备注 |
|---|---|---|---|---|---|
| 约 15:5x | d3 重试 | https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/ | curl `--max-time 60`（一次） | 失败：`curl: (92) HTTP/2 stream 1 was not closed cleanly: INTERNAL_ERROR` | 同一来源两次失败，按派工单止步换来源；`d3-retry.html` 未写入仓库（保存在 scratchpad） |
| 约 16:0x | DeepX 产品页 | https://deepx.ai/products/dx-m2/ | curl `--max-time 60` | 200，1 654 402 字节 | 用于核查 tape-out、6.7 倍，未见；页面保存在 scratchpad，未入库（HTML 原件只存本地） |
| 约 16:0x | EE Times（第三次，opencli） | https://www.eetimes.com/deepx-tapes-out-second-gen-chip-projects-6-7x-performance-gain-with-lpddr5x-pim/ | `opencli browser --session d3eet open <url> --window background`，再 `get text`（失败：usage_error，缺目标）、再 `extract`（成功，16 335 字符） | ok，正文存为 `d-d3-eetimes-opencli.txt` | 这是第三次尝试，结束后不再重试；文章页面日期 11.09.2026（作者 Sally Ward-Foxton） |
| 约 16:0x | Anthropic 回应（WebSearch，一次） | — | WebSearch（standard） | 线索 | 搜索摘要称据 Business Insider 报道 Anthropic 未回复置评请求（未存档，L4 线索） |
| 16:1x | 只读核验（非网络） | evidence-d.md 与存档 `.txt` | Python 脚本逐字核对 evidence-d.md 中的直引（直引号与中文引号）与存档；脚本见 scratchpad，未入库 | 结果：除一处缺 “further” 已修正外，其余直引均在存档中；其余“未找到”项多为中文自述或 HN 标题（HN 数据在 `.json`） | 泄露检查：`grep` 对 `d-*.txt`、`d-*.html`、`evidence-d.md`、`capture-log-d.md` 查 avatar/nickname/handle/login/cookie/password/api key；只命中站点界面文字（页脚 Cookies 链接、登录链接、LWN 表单标签 “Password:”），无抓取者的头像、昵称或凭据。评论区公开用户名见 `d-d5-aaronson-post-10295.txt`，属公开发言，未另处理 |

## 热度（`barking heat docs/2611/1109 d`）

| 时间（BJ） | 调用 | 结果 | 备注 |
|---|---|---|---|
| 15:56:54 | `--item 50420532` | 写入 `d-hn-50420532.json`（70 分、35 评） | 之后因覆盖保护被移入 scratchpad，见下 |
| 15:57 前后 | 逐条 `--item 50429024`、`50432831`、`50429545`、`50429906` 追加 | 4 次均报错：`d-heat.json` 与 `d-hn-*.json` 已存在，拒绝覆盖，未写入 | `50429906` 不属于 D 组，未保留 |
| 15:59:46 | `--item 50429024 --item 50432831 --item 50429545 --item 50420532 --hn "Aaronson-Ambainis"` | 写入 `d-heat.json`（HN 1 组，条目 4 个，0 个请求失败） | 为覆盖保护，先将旧的 `d-heat.json` 与 `d-hn-*.json` 移到 scratchpad（`heat-old/`、`d-heat-first.json`、`d-hn-50420532.json` 副本），未删除；最终 `d-heat.json` 与 `d-hn-*.json` 为本次结果 |

HN 快照（15:59 BJ 取得）：

| HN id | 标题（HN） | 发帖（BJ） | 分数 | 评论（嵌套计数） |
|---|---|---|---|---|
| 50429024 | Muse Ember: Our smallest, most efficient model | 11-10 01:58:46 | 291 | 118 |
| 50432831 | TPU 8tx and TPU 8i Max | 11-10 05:56:57 | 244 | 75 |
| 50429545 | DeepX claims inference up to 100B parameters at less than 5 W | 11-10 02:33:58 | 152 | 24 |
| 50420532 | Asahi Linux Drops MIT, Goes Full GPL After AI Used to Steal Unreleased Code | 11-09 11:06:55 | 70 | 35 |
| 50393770（搜索命中，PSI 页） | A proof of the Aaronson‑Ambainis conjecture | 11-06 23:45:08 | 11 | 2 |
| 50402211（搜索命中，PSI 页，重复提交） | 同上 | 11-07 14:38:59 | 3 | 0 |
| 50432324（搜索命中，PSI 页，重复提交） | 同上 | 11-10 05:19:22 | 3 | 0 |
| 50425945（搜索命中，Aaronson 博客） | Aaronson-Ambainis Conjecture proved (By AI, of course) | 11-09 22:14 | 1 | 0 |

## 搜索（WebSearch，只作线索）

| 时间（BJ） | 查询 | 用途 | 结果 |
|---|---|---|---|
| 约 15:4x | “Aaronson Shtetl-Optimized Aaronson-Ambainis conjecture proof Physical Superintelligence” | D5：找 Aaronson 博客与社区报道 | 找到 PSI 与多家英文报道；未直接给出博客 URL（后由 scottaaronson.blog 首页补齐） |
| 约 15:4x | “DeepX DX-M2 tape out second generation chip 6.7x LPDDR5X PIM” | D3：找替代来源 | 未找到流片与 6.7 倍的来源；命中 Rambus 与 DeepX 产品页，只有规格与 CES 2026 报道的摘要 |
| 约 15:4x | “Anthropic developer blog choosing a Claude model guide …” | D6：找博客 | 未找到博客；命中官方文档页 “Choosing the right model” |
| 约 16:0x | “Aaronson-Ambainis 猜想 Physical Superintelligence AI 证明 Fable” | D5：中文稿是否写成 Fable 5.2 | 返回英文报道为主；未找到中文稿写成 “Fable 5.2” 的例子；检索仅一次 |

## 本组文件与移动记录

- 仓库内新增（`docs/2611/1109/sources/`）：`d-d1-meta-muse-ember.*`、`d-d1-meta-eval-report.*`、`d-d2-google-tpu-8i-max-8tx.*`、`d-d3-deepx-eetimes.headers`、`d-d3-eetimes-opencli.txt`、`d-d3-deepx-newsroom.*`、`d-d4-*`、`d-d5-psi-aaronson-ambainis.*`、`d-d5-github-aaronson-ambainis.*`、`d-d5-aaronson-post-10295.*`、`d-d5-aaronson-blog.*`（待删）、`d-d6-anthropic-choosing-model.*`、`d-d6-claude-dev-blog.*`、`d-d6-claude-dev-which-model.*`、`d-fetch-log.jsonl`、`d-heat.json`、`d-hn-50429024.json`、`d-hn-50429545.json`、`d-hn-50432831.json`、`d-hn-50420532.json`；另有本组交付 `evidence-d.md` 与本文件。
- 本组的 `d-*.html`（HTML 原件）与 PDF 原件只存本地，不入库；入库的是 `.txt`、`.json`、`.jsonl`、`.headers`。
- 为绕开 `barking heat` 的覆盖保护，把本组自己生成的 `d-heat.json`、`d-hn-*.json` 移到 scratchpad（`/private/tmp/claude-501/-Users-develata-gitclone-AI-Barking/d700879c-d9ce-4b32-a35b-8be2e6872144/scratchpad/heat-old/`、`…/scratchpad/d-heat-first.json`、`…/scratchpad/d-hn-50420532.json`）。这些文件已不在仓库中，内容被后面一次正式结果覆盖，无需删除。
- 待删除（仓库内，由 Claude 收尾时一次删除）：
  - `/Users/develata/gitclone/AI-Barking/docs/2611/1109/sources/d-d5-aaronson-blog.headers`
  - `/Users/develata/gitclone/AI-Barking/docs/2611/1109/sources/d-d5-aaronson-blog.html`
  - `/Users/develata/gitclone/AI-Barking/docs/2611/1109/sources/d-d5-aaronson-blog.txt`（博客首页快照，已由单篇 `d-d5-aaronson-post-10295.*` 取代）
  - `/Users/develata/gitclone/AI-Barking/docs/2611/1109/sources/d-d3-deepx-eetimes.headers`（EE Times 失败时的响应头，无正文，可删可留）
- 之前 `d-d5-aaronson-post-10295.*` 与 `d-fetch-log.jsonl` 的新增行为本组正式记录，保留。
- 只读检查：未触碰 `.DS_Store`、`docs/.DS_Store`、`.handoff/README.md`、`.handoff/2026-11-09-1109-evidence.md`，未碰 A/B/C 组文件，未 commit、未 push、未调用 `rm`。

## 失败与限制

- D3 EE Times：barking fetch 超时、curl HTTP/2 错误两次失败；第三次改用 opencli `extract` 取得正文（`d-d3-eetimes-opencli.txt`）。已按 L4 媒体记录，DeepX 官方新闻稿未找到。
- opencli 第一次 `get text` 因参数不全报 usage_error，未重试（改用 `extract`）。
- D3 DeepX 新闻页快照可能被截断（600 001 字节），未核。
- D5 Anthropic 回应：未查到。
- D5 arXiv 编号：未找到；论文 PDF 未取。
- D6：只找到官方文档页，未找到扫描所说的博客；15 分钟检索限制内止步。
- 中文稿检索只做一次。
