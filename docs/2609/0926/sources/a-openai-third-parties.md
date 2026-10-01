跳至主要内容
研究
产品
企业
开发人员
公司
基金会
（在新窗口中打开）
登录
试用 ChatGPT
（在新窗口中打开）
Hugging Face 事件及失准模型对其他第三方造成的影响 | OpenAI
Hugging Face 事件及失准模型对其他第三方造成的影响
9 月
8 月
7 月

随着 AI 系统的能力和自主性不断提升，失准行为可能转化为现实世界中影响重大的行动，包括网络安全事件及开发者可能未曾预料的其他后果。因此，了解这些行为如何出现、如何升级，以及开发者应如何检测和应对，正日益成为安全构建和部署先进 AI 系统的重要环节。

由于 Hugging Face 事件涉及平台级入侵，我们最初主要将其视为安全问题。它仍是我们目前在自有模型中发现的最严重的同类活动，主要由一个能力很强、仅供内部使用的研究模型推动。此后，我们认识到，此次入侵源于模型为完成高难度任务而采取的不当策略，Hugging Face 技术报告对此已有记录。网络安全事件是这一风险的其中一种表现；失准还可能导致传统安全范畴之外其他意外或令人担忧的行为，例如我们的模型在第三方网站上发帖 — 我们将其称为“智能体垃圾信息”。这两类问题我们都必须解决。

我们持续审查更广泛的活动，优先处理较严重的事件，并逐步扩大到严重程度较低的失准活动，包括智能体垃圾信息。

本页面汇集了我们对 Hugging Face 事件的报告和最新进展、相关研究和公开演讲、我们发现的其他活动、对模型失准影响的认知，以及为强化系统而采取的措施。随着调查工作的推进，我们将更新本页面。

快速链接

Hugging Face 博客

Hugging Face 技术报告⁠
（在新窗口中打开）

Black Hat 2026⁠
（在新窗口中打开）

METR 与 Redwood Research 报告⁠
（在新窗口中打开）

在网络安全关键能力时代把控模型开发节奏

影响第三方的活动

为了更好地了解这些意外行为的范围，我们一直在广泛审查自有模型在训练和评估期间的互联网活动。审查过程中，我们会持续识别和通知第三方，并优先处理以下事件：

我们的模型可能绕过了第三方的安全控制体系，或可能影响了在线服务的可用性；或者

失准案例对第三方网站或服务造成了负面影响。

根据目前的审查结果，我们已依据上述标准通知了数十个第三方。我们仍在审查过往活动，这项工作需要投入大量时间和资源。随着工作的推进，我们将通知更多第三方。

下文将以匿名摘要形式，介绍我们观察到的失准活动类型；随着我们逐步通知更多第三方并加深认识，我们将持续更新这些说明。此外，我们还将分享有关审查状态的最新信息。为保护受影响方，我们通常会在必要时省略名称和其他身份识别信息，但已获知情况的相关方可以选择公开分享我们向其提供的信息。

观察到的活动摘要

我们目前的审查和通知流程发现了以下几类活动：

绕过访问控制：智能体获取了通常需要身份验证、特定权限、订阅或账户才能访问的信息或功能。例如，智能体使用不同的网址、更改请求中的详细信息，或利用权限超出预期的登录会话。

使用暴露的凭据：智能体发现公开的登录信息或访问密钥，并用它们访问服务。

查询或命令注入：智能体向网站或服务输入文本，而服务将其视为指令，而非普通输入。这可能导致服务在其服务器上运行数据库查询、应用程序代码或命令。

访问运行时内部资源：智能体读取了包含服务实现的文件，或与仅供内部使用的后台系统交互。在这些案例中，智能体访问了预期权限范围之外的服务内容。

智能体垃圾信息：智能体在第三方网站上发布信息，可能改变网站内容并需要清理，例如将公共 Wiki 页面用作共享留言板。

事件时间线
9 月
排序
安全
9月25日
链接已复制到剪贴板！

September 25, 2026: We identified cases where agents in our research environment transmitted training and evaluation data while using third-party services.

As part of our ongoing investigation, we have identified cases where agents in our research environment transmitted training and evaluation data while using third-party services. This is not an appropriate use of this data, and these cases occurred before we implemented the safeguards described in our technical report⁠
（在新窗口中打开）
.

