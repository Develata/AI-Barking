<!-- 来源 https://hy.tencent.ai/research/hy4?langVersion=en ；opencli extract；存档 2026-11-04 北京时间约 21:20；签名参数已去除 -->
Research

November 4, 2026

Introducing Hy4

[

![Tencent Cloud](https://hunyuan-portal-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/ai-logo-svgs/tencentcloud-color.svg)Tencent Cloud

](https://console.tencentcloud.com/tokenhub/models/detail?modelId=hy4)[

![OpenRouter](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026070622081339_db7b4e83c4ccd253de47d27e656c6a1f.svg)OpenRouter

](https://openrouter.ai/tencent/hy4)[

![Hy AI Studio](https://hunyuan-portal-prod-1258344703.cos.ap-guangzhou.myqcloud.com/public/ai-logo-svgs/hunyuan-color.svg)Hy AI Studio

](https://aistudio.tencent.ai/)

Today, we’re releasing Hy4, another major leap forward that places Hy among the strongest open-source models for the first time.

Compared with Hy4 preview, Hy4 substantially scales up both pre-training and post-training. It adds vision capabilities for the first time and delivers major gains in instruction following, long-context understanding, long-horizon tasks, deep research, and more.

![Bench-最终版本.png](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110413013841_c7b0f472f484b23cc564fd19c3fba30d.png)

# [](#unlocking-real-world-productivity)Unlocking real-world productivity

With broad improvements in core capabilities and ongoing co-design with product teams across Tencent, Hy4 moves beyond benchmarks to open up more real-world use cases.

## [](#vision-and-computer-use)Vision and computer use

Hy4 supports vision perception for the first time. By combining perception, reasoning, and tool use, it can understand fine-grained image details and interpret complex charts. Hy4 also supports computer use for the first time, working across command-line interfaces (CLI) and graphical user interfaces (GUI). Through targeted training on specialized software such as Blender and CAD, as well as long-horizon workflows that span multiple applications, we significantly improved its ability to plan and carry out professional tasks.

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412372735_9f360c5ab7736510df54c882e9dbf188.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320245856_df72b7ce4b0fc63bf4b0ca0fe1f021c8.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412412436_5fb6b9fb09f1795dc718ea6dcb234c98.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412450872_1b65d98f3c4a2af27230ed9dbb91419e.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412465415_7009194ce5c2541529e10465a0fb40e6.jpeg)

Locate a rooftop component approximately 2 meters in size in a 2.8-gigapixel aerial image

Prompt: Find the easternmost white square component in the solar panel area to the upper right of the warehouse with “KLOOSTERBOER” written on its roof.

## [](#frontend-and-game-development)Frontend and game development

Integrating visual understanding has driven a step change in Hy4’s frontend engineering and game development capabilities.

For frontend work, we optimized Hy4 for complex UI interactions, business logic, state management, and full-stack integration, so that generated interfaces are not only visually polished but also functionally complete and production-ready.

For game development, we partnered with professional developers at Tencent Interactive Entertainment Group (IEG) to curate high-quality training data and build evaluations. We focused on feature implementation, gameplay logic, state tracking, and asset pipelines. We also extensively adapted Hy4 for major engines such as Unreal Engine, Unity, and Godot, and expanded its capabilities to multiplayer game development. Compared with Hy4 preview, Hy4 improves by 19.1% on GameDev and 9.8% on GameLogic.

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320005933_0129832274e3aabfb6200975a2687bc7.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110320012269_711abfd18182d627066f8ecaf5069256.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412544573_d2b5ca33bd970f64a6301fa75ae2eb22.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412570542_57da346b8c057f411d5e11b66ffaf60b.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110412590519_442f8fd6ef494e2d1a30d10b4ae2e0e3.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/c9e0c958/2026110402331028_5c415d3e085bf59e329866efc4e0911d.jpeg)

Tencent Penguin Island Digital Sand Table

Prompt: Objective Build a 3D digital scale model of Tencent Penguin Island in Three.js. Deliver a single, self-contained HTML file that opens offline by double-clicking. Embed Three.js 0.184.0 directly in the file. It must have no remote dependencies, including CDNs, fonts, or textures. Model every building, road, green space, stretch of coastline, tree, person, and vehicle within the same 3D scene. Users must be able to rotate the view freely and walk through the island at ground level. A static bird’s-eye view, a scrolling landing page, or a scene rendered as a background image will not meet this requirement. Site and architecture The site occupies a peninsula that runs north–south through a bay. Use blue-gray water, white and silver buildings, a central green spine, and a road network that circles the island. Give particular attention to five architectural groups. 1. Tengyun Center (腾云中心) — Three low-rise, cloud-like buildings with raised cylindrical volumes and steel-truss skybridges. 2. Yunhai Towers (云海大厦) — Six staggered towers of varying heights, with parametrically modeled metal fins. 3. Penguin Apartments (企鹅公寓) — Eleven residential blocks offset in a sawtooth arrangement, with shared stepped terraces. 4. Tencent Helix (腾讯螺旋) — highest priority. Shape the tower from four facade surfaces. Each surface sweeps around continuously and broadens as it rises. At ground level, the facades turn inward to form J-shaped feet. The spiral must come from the geometry of the facades themselves. Do not add external rings or model the tower as four twisted ropes. 5. Science and Technology Museum (科技馆) — A continuous ring-shaped green roof encloses an inner courtyard. Display the Chinese annotation “图像拟合、非测绘” (“Modeled from imagery, not survey data”). Interaction Provide orbit controls and ground-level exploration in both first and third person. Show a penguin avatar in third-person mode. Include weather and day/night controls, a minimap, building details, floor-by-floor cutaway views, a cinematic guided tour, screenshot capture, and video recording. Interface Use light-colored frosted-glass control panels, clear Chinese typography, and a collapsible tool overlay. Night mode should change only the 3D environment. Keep the panels light and legible after dark, and avoid a black, neon-lit cockpit aesthetic.

## [](#finance-and-office-workflows)Finance and office workflows

To ground Hy4 in real-world financial work, we built thousands of realistic workspaces and tens of thousands of tasks spanning fund research, equity analysis, quantitative strategies, asset allocation, corporate finance, and tax accounting. Working with Tencent Financial Technology (FiT), we evaluated Hy4 with 213 finance professionals, including 153 experts from leading financial institutions. More than 70% of these experts have over five years of industry experience. Across 579 real-world financial tasks, Hy4 ranks first among open-weight models (Hy4: 3.04; DeepSeek-V4.1-Flash-1017: 2.97; Kimi K3.1 Pro: 2.68; GLM-5.3: 2.68).

On IRAB, an internal benchmark from financial AI agent company RabyteTech, Hy4 also scores 12.03 percentage points higher than Hy4 preview. To make Hy4 more reliable for financial and office workflows, we made targeted upgrades to search, focusing on time-sensitive retrieval, multi-source synthesis, and factual consistency verification. In internal evaluations, Hy4 cut the hallucination rate from 15.76% (on Hy4 preview) to 5.45%, and reduced speculative errors from 23.1% to 5.6%. We also improved Hy4’s ability to co-edit Word, Excel, and PowerPoint files with people in real time. In blind evaluations, Hy4 shows clear gains over Hy4 preview in document formatting, reliability, and overall usability.

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413054286_13f7730993133e2e8bc6a2152e6e1707.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413073154_729df0f23d8001165a680a4704869a6a.jpeg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413092572_576ecca397c25da99dcc09229979b5be.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413105949_b21b785631590bb85207ba13639acc0a.png)

