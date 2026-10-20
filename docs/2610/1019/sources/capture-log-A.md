# 1019 期 A 组抓取日志（DeepSeek 开源 DeepInfer）

执行方：claude-sonnet-5-5（Claude Code 子代理）。时刻为北京时间（UTC+8；本机美东 EDT = 北京 − 12 小时）。开工 19:12，收到催交时约 20:30，交卷约 20:40，超过 60 分钟目标。

工具：`barking fetch / heat / shot`、curl（GitHub API 与 raw、fxtwitter）、opencli（`browser`、`twitter tweets/thread`、`weixin search`）、WebSearch（2 次，只作线索与 URL 发现）。未使用 firecrawl，未登录任何站点，未输入凭据，未运行读取浏览器 Cookie 的工具。opencli 有两个浏览器 profile 同时连接，需指定 `OPENCLI_PROFILE=5df8dbx7`（选用较新扩展版本的那个）；只做读取。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档（`sources/a-…`） |
|---|---|---|---|---|
| 19:12 | DeepInfer README raw、仓库页、deepseek-recipe README raw、仓库页、DeepSeek 价格页（中英）、api-docs `news1017`（猜的 URL，落到文档首页，不可用）、快科技、知乎 | barking fetch（`a-list1.tsv`） | 8 ok；知乎 403 blocked（改用 opencli） | `a-readme-raw.txt`、`a-repo.*`、`a-recipe-readme-raw.txt`、`a-recipe-repo.*`、`a-pricing*.`、`a-news.*`（无效）、`a-mydrivers.*`、`a-zhihu.html/.headers`（403 页） |
| 19:13 | GitHub API：repo、commits、releases、tags、license；recipe repo 与 commits | curl | 200；releases、tags 为空数组 | `a-gh-*.json` |
| 19:14 | 知乎文章（opencli browser，session `a-shot`，后台窗口） | opencli | 成功；`document.body.innerText` 存 `a-zhihu-dom.txt`（已删去 H1 之前含登录态导航的行）；知乎文章 API（页面内同源 fetch，仅保留标题、时间、作者、正文字段）存 `a-zhihu-api.json` | `a-zhihu-dom.txt`、`a-zhihu-api.json` |
| 19:14 | 知乎图片 CDN 直连 4 张 | curl | 返回 HTML（防盗链）；自己生成的 4 个试验文件与目录已用 `rm`、`rmdir` 精确路径删除 | 无 |
| 19:16 | DeepSeek 官方 X 帖 1/4、3/4、4/4；2/4 经 opencli thread 找到 id | curl（fxtwitter）、opencli twitter tweets / thread | 200；2/4 `status/2112476498598957056` 取得 | `a-fx-2112476496283775135/2112476498598957056/2112476501065286085/2112476503673864665.raw.json` |
| 19:19 | 新浪科技、新浪新闻、网易×2、80aj、LINUX DO | barking fetch（`a-list2.tsv`） | 5 ok；LINUX DO 403 Cloudflare blocked（不绕过） | `a-sina-fin.*`、`a-sina-k.*`、`a-163-1/2.*`、`a-80aj.*`、`a-linuxdo.html/.headers` |
| 19:26 | IT之家、21财经、FX168、新浪微博转载×2、RuntimeWire、Digg | barking fetch（`a-list3.tsv`） | 7 ok | `a-ithome.*`、`a-21jingji.*`、`a-fx168.*`、`a-sina-wb1/2.*`、`a-runtimewire.*`、`a-digg.*` |
| 19:2x | 搜狗微信搜索 “DeepInfer 全面开源” | opencli weixin search | 成功，结果均为转载号；再按 “为善与众行之” 搜一次 | `a-weixin-search.json` |
| 19:3x | 搜狗链接跳转到 mp.weixin.qq.com，取官方号 “DeepSeek” 文章 | opencli browser（session `a-wx`）读 `#js_content` | 成功；`ct` 时间戳 1792486494 = 北京 16:54:54 | `a-wx-official.txt`、`a-wx-official.json`（链接带签名，不稳定） |
| 19:5x | DeepInfer eval/README.md、两张 Pareto 原图 | curl（raw） | 200 | `a-raw-eval-README.md`、`a-raw-doc-images-prefill_pareto.png`、`a-raw-doc-images-decode_pareto.png`（PNG 不入库，未目视核对） |
| 19:58:57 | HN 50163277 | barking heat `--item` | 成功，551 分、168 条嵌套评论 | `a-heat.json`、`a-hn-50163277.json` |
| — | WebSearch ×2 | WebSearch | 只作线索（发现网易、新浪、80aj、IT之家、观察者网、RuntimeWire 等 URL），均另行取一手页验证 | 无 |

清单文件 `a-list1.tsv`、`a-list2.tsv`、`a-list3.tsv`；fetch 日志 `a-fetch-log.jsonl`；截图日志 `a-shots-log.jsonl`。

## 2. 图片（`images/`，本组用 01–04，2 倍像素）

用 opencli 渲染（前台窗口，后台窗口截图会超时；`zoom=2`，`--width 1400`），再用 `barking shot cropbox` 只裁切，不改像素；已逐张打开目视确认文字清楚、范围正确。注入的 JS 只隐藏了页面固定浮层（登录头像、顶栏、目录按钮、底部点赞栏）与 GitHub 右侧栏，不改正文；浅色底。中间整页图在 scratchpad（不入库）。

| 文件 | 尺寸 | 内容 |
|---|---|---|
| `01-readme-scope-not-included.png` | 1234×1545 | README 开头至 “Required for a complete inference service (not included in this release)” 与其三条 |
| `02-zhihu-155t-38usd.png` | 1310×945 | 知乎报告：三目标末项、“今天，我们正式开源……155 万亿 Tokens/天……38 美金（按谷价）”、recipe 段、“尚未完整开源”段 |
| `03-zhihu-online-stats-table.png` | 1310×1375 | 小节标题 + 线上实测表图 + 脚注 “不考虑波峰波谷……（按谷价）” |
| `04-zhihu-offline-pareto.png` | 1310×2050 | 小节标题 + prefill、decode 两张离线图（含条件行与点标说明） |

01 号图第一次裁切（底部多露出一行标题）已用 `rm` 精确路径删除后重裁，`a-shots-log.jsonl` 有两行 01 记录。

## 3. 隐私自查

`docs/2610/1019/sources/a-*` 与 `images/01–04` 已 grep 本机用户名与路径，无命中；知乎截图不含登录头像、私信数（顶栏已隐藏）；`a-zhihu-dom.txt` 已删去 H1 之前的登录态导航行（残留评论区文本为公开内容）。fxtwitter JSON 只含 DeepSeek 官方账号信息。HTML/PNG 原件（`a-*.html`、`a-raw-doc-images-*.png`）按规则只存本地，不入库。
