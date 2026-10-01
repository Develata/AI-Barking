# 0927 抓取日志

本轮实际HEAD为892c8c2。开始时已有未跟踪.handoff/2026-09-27-0927-evidence.md，未读写此文件。仅新增docs/0927/sources和images内材料；无commit、push、安装、删除或ZIP生成。

浏览器：OpenCLI 1.8.7、扩展1.0.24；doctor通过；agent-reach 1.5.0。Chrome既有会话，未登录、未输入凭据。Zenodo只点击Accept only essential cookies。网页使用浅色原站、100%缩放；OpenAI现场visualViewport.scale=1。截图视口宽1264，裁剪不缩放；正文CSS：DNS14.6667px、Anthropic17px、TechCrunch19px、OpenAI17px。Zenodo页面原生字体约14px。没有重绘、改字、调色、注释或拼接正文HTML。裁剪/竖向拼接使用System.Drawing，分隔线2px灰；复现脚本b-crop-candidates.ps1。

表中北京时间是文件保存完成时间（文件系统UTC时间+8），不是服务器发布时间。页面请求起点约01:24，主要正文抓取01:25–01:36；后续报告生成到约01:42。精确截图来源时点以原始PNG条目为准，images时间是裁剪生成时间。DOM HTML去除了openaicom-did运行时跟踪标识值，替换为[redacted-runtime-id]，未改正文。没有保存Cookie或浏览器会话状态。

## 页面及原始采集文件

HTML由curl直接原站抓取；MD为OpenCLI browser eval读取innerText，未改写，不是翻译；动态OpenAI汇总页首次被站点重定向到zh-Hans-CN，随后使用原站en-US入口重定向回英文canonical。浏览器HTML为动态DOM正文存档。sources中的完整PNG是原始取证底片，可能有导航、浮窗或Cookie，仅images内通过验收者作为候选配图。

