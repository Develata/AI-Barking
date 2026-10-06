# 1005 B组抓取日志

工作区基线ac064e681343b7ce6401448a250b1ab2f01d4c63。开始已存在EDITORIAL.md、daily-scan.md修改、两份handoff与1004未跟踪材料；不归本组。本组只新增1005下B文件，未commit/push/删除。

浏览器：OpenCLI 1.8.7，session固定1005-b，doctor扩展连接正常。使用已有授权会话；未登录、未提交表单、未同意非必要Cookie、未发帖互动。社交DOM仅采公开正文、分数、时间与链接；抓取者账号栏/头像不写盘。公开HN API无需Cookie。

## 原站浏览器采集

| 开始时间（北京时间UTC+8） | URL | 工具 | 结果与文件 |
|---|---|---|---|
| 2026-10-06T09:14:43.502+08:00 | https://openai.com/index/eu-text-provenance/ | opencli browser 1005-b open/eval | Our approach to EU text provenance rules | OpenAI；10996；b-openai.json |
| 2026-10-06T09:16:01.740+08:00 | https://help.openai.com/en/articles/8912793-provenance-signals-in-openai-generated-content | opencli browser 1005-b open/eval | Provenance signals in OpenAI-generated content | OpenAI Help Center；19499；b-help.json |
| 2026-10-06T09:16:12.029+08:00 | https://www.anthropic.com/news/claude-text-watermark | opencli browser 1005-b open/eval | How Claude's text watermarking works \ Anthropic；15913；b-anthropic.json |
| 2026-10-06T09:16:20.272+08:00 | https://deepmind.google/models/synthid/ | opencli browser 1005-b open/eval | SynthID — Google DeepMind；2252；b-google.json |
| 2026-10-06T09:16:36.889+08:00 | https://digital-strategy.ec.europa.eu/en/policies/code-practice-ai-generated-content | opencli browser 1005-b open/eval | Code of Practice on Transparency of AI-generated Content | Shaping Europe’s digital future；5124；b-eu-code.json |
| 2026-10-06T09:17:19.284+08:00 | https://support.claude.com/en/articles/16266773-how-claude-marks-ai-generated-content | opencli browser 1005-b open/eval | How Claude marks AI-generated content | Claude Help Center；9211；b-anthropic-help.json |
| 2026-10-06T09:17:39.550+08:00 | https://deepmind.google/blog/watermarking-ai-generated-text-and-video-with-synthid/ | opencli browser 1005-b open/eval | Watermarking AI-generated text and video with SynthID — Google DeepMind；8777；b-google-text.json |
| 2026-10-06T09:17:51.089+08:00 | https://techcrunch.com/2026/10/05/openai-will-start-watermarking-chatgpts-text-in-the-eu/ | opencli browser 1005-b open/eval | OpenAI will start watermarking ChatGPT's text in the EU | TechCrunch；5687；b-techcrunch.json |
| 2026-10-06T09:18:14.208+08:00 | https://www.theverge.com/ai-artificial-intelligence/1004880/openai-chatgpt-text-watermarks-eu-ai-act | opencli browser 1005-b open/eval | OpenAI is adding text watermarking in ChatGPT and Codex | The Verge；4885；b-verge.json |
| 2026-10-06T09:18:35.732+08:00 | https://www.ithome.com/1/009/903.htm | opencli browser 1005-b open/eval | OpenAI 将在欧盟为 ChatGPT 和 Codex 文本输出添加隐形水印 - IT之家；3025；b-ithome.json |
| 2026-10-06T09:18:53.103+08:00 | https://aihot.news/story/b92e615b-0baf-4821-a3ac-e621dde3db2c | opencli browser 1005-b open/eval | OpenAI公布欧盟文本溯源水印方案 · AIHOT；3267；b-aihot.json |
| 2026-10-06T09:19:15.289+08:00 | https://news.ycombinator.com/item?id=49968716 | opencli browser 1005-b targeted public DOM | OpenAI 的 TextGrain 根据《欧盟人工智能法案》为 ChatGPT 文本添加水印 | Hacker News --- OpenAI TextGrain Watermarks ChatGPT Text Under EU AI Act | Hacker News；定向字段；b-hn.json |
| 2026-10-06T09:19:22.584+08:00 | https://old.reddit.com/r/ChatGPT/comments/1wymu43/i_read_openais_whole_post_about_watermarking/ | opencli browser 1005-b targeted public DOM | I read OpenAI's whole post about watermarking ChatGPT text. Here is what it says and what it leaves out : ChatGPT；定向字段；b-reddit.json |
| 2026-10-06T09:19:33.734+08:00 | https://old.reddit.com/search/?q=textGrain&sort=top&t=week | opencli browser 1005-b targeted public DOM | reddit.com: search results - textGrain；定向字段；b-search.json |
| 2026-10-06T09:19:43.332+08:00 | https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-50 | opencli browser 1005-b open/eval | Article 50: Transparency obligations for providers and deployers of certain AI systems | AI Act Service Desk；6727；b-eu-article50.json |
| 2026-10-06T09:19:56.462+08:00 | https://www.ic.work/article/openai-releases-textgrain-text-watermarking | opencli browser 1005-b open/eval | OpenAI 推出文本水印 textGrain，改写 25% 即失效背后的欧盟合规账本 - ic.work；3848；b-overclaim-cn.json |
| 2026-10-06T09:20:05.878+08:00 | https://interestana.com/articles/openai-is-adding-text-watermarking-in-chatgpt-and-codex-009tct91 | opencli browser 1005-b open/eval | OpenAI Rolls Out Text Watermarking in ChatGPT and Codex | Interestana；4270；b-overclaim-en.json |
| 2026-10-06T09:20:58.435+08:00 | https://gizmodo.com/openai-is-adding-text-watermarks-in-the-eu-because-regulation-works-2000821852 | opencli browser 1005-b open/eval | OpenAI Is Adding Text Watermarks in the EU Because Regulation Works；7388；b-gizmodo.json |
| 2026-10-06T09:21:19.973+08:00 | https://aihot.news/items/xx5mgdemrqmqw5zw410sbawcz | opencli browser 1005-b open/eval | OpenAI 公布 EU AI Act 下的文本溯源方案，推出 textGrain 文本水印 · AIHOT；3747；b-aihot-item.json |