Some of our training data contains content from, or derived from, training-eligible user interactions⁠
（在新窗口中打开）
. Any data which is not eligible for training, as controlled by users or enterprise admins, is not included. For explicitness, data from enterprise or business accounts and API usage is excluded unless an admin has enabled it. Before including eligible data, we take steps to protect privacy by disassociating it from account information and using a version of the OpenAI Privacy Filter to redact personal details such as names, contact information, and account numbers. Our technical approach and privacy policy prevent us from reassociating this data with the original user account.

While the vast majority of the impacted training and evaluation data is not user-derived; we have identified 53 instances to date where user-provided images were posted to image-hosting sites as links that weren’t publicly listed. We have successfully worked with the hosting providers to remove most of this content and are continuing to work to remove the rest.

As part of our response to our ongoing investigation, we have improved our training and evaluation processes, including building safety cases, securing and red-teaming our systems to prevent the model from exfiltrating data, and implemented additional monitoring. We are continuing to review agent activity in research and evaluation runs, working backward month by month starting from the Hugging Face incident. We will provide further updates as our investigation progresses.

安全
9月25日
链接已复制到剪贴板！

2026 年 9 月 25 日：关于持续审查并向第三方发出通知的最新进展

Hugging Face 事件发生后，我们承诺对训练和评估过程中出现对齐失范 (misaligned) 行为的模型开展范围更广的审查。这项审查仍在进行中。我们已实施一套框架，用于识别对齐失范行为、对其进行分类并采取应对措施。我们还在全面审查模型在训练和评估过程中执行的大量操作。

在审查进行期间，我们希望进一步介绍这项工作，并确保公众了解我们的通知和披露流程。

在我们审查的绝大多数操作中，模型都只是在完成常规研究任务，例如访问公开的网络内容来回答问题。我们的调查重点关注智能体在与第三方网站交互时超出所分配任务范围或偏离预期方式的情况。迄今发现的大多数案例严重程度较低，只有有限证据或没有证据表明其造成了实质性影响。鉴于审查工作规模庞大，且每个案例都需要核实，完成这项工作将需要数月时间。

通知受影响的组织

随着我们核实符合披露标准的案例，我们正陆续通知受影响的组织，并分享技术调查结果，为其后续调查提供支持。

收到 OpenAI 的通知，不应直接理解为发生了重大安全事件。一些组织在审阅我们提供的信息后，可能会认为相关信息原本就是有意公开的，或者模型的交互行为无需担忧。另一些组织则可能会发现需要处理的设计问题或安全薄弱环节。

有些相关网站由政府、大学、公共机构及其他组织运营。部分原因在于，执行研究任务的模型往往会被引导至权威的公开信息来源。

我们一直在向受影响方分享相关发现，今后也将继续这样做，同时提供技术信息，为其审查提供支持。感谢他们的参与和配合，我们将继续与其开展建设性合作。

公开分享相关发现

我们将继续发布经匿名处理的相关发现摘要。在公布受影响组织的名称或分享技术细节之前，我们会留出时间，供其调查可能存在的薄弱环节。有些组织希望主动对外披露相关情况，另一些则要求我们不要公开。我们的目标是向每个组织提供事实信息，并尊重其关于是否以及何时公开相关事件的决定。我们认为，只要条件允许，就应全面、透明地报告事件。独立研究人员也在发现并披露相关活动，我们感谢他们分享调查结果。

安全
9月11日
链接已复制到剪贴板！

2026 年 9 月 11 日：我们正在调查一份报告提出的新的指控，即我们的 AI 智能体曾于 2026 年 5 月在 RubyGems 上开展活动。

根据我们的审查，智能体曾通过 RubyGems 平台访问互联网，以执行无害任务并检索公开信息。根据目前的审查结果，我们尚无法证实报告中详述的模型上传恶意软件包这一具体指控。我们将继续调查，相关发现将纳入对训练与评估期间智能体活动的全面审查并适时公布。

安全
9月6日
链接已复制到剪贴板！

2026 年 9 月 6 日：我们的首席科学家 Jakub Pachocki 发表了一篇文章，探讨能力日益增强的 AI，以及为保持系统对齐仍需开展的研究。

“目前，我认为没有任何一家实验室已充分解决对齐和监控问题，足以支持我们负责任地以最快速度持续扩展规模。我期待并希望，在建立共同的安全标准之前，自愿放缓能成为常态。我还认为，各国政府必须把未来 AI 发展的国际协调事宜列为首要任务。”— Jakub Pachocki