| 文件 | 原站URL | 北京保存时间 | 工具/结果 |
|---|---|---|---|
| a-build-log.ps1 | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:41:32 | 本地裁剪/日志复现脚本 |
| a-evidence-records.json | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:36:55 | 本地18项逐字引用校验输入 |
| a-hn-49563355.md | https://news.ycombinator.com/item?id=49563355 | 2026-09-27 01:29:01 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-openai-53-foreground.png | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:39:04 | OpenCLI前台重试；时间线仍渐淡，不合格底片 |
| a-openai-53-visible.png | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:33:00 | OpenCLI viewport screenshot；OpenAI时间线文字渐淡，不合格底片 |
| a-openai-dns-full.png | https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/ | 2026-09-27 01:26:28 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-openai-dns-report.html | https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/ | 2026-09-27 01:25:00 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| a-openai-dns-report.md | https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/ | 2026-09-27 01:26:27 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-openai-hugging-face-detail.html | https://openai.com/index/hugging-face-incident-and-the-road-ahead/ | 2026-09-27 01:32:58 | curl HTTP403，反爬壳；成功正文另存MD |
| a-openai-hugging-face-detail.md | https://openai.com/index/hugging-face-incident-and-the-road-ahead/ | 2026-09-27 01:29:53 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-openai-third-parties-browser.html | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:39:01 | OpenCLI动态DOM；已脱敏运行时跟踪ID |
| a-openai-third-parties-en-full.png | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:28:10 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-openai-third-parties-en.md | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:27:51 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-openai-third-parties-full.png | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:27:10 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-openai-third-parties.html | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:24:57 | curl HTTP403，反爬壳；成功正文另存MD与browser.html |
| a-openai-third-parties.md | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:27:07 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-openai-third-visible.png | https://openai.com/hugging-face-incident-and-misalignment/ | 2026-09-27 01:32:56 | OpenCLI viewport screenshot；OpenAI时间线文字渐淡，不合格底片 |
| a-overclaim-taisounds-full.png | https://www.taisounds.com/news/content/84/290752 | 2026-09-27 01:30:51 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-overclaim-taisounds.html | https://www.taisounds.com/news/content/84/290752 | 2026-09-27 01:33:01 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| a-overclaim-taisounds.md | https://www.taisounds.com/news/content/84/290752 | 2026-09-27 01:30:50 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-overclaim-tnw-full.png | https://thenextweb.com/news/openai-sandbox-agent-ai-kill-switch | 2026-09-27 01:32:13 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-overclaim-tnw.html | https://thenextweb.com/news/openai-sandbox-agent-ai-kill-switch | 2026-09-27 01:30:46 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| a-overclaim-tnw.md | https://thenextweb.com/news/openai-sandbox-agent-ai-kill-switch | 2026-09-27 01:32:12 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-search-escape.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:29:07 | Exa经mcporter；检索结果仅线索，非原站正文 |
| a-search-overclaim.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:27:07 | Exa经mcporter；检索结果仅线索，非原站正文 |
| a-search-reuters.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:27:08 | Exa经mcporter；检索结果仅线索，非原站正文 |
| a-techcrunch-53-images.html | https://techcrunch.com/2026/09/25/unsecured-openai-agents-posted-53-user-images-on-the-internet-without-the-labs-knowledge/ | 2026-09-27 01:25:00 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| a-techcrunch-53-images.md | https://techcrunch.com/2026/09/25/unsecured-openai-agents-posted-53-user-images-on-the-internet-without-the-labs-knowledge/ | 2026-09-27 01:28:29 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| a-techcrunch-full.png | https://techcrunch.com/2026/09/25/unsecured-openai-agents-posted-53-user-images-on-the-internet-without-the-labs-knowledge/ | 2026-09-27 01:28:30 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| a-validation.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:41:57 | 本地取证辅助文件 |
| b-anthropic-full.png | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | 2026-09-27 01:25:11 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| b-anthropic-nine-loops.html | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | 2026-09-27 01:24:58 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-anthropic-nine-loops.md | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | 2026-09-27 01:25:09 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-arxiv-search.html | https://arxiv.org/search/?query=%22nine%22+%22Song+He%22&searchtype=all | 2026-09-27 01:30:46 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-crop-candidates.ps1 | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:35:00 | 本地裁剪/日志复现脚本 |
| b-hn-49848033.md | https://news.ycombinator.com/item?id=49848033 | 2026-09-27 01:29:09 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-inspire-search.html | https://inspirehep.net/api/literature?q=a%20Song.He.1%20and%20nine&size=10 | 2026-09-27 01:30:50 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-overclaim-36kr-full.png | https://www.36kr.com/p/3999414374174598 | 2026-09-27 01:30:12 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| b-overclaim-36kr.html | https://www.36kr.com/p/3999414374174598 | 2026-09-27 01:33:05 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-overclaim-36kr.md | https://www.36kr.com/p/3999414374174598 | 2026-09-27 01:30:11 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-overclaim-blockchain-full.png | https://blockchain.news/ainews/claude3-solves-nine-loops-breakthrough-analysis | 2026-09-27 01:32:23 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| b-overclaim-blockchain.html | https://blockchain.news/ainews/claude3-solves-nine-loops-breakthrough-analysis | 2026-09-27 01:30:47 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-overclaim-blockchain.md | https://blockchain.news/ainews/claude3-solves-nine-loops-breakthrough-analysis | 2026-09-27 01:32:22 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-results-method-and-validation.md | https://smsharma.io/cosmic-nine-loops/validation/method_and_validation.md | 2026-09-27 01:34:15 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-results.html | https://smsharma.io/cosmic-nine-loops/ | 2026-09-27 01:30:48 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-results.md | https://smsharma.io/cosmic-nine-loops/ | 2026-09-27 01:34:11 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| b-search-inspire.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:29:19 | Exa经mcporter；检索结果仅线索，非原站正文 |
| b-search-jiqizhixin.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:29:13 | Exa经mcporter；检索结果仅线索，非原站正文 |
| b-search-overclaim.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:27:08 | Exa经mcporter；检索结果仅线索，非原站正文 |
| b-search-preprint.txt | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:27:09 | Exa经mcporter；检索结果仅线索，非原站正文 |
| b-song-he-full.png | https://zenodo.org/records/22800071 | 2026-09-27 01:26:01 | OpenCLI browser screenshot --full-page --width1264；底片非交付成品 |
| b-song-he-zenodo.html | https://zenodo.org/records/22800071 | 2026-09-27 01:26:00 | curl HTTP200；INSPIRE文件内容实际为JSON；arXiv为搜索页 |
| b-song-he-zenodo.md | https://zenodo.org/records/22800071 | 2026-09-27 01:25:45 | OpenCLI浏览器innerText，可读正文；含页面原生导航时照存 |
| evidence.md | 本地派生/搜索记录；见下文检索范围 | 2026-09-27 01:38:32 | 本地事实清单，无发布正文 |

