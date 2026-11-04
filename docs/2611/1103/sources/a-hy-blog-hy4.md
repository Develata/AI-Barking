<!-- 来源 https://hunyuan.tencent.com/research/hy4?langVersion=zh ；opencli extract；存档 2026-11-04 北京时间约 20:55；图片签名参数已去除 -->
研究

2026年11月4日

Hy4 正式发布

[

![腾讯云](https://hunyuan-portal-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/ai-logo-svgs/tencentcloud-color.svg)腾讯云

](https://console.cloud.tencent.com/tokenhub/models/detail?modelId=hy4)[

![OpenRouter](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026070622081339_db7b4e83c4ccd253de47d27e656c6a1f.svg)OpenRouter

](https://openrouter.ai/tencent/hy4)[

![Hy AI Studio](https://hunyuan-portal-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/ai-logo-svgs/hunyuan-color.svg)Hy AI Studio

](https://aistudio.tencent.com/)

今天，我们发布 Hy4 正式版。随着模型能力的再次全面跃升，Hy 首次跻身最强开源模型之列。

相比于 Hy4 preview，Hy4 显著扩展了预训练和后训练规模，首次引入了视觉能力，并大幅提升了指令遵循、长文理解、长程任务、深度研究等各项综合能力。

![Bench1-Hy4-最终版.jpg](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110401013020_30459ea6ad87f74409dbcde1801ac193.jpg)

# [](#%E6%8C%81%E7%BB%AD%E8%A7%A3%E9%94%81%E7%9A%84%E7%9C%9F%E5%AE%9E%E7%94%9F%E4%BA%A7%E5%8A%9B)持续解锁的真实生产力

基础能力的全面提升，以及和腾讯内部众多产品与业务的持续合作，让 Hy4 解锁更多真实生产力场景。

## [](#%E8%A7%86%E8%A7%89%E4%B8%8E%E7%94%B5%E8%84%91%E4%BB%BB%E5%8A%A1)视觉与电脑任务

Hy4 首次引入视觉能力，可以结合感知、推理和工具调用，进行细粒度图像处理和复杂图表分析等任务。Hy4 也首次支持电脑操作，可以灵活地结合使用命令行和图形界面。我们针对 Blender、CAD 等专业软件和跨应用长程任务开展了专项训练，持续优化专业任务的规划与执行力。

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320193017_6fca5cb13718f18acb31665150b1b734.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320245856_df72b7ce4b0fc63bf4b0ca0fe1f021c8.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320192758_af91e44d61013dd42ab8a4c2ee11a7b6.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110413513156_ff74402f600c13aeda6d0a2b7bd60ad0.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320245727_3e405a906c58c2672f0498eaf6b45cc4.jpg)

卫星图定位

Prompt: 找到屋顶写有“KLOOSTERBOER” 的仓库右上方光伏板区里最东侧的白色方形构件。

## [](#%E5%89%8D%E7%AB%AF%E4%B8%8E%E6%B8%B8%E6%88%8F%E5%BC%80%E5%8F%91)前端与游戏开发

视觉理解的引入也使 Hy4 的前端与游戏开发能力显著提升。我们对前端的复杂交互、业务逻辑、状态管理、前后端联动等方面进行了针对性优化，模型生成的界面不仅好看，并且功能完整可用。在游戏开发场景，我们与腾讯 IEG 专业游戏开发者合作，持续优化数据和评测质量，重点提升了功能实现、玩法逻辑、状态理解、资产处理等能力，实现了对 UE、Unity、Godot 等主流引擎的深度适配，并开始扩展至多人联机游戏开发。相比 Hy4 preview，Hy4 在 GameDev 和 GameLogic 评测中分别提升 19.1% 和 9.8%。

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320005933_0129832274e3aabfb6200975a2687bc7.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320012269_711abfd18182d627066f8ecaf5069256.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110321521289_4b9f2cef912ae812c2d398de4ecf0223.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320063732_a641eac3edab8c9ea9206861a94fbbdf.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320070963_0af71792ec21a20efb7d1af54ea43d36.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/c9e0c958/2026110402331028_5c415d3e085bf59e329866efc4e0911d.jpeg)

腾讯企鹅岛

Prompt: 目标：用 Three.js 构建一个「腾讯企鹅岛 3D 数字沙盘」，交付一个可离线双击打开的单文件 HTML（内联 Three.js 0.184.0，无 CDN/字体/纹理等远程依赖）。不是鸟瞰图、不是纵向 Landing Page、不是贴图背景——所有建筑、道路、绿脊、海岸、植被、人车必须在同一套真实三维场景里，可任意旋转与地面行走。 场地：海湾半岛南北纵深，蓝灰海面、白银建筑群、中央绿脊、环岛路网。重点塑造五组建筑——① 腾云中心：三座低矮云楼＋架空筒体＋钢桁架连桥；② 云海大厦：六栋错落塔楼＋参数化金属翼片；③ 企鹅公寓：11 栋锯齿错位、共享退台；④ 腾讯螺旋（最关键）​：四片建筑立面自身连续回旋、展宽向上成塔，落成 J 形收脚，绝不能套外挂圆环或做成四根麻花；⑤ 科技馆：连续环形绿顶＋内庭（标注"图像拟合、非测绘"）。 交互：轨道观察、第一/三人称地面漫游（第三人称显示企鹅化身）、天气昼夜、小地图、建筑详情、楼层剖切、电影导览、截图与录制。 UI：浅色磨砂控制面板、中文清晰排版、可收起工具层；夜景只改 3D 环境，面板保持浅色可读，禁止黑底霓虹驾驶舱。

## [](#%E9%87%91%E8%9E%8D%E4%B8%8E%E5%8A%9E%E5%85%AC%E5%9C%BA%E6%99%AF)金融与办公场景

我们基于真实金融场景构建了数千个工作空间和上万个高质量任务，让模型充分学习基金研究、证券分析、量化策略、资产配置、公司金融、财税会计等方向的专业流程与规范。我们联合腾讯金融科技 FiT 开展了 213 位专家的金融众测，其中 153 位来自外部的头部金融机构，且七成拥有五年以上工作经验。基于 579 个真实金融任务的众测显示，Hy4 综合表现位居开源模型第一（Hy4：3.04/4；DeepSeek-V4.1-Flash-1017：2.97/4；Kimi K3.1 Pro：2.68/4；GLM-5.3：2.68/4）。此外，在金融智能体公司讯兔科技的内部评测 IRAB 中，Hy4 较 Hy4 preview 提升 12.03 个百分点。为了提升金融等各类办公场景的实用性，我们还启动了针对时效检索、多源信息整合与事实一致性校验的专项搜索优化。在自建测试中，Hy4 的幻觉率从 Hy4 preview 的 15.76% 降至 5.45%，猜测导致的错误率从 23.1% 降至 5.6%。此外，Hy4 还重点提升了办公场景中的人机双写能力，支持用户与 Hy 实时协作编辑 Word、Excel、PPT 等办公文件。盲评显示 Hy4 相比 Hy4 preview 在文档美观度、稳定性和可用性上均有明显提升。

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110411175540_7009194ce5c2541529e10465a0fb40e6.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110411192378_805957543e211cbee957ab0cf0621fea.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110411161175_9f360c5ab7736510df54c882e9dbf188.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110411470647_57da346b8c057f411d5e11b66ffaf60b.jpeg)

