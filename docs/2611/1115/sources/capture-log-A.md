# 1115 期 A 组抓取日志

执行：Claude Sonnet 5.5（`claude-sonnet-5-5`）。时间为北京时间（本机时区，开始于 2026-11-16 23:58，其后本机时钟跨过午夜，日期记 2026-11-17），取自 `a-fetch-log.jsonl`、`a-shots-log.jsonl`、`a-heat.json` 与终端时间。文件前缀 `a-`。未登录任何账户，未提交表单，未发帖。浏览器为 opencli 驱动的真实 Chrome（会话名 `a-shot`，前后台窗口交替）。截图页为匿名视图；Qwen / 混元 / Vals / TokenHub 页上的浏览器翻译插件浮标与回到顶部按钮已在截图前用页面脚本隐藏（只改显示，不改页面文字）。`opencli doctor` 与扩展连接由派工方预先确认。

## 抓取

| 时刻 | URL | 工具 | 结果 |
|---|---|---|---|
| 23:59 | https://qwen.ai/blog?id=qwen4-max | opencli browser open + eval innerText / outerHTML | 成功。中文版 `a-qwen-blog-zh.rendered.txt`、`a-qwen-blog-zh.html`（389 KB）；中文版价格句为 26.58 元→11.16 元（-58%） |
| 00:00 | https://qwen.ai/blog?id=qwen4-max&lang=en | 同上（点击页面“翻译:English”） | 成功。`a-qwen-blog-en.rendered.txt`、`a-qwen-blog-en.html`；英文版为 $4.43→$1.67（-62%） |
| 00:00 | 博客两张图（img.alicdn.com，5400×3600 综合性能图、3060×1440 token 效率图） | curl（`--max-time 60`） | 成功。`a-qwen-img-benchmarks.png`（约 1.3 MB）、`a-qwen-img-token-eff.png`；图片不入库 |
| 00:03 | AA：models/qwen4-max、methodology/intelligence-benchmarking、leaderboards/models、models/kimi-k3-1-pro、models/qwen3-8-max、providers/tencent；models/hy4（猜测） | `barking fetch`（`a-fetch-list-1.tsv`） | 6 条 ok；`models/hy4` 404 failed。html 均 1–1.4 MB 只存本地，txt 均 < 200 KB |
| 00:04 | AA：models/tencent-hy4（ok）、models/tencent-hy4-flash（猜测，404 failed）（`a-fetch-list-2.tsv`） | `barking fetch` | Hy4 flash 在 AA 无条目 |
| 00:04 | AA：models/releases、models/creators/tencent、models/qwen4-max/providers（`a-fetch-list-3.tsv`） | `barking fetch` | 3 条 ok；Tencent 名下 AA 只跟踪 Hy4 一款 |
| 00:05–00:16 | AA / Hugging Face / ithome / Vals 页面渲染（aa-q4、aa-q4full、aa-q4tall、aa-cmp、aa-q38、hf-org、hf-search、ithome、ithome2、vals） | `barking shot render`（headless Chrome，700 CSS px、DPR 2；`BARKING_CHROME` 指向 Chrome 应用二进制） | 成功，中间图在系统临时目录，不入库。`aa-cmp`（带 `?models=` 的比较页）参数未生效，未使用；`aa-q4full` / `aa-q4tall` 用于找 Output Tokens 图，该图被裁，未采用 |
| 00:07 | https://hunyuan.tencent.com/research/100145?langVersion=zh | opencli browser open + eval | 成功。`a-hy4flash-zh.rendered.txt`、`a-hy4flash-zh.html`（含页面内图表数据数组）。html 里含站点自带的腾讯云 COS 签名图片链接（与抓取者无关），仅存本地 |
| 00:08 | https://hy.tencent.ai/research/hy4-flash | opencli browser open + eval（等待约 10 秒后取到 8466 字符） | 成功。`a-hy4flash-en.rendered.txt`、`a-hy4flash-en.html`。（此前 curl 取该页为 HTTP 567） |
| 00:09–00:11 | 中文页内“Open Benchmarks”切换 | opencli eval 点击页面元素 | 成功；仅切换图表标签，无提交 |
| 00:11 | https://cloud.tencent.com/tokenhub/hy4-flash | `barking fetch`（txt 仅 24 行，内容不全）→ opencli 渲染 | 渲染成功，`a-tokenhub.rendered.txt`；价格含“限时 5 折” |
| 00:11–00:12 | HF API：`models?author=tencent&sort=lastModified…limit=30`、`models?author=tencent&search=Hy4`；HF 页 `huggingface.co/tencent`；直接 `tencent/Hy4-flash`、`tencent/Hy4`；GitHub API：`search/repositories?q=Hy4+org:Tencent-Hunyuan`、`orgs/Tencent-Hunyuan/repos`；OpenRouter 页 `openrouter.ai/tencent/hy4-flash`；新京报 `bjnews.com.cn/detail/1794811095129773.html`；AIHOT（`a-fetch-list-4.tsv`） | `barking fetch` | HF API / 组织页 / GitHub API / OpenRouter / 新京报 ok；**`tencent/Hy4-flash`、`tencent/Hy4` 直接访问 HTTP 401 blocked**（HF 对不存在仓库同样返回 401，不能据此判定）；**AIHOT 协议错误 failed**，未重试。`a-hf-tencent-api.json` 等 |
| 00:12:32 | HN：`--item 50513215`、`--hn` “Qwen4-Max” / “Hy4 flash” / “Tencent Hy4”；Reddit 两条 | `barking heat docs/2611/1115 a` | HN 成功：50513215 为 745 分 / 374 评（11-16 10:14 北京时间）。**Reddit 两条搜索被重定向到登录页，未取**。`a-heat.json`、`a-hn-50513215.json`（Algolia 评论树，368 节点，不含评论得分） |
| 00:12 | Exa 检索（`web_search_exa`，共 3 次：Qwen4-Max 中文报道、Hy4 flash 成本报道、官方 X 帖 “18x”） | Exa MCP | 仅作线索，产出媒体清单与官方 X 帖 id |
| 00:13 | https://api.fxtwitter.com/status/2122217770314301469 | curl（`--max-time 40`） | 成功。`a-fx-2122217770314301469.json`：发布者为官方账号 @TencentHunyuan，发布时间 Mon Nov 16 06:38:39 +0000 2026；JSON 不含抓取者信息，无需替换 |
| 00:13:49 | 媒体：InfoWorld、IT之家、the-decoder.de、Mint、阿里云新闻稿、TechNode、科技日报、cryptobriefing、Sulat、HN 条目页 50517032 与 50513215（`a-fetch-list-5.tsv`） | `barking fetch` | 11 条全 ok（`a-m-*.txt`） |
| 00:14 | https://arena.ai/leaderboard/code 、/leaderboard/webdev | curl（`-A "Mozilla/5.0"`，`--max-time 30`） | **403，Cloudflare “Just a moment...”，未绕过** |
| 00:14 | https://www.vals.ai/benchmarks/vals_index | curl → `barking fetch`（`a-fetch-list-6.tsv`） | ok，`a-vals-index.html`（753 KB）、`a-vals-index.txt`（含 Qwen 4 Max 第 11、Qwen 3.8 Max 0902 第 18 的行） |
| 00:15 | Vals 页展开折叠区并滚动取 Qwen3.8 行 | opencli eval（点击 “VIEW 22 MORE MODELS”后缩放滚动） | 展开成功但截图为空白 / 未滚到目标，**放弃**，改用文本存档 |

