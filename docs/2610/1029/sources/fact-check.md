# 1029 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`～`evidence-d.md`，截图在 `images/`。

## 省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 编程软件 Cursor 的插件市场混进被篡改的 Claude Code 插件，会往电脑埋后门，Cursor 称最多5000人受影响 | L1 | ✅ | 见 A 节 | “埋后门”＝“attempts to install a persistent SSH backdoor”；5000＝“At most 5,000 users were affected”（上限）。“插件”＝extension 的通俗说法。 |
| “换种语言用 AI 贵17倍”只出自 Falcon-3 一个模型，多数模型里最省的就是英文或简体中文 | L3 | ✅ | 见 B 节 | 17.0x 仅表 1 Falcon 3 (TII) 一行；表 2 脚注 “English or Mandarin Chinese (Simplified) in most cases”（b-report-rendered.txt）。 |

## A Cursor 插件投毒

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Cursor 称，北京时间29日下午到30日凌晨，它插件市场里的 Claude Code 插件2.1.313版遭恶意篡改 | L1 | ✅ | https://cursor.com/blog/claude-code-extension-security-notice ；a-cursor-blog.txt；图 01、02、06 | “Between about 2:27am PT and 1:35pm PT today”；处置段 “on October 29, 2026”；PDT（UTC-7）→ 北京 10-29 17:27 至 10-30 04:35（“about”为原词，正文写“下午到凌晨”）。“a maliciously modified version … was made available in the Cursor VS Code extension Marketplace”。博文只有日期 Oct 29, 2026。 |
| 不是 Anthropic 开发的 | L1 | ✅ | 同上；图 01 | “This malicious version of the extension was not developed by Anthropic.” |
| 装上后，它会替攻击者在电脑上留一把远程登录钥匙、装自启脚本，并把电脑信息发出去 | L1 | ✅ | 同上；图 03、04 | 钥匙＝“attacker-controlled SSH key to authorized_keys”；自启脚本＝持久化 “via a LaunchAgent (macOS) or a scheduled task … with a Startup-script fallback (Windows)”；信息＝“Reports host details … via HTTP POST to /beacon on port 8099”（hostname、username、OS、public IP 等）。“still adds the attacker’s SSH key, installs persistence, and sends host details”。 |
| Cursor 已下架这个版本，攻击者服务器也已下线 | L1 | ✅ | 同上；图 04、05 | “Removed the malicious version from the marketplace immediately upon detection and blocked further distribution.”；“Note: The attacker server no longer functions. We worked with the cloud provider that was hosting it to take it offline, so the reverse shell and host detail POST are not functional.” 下线时刻未给。 |
| 受影响用户要查钥匙、用官方脚本清理、换掉密码和密钥 | L1 | ✅ | 同上；图 06、07 | 处置五步：“Run the check below”（查 svc-telemetry 公钥）；“Remove … using the remediation scripts below”；“Rotate credentials and sessions accessible from affected hosts, from a trusted system.”（“密码和密钥”为 credentials 的通俗说法）。对象为 10-29 该时段从 Cursor 市场安装或更新到 2.1.313 的用户。 |
| 只卸载插件可能不够 | L1 | ✅ | 同上；图 06 | “Uninstalling the extension may not be sufficient — the payload drops standalone persistence scripts that keep running.” |
| ①有安全研究者的博文标题写“Anthropic 被黑” | L1（研究者本人博客，核的是其标题文字） | ✅ | https://doublepulsar.com/anthropic-hacked-to-deliver-remote-access-trojan-to-claude-developer-laptops-worldwide-1148a92baae2 ；a-doublepulsar.txt；图 12b（v2） | Kevin Beaumont 标题 “Anthropic hacked to deliver remote access trojan to Claude developer laptops worldwide”；datePublished 北京 10-30 18:08；HN 50301983 393 分。正文未点名（字数）。 |
| ①Anthropic 员工 Felix Rieseberg 称公司系统没被攻破，官方渠道不受影响 | L2 | ✅ | https://news.ycombinator.com/item?id=50304129 ；a-hn-hn-50300683.json；身份 a-felix-gh-user.json、a-felix-hn-user.json；批注图 felix-hn | “Felix from Anthropic here. … it wasn’t a compromise of Anthropic’s systems or employee laptops. … The versions on our official channels are not affected and this only impacts Cursor users.” 北京 10-30 22:08。身份：GitHub 账号 felixrieseberg 资料 company “Anthropic”、主页 felixrieseberg.com，与 HN 账号 about 栏同一主页（Claude 北京 10-31 查）。同帖另有自称 Anthropic 员工 ddworken 称 “It did not impact the official VS Code extension marketplace or OpenVSX”（未核身份，未进正文）。npm、Open VSX、GitHub releases 均无 2.1.313（a-npm-times.json 等），与此相容。 |
| ①恶意版本怎么上的架，Cursor 没说 | L1 | ✅ | a-cursor-blog.txt、a-cursor-blog-cn.txt、a-cursor-x-fx.json | 已读范围：博文英文、中文版与 Cursor 官方帖，均未写入侵方式，只写 “rotated our keys, rebuilt the marketplace”。BleepingComputer 同样称 Cursor “has not explained how attackers distributed the compromised package under Anthropic's name”（L4，未进正文）。 |
| ②5000人是 Cursor 给的上限，没说按安装还是下载算，也没说多少台电脑真中招 | L1 | ✅ | a-cursor-blog.txt、a-cursor-x-fx.json；图 01、08 | 博文 “At most 5,000 users were affected”，官方帖 “impacting at most 5,000 users”；两处均未定义 users，未给执行恶意代码的主机数（已读：博文中英文、官方帖）。“真中招”＝实际执行了恶意代码。扫描里的“4100”在所有已存来源中找不到。 |
| ③Cursor 说恶意代码出错，“这一步会失败”，又说多数 Mac 和 Windows 电脑上插件没权限开启远程登录，攻击者登不进来 | L1 | ✅ | a-cursor-blog.txt；图 03、04 | “Outside of highly unusual circumstances where Cursor was started with administrator privileges, due to errors in the malicious code, this step would fail.”；“turning on the local SSH service requires admin rights, which the malicious extension does not have on most macOS and Windows hosts. On those hosts the step fails and the attacker could not have logged in through the tunnel.” 第一句的 “this step” 在原文同一条里包括加公钥、开 SSH 服务、开反向隧道，后文又说加公钥仍成功——正文把两句并列照述，不替 Cursor 判断失败的是哪一环（第二轮反向核验 SHOULD_FIX 3 后改，删去“只是”）。“远程登录”＝本地 SSH 服务。 |
| ③但钥匙和脚本照样装上，服务器下线前电脑信息也照样外发 | L1 | ✅ | a-cursor-blog.txt；图 04 | “The extension still adds the attacker’s SSH key, installs persistence, and sends host details to the attacker.” 外发只在攻击者服务器在线时有效（“The attacker server no longer functions … host detail POST are not functional”），吠点句内写明“服务器下线前”（两轮反向核验 BLOCKER 2 后补）。 |