## 候选截图与裁剪

坐标均为对应sources底片的像素(x,y,width,height)。每张图都已打开检查；不合格项明确保留而不删除。01–08及四张10候选完成；09因未找到arXiv预印本，改交真正找到的Zenodo Dataset并如实改名；这不代表完成了arXiv摘要截图。

| 截图 | 原站URL | 源文件与裁剪/拼接 | 结果 |
|---|---|---|---|
| 01-openai-dns-pause.png | https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/ | a-openai-dns-full.png：(285,120,660,525) | 通过：标题、三个日期、暂停范围 |
| 02-openai-dns-timeline.png | https://alignment.openai.com/misalignment-reports/an-agent-used-dns-to-reach-an-external-chatbot/ | a-openai-dns-full.png：(285,3710,660,640) | 通过：逐秒时间线、operational gaps |
| 03-openai-third-parties.png | https://openai.com/hugging-face-incident-and-misalignment/ | a-openai-third-parties-en-full.png：(215,140,820,205)+(215,1978,820,155)，2px分隔 | 通过：标题和dozens段落 |
| 03b-openai-53-official.png | https://openai.com/hugging-face-incident-and-misalignment/ | a-openai-third-parties-en-full.png：(240,3300,780,735) | 不合格，勿用：官方时间线渐淡渲染，正文无法清楚辨认。前台/滚动复拍仍然如此，原因未确认；没有改CSS或调色。证据以MD/HTML为准 |
| 04-techcrunch-53-images.png | https://techcrunch.com/2026/09/25/unsecured-openai-agents-posted-53-user-images-on-the-internet-without-the-labs-knowledge/ | a-techcrunch-full.png：(630,620,620,330)+(105,1190,705,375)，2px分隔，窄块右侧白色留边 | 通过；原站标题块绿色，是网站原色而非暗色主题，未调色 |
| 05-anthropic-toy-model.png | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | b-anthropic-full.png：(235,150,780,155)+(235,2985,780,190)，2px分隔 | 通过：标题、日期、非现实模型原话 |
| 06-anthropic-known-methods.png | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | b-anthropic-full.png：(290,5180,670,220) | 通过 |
| 07-anthropic-cost.png | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | b-anthropic-full.png：(290,4545,670,250) | 通过；移除裁剪边缘的下一段残行 |
| 08-anthropic-song-he.png | https://www.anthropic.com/research/yes-claude-can-do-nine-loops | b-anthropic-full.png：(290,4790,670,270)+(290,8335,670,190)，2px分隔 | 通过；两位署名者相关段落分开拼接 |
| 09-song-he-dataset.png | https://zenodo.org/records/22800071 | b-song-he-full.png：(250,100,750,320) | Dataset来源截图通过：标题、作者、日期、Introduction；arXiv预印本截图未完成，因为未找到对应预印本 |
| 10-overclaim-a-taisounds.png | https://www.taisounds.com/news/content/84/290752 | a-overclaim-taisounds-full.png：(0,235,940,260) | 通过：标题、时间、作者 |
| 10-overclaim-a-tnw.png | https://thenextweb.com/news/openai-sandbox-agent-ai-kill-switch | a-overclaim-tnw-full.png：(15,170,1220,300) | 通过：标题、日期；UTC在正文页尾，见MD |
| 10-overclaim-b-36kr.png | https://www.36kr.com/p/3999414374174598 | b-overclaim-36kr-full.png：(180,55,730,220) | 通过：标题、日期、署名；时区显示差异见evidence |
| 10-overclaim-b-blockchain.png | https://blockchain.news/ainews/claude3-solves-nine-loops-breakthrough-analysis | b-overclaim-blockchain-full.png：(100,305,785,175) | 通过：标题、UTC时间 |