未登录；被拒或失败：`aa models/hy4`（404，猜测 URL）、`aa models/tencent-hy4-flash`（404，猜测 URL）、HF `Hy4`/`Hy4-flash` 直接访问（401）、AIHOT（协议错误）、Arena 两页（403 Cloudflare）、Reddit（登录重定向）、官方公众号原文（未尝试破解验证码）。未做：HN 页截图、新智元 / 环球网 / 中新经纬 / 北京商报单独存档。

## 截图

渲染来源：中间图均在系统临时目录（`barking-shot/26111115-a/`），再用 `barking shot cropbox / stack` 只裁切不改像素写入 `docs/2611/1115/images/`。Qwen、混元、TokenHub 页用 opencli（`document.documentElement.style.zoom='2'`，`--width 1400`）取图；AA、HF、IT之家、Vals 用 `barking shot render`（700 CSS px，DPR 2）。全部匿名。逐张已打开确认文字清楚、范围正确。

| 文件 | 来源页 | 说明 |
|---|---|---|
| `01-qwen-title-en.png` | qwen.ai 英文博客 | 标题与日期行（1400×180）；**深色底页面** |
| `02-qwen-cost-zh-en.png` | qwen.ai 中文 / 英文博客 | 上半中文版成本段（-58%，元）、下半英文版成本段（-62%，美元），黑色拼接底；**深色底页面**（1400×635） |
| `02a-qwen-cost-en.png` | qwen.ai 英文博客 | 英文成本段单张，已被 02 覆盖，**待删除**（见回报） |
| `03-qwen-bench-table.png` | qwen.ai 英文博客 | 综合性能图 16 项全表含图例（1400×910）；页面截图，数字小，原图另存 `a-qwen-img-benchmarks.png` |
| `04-qwen-token-chart.png` | qwen.ai 英文博客 | Work360 与 Terminal-Bench 4.0 的 Score vs. Tokens 图含标题与图例（1400×622） |
| `05-aa-qwen4max-summary.png` | artificialanalysis.ai/models/qwen4-max | 页头、四个摘要卡、三段说明文字（1400×1950） |
| `06-aa-cost-per-task-chart.png` | 同上 | Cost per Task 图：GPT-6.1 Sol (max) $1.27、Qwen4 Max $3.27、Kimi K3.1 Pro $3.68…（1400×770） |
| `07-aa-qwen38max-summary.png` | artificialanalysis.ai/models/qwen3-8-max | Qwen3.8 Max (0902) 页头与四个摘要卡：34 / $10.02 / 150M（1400×1560）；顶部 “deprecated” 横幅与登录按钮未入框 |
| `08-hy4flash-title-zh.png` | hunyuan.tencent.com 研究页 | 日期、标题、参数段（两段拼接，1400×770）；页顶导航与大图未入框 |
| `09-hy4flash-params-price-zh.png` | 同上 | 参数句、价格表、预训练 / 后训练 / 评估 / 推理四条（含 -50%、150 tps、KV cache）（1400×1205） |
| `10-hy4flash-1over20-workindex-fig1.png` | 同上 | “1/13、1/20、1/8” 句 + 图 1（WorkIndex 页签）+ 图注（1400×1575） |
| `10b-hy4flash-open-benchmarks-fig1.png` | 同上 | 图 1 切到 Open Benchmarks 页签 + 图注（1400×1360） |
| `11a-hy4flash-fig2-cost.png` | 同上 | 图 2：¥5.7 / ¥3.2 / ¥0.2，13.8×、24.5× cheaper，Performance 51.4 / 39.7 / 42.1（1400×620）；图的上缘（Performance 标题）略贴边但数据完整 |
| `11b-hy4flash-blind-test.png` | 同上 | 85 人 / 259 任务盲测句 + 盲测表 + “外部模型均为 high 档”注（1400×985） |
| `12a-hy4flash-availability-open-source.png` | 同上 | 上线产品、11-30 前免费与 5 折、推荐场景、“计划开源”句（1400×315） |
| `12b-hf-tencent-hy4-search.png` | huggingface.co/models?search=tencent hy4&sort=modified | 深色底；Models 2：Hy4-preview-FP8、Hy4-preview，“Updated Aug 28”（1400×660） |
| `12c-ithome-headline-over60.png` | ithome.com/1/027/512.htm | 深色底；标题“任务成本降低超 60%”与 2026/11/16 10:07:36（1400×240） |
| `12d-hy4flash-workbuddy-quote.png` | hunyuan.tencent.com 研究页 | WorkBuddy 产品经理 Anne 引语（“1/15 左右……基本持平”）（700×572，三段拼接，无缝） |
| `12e-tokenhub-price-5off.png` | cloud.tencent.com/tokenhub/hy4-flash | 价格块：输入 0.5→0.25，输出 2→1，缓存命中 0.02→0.01（1400×275） |
| `12f-vals-index-table.png` | vals.ai/benchmarks/vals_index | 前 16 行：Qwen 4 Max 第 11，63.20% / $9.33 / 27.27m（1400×1620）；“Systems (38)” |

编号说明：A 组区间 01–12；为不占用 B 组的 13 号起，12 号下用字母后缀。**候选配图 “中文夸大标题实例”** 对应 12c。HF 组织页（`hf-org` 中间图）没裁成成品：组织页首屏没有模型列表，用搜索页 12b 代替。