Financial Valuation Modeling & Research Report Generation

Prompt: use global-equity-research and neodata to build a financial model and research report on Tesla. Deliver both the Excel model and the matching PDF report. Pay close attention to your valuation logic and to where every number comes from. You can search for and draw on research from multiple institutions. List the research firm as HY RESEARCH.

## [](#scientific-research-and-ai-rampd)Scientific research and AI R&D

With deep reasoning, scientific computing, and long-horizon execution, Hy4 pushes the frontier of end-to-end scientific research. On TenureTrail, our internal benchmark of 123 research tasks across seven disciplines, Hy4 ranks first among open-weight models. It shows strong capabilities in mathematical construction, physical modeling, earth sciences, systems engineering, and socioeconomic decision-making. We are also laying the groundwork for recursive self-improvement by advancing Hy4’s AI R&D capabilities. Paired with our [Hyra 2.0](https://hy.tencent.ai/research/hyra2) harness, Hy4 optimized our internal pre-training infrastructure for smaller models with the same architecture, and autonomously trained a variant that outperforms the official Jev baseline on multiple benchmarks. To make these AI-accelerated results reusable for the broader community, we are open-sourcing two foundation models co-developed by Hy4 and Hyra 2.0: HyCell for single-cell representation and HyPDE for physical field modeling.

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110400015543_23d03c33464f967b92db9b5e30c6ab1a.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402192830_964ebf3778ed952c8db86d57e1ad3431.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402200438_8c613f6a2453c45131249da7cd1bc9a8.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402202743_19dff19e882464abfd963f70a8ec7755.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402212194_218211c5f701f1190bccfb9f6deed1ce.jpg)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402213716_05c531fa72d7a95dfc01c529ee1d739c.jpg)