## B CLEAR 分词器税

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 大模型接口通常按 token 计费，token 是文字被切成的小块，同一句话切得越碎越贵 | L1 | ✅ | https://clear-languages.org/blog/the-tokenizer-tax ；b-report-rendered.txt；图 13 | “LLM providers typically charge by the token, irrespective of language.” token 定义为通俗解释（分词器把文本切成的单位）。 |
| 研究联盟 CLEAR 9月29日发报告 | L1 | ✅ | b-report-rendered.txt；图 13 | “Coalition for Language Equity in AI Research”；“September 29, 2026”，官方未给时刻；作者4人。成员见 evidence-b B1c，未见 TII 或资助方声明。 |
| 用约200种语言的同一批句子测19组模型 | L3 | ✅ | b-report-rendered.txt；图 14、16、20 | FLORES-200，“professionally translated into roughly 200 languages”，实际 203 个语言-文字对；表 1 共19行（部分行合并多个模型，如 Gemini 3.8 Flash / 3.1 Pro / Gemma 3 / 4），测的是各模型的分词器。 |
| 看每个模型里最费 token 的语言是最省的几倍 | L3 | ✅ | b-report-rendered.txt；图 14 | “max/min token count ratio, which measures how many times more tokens the most expensive language needs compared to the cheapest one for the same content”。 |
| 最差的 Falcon-3 是17倍，Gemini 一组5.1倍，GPT-5 一组13.8倍 | L3 | ✅ | b-report-rendered.txt；图 16；批注图 clear-table | 表 1：Falcon 3 (TII) 17.0x；Gemini 3.8 Flash / Gemini 3.1 Pro / Gemma 3 / Gemma 4 (Google) 5.1x；GPT-5 / gpt-oss (OpenAI) 13.8x。 |
| （正文已删）国内媒体30日转载时写“最高可相差17倍” | L1（核的是刊载文字本身，原站可查） | ✅ | https://www.ithome.com/1/020/026.htm （12:25:52）、https://tech.ifeng.com/c/8xdsdGObcbK （12:27）、https://www.163.com/dy/article/LAG5HJUK0511B8LM.html （12:30）；b-ithome 等；图 24 | 三篇同标题同文：“新研究称 AI 模型使用成本受不同语种影响，Token 消耗量最高可相差 17 倍”；正文写了“在其中一个 AI 模型中”。 |
| ①报告称多数模型里最省 token 的语言就是英文或简体中文 | L3 | ✅ | b-report-rendered.txt（表 2 脚注）；图 20；批注图 clear-cheapest | “The model min is the token count for the most efficiently tokenized language for that model, which is English or Mandarin Chinese (Simplified) in most cases.” 报告未单列中文相对英文的比值；第一轮 BLOCKER 1（旧稿“比的不是英文”）改为“说中文不算贵”，第二轮 SHOULD_FIX 4 指出推不出，删去该概括，只照述报告。 |
| ①吃亏最多的是缅甸文、藏文这类少见文字 | L3 | ✅ | b-report-rendered.txt；图 21；批注图 clear-cheapest | “The languages that are at the biggest disadvantage are written in scripts the tokenizer rarely saw: Burmese, Tibetan, Tifinagh, Khmer, Lao, and Ol Chiki. This pattern is consistent across nearly all models in our analysis.” |
| ②17倍只出自 Falcon-3 一个模型 | L3 | ✅ | b-report-rendered.txt 表 1；图 16 | 其余18行为 2.7–15.0x。 |
| ②开发方发布时列出的语言只有英、法、西、葡4种 | L1 | ✅ | https://www.tii.ae/news/falcon-3-uaes-technology-innovation-institute-launches-worlds-most-powerful-small-ai-models ；b-tii-falcon3.txt；批注图 falcon3-langs | “Falcon 3 is available in English, French, Spanish and Portuguese.”（TII 2024-12-17 发布稿）。原文无 only；正文写“列出的语言只有这4种”，不说其他语言不能处理（反向核验 SHOULD_FIX 4 后改）。报告本身没提这一点。 |
| ③只看网上最常见的20种语言，各模型里最费比最省只差1.5到3.1倍，唯一例外也是 Falcon-3 | L3 | ✅ | b-report-rendered.txt；图 15、16；批注图 clear-top20 | “For the world’s top 20 languages by web presence … max/min token ratios sit between 1.5x and 3.1x with only one exception.” 例外为读表所得：Falcon 3 “Top 20” 列 4.5x。“网上最常见”＝by web presence（反向核验 SHOULD_FIX 3 后改）。 |
| ④报告只数了 token，没测回答质量 | L3 | ✅ | b-report-rendered.txt；图 13 | 报告只给 token 计数比值；“A companion study on quality equity across languages is underway.” |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 这个恶意版本不是 Anthropic 开发的 | L1 | ✅ | a-cursor-blog.txt:97；底图 cursor-notice-raw | “This malicious version of the extension was not developed by Anthropic.” |
| 受影响用户最多5000名，我们会直接联系 | L1 | ✅ | a-cursor-blog.txt:97 | “At most 5,000 users were affected, and we will reach out directly.” 第二轮 SHOULD_FIX 5：删去括号里的编辑判断。 |
| 多数 Mac 和 Windows 电脑上，恶意插件没有开启远程登录所需的管理员权限 | L1 | ✅ | a-cursor-blog.txt；底图 cursor-still-works-raw | “turning on the local SSH service requires admin rights, which the malicious extension does not have on most macOS and Windows hosts”。批注图标题改为“恶意插件多数无权开远程登录”（版面限宽；Mac、Windows 范围在译注①）（第二轮 BLOCKER 2 删“其余照样成功”，视觉复核再改）。 |
| 插件照样加入攻击者的钥匙、装上常驻脚本，并把主机信息发给攻击者 | L1 | ✅ | 同上 | “The extension still adds the attacker’s SSH key, installs persistence, and sends host details to the attacker.” 逐句翻译；服务器下线一事在吠点卡③句内。 |
| 只卸载插件可能不够：恶意代码会留下独立运行的常驻脚本 | L1 | ✅ | a-cursor-blog.txt；底图 cursor-uninstall-raw | “Uninstalling the extension may not be sufficient — the payload drops standalone persistence scripts that keep running.” |
| 在可信的设备上，轮换受影响电脑能访问的凭据和登录会话 | L1 | ✅ | 同上 | “Rotate credentials and sessions accessible from affected hosts, from a trusted system.” |
| 这不是 Anthropic 的系统或员工电脑被攻破 | L2 | ✅ | a-hn-hn-50300683.json（评论 50304129）；底图 felix-hn-raw | “it wasn’t a compromise of Anthropic’s systems or employee laptops.” |
| 我们官方渠道上的版本不受影响 | L2 | ✅ | 同上 | “The versions on our official channels are not affected” |
| Falcon 3（TII）最大÷最小 token 数比：前20种语言4.5倍，FLORES-200 全部语言17.0倍；后者基尼系数0.34，CLEAR 评级 F | L3 | ✅ | b-report-rendered.txt 表1；底图 clear-table-raw | 逐格翻译该行；第二轮 SHOULD_FIX 5：删去“只出自这一行”的排他判断。底图为表1四行节选，图注已写明。 |
| 各模型最省 token 的语言，多数情况下是英文或简体中文 | L3 | ✅ | b-report-rendered.txt 表2注；底图 clear-cheapest-raw | “which is English or Mandarin Chinese (Simplified) in most cases.” |
| 最吃亏的语言用的是分词器很少见过的文字：缅甸文、藏文、提非纳文、高棉文、老挝文、Ol Chiki 字母（桑塔尔语用） | L3 | ✅ | b-report-rendered.txt；底图 clear-cheapest-raw | “The languages that are at the biggest disadvantage are written in scripts the tokenizer rarely saw: Burmese, Tibetan, Tifinagh, Khmer, Lao, and Ol Chiki.” 第二轮 SHOULD_FIX 7：Ol Chiki 是文字，用于桑塔尔语。 |
| 网上最常见的20种语言里，各模型的最大÷最小 token 比在1.5到3.1倍之间，只有一个例外 | L3 | ✅ | b-report-rendered.txt；底图 clear-top20-raw | “For the world’s top 20 languages by web presence … max/min token ratios sit between 1.5x and 3.1x with only one exception.” |
| Falcon 3 可用于英语、法语、西班牙语和葡萄牙语 | L1 | ✅ | b-tii-falcon3.txt；底图 falcon3-langs-raw | “Falcon 3 is available in English, French, Spanish and Portuguese.” |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Cursor 里的假 Claude 插件（主帖） | — | — | 见 A 节 | 逐字取正文小标题。 |
| 换种语言，AI 贵17倍？（主帖） | — | — | 见 B 节 | 逐字取正文小标题。 |
| OpenAI 更新模型行为规范，改按五项价值组织，把模型称作“GPT” | L1 | ✅ | https://openai.com/index/updated-model-spec ；https://model-spec.openai.com/2026-10-29.html ；https://github.com/openai/model_spec/blob/main/CHANGELOG.md ；d-d1-openai-spec.txt、d-d1-spec-changelog.txt | “reorganizes its guidance around five underlying values: faithfulness to human intent, harm minimization, honesty and transparency, human agency, and care”；“we now refer to the model as “GPT” rather than “the assistant” in the Spec”。RSS 北京 10-30 01:00。“行为规范”＝Model Spec（“our public description of how we want our models to behave”）。 |
| Google 称 Gemini 应用可在对话里改文件，PDF 外的格式限网页版 | L1 | ✅ | https://blog.google/innovation-and-ai/products/gemini-app/edit-files-directly/ ；c-c1-gemini-blog.txt | “PDF preview and editing is supported on both the web and mobile, while all other formats (Google Docs, Google Sheets, slide decks, Markdown files, TXT files, and code) are supported on the web only.” 10-29，官方未给时刻。 |
| 维基媒体申请递交意见书：ChatGPT 预训练属合理使用，检索时用副本不算 | L1 | ✅ | https://wikimediafoundation.org/news/2026/10/30/wikimedia-foundation-amicus-brief-new-york-times-v-open-ai-case/ ；https://commons.wikimedia.org/wiki/File:26.10.29_WMF_motion_and_amicus_In_re_OAI.pdf ；d-d2-amicus-pdf.txt | 动议函（10-29，案号 1:25-md-03143）“requests leave to file the [P]roposed Amicus Curiae Brief … in Support of Neither Party”（需法院批准）；结论 “while the pre-training of ChatGPT’s general LLM was fair use, OpenAI’s use of copies in ChatGPT’s RAG feature is not fair”。是维基媒体的主张，不是裁定；“检索时用副本”＝use of copies in RAG feature（反向核验 SHOULD_FIX 6 后改）。扫描与 The Verge 摘要的“复述原文”与意见书不符。 |
| Waymo 称将与丰田2027年在日本先期运营，测试 bZ4X 自动驾驶车队 | L1 | ✅ | https://waymo.com/blog/2026/10/toyota-bz4x-partnership ；c-c5-waymo-toyota.txt | “will begin initial operations to test and validate an autonomous Toyota bZ4X fleet in Japan in 2027”。10-30，官方未给时刻；未写城市、规模、安全员、是否载客。 |
| Alphabet 第三季度营收同比增25%，谷歌云收入增103% | L1 | ✅ | https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q3-2026/ ；c-c3-alphabet-q3-ceo.txt | “Alphabet revenues grew 25% year-over-year.”；“Google Cloud revenues grew 103%.”（同比口径见逐字稿，L4）。10-28 美东，官方未给时刻（窗口前）。AI Mode 变现说法只有第三方逐字稿，不收。 |
| Amazon 称第三季度 AWS 收入460亿美元，同比增39% | L1 | ✅ | https://ir.aboutamazon.com/news-release/news-release-details/2026/Amazon-com-Announces-Third-Quarter-Results/ ；d-d3-amazon-opencli.txt | “AWS segment sales increased 39% year-over-year to $46.0 billion.” 10-29，官方未给时刻。 |
| 据 TechCrunch 报道，Meta 在东南亚三国上架 AI 穿搭应用 | L4 | ⚠️ | https://techcrunch.com/2026/10/29/meta-quietly-launches-an-ai-fashion-app-called-style-sketch/ ；App Store 印尼区页面（L1，开发者 Meta Platforms, Inc.）；d-d4-* | “is available in Indonesia, Singapore, and Thailand on iOS and Android.” 三国范围只有 TechCrunch；App Store 只核了印尼区。北京 10-30 03:14。 |
| Martin Fowler 撰文，劝作者别让大模型写初稿 | L2 | ✅ | https://martinfowler.com/bliki/LLM-Voice.html ；d-d7-fowler.txt | “I strongly recommend that authors should not fall into the trap of getting the LLM to write a draft.” 10-29；HN 50290177 540 分／288 评（北京 10-30 晚查）。 |

