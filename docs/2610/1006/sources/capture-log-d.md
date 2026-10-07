# 1006 D组抓取日志

所有抓取时间为北京时间UTC+8，日期2026-10-07（本机原时区EDT，仍是10/6）。不是把采集时刻当新闻发布时刻。

## 完整逐次记录

- [d-http-records.jsonl](d-http-records.jsonl)：每次匿名curl请求的URL、北京时间、HTTP状态、最终地址、字节数、文件基名。**saved不等于正文有效**，验收以下表为准。
- [d-browser-records.jsonl](d-browser-records.jsonl)：匿名Chrome页面读取/截图时间、原站URL、设备比例、CSS尺寸、截图区域、输出名。浏览器profile在系统临时目录，本组未清理/删除，未存入仓库。
- [d-capture-records.jsonl](d-capture-records.jsonl)：OpenCLI browser **1006-d**首次抓取失败。
- [d-twitter-records.jsonl](d-twitter-records.jsonl)：Twitter CLI只读搜索/读帖逐次失败，查询、北京时间、脱敏错误。
- [d-community.md](d-community.md)：web/Exa检索的分钟级窗口、查询范围、Reddit公开摘录、永久链接、计分缺口。

## HTTP与浏览器内容验收

| 文件基名 | URL/结果 |
|---|---|
| d-mistral-blog、d-mistral-browser、d-mistral-shot、d-gpu-clean、d-blog-full-opening、d-blog-context | https://mistral.ai/news/mistral-large-4/ ：匿名HTTP200及浏览器正文成功，含preview、权重时点、训练GPU、RL规模、公开访问限定、价格卡。重复浏览器快照对应不同截图，不代表多个独立来源。 |
| d-mistral-doc、d-doc-browser、d-doc-shot | https://docs.mistral.ai/models/mistral-large-4-0 ：HTTP200和浏览器成功；宽版HTML有划线旧价，700 CSS移动布局只显示低价且省略三类token标签，证据表明确说明。 |
| d-mistral-pricing | https://docs.mistral.ai/inference/pricing ：HTTP200，有Large 4低价行；未发现折扣原因/期限。 |
| d-aa、d-aa-browser、d-aa-shot | https://artificialanalysis.ai/models/mistral-large-4 ：HTTP200/浏览器成功，核到38、64/225、116.1、Proprietary、1.36/4.18。FAQ中权重不公开的逐字句来自匿名HTTP的d-aa.txt；浏览器未展开FAQ，不声称该句在初始可见正文中。 |
| d-liquid、d-liquid-browser、d-liquid-shot | https://www.liquid.ai/blog/d1-decision-model ：HTTP200/浏览器成功，methodology及未来模型权重表述完整。 |
| d-dust、d-dust-browser | https://qlabs.sh/research/dust ：HTTP200/浏览器成功；限制句、243M规模、October 2026日期可核。 |
| d-bloomberg | https://www.bloomberg.com/news/articles/2026-10-06/deepseek-to-raise-at-least-12-billion-in-tencent-backed-funding ：HTTP200，仅公开前两段含融资数字、参与者；两段后直接导航，不能声称全文。未提取或解码隐藏付费正文。 |
| d-bloomberg-browser | 同上；**失败**，页面“unusual activity”验证，不是新闻正文。没有点击挑战/更换代理绕过。 |
| d-reuters、d-reuters-browser | https://www.reuters.com/world/asia-pacific/deepseek-raise-least-12-billion-tencent-backed-funding-bloomberg-news-reports-2026-10-06/ ：**失败**，curl401（771字节JS提示），web工具不可打开，匿名Chrome正文0字符。没有拿搜索摘录/转载替代；30–31无截图。 |
| d-cnbc-deepseek | https://www.cnbc.com/2026/10/06/deepseek-funding-round.html ：HTTP200完整新闻正文；另家原始采访补充，明确不冒充路透原文。 |
| d-tnw-deepseek | https://thenextweb.com/news/deepseek-12bn-funding-round-tencent-catl ：HTTP200，明确腾讯/宁德具体数字在first round段；未升为当前轮一手数据。 |
| d-tnw | https://thenextweb.com/news/mistral-releases-large-4-a-1-trillion-parameter-open-weight-ai-model ：HTTP200；当前标题与URL旧措辞不同；正文保留preview/未来权重释放。 |
| d-overclaim-zh | https://www.winzheng.com/article/deepseek-12b-pre-ipo-round-tencent-catl-2027-ipo ：HTTP200；标题“完成”与正文“接近完成”对照，JSON-LD发稿/更新时间可核。 |
| d-codex-tracker、d-claude-tracker、d-rank-tracker | https://codex-resets.com/ 、https://claude-resets.com/ 、https://opentherank.com/codex-reset/ ：均HTTP200；仅L5线索，未发现10/4之后的新事件，不支持穷尽否定。 |
| d-openai-banked、d-openai-help-browser | https://help.openai.com/en/articles/20001498-how-banked-codex-resets-work ：**失败**，HTTP403/Chrome空正文；不能声称当前帮助条款核验通过。浏览器命令写出空快照后进程退出-1073741819，未作为成功正文。 |
| d-claude-help | 猜测12429409-what-is-a-limit-reset被站方重定向到manage-usage-credits-for-paid-claude-plans；**不是目标来源**，不引作reset证据；后经搜索找到正确17007452地址。 |
| d-claude-reset-help | https://support.claude.com/en/articles/17007452-what-is-a-limit-reset ：HTTP200正文完整；只讲一般机制，没给10/4以后新发放公告。 |
| d-deepseek-official、d-deepseek-news | https://api-docs.deepseek.com/news 、https://api-docs.deepseek.com/updates ：HTTP200但返回Your First API Call；**否决为新闻栏目采集**。另做DeepSeek/Tencent/CATL官方域名检索，未找到融资公告，不据此证明不存在。 |
| d-hn-query、d-hn-mistral、d-hn-liquid-query、d-hn-deepseek-query、d-hn-deepseek、d-hn-dust-api | hn.algolia.com公开API，具体URL见逐次台账；`.html`是通用保存脚本的扩展名，实际JSON原件，解析使用这些原件。自动生成的同名`.txt`会剥离JSON字符串中的HTML标签，不当原始JSON。 |
| d-hn-dust | https://news.ycombinator.com/item?id=49970871 ：原帖HTTP200，与API并存；评论分数不公开。 |
| d-mistral-oembed | https://publish.twitter.com/oembed （重定向publish.x.com）官方API；取得Mistral主帖公开截断摘要，不是完整长帖，不用它判全文没有价格说明。只含发帖者资料，无抓取者资料。 |