Hy4 Reconstructs a 3D Mouse Embryo from 7 Million Cells in 1.5 Hours on a Single Machine

## [](#long-horizon-personal-assistant)Long-horizon personal assistant

With stronger context learning capabilities, Hy4 can synthesize complex information from multiple sources. Through WorkBuddy’s integration with WeCom, Hy4 connects directly to everyday work context, including chat history, emails, documents, calendars, and meeting notes. When you’re managing multiple projects at once, Hy4 can track each project’s history and latest updates, surface action items, prepare meeting briefs, flag upcoming deadlines, and help prioritize tasks. Hy4-powered team collaboration features will also launch in WorkBuddy alongside the new version of Projects.

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413223759_fafd01aac799e6355dfb28b045fce72c.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413244849_6d977c6cc2885e481cd0b5b1da3a11eb.png)

![](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/a951c950/2026110413264495_d6668a3034f8d09da4d78d23603472d3.png)

Bring together what needs my attention across multiple sources

Prompt: Goal: Look at my chats (group chats, private chats and files), emails, calendar, meetings and documents. Use them to make a "Daily Focus" board of my tasks for each day. Give it to me as one HTML file that I can click around in and open without the internet. Don't just sum up my chats or list everything that happened. I want to see right away what matters today and what I should do first. Content: Pull the newest information from all of these places and remove anything that shows up twice. Find the things that matter in my work, my personal life and my daily chores. Sort them by how important and how urgent they are. For each one, say clearly what happened, where it stands now and what to do next. Write for anyone, in simple, everyday words. Don't assume the reader knows the background. Avoid jargon, inside terms, short forms, hard-to-follow sentences and the stock phrases AI tends to use. Keep the sources that matter.

# [](#product-feedback)Product feedback

![Manus](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110401453515_2a709c2caeeff98490a465548f2c17e0.png)


<!-- chunk 2 -->
In our internal evaluations, Hy4 delivered strong results across deep research, data analysis, and office document tasks, ranking among the top models overall. It performed particularly well in document creation and editing, achieving a task completion rate of nearly 97%. Hy4 also demonstrated strong instruction-following capabilities, accurately handling complex requirements such as citation formatting, heading numbering, and detailed revision instructions. For Manus users, this means less rework, faster execution, and more reliable results in real-world workflows.

Manus

![RabyteTech](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403103430_6c82b854d9c3e7d94c315d33d9e663a2.png)

As an AI service provider focused on investment research, we evaluated Hy4 on the IRAB benchmark for investment research agents, using 101 investment research tasks. Hy4 scored 61.63 overall, 12.03 points higher than Hy4 preview, showing clear progress among the models that we’ve been tracking. Hy4 is firmly in the top tier of Chinese models and has further narrowed the gap with leading proprietary models. The value of an investment research agent lies in delivering outputs that are truly usable, and Hy4 is improving quickly at completing investment research tasks end to end. Hy3 has already been integrated into some AlphaPai products, and we look forward to deepening our collaboration with the Hy team to push the frontier of AI in finance.

RabyteTech

02 / 06

![WorkBuddy](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403005137_38f7d1fa6bb1f062c21920b162d05167.png)

Across web tasks, visual creation, and 3D use cases, Hy4 strikes a much better balance between quality and speed. In multi-step browser tasks, it uses feedback from its actions to adjust its approach and verify results on its own. In our evaluations, Hy4 achieved an average score of 83.18 and a 96.15% pass rate, ranking #1 among Chinese models in 9 of 12 scenarios.

WorkBuddyAnne, Product Manager

![Yuanbao](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110403012828_b52b5e93c4dadeea0b56cb690a633d10.png)