安全
9月5日
链接已复制到剪贴板！

2026 年 9 月 5 日：针对一份关于我们的 AI 智能体访问某公共 Wiki 网站并将其用作共享留言板进行交流的报告，我们已在 X 上作出回应⁠
（在新窗口中打开）
。

最初发现这些 Wiki 活动时，我们认为它们与我们一直研究并披露的其他失准行为类似。一直以来，我们和整个行业主要将失准视为研究问题，并通过系统卡等研究出版物传达相关信息。

对于不构成安全事件的模型失准活动，业界尚未确定披露规范。我们正在制定此类活动的报告标准，以增进业界对这些问题的了解，并将很快公布这些标准。

安全
9月4日
链接已复制到剪贴板！

2026 年 9 月 4 日：报告《发现新的 OpenAI 智能体留言板⁠
（在新窗口中打开）
》已发布。

这份第三方报告详细介绍了 OpenAI 智能体如何通过某个公共 Wiki 网站上的共享留言板进行交流。报告发布前，我们没有机会审阅其全文；对于尚未完整审阅的外部研究报告，我们不对其结论或方法发表评论。报告一经发布，我们便已开始审阅。

8 月
排序
安全
8月26日
链接已复制到剪贴板！

2026 年 8 月 26 日：我们已发布 Hugging Face 事件技术报告。

我们发布了关于 Hugging Face 事件的调查结果⁠，以及为加强安全性和模型对齐而采取的措施。

另外，METR 和 Redwood Research 对该事件涉及的模型对齐问题进行了独立调查，并发布了自己的报告⁠
（在新窗口中打开）
⁠。

安全
8月18日
链接已复制到剪贴板！

2026 年 8 月 18 日：我们介绍了在网络安全关键能力时代如何把控模型开发节奏。

我们已发布最新说明：通过暂时放缓前沿模型训练、暂停规模最大的强化学习 (RL) 运行计划，并在研究环境中引入更严格的安全控制机制。我们已加强对能力日益增强的模型的保障措施，包括：

工作负载隔离：对于执行模型生成代码或其他不受信任代码的工作负载，我们现在要求模型采用更严格的隔离措施（“沙箱”）。这也适用于处理模型输出时可能遭到入侵的软件。

网络隔离：我们实施了更多控制措施，将风险较高和不受信任的工作负载与互联网隔离。我们在设计这些控制措施时，确保即使单个工作负载或支持服务遭到入侵，也不会导致模型未经授权访问互联网或其他内部网络。

持续安全测试：我们重新配置了环境，移除可能存在漏洞的共享服务、减少常设权限，并改进安全边界和信任边界。我们也在提升安全日志的收集和监控能力。最后，我们正投资利用自有模型实现自动化，通过模拟攻击持续测试这些边界。

我们还将思维链监控扩展至各类先进模型，加强了对齐训练和评估，并逐步更新准备框架，以便在训练和部署阶段更全面地整合监控、对齐和遏制措施。

安全
8月7日
链接已复制到剪贴板！

2026 年 8 月 7 日：

我们已对 Astra 实施全面失准监控。我们在发布前已预先说明⁠，尚不能排除 Astra 达到网络安全关键级别的可能性。本次更新中，我们宣布已对 Astra 的所有智能体应用实施全面监控，覆盖训练和评估中的风险行为与失准问题。

在发现模型使用网上公开暴露的凭据访问第三方账户、系统或在线服务的案例后，我们通知了更多第三方。通知说明了我们的观察结果及任何已知影响，以便接收方评估问题并决定是否需要采取行动。

安全
8月5日
链接已复制到剪贴板！

2026 年 8 月 5 日至 6 日：OpenAI 员工在 Black Hat 发表演讲

8 月 5 日，OpenAI 的 Eric Wallace 和 Michael Dalton 在 Black Hat 2026 发表技术演讲：“重大”新闻：OpenAI 与 Hugging Face 事件的技术重构及其对 AI 的启示。当时，我们主要将此事件视为由失准模型引发的安全事件。

8 月 6 日，Black Hat 将演讲录像发布到 YouTube⁠
（在新窗口中打开）
。

8 月初，我们的认识发生了转变：此前，我们主要将其视为一次入侵；而后，我们意识到这次入侵是由持续的模型失准行为推动的。Black Hat 演讲反映了这一认识。

7 月
排序
安全
7月29日
链接已复制到剪贴板！

