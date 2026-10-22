# B组抓取日志

所有时刻均北京时间 UTC+8。详细每条URL、HTTP状态、字节数见 b-fetch-log.jsonl；截图动作/坐标/时刻见 b-shots-log.jsonl。网页获取统一用 barking fetch，截图统一用 barking shot render/cropbox，未写替代抓取脚本。

## 网页存档

2026-10-22 16:43:31 开始，清单 b-fetch-list.tsv，7条全200/ok，已读纯文本确认是真正文：

- https://docs.litellm.ai/blog/security-incident-oct-21-2026 → b-litellm.html/.headers/.txt
- https://www.php.net/archive/2026.php → b-php.html/.headers/.txt
- https://eliastaylor.com/noexploit/ → b-php-poc.html/.headers/.txt
- https://nodejs.org/en/blog/vulnerability/october-2026-security-releases → b-node.html/.headers/.txt
- https://blog.railway.com/p/security-disclosure-wednesday-october-21st → b-railway.html/.headers/.txt
- https://status.railway.com/incident/QH8L8B3E → b-railway-status.html/.headers/.txt
- https://github.com/rust-lang/rustup/releases/tag/1.29.2 → b-rustup.html/.headers/.txt

2026-10-22 16:44:18 开始，清单 b-fetch-rust.tsv，2条全200/ok，已读正文：

- https://blog.rust-lang.org/2026/10/20/Rustup-1.29.2/ → b-rust-blog.html/.headers/.txt
- https://rustsec.org/advisories/RUSTSEC-2026-0382.html → b-rustsec.html/.headers/.txt

## 截图

公开匿名headless Chrome，700 CSS px、DPR 2，不碰登录态。所有成图已逐张打开目视确认，清楚、无个人登录信息；浅色页面。

- LiteLLM带#confirmed-affected-versions首次render返回白图（失败1）；不带锚点重试成功。再次加高至3500 CSS px取得整段。原页截图存sources/b-litellm-original.png。
- PHP带#2026-10-21-5 render成功；原页sources/b-php-original.png。裁14-php-raw.png，完整标题、日期、修复版本和Who is at risk两段。
- Node.js不带锚点render成功；带Brotli小节锚点尝试得到白图（失败1），未再重试该锚点。使用原成功截图，sources/b-node-original.png，裁15-node-raw.png，完整Security releases available及依赖列表。Brotli具体条目依赖文本证据，不声称已截在图中。
- Rust博客render成功；sources/b-rust-original.png，裁16-rust-raw.png，包含文章标题日期、工具定义、更新内容与引用，文字无截断。
- 13-litellm-raw.png完整保留Confirmed Affected Versions、所有列项、source tag正常、移除说明，1105×1905（比目标高，为避免割裂条件）；17-litellm-intro-raw.png为较短备用（1105×1055），包括状态时刻、恶意软件/企业私版范围与厂商攻击链声明。
- 未给Railway另截图：保留5张预算，并使用一手正文存档。

## 热度与搜索

- web工具搜索Node.js、Rust安全公告回溯一手。未把搜索摘要作为事实验收，均以b-*原站存档核对。
- barking heat 首次 --since误用日期字符串，工具参数校验失败，没有网络取样；改Unix秒后成功。
- 16:45:11 b-heat.json：HN LiteLLM/PHP/Rustup；0请求失败。Rustup0命中如实保留。
- 16:46:08 b-comments-heat.json与两份 b-comments-hn-*.json：两主帖及评论。HN不公开评论分数，未编数字/冒称高赞。
- opencli Extension disconnected为派工前已确认环境，本组未反复重试；不进入私人浏览器、不提取cookie。

## 未覆盖与安全边界

未运行攻击代码，未复现PHP/Node.js漏洞，未审查每份PoC。未确认LiteLLM与Railway同源。Railway动态进度仅截至快照08:01 UTC，正式发布前需要刷新。没有文章时刻的不推定为UTC午夜。全部来源公开，未持久化浏览器会话；公开作者的职业署名与公开联系方式属于原文，未收集私人信息。未改正文、cards.toml、README及其他组文件，无提交推送。