上述“成功”仅指取得正文/字段；b-search.json虽有数据，但Reddit textGrain站内搜索返回大量不相关条目，判为检索质量失败，不作覆盖证据。Google模型页轮播文字不全，已另存2024官方文本水印公告补证。

## 其他抓取、失败与范围

| 北京时间 | URL/检索 | 工具与结果 |
|---|---|---|
| 10-06约09:04–09:08 | https://openai.com/index/eu-text-provenance/ | 首次open后eval返回空title/text；第二次open并等加载后成功，见正式快照。未以空响应当存档。 |
| 10-06约09:07 | Exa：textGrain OpenAI October 5 2026 watermark | agent-reach路由mcporter；免费MCP额度耗尽，无结果。未配置新凭据或升级。 |
| 10-06约09:08 | https://cdn.openai.com/pdf/e9508624-d767-41b6-a26d-e34ca798ada6/textgrain-entropy-calibrated-watermarking-for-language-model-text.pdf | curl下载到b-textgrain.pdf的命令被自动审批拒绝：approval required by policy, but AskForApproval is set to Never。没有下载文件；未改工具规避该拒绝。 |
| 10-06约09:08–09:19 | 同上PDF | web.open读取到20页/1013行；读取p.1作者与日期、p.6检测校准；find ELI5无命中。find对子串的结果可能不可靠：paraphras无命中但参考文献可见Paraphrasing，故不以一次find证明全文不存在。未获得本地PDF、pdftotext和pdftoppm产物；web截图调用返回引用但未形成可交付本地图片。 |
| 10-06约09:12–09:16 | https://news.ycombinator.com/item?id=49968716 ; https://hn.algolia.com/api/v1/items/49968716 | web工具打开失败；后用规定浏览器读取HN页成功。 |
| 10-06约09:16 | https://eur-lex.europa.eu/eli/reg/2024/1689/oj/eng | web读取超时；改读EU Code政策页所链官方AI Act Service Desk Article 50，成功。 |
| 10-06约09:18 | https://ec.europa.eu/newsroom/dae/redirection/document/129555 | web读取失败；未读到Code PDF，不把OpenAI帮助中心对Code的转述当EU原文。 |
| 10-06约09:15 | https://www.theverge.com/ai-artificial-intelligence/1004880/openai-chatgpt-text-watermarks-eu-ai-act | web工具失败；浏览器原站成功，元数据时间已核。 |
| 2026-10-06T09:20:06.5575053+08:00 | https://hn.algolia.com/api/v1/search?query=eu-text-provenance&tags=story ; https://hn.algolia.com/api/v1/items/49966293 | PowerShell Invoke-RestMethod公开API，存b-hn-search.json、b-hn-main.json；63分/52评论，评论points=null。 |

web搜索查询组合：textGrain/OpenAI官方；Anthropic watermark August 2（anthropic.com、support.claude.com）；SynthID Gemini detector（deepmind.google）；EU Article50/code practice（EU官方）；textGrain + TechCrunch/The Verge/Reuters/IT之家/Reddit/AIHOT；中文“全球”“一键检测”；英文“all text”。搜索只是定位，实际事实优先以上原站存档。未找到Reuters或指定全球/公开作业检测器说法的可靠实例；不等于不存在。搜索结果的转载域名未用作原文替代。

agent-reach doctor未实测Reddit登录态。为遵守固定1005-b，不运行会另建adapter session的reddit search命令；使用agent-reach所列OpenCLI浏览器后端读取公开旧版页面。未进行X登录/搜索或跨平台总体抽样。Agent Reach check-update显示v1.5.0已是最新，未升级。

## 截图与验收

10–13为原站CDP Page.captureScreenshot，700CSS px、DPR2，文件宽1400px；不重绘或改数字。第一次视觉核验发现11标题/13末句裁切，已扩大边界重截，保留更多上下文。12含八行完整表、两列标头和条件；11含两图的标题、坐标、图例与图注。14未交付，原因见PDF拒绝。图片均在本组区间，正式编辑采用前仍可按需要排版。

文件清单与SHA256见b-files.tsv（文件本身及最终验收文件不循环自哈希）。本地图片是忽略文件，不提交。未生成ZIP和发布正文。隐私检查与结构核验结果见b-validation.json。
