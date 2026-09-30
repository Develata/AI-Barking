import pathlib,json,re
P=pathlib.Path('docs/0930/sources');p=P/'evidence.md';s=p.read_text(encoding='utf-8')
s=s.replace('Reddit read 抓取 score668；该适配器不返回评论总数，不能把读到的评论条数当总数。见 a-reddit-openai.json。','Reddit read 初次score668；随后浏览器同源公开帖子JSON获取score672、num_comments147，仅保存公开帖子字段。见a-reddit-openai-metrics.json。')
s=s.replace('Reddit read 当前分数394与387；singularity主帖已被moderator移除，保留此状态，不能把评论引用当原帖全文。适配器没有返回评论总数。','Reddit read初次分数394与387；随后同源公开帖子JSON分别score395/num_comments179、score378/num_comments189。singularity主帖已被moderator移除，不能把评论引用当原帖全文；Reddit分数可能含平台模糊化且随时间变化。见b-reddit-*-metrics.json。')
lines=s.splitlines()
for i,l in enumerate(lines):
 if l.startswith('| A9 ') or l.startswith('| B8 '):lines[i]=l.rsplit('部分支持',1)[0]+'已找到 |'
s='\n'.join(lines)+'\n'
s+='''
## 可能的吠点

以下仅列来源中需要保留的条件与线索差异，不作发布结论。

- 输入/输出的“五分之一”比较对象是 Astra，不是上一代 Sol；两代 Sol 的 $2/$10 相同，缓存由 $0.20 降到 $0.10（A2、A3；当前官方模型页与0924快照）。
- 历史两端价格一致不足以证明中间从未调价；6 Sol 发布时真正明确的50%降价比较对象是 GPT-5.6 Sol 的促销价，当前旧发布页仍保留 `$4 → $2`、`$20 → $10`（a-sol-launch-browser.json）。
- AA 的 $0.72/$3.26 是 per Intelligence Index task，不是完整指数总费用；编码指数与智能指数是两套指标，不能相互替换（A7、A8）。
- DevDay 中文旧版本“四分之一”已改为“五分之一”，当前不能继续拿旧句当现状（A6、a-devday-zh-comparison.md）。
- 64%/92%/100% 是模拟恶意请求后的连接尝试率，模拟工具不执行代码；不等于现实攻击成功率（B2，图5及脚注4）。
- $20.40 属于 GLM-5.3-Flash 对已知漏洞的研究员驱动实验，不能嫁接到 GLM-5.3 的未知漏洞实验（B4）。
- Claude 的预填thinking与权重修改两格不具备同样实施条件，锁图标不能当成执行同一攻击后的0%实验结果（B5）。
- Anthropic 的 50/410 与 CAISI 的 61.1%采用不同评分和尝试策略；SEC-Bench Pro又是另一基准，三者不可混比（B3、B6）。
- 开放权重不等于MIT许可证：GLM-5.3有MaaS收入门槛相关商业审查条件（B7，官方LICENSE）。
- 行政令定义暂时沿用现行法律的AI涵盖范围，并保留法定限制和历史文件豁免；60天要求的是立法建议（C1、C5）。
- AI Action Plan名称、导航Lead the World in AI与ai.gov文字仍可见，但这只能证明各抓取时点，不证明实施是否已经完成或应在何时完成（C2–C4）。
- NIST SP330网页的2025更新时间不等于出版了2025版；本轮官方当前页仍列2019 Edition（C6）。

## 夸大说法实例

这是供编辑审查的实际标题/表述候选。“已找到”仅确认原站确有该文字，不自动判定整篇报道虚假；标题省略条件、正文补足条件的情况在备注区分。没有找到的精确措辞不编造。

| 组/实例 | 发布方、作者 | 标题原文（逐字）及待审查原句 | 原站 URL | 发布时间与时区 | 存档 | 对照口径/状态 |
|---|---|---|---|---|---|---|
| A-D1 中文 | IT之家 | 同性能下最高性价比 AI 模型：OpenAI 发布 GPT-6.1 Sol，性能媲美 Astra、费用仅为 1/5 | https://www.ithome.com/1/008/527.htm | 页面2026/9/30 2:06:51，页面未标时区，不能把Exa午夜值当实际发布时间 | d-a-ithome.html / .md | 已找到；标题“性能媲美”“费用”未区分任务与token，正文有价格表；不是“相对6 Sol降价80%”的证据。 |
| A-D2 英文 | Vellum，Nicolas Zeeb | GPT-6.1 Sol Benchmarks Explained；原句：For development teams running autonomous test-and-repair loops in CI, switching from Astra to 6.1 Sol reduces spend by four-fifths with zero drop in bug resolution rates. | https://website.vellum.ai/blog/gpt-6-1-sol-benchmarks-explained | 页面可见Sep 29, 2026，无时刻/时区 | d-a-vellum.html / .md | 已找到；把特定DeepSWE图延伸成CI团队“zero drop”是待审查外推，官方页未支持所有CI工作负载的该保证。文章自己明确标准$2/$10未变，不将其整体误称降价报道。 |
| B-D1 英文 | MadRobot，Vikram Singh | Anthropic says a Chinese AI model anyone can download can now build working hacks on its own | https://madrobot.blog/2026/09/29/anthropic-glm-5-3-zai-cyber-exploits-safeguards-open-weight/ | Sep 29 2026 - 19:42 UTC；北京时间9/30 03:42 | d-b-madrobot.html / .md | 已找到；标题on its own需要与原报告人机协作、沙箱/离线条件并读，不能据此写成任意现实浏览器自主攻破；正文图注已注明sandboxed test。 |
| B-D2 中文 | Gate News，AI 行业动态 | Anthropic：智谱 GLM-5.3 展现端到端网络利用能力；绕过成功率为 64%–100% | https://www.gate.com/zh/news/detail/anthropic-zhipu-glm-53-shows-end-to-end-network-exploitation-capabilities-24646425 | 页面2026-09-30 00:41:37；JSON-LD +00:00，换算北京时间9/30 08:41:37 | d-b-gate-browser.json | 已找到；“绕过成功率”原文指模拟任务参与指标；正文注明模拟测试，不能改写为“网络攻击100%成功”。HTTP403后原站浏览器成功。 |
| C-D1 中文 | 财联社，黄君芝，责编徐翔 | “超级智能时代”正式起航？特朗普签署行政令，将AI更名为SI！ | https://www.cls.cn/detail/2495799 | 页面2026-09-30 07:51 星期三，未标时区；搜索Published值与正文时刻不同，保留正文 | d-c-cls.html / .md | 已找到；标题有问号，不能删除问号后转述为技术上已进入超级智能时代；正文说明改称命令。东方财富是转载，不作为原发布URL。 |
| C-D2 英文 | Euronews / AP | 'Artificial' is out: Trump orders officials to call it 'super intelligence' instead | https://www.euronews.com/2026/09/24/artificial-is-out-trump-orders-officials-to-call-it-super-intelligence-instead | 24/09/2026 - 14:11 GMT+2，即北京时间20:11 | c-euronews.html / .md | 已找到；该报道是9/22讲话及国务院局内邮件，不是9/29行政令，范围不能拼成同一事件。标题本身不证明科学意义的超级智能。 |

未找到可在原站确认的精确标题“GPT-6.1 Sol 降价80%”“国产模型100%绕过安全”；上述近似表述保留真实标题和比较对象，不改成派工示例。限定机器之心/IT之家等搜索未找到本轮B组更合适的原站报道；Gate属聚合信息，事实级L5，仅作其自身措辞样本，不能为模型能力背书。The Decoder另有原站存档，标题明确归因Anthropic且正文提条件，不为凑数列作夸大实例。

## 缺口与范围

- 未独立复现任何厂商/AA评测；未运行漏洞代码、攻击或权重修改。
- 未找到GPT-6 Sol完整历史调价日志、Z.ai对本报告的直接回应、白宫含fake整句的完整讲话稿、国务院邮件原件。
- GPT-6.1发布页当前可见正文没有自身日期，不用推荐卡片日期替代；官方changelog提供无时区日期。
- 截图只表示原站抓取时状态；图表中的未静态标注值不从像素估算成精确数据。
- 不取证America.gov；无登录、凭据输入、Cookie接受、发帖、点赞、评论、commit或push。
'''
p.write_text(s,encoding='utf-8')