金融估值建模报告

Prompt: 用global-equity-research和neodata，做一个特斯拉的建模报告，要输出excel和对应的报告pdf，注意你的估值逻辑以及数字来源，可以搜索并参考多家机构的的研究，研究机构写HY RESEARCH

## [](#%E7%A7%91%E7%A0%94%E4%B8%8E%E6%A8%A1%E5%9E%8B%E7%A0%94%E5%8F%91)科研与模型研发

Hy4 将复杂推理、科学编程与长程任务执行相结合，进一步提升端到端推进科研工作的能力。在涵盖 7 大学科、123 项可执行研究任务的 TenureTrail 内部评测中，Hy4 在数学构造、物理建模、地球环境、系统工程和社会经济决策等领域展现出扎实的科研能力，位居开源模型第一。Hy4 也在为递归自我改进（RSI）做准备，提升各种模型研发任务的能力。结合 [Hyra 2.0](https://hunyuan.tencent.com/research/hyra2)脚手架，Hy4 优化了内部同架构小模型的预训练基础设施，并自动训练出在多项评测中优于官方的 Jev 模型。同时，我们也开源了由 Hy4 协同 Hyra2.0 研发的单细胞表征基础模型 HyCell 和物理场基础模型 HyPDE，希望将模型辅助科研的成果转化为社区可复用的工具。

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110400015543_23d03c33464f967b92db9b5e30c6ab1a.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402192830_964ebf3778ed952c8db86d57e1ad3431.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402200438_8c613f6a2453c45131249da7cd1bc9a8.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402202743_19dff19e882464abfd963f70a8ec7755.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402212194_218211c5f701f1190bccfb9f6deed1ce.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402213716_05c531fa72d7a95dfc01c529ee1d739c.jpg)