On our internal evaluation set, Hy4 scored 72.53, up from 65.25 for Hy4 preview, with particularly strong gains in multi-turn conversation and writing. Multi-turn memory improved by 6.67 points, bringing better reference resolution and smoother flow across turns, while writing quality jumped 10.89 points with much greater content diversity. Compared with our previous VLM, Hy4 also boosted multimodal agent task completion by 36.8% and multimodal Q&A by 15.8%, delivering major gains in image understanding, joint image-text reasoning, and cross-modal context.

YuanbaoNyx and Ranmer, Product Managers

02 / 09

# [](#responsible-open-sourcing-and-release)Responsible open-sourcing and release

We recognize the potential safety risks posed by advanced cyber and biological capabilities, and we are committed to a more responsible approach to model training, open-sourcing, and release.

During Hy4’s training, we limited cybersecurity data to defensive material related to proof-of-concept (PoC) generation and vulnerability patching. We filtered out offensive content such as exploit development, reverse engineering, and penetration testing to avoid amplifying potential offensive capabilities. We also worked with experts from Tencent Xuanwu Lab and Tencent Keen Security Lab to conduct red-team audits covering cryptanalysis, reverse engineering, and vulnerability exploitation. Their findings indicate that Hy4 does not pose elevated risks in offensive tasks such as reversing cryptographic algorithms, defeating binary obfuscation, or exploiting vulnerabilities.

In biosecurity evaluations, Hy4’s open-ended experimental design capabilities remain well below our high-risk threshold, with a score of 30.4/100 on Hy-BioExperimentBench. Internal experts audited the model’s reasoning traces and found that although Hy4 has a solid grasp of foundational biology, its ability to turn that knowledge into actionable experimental protocols, a critical bottleneck, remains limited. We found no evidence that Hy4 meaningfully lowers the barrier to causing biological harm.

Hy4 is now available on Tencent Cloud and OpenRouter, as well as in products including CodeBuddy, WorkBuddy, Yuanbao, and ima. It delivers major performance gains over Hy4 preview at the same API price.

Hy4 API pricing (per million tokens)

Cached input

Input

Output

$0.042

$0.834

$2.501

We plan to open-source Hy4’s model weights and have partnered with the vLLM (Inferact Inc.) and SGLang communities to provide ecosystem support. When we release the weights, we will also publish a comprehensive system card detailing our safety evaluations and additional technical details.

# [](#looking-ahead)Looking ahead

Although Hy4 is one of the strongest open-weight models available today, a notable gap remains between it and top proprietary models, particularly on challenging internal benchmarks for research-level mathematics, long-horizon coding, model self-improvement, and advanced visual question answering. We will continue to steadily scale pre-training and post-training to narrow this gap.

![Bench2-Hy4-最终版（横版）.jpg](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402015231_ac89ad6ff52f8d565fa568ebfa6928d4.jpg)

We also plan to release Hy4 flash in the near future, with significantly improved cost-efficiency, faster inference, and closer co-design with products to improve utility.

# [](#appendix-benchmark-results)Appendix: benchmark results

![大Bench图表.jpg](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026110402023072_85b416b6484a88ef96ef6edda563b560.jpg)

目录

Unlocking real-world productivity Vision and computer use Frontend and game development Finance and office workflows Scientific research and AI R&D Long-horizon personal assistant Product feedback Responsible open-sourcing and release Looking ahead Appendix: benchmark results

Comments（0）

0/2000

### Keep Reading

![Hyra 2.0: The Next Step for Auto Research](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/210b8ba1/2026102517260212_cfa89033e01cdb8528854792d1ac71b6.jpg)

Hyra 2.0: The Next Step for Auto Research

Nov 1, 2026Hyra Team

![When Do Larger Batches Help Scale LLM Reinforcement Learning?](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/71e4f885/2026091812582093_478a0e952aed78993dfd31c8a980a9b6.jpg)

When Do Larger Batches Help Scale LLM Reinforcement Learning?

Sep 21, 2026RL Team

![Introducing Hy4 preview](https://hy-model-ap-prod-1258344703.cos.ap-guangzhou.myqcloud.com/llm-blog/default/9a5ea02f/2026082914164024_a99d8e4fdac07d233f015c0206772b08.png)

Introducing Hy4 preview

Aug 27, 2026Hy LLM