## 检索范围、失败及未验证事项

- web搜索初次直开Anthropic、TechCrunch返回Internal Error，但同一原站curl与浏览器成功，不是用转载替代。
- OpenAI汇总页、HF详细页curl HTTP403；浏览器同站成功，逐字正文另存MD。汇总页web读取不含动态时间线；不能据此称官方没写53。
- arXiv原站查询all:"nine" "Song He"仅1条2014年1412.5606；INSPIRE API查询a Song.He.1 and nine返回total=0。Exa额外查询site:arxiv.org "nine" "Song He" 2026和site:inspirehep.net nine loops Song He Jirong Jing，未找到本次对应论文。检索非穷尽，不能断言没有论文。
- 何颂数据集通过Anthropic原始链接doi.org/10.5281/zenodo.22800071到Zenodo；原始描述页成功，未下载数据包，未检查包内README。GPT-6团队自述的缺口仅针对本轮所检查材料。
- Reuters：web与Exa查询site:reuters.com OpenAI 53 images September 2026及同义查询，只定位到9/5、9/9、9/11相关事故链报道，未找到53图片原站报道。不用其他媒体代替Reuters。
- A8/B10：Exa查询OpenAI 逃逸 停训 53 图片 2026 9月、OpenAI DNS agent escaped all training stopped September 26 2026、Claude 九环 理论物理 突破 机器之心；web补充英文breakthrough与中文关键词。36氪实际原页成功。对jiqizhixin.com站内定向搜索未找到该篇自站文章；不把搜索出的无关页面纳入实例。未逐一查询微博/知乎/公众号登录态搜索，中文媒体取证以36氪和太報为边界。
- HN原站浏览器成功；自动审批拒绝了直接curl Firebase API命令，未取得API JSON，也未安装工具。另有Python依赖探测及方法页curl被自动审批拒绝（仅给approval required by policy，无细分理由）；裁剪改用现有PowerShell System.Drawing，方法页通过原站浏览器正常读取。没有请求凭据、规避站点控制或修改审批策略。
- 初次裁剪脚本因PowerShell单矩形数组展开报参数错误，已修正并重跑，所有最终候选已打开检查；无删除。
- 未把检索结果摘要、浏览器原文与事实校验视为数学复算；未跑九环代码，未下载大型结果文件。
- 本轮参考旧记忆仅为截图验收流程，事实全部来自本轮原站与明确标出的0925历史材料。没有修改记忆。

## 截图生成时间与尺寸

| 文件 | 北京保存时间 | 尺寸 |
|---|---|---|
| 01-openai-dns-pause.png | 2026-09-27 01:35:01 | 660 × 525 |
| 02-openai-dns-timeline.png | 2026-09-27 01:35:01 | 660 × 640 |
| 03-openai-third-parties.png | 2026-09-27 01:35:01 | 820 × 362 |
| 03b-openai-53-official.png | 2026-09-27 01:35:01 | 780 × 735 |
| 04-techcrunch-53-images.png | 2026-09-27 01:35:01 | 705 × 707 |
| 05-anthropic-toy-model.png | 2026-09-27 01:35:01 | 780 × 347 |
| 06-anthropic-known-methods.png | 2026-09-27 01:35:01 | 670 × 220 |
| 07-anthropic-cost.png | 2026-09-27 01:35:01 | 670 × 250 |
| 08-anthropic-song-he.png | 2026-09-27 01:35:01 | 670 × 462 |
| 09-song-he-dataset.png | 2026-09-27 01:35:01 | 750 × 320 |
| 10-overclaim-a-taisounds.png | 2026-09-27 01:35:01 | 940 × 260 |
| 10-overclaim-a-tnw.png | 2026-09-27 01:35:01 | 1220 × 300 |
| 10-overclaim-b-36kr.png | 2026-09-27 01:35:01 | 730 × 220 |
| 10-overclaim-b-blockchain.png | 2026-09-27 01:35:01 | 785 × 175 |