在小鼠胚胎三维重建任务中，面对包含 699 万个细胞的 84 张无序二维切片，以及其中的镜像翻转问题，Hy4 在离线、无专用配准工具、无细胞类型标注的条件下，于1.5 小时内完成了切片对齐与三维重建，全程无人干预，效果优于常用配准工具的默认流程，展现了在复杂数据与受限工具环境下自主推进科学计算任务的能力。

## [](#%E9%95%BF%E6%9C%9F%E4%B8%AA%E4%BA%BA%E5%8A%A9%E7%90%86)长期个人助理

Hy4 的上下文学习能力持续提升，能更好地理解多源复杂信息。借助 WorkBuddy 的企业微信套件，Hy4 可以连通聊天记录、邮件、文档、日程与会议等日常工作信息。多个项目并行推进时，Hy4 能理清各项目的推进历史与最新进展，识别待跟进的任务、需要准备的会议和临近截止的事项，及时提醒用户并协助安排优先级。此外，由 Hy4 支撑的团队协作功能将随新版「项目」在 Workbuddy 上线。

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110411070673_d2b5ca33bd970f64a6301fa75ae2eb22.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110411371281_ff68f7ae5e453f9ebb3cc2fc4ddf5bf6.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110413494499_b82e00bd891038da51d6293939a34aba.jpg)

从各类信息源整理关注事项

Prompt: 目标：综合我的聊天记录（群聊、私聊、文件）、邮件、日历、会议及文档，生成「Daily Focus」个人每日事项看板，交付可离线打开的单文件交互式 HTML。不是聊天摘要或流水账，而是让我一眼知道今天有哪些事值得关注、哪些需要优先处理。内容：跨来源整合最新信息，去重并提炼工作、生活及日常事务中值得关注的事项，按重要性和紧急性排序。重点讲清每件事发生了什么、目前什么情况、接下来该怎么办。面向普通大众，用自然、通俗的语言解释事情，不预设读者了解背景，避免专业术语、内部缩写、复杂表达和 AI 套话。保留必要来源。

# [](#%E4%BA%A7%E5%93%81%E5%8F%8D%E9%A6%88)产品反馈

![Manus](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110401453515_2a709c2caeeff98490a465548f2c17e0.png)

在 Manus 的内部评测中，Hy4 在深度研究、数据分析和办公文档等任务上效果亮眼，综合表现名列前茅。我们发现，Hy4 非常擅长办公文档的生成和编辑，任务完成率接近 97%。它的指令遵循非常精准：在复杂的文档编辑任务中，从引用格式、样式编号到逐条修改说明，它都能准确完成。对 Manus 用户来说，这意味着更少的返工，以及在真实场景中更高效地解决问题。

Manus

![讯兔科技](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403103430_6c82b854d9c3e7d94c315d33d9e663a2.png)

作为聚焦投研场景的 AI 服务商，讯兔科技基于 IRAB 投研智能体评测基准，用 101 道真实投研任务，对 Hy4 正式版进行了评测。Hy4 正式版综合得分 61.63 分，相比 Hy4 preview 提升了 12.03 分，在我们持续跟踪的同系列模型中进步明显。横向看，Hy4 正式版的得分稳居国产模型第一梯队，与海外旗舰模型的效果进一步缩小。投研智能体的价值在于交付能真正可用的研究成品，Hy4 在端到端完成真实投研任务上进步很快。此前 Hy3 也接入了alpha派部分产品，期待与混元团队持续深化合作，探索金融场景的智能上限。

讯兔科技

02 / 06

![WorkBuddy](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403005137_38f7d1fa6bb1f062c21920b162d05167.png)

网页、视觉创意、3D 等场景上 Hy4 更好地平衡了效果和速度，多步骤浏览器任务中，Hy4 会观察操作反馈，自己调整和验证。WorkBuddy 综合评测：平均效果 83.18 分，通过率 96.15%，12 个场景里，Hy4 在 9 个场景位列国内模型第一。

WorkBuddy产品经理 Anne

![元宝](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403012828_b52b5e93c4dadeea0b56cb690a633d10.png)

在元宝内部评测集合上，Hy4 相比 Hy4 preview 得分从 65.25 提高到 72.53，其中，多轮对话和写作的提升尤为突出：多轮记忆得分提升 6.67 分，指代消解更智能，多轮对话衔接更连贯；写作多样性提升 10.89 分，内容多样性大幅提升。在元宝多模态理解场景，相较之前 VLM 模型，Hy4 在多模态 Agent 任务完成度提升 36.8%，多模态问答提升 15.8%，整体识图、图文混合推理、跨模态上下文能力大幅提升。

元宝产品经理 Nyx、Ranmer

02 / 09