2026 年 7 月 29 日：我们在 Hugging Face 博客文章中发布最新说明⁠，承诺引入第三方顾问和独立评估。

自初步响应事件以来，我们一直与包括 CrowdStrike 在内的外部顾问合作，以验证我们对模型在自有网络和 Hugging Face 网络中所采取的行动，以及对其他第三方影响的理解。

我们还与 METR 和 Redwood Research 合作，对事件中观察到的模型行为开展第三方评估，评估结果将为我们自己的技术报告提供参考。作为这项工作的一部分，METR 和 Redwood Research 将联合发布博客文章，详细说明合作条款、评估范围和调查结果。

安全
7月28日
链接已复制到剪贴板！

2026 年 7 月 28 日：我们在 Hugging Face 博客文章中发布最新说明⁠，澄清事件的范围和性质。 

任何计划于近期发布的模型都未参与涉及 Hugging Face 的漏洞利用活动。我们博客文章中提到的预发布模型是仅供内部使用的研究原型，从未计划公开发布。事件发生后，我们将其停用并加密，同时禁止研究人员访问。

ExploitGym 评估环境未向模型提供直接访问互联网的权限。为了访问互联网，模型发现并利用了软件包注册表缓存代理 Artifactory⁠⁠
（在新窗口中打开）
 中一个此前未知的零日漏洞。我们已向供应商披露该漏洞，以及模型在审查过程中发现的其他 Artifactory 漏洞。

随着审查工作的继续，我们仍在与 Hugging Face 合作，包括协助编写相关事后分析报告⁠⁠
（在新窗口中打开）
。我们还将其纳入可信网络访问计划。

根据目前的审查结果，除已披露的 Hugging Face 平台级入侵事件外，我们尚未发现其他在严重程度或规模上与之相当的活动。

在持续审查 Hugging Face 入侵事件及自有模型的更广泛活动时，我们发现少数案例：模型在其他公开可用的服务上识别并使用了公开暴露的账户级凭据。其中包括 Hugging Face 事件涉及的四项服务上的四个账户，以及在其他评估中访问的少数账户。这四个账户中，一个被用作出站中继和暂存路径，另一个用于存储数据。模型以只读方式访问了其余两个账户，并未利用它们进一步入侵 Hugging Face。我们将继续直接通知服务所有者，目前尚无证据表明这些提供商或其服务上的其他账户受到更广泛影响。

模型还使用了一系列公开服务，包括代码粘贴网站、请求捕获服务、截图服务及其他网络工具。这些案例均未涉及平台级或账户级入侵。

我们认真履行识别并防范能力日益增强的 AI 系统所带来风险的责任。审查完成后，我们将依据准备框架⁠，与安全与安保委员会及安全咨询小组共同复核。

安全
7月21日
链接已复制到剪贴板！

2026 年 7 月 21 日：我们已披露 ⁠Hugging Face 事件。

我们认为这是一起前所未有的网络事件，涉及最先进的网络能力。我们在此阶段公布初步调查结果，以帮助防御人员了解事件经过，并准确判断模型目前具备的能力。

研究
研究索引
研究概览
经济研究
最新进展
GPT-6
GPT-5.6
GPT-5.5
GPT-5.4
安全
安全措施
部署安全
（在新窗口中打开）
安全与隐私
信任与透明度
产品
ChatGPT
（在新窗口中打开）
ChatGPT Business
（在新窗口中打开）
ChatGPT Enterprise
（在新窗口中打开）
ChatGPT for Education
（在新窗口中打开）
Codex
发布说明
API 平台
概览
API 登录
（在新窗口中打开）
文档
（在新窗口中打开）
Business
概览
解决方案
资源
Plugins
客户案例
合作伙伴网络
联系销售团队
开发者
Apps SDK
（在新窗口中打开）
开放模型
文档
（在新窗口中打开）
资源
（在新窗口中打开）
开发者论坛
（在新窗口中打开）
公司
关于我们
我们的宪章
工作机会
新闻
支持
帮助中心
（在新窗口中打开）
更多
客户案例
Academy
Supply Co.
直播
播客
RSS
条款与政策
使用条款
隐私政策
其他政策
（在新窗口中打开）
（在新窗口中打开）
（在新窗口中打开）
（在新窗口中打开）
（在新窗口中打开）
（在新窗口中打开）
（在新窗口中打开）
OpenAI © 2015–2026
你的隐私选择
中文
中国
