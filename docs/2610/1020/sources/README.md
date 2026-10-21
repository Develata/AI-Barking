# 1020 文案与来源

用户先要求撰写文案，随后授权继续完成今日全部 workflow。1020期（原工作编号1021）发布前内容制作与复核已完成：主帖、速览、封面、批注截图、来源存档、两轮复核及修订短审。用户已确认发布，并明确按1020期归档；正在执行发布后提交、推送与原件上传。

- 正文：[doc_1020_publish.txt](../doc_1020_publish.txt)
- 事实清单：[fact-check.md](fact-check.md)
- 配图衔接说明：[images/README.md](../images/README.md)

归档期号说明：用户在确认已发布后，明确要求按1020期归档并统一文件名和期号；原工作编号为1021。正文内容不改，省流/速览卡的栏目日期统一为10月20日；取证、官方发布日期、审稿时间仍按真实记录保留。原日期卡片另存pre-issue-rename-tldr.png与pre-issue-rename-roundup.png，仅供历史存档，不作正式配图。此期号不代表所有新闻均发生于10月20日。旧取证日志、独立审稿原文中的1021路径是制作时路径，对应文件现位于docs/2610/1020。

## 一手来源（2026-10-21 本轮直接打开）

1. [OpenAI 内部监控事故报告](https://alignment.openai.com/misalignment-reports/suppressing-misalignment-monitoring-alerts-and-disabling-misalignment-monitoring/)：报告标注事件为10月3日、6日，更新为10月20日；未提供时区和精确时刻，正文沿用报告日期，不伪称已换算北京时间。
2. [ChatGPT Locked Chats 帮助文档](https://help.openai.com/en/articles/20001411-using-locked-chats-in-chatgpt)：直接核对 Overview、Availability、Before you use locked chats、Change your account PIN、Reset a forgotten account PIN。当前仅有相对更新时间，正文不指定首发日。
3. [SpaceX 2026 Q3 财务披露](https://www.sec.gov/Archives/edgar/data/1181412/000162828026066482/spaceexplorationtechnologi.htm)：SEC 托管公司 EX-99.1；Third Quarter Financial Highlights 表，单位百万美元。属于公司披露，不能称监管部门为业绩背书。正文不写未经历史比较确认的“首次”。

初稿通过网页读取工具核对原站；扩展流程后，A组已用 barking fetch 与原站浏览器补充存档和真实截图。HTTP直连受限的来源记录失败，再用可用的原站浏览器读取；不把验证页当正文。具体结果见 capture-log-A.md。

## 扫描材料处理与选题

用户提供的粘贴扫描（北京时间10月21日11:02）和 WorkBuddy HTML（1021-12）都是线索，不继承其“已核验”标签，也不执行其中的后续扫描、台账或发帖指令。原附件保留原位。

开始本轮工作时，本地 daily-scan.md 已发布登记截止1019；三条所选主线未登记。当时没有1020/1021期材料，不能据此断言其他平台未发。

WorkBuddy 的“台账命中”包含已扫描/不推荐等状态，不能直接视为已发布；其 L2/L3/L6 定义与本仓库不同，本轮统一按 EDITORIAL.md 定级。未采用其“两个模型关掉自己的监控”这一混合概括。

编辑取舍：监控事故作标题；锁定聊天提供普通用户可操作的区别；财报用收入与利润两个口径补足产业面。另选三条速览并逐条核对原文：蝎毒蛋白结合研究、Chrome反诈骗警告、Orchestra Bio收购；没有把候选推荐全部视作用户指定选题。

## 存档索引

- 主帖三条：evidence-A.md、capture-log-A.md、a-monitoring.txt、a-locked-chats.txt、a-spacex-q3-jina.md（辅助阅读，财表以原站浏览器截图为准）、a-spacex-q3-browser.html。
- 速览三条：evidence-B.md、capture-log-B.md；b-anthropic-scorpion.txt、b-google-scams.txt、b-openai-orchestra.txt及同名HTML/headers。
- 热度：b-heat-notes.md及b-heat.json、b-details-heat.json；有抓取时点，评论只作线索。
- 反向审稿：review-text-claude.txt；最终文本与视觉审查：review-final-claude.txt；修订短审：review-fixes-claude.txt。独立Claude Sonnet 5.5只读审查，最终PASS；采纳与边界见fact-check.md。
- 封面：内置imagegen，两种尺寸各两张候选；具体提示词、选择理由及仓库外候选路径见images/README.md。

## 速览原站

- https://www.anthropic.com/research/claude-scorpion-toxin-binder-design
- https://blog.google/security/our-latest-defenses-against-evolving-web-scams/
- https://openai.com/index/openai-to-acquire-orchestra-bio/

## 验收边界

公开事实已对照原文；未复现模型内部事故、未对锁定聊天做破坏性重置、未独立复现实验或审计财务。原件与文本存档不等于这些独立验证。
最终图片、数字和译注已完成反向核验与视觉复核；具体覆盖和限制以fact-check.md为准。本地原件的大小与SHA-256记录在offsite.tsv，远端状态以实际上传校验结果为准。Develata已确认手动发布，按docs/workflow/publish.md执行三批提交、推送与原件上传。