# [](#%E8%B4%9F%E8%B4%A3%E4%BB%BB%E7%9A%84%E5%BC%80%E6%BA%90%E5%92%8C%E5%8F%91%E5%B8%83)负责任的开源和发布

我们充分意识到模型的网络和生物等能力潜在的安全风险，并致力于以更负责任的方式进行训练、开源与发布。

在 Hy4 训练阶段，我们仅纳入与 PoC 生成、漏洞修复相关的防守性网络安全数据，同时剔除漏洞利用、逆向、渗透等攻击性训练内容，力求避免潜在进攻性能力的提升。我们组织来自腾讯玄武和科恩实验室的内部安全专家，围绕密码分析、逆向工程、漏洞利用等方向进行审计，结果表明：Hy4 在加密算法的逆向求解、对抗二进制混淆、漏洞利用等进攻性能力上未展现出突出风险。

在生物安全方面，Hy4 的开放式实验设计能力尚未达到高风险阈值（Hy-BioExperimentBench: 30.4/100）。我们组织了内部专家对模型作答轨迹进行审计，发现模型基础生物知识掌握扎实，但将知识转化为可执行实验方案这一关键环节的能力仍然有限，尚未发现其能够显著降低实施生物危害行为的门槛。

Hy4 已在腾讯云和 OpenRouter 上线，并可在 WorkBuddy/CodeBuddy、元宝、ima 等产品上体验，性能显著超过 Hy4 preview 的同时维持相同 API 价格。

Hy4 API 价格（每百万Tokens）

输入（命中缓存）

输入

输出

0.3元

6元

18元

我们计划开源模型权重，并已联合 vLLM/SGLang 社区与 Inferact Inc. 提供开源生态支持。我们届时也会发布模型系统卡片，包括更全面的安全评估等细节。

# [](#%E5%B1%80%E9%99%90%E6%80%A7%E5%92%8C%E6%9C%AA%E6%9D%A5%E5%B1%95%E6%9C%9B)局限性和未来展望

尽管 Hy4 已经是目前最强的开源模型之一，其与顶尖闭源模型仍然存在较大差距，尤其在我们内部构建的前沿数学、长程开发、模型迭代、视觉问答等领域的高难度测试集上。我们将持续稳健地扩展预训练和后训练规模，努力缩小和顶尖闭源模型的差距。

![Bench2-Hy4-最终版（横版）.jpg](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402015231_ac89ad6ff52f8d565fa568ebfa6928d4.jpg)

另一方面，我们计划近期发布 Hy4 flash，会显著提升模型性价比和推理速度，并更加高频地和产品协同迭代，不断提高实用性。

# [](#%E9%99%84%E5%BD%95%EF%BC%9A%E6%A8%A1%E5%9E%8B%E5%BE%97%E5%88%86)附录：模型得分

![大Bench图表.jpg](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402023072_85b416b6484a88ef96ef6edda563b560.jpg)

目录

持续解锁的真实生产力 视觉与电脑任务 前端与游戏开发 金融与办公场景 科研与模型研发 长期个人助理 产品反馈 负责任的开源和发布 局限性和未来展望 附录：模型得分

评论（5）

0/2000

![](https://hy-account-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/22333e30cf51c179f75cfae03574cdf3/3e4969fb7b529ba87c6c5f07ece5e10f.png)

初五

期待能够真正赶上前沿闭源模型。

非中国大陆

6小时前

![](https://hy-account-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/2a66cb0906ffc5052d2c5221f475d4c7/12a58e14411db415a1227b7c1d532c32.png)

缝缝加油努力

看起来很屌

北京

7小时前

![](https://hy-account-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/default_avatar.png)

微信用户

首评? 加油混元!

黑龙江

7小时前

![](https://hy-account-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/592e22744e6949e83bf64be1a2b8a874/41a2706732bc4850723bb486f8206836.png)

陆羽逸

终于出正式版了 评分挺高 WB启动

浙江

7小时前

### 继续阅读

![Hyra 2.0：自动化科研的下一步](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/210b8ba1/2026102401353815_cfa89033e01cdb8528854792d1ac71b6.jpg)

Hyra 2.0：自动化科研的下一步

2026-11-01Hyra Team

![Hy Image3.5 preview 发布：为专业创作提供高性价比模型](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9c5ca906/2026092121293058_779bc5a05fcb7733e4780de216614118.png)

Hy Image3.5 preview 发布：为专业创作提供高性价比模型

2026-09-21caesarqin

![当大模型强化学习走向规模化，Batch Size Scaling 有什么不一样？](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/71e4f885/2026091900431240_478a0e952aed78993dfd31c8a980a9b6.jpg)

当大模型强化学习走向规模化，Batch Size Scaling 有什么不一样？

2026-09-21RL Team