## 工具失败/非网络失败

- 00:41–00:45：OpenCLI doctor显示Daemon正常但Extension未连接；按派工单执行一次daemon restart，仍未连接；1006-d open失败。依据opencli-autofix技能的BROWSER_CONNECT硬停止规则，没有改适配器、扩展或浏览器设置。
- 约00:45：两次只读Python依赖检查命令被自动审批拒绝（`approval required by policy, but AskForApproval is set to Never`），没有执行，没有审批权限，也没有继续重试Python。之后使用已安装Node/Chrome完成公开抓取与截图。
- 约00:48：初版d-browser.mjs指向Puppeteer缓存路径错误；修正入口后发现缓存包缺`@puppeteer/browsers`依赖；仅改本组脚本为已有Playwright Core，不安装、不升级，不改系统包。
- 约00:51：一次含两条Node命令的PowerShell调用出现“node.exe被当JS解析”的SyntaxError；未生成d-hn-liquid-query；后独立调用已成功，记录在00:52:22 HTTP台账。
- 00:54–00:55：Twitter CLI已配置显式凭据，但ClientTransaction初始化失败；OpenAI重置搜索按技能重试一次仍失败，Claude搜索/Mistral帖也失败。不升级客户端（本轮仅可写本期文件），不自动读取浏览器Cookie。精确时间见d-twitter-records.jsonl。
- 约00:56：Reddit评论永久链接单独打开cache miss；原帖缓存可读，但得分不暴露，无法验收“高赞”。
- Agent Reach check-update：v1.5.0为最新；未更新任何工具。