## 反向核验

第一轮（WSL Codex gpt-6-astra，`barking review --reverse --run`，报告存于临时目录 `barking-review/1029/reverse-report.md`）审的是读者稿检查前的旧稿。逐条处理：

1. BLOCKER“比的不是英文”与来源不符：采纳。改为“说中文不算贵：报告称多数模型里最省 token 的语言就是英文或简体中文”，不再说分母不是英文。
2. BLOCKER“发信息照样成功”丢了服务器已下线：采纳。事实段补“攻击者服务器也已下线”。
3. SHOULD_FIX“前20种常用语言”丢口径：采纳，改“网上最常见的20种语言”。
4. SHOULD_FIX“只支持”强于原文：采纳，改“开发方发布时列出的语言只有英、法、西、葡4种”。
5. SHOULD_FIX“不是 Anthropic 被黑”先于署名：采纳，改为先写研究者标题、再写员工说法。
6. SHOULD_FIX 检索功能对象扩大：采纳，改“检索时用副本不算”。
7. SHOULD_FIX L4/L5 标 ✅：采纳。转载标题一行改为核刊载文字本身；正文删去“外媒写法”半句。
8. SHOULD_FIX 来源指针断链：采纳，B 节改为直接写 `b-report-rendered.txt`；Rest of World 存档实际文件名为 `b-rotw.txt`。
9. NICE_TO_HAVE 时间丢“约”：正文已改为“29日下午到30日凌晨”。
10. NEEDS_VERIFICATION 卡片未提供：本轮只审正文与清单；卡片随第二轮与视觉复核一并审。