## 截图QA

截图来自无登录态原站Chrome；CSS宽700、DPR2、物理宽1400；没有重绘/修改原文/数字，没有转载图。图片命名为NN-d-英文名，NN均在D组25–34区间。截图只有原站内容，不带扩展浮标或抓取者导航身份。

| 文件 | 判定 |
|---|---|
| 27-d-mistral-opening.png | 可用正文局部，1400×1620；含preview、权重时点、Frontier performance标题与完整前后段；上沿留白较大。主标题另看context全截图。 |
| 27-d-mistral-gpus-clean.png | 可用，1400×790；标题与GPU段、完整后文均在图内。 |
| 28-d-mistral-prices.png | 可用但有明确范围，1400×1440；预览/1.05T卡片及低价完整；移动布局没有旧价划线和输入/缓存/输出文字标签，不能声称此图独立证明三种计费角色。 |
| 29-d-aa-summary.png | 可用，1400×1360；主标题、Proprietary、全部四张摘要卡、排名/指标名/价格单位完整。 |
| 27-d-mistral-gpus.png | **不使用**，1400×940；下沿切进第三段，按不删除要求保留，clean版替代。 |
| 27-d-mistral-title-opening.png | **不使用**，1400×3820；上沿切入hero文字，另补context版。 |
| 27-d-mistral-context.png | 已目视核验，可用上下文备用，1400×5120；原站主标题/hero与开头完整段落，大幅留白为原站布局，未移除或重排。此窄版没有可见日期，日期依文字存档核验。 |
| 32-d-liquid-method.png | **不使用**，1400×600；图片加载造成坐标漂移，方法段开头被截，按不删除规则保留。 |
| 32-d-liquid-method-clean.png | 已目视核验，可用，1400×562；改为当次DOM按Methodology段落定位，包含once per model与整段测试条件，不再复用旧快照坐标。 |

25–26未生成：未找到新的重置原帖。30–31未生成：路透正文未取得。上述缺口不拿本地排版、转载截图或其他媒体替代。

## 交付边界

图片和HTML原件不入库；本组没有做网盘上传或offsite.tsv（统一由后续发布流程处理）。所有新增文件及大小、SHA-256列入d-manifest.md，HTML原件大于1MB不等于拟提交文件大于1MB；未提交任何文件。其他组同时写本期目录，终检按D组文件前缀认领，不把整个1006目录都算本组。

## 终检记录

- 22行D1–D4说法均使用四种允许状态；引用的图片全部存在，所有9张图片宽1400，含6张可用/上下文备用、3张失败保留图。没有捏造25–26、30–31。
- 抽核9项关键原句；首次脚本将AA的FAQ句错误地检查于未展开FAQ的浏览器文本，失败后定位到原始HTTP的d-aa.txt，修正验证来源，未修改原句或伪造展开结果。最终机器结果在d-validation.json。
- 对整个1006的文本/JSON/HTML/日志完成rg隐私关键词检查，未命中抓取者路径、Develata、Bearer凭据或明文认证模式。另用本机已配置Twitter两项凭据做精确匹配，仅记录泄漏文件名，结果为空；不输出凭据。D组无登录态全页存档，社交只有官方oEmbed与公开评论摘录。
- git diff --check无错误；git status仅原有派工单、1005的c-repo-tree.json和本期新增目录，未改已跟踪文件。A/B组同期文件只观察、未修改，不能声称本期目录全部归D组。
- D组拟入库（未被Git忽略）的文件没有超过1MB。AA及Mistral文档HTML和若干图片大于1MB，但均被既有忽略规则排除；本次不提交也不上传。逐个大小/SHA-256见d-manifest.md。
- 25/26重置原帖、30/31路透图、官方折扣期限、高赞评论得分等缺口仍然存在，不能宣布通用验收全部通过。D组结果为已交付、有明确未取得项。