第二轮（WSL Codex gpt-6-astra，报告 `barking-review/1029/reverse-report-2.md`，审读者稿检查后的新稿与卡片）。逐条处理：

1. BLOCKER 分词器税省流卡丢了“同一模型内、最费比最省”的口径：采纳，省流卡改用“研究联盟 CLEAR 9月29日发报告；看每个模型里最费 token 的语言是最省的几倍：……”；吠点③改“只看网上最常见的20种语言，各模型里最费比最省只差1.5到3.1倍”。
2. BLOCKER 吠点卡与批注图“信息照样发出”未带服务器下线：采纳，吠点③改“服务器下线前电脑信息也照样外发”，批注图标题删“其余照样成功”。
3. SHOULD_FIX“失败的只是开启远程登录”超出来源：采纳，改为并列照述 Cursor 的两句话。
4. SHOULD_FIX“说中文不算贵”推不出：采纳，删去概括，只照述报告。
5. SHOULD_FIX 两处译注混入判断：采纳，见“批注译注”节。
6. SHOULD_FIX 清单未覆盖译注：采纳，新增“批注译注”节。
7. SHOULD_FIX Ol Chiki 译名：采纳。
8. SHOULD_FIX“AI 接口按 token 计费”扩大范围：采纳，改“大模型接口通常按 token 计费”。
9. SHOULD_FIX CLEAR 测量数据应为 L3：采纳，相关行改 L3；正文以“研究联盟 CLEAR 发报告”“报告称”署名。

## 视觉复核

WSL Codex gpt-6-astra（medium），`barking review --visual --run`，报告 `barking-review/1029/visual-report-2.md`：PASS / no blocker。逐条处理：

1. 05 译注②超出高亮：采纳，quote 延伸到 “and we will reach out directly.”。
2. 07 译注①后半句未高亮：采纳，quote 延伸到 “scripts that keep running.”。
3. 06 标题把“插件没权限”扩大成“电脑开不了”：采纳，改“恶意插件多数无权开远程登录”（完整句标题溢出版面；Mac、Windows 范围在译注①）。
4. 11 译注未译出列头：采纳，补“最大÷最小 token 数比”并标明基尼系数的范围。
5. 14 译注“版本”易误读：采纳，改“可用于”。
6. 05、06、08 英文缩放偏小、留白多：不采纳，属批注模板版式；底图已是页面 700 CSS px 宽的原始裁切，本期不改模板。
