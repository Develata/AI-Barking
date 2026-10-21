# 1020 采集与制作总记录

发布后归档：用户指定1020期；目录由用户移动，统筹将正文及派工单文件名、索引和卡片栏目日期同步。采集/审稿原始时间保持2026-10-21。原10月21日省流与速览图另存sources/pre-issue-rename-tldr.png和sources/pre-issue-rename-roundup.png；正式卡片重新渲染，仅栏目日期改变，原始取证底图未改。

- A组：capture-log-A.md / a-fetch-log.jsonl / a-shots-log.jsonl。OpenAI原始HTML和文本成功；SEC直抓403，原站浏览器图表成功，另留渲染DOM与Jina转换文本。
- B组：capture-log-B.md / b-fetch-log.jsonl，三条官方速览HTML与文本成功。
- 热度：b-heat-notes.md及b-heat.json，记录北京时间采样。
- 统筹派生裁片：c-shots-log.jsonl。02由测试段与否认答复原图纵向拼接；03/06由同页真实截图拆分为更改/重置两图。所有裁片只裁切拼接，不重绘文本。
- 图像生成：内置imagegen，四张候选都留在仓库外generated_images目录；提示词、选用输出ID在images/README.md。选用两张复制为00-cover/00-cover-wide；未对生成结果另改字。
- 模板渲染：barking card，省流/速览/六张批注，最终1080×1440。每张已打开核对高亮，特别核对财表当前季度和九个月累计不是相邻期间。
- 原站截图限制：OpenCLI回报物理DPR1；OpenAI用CSS zoom2，SEC用1.55以容纳完整表。最终文字可读，未声称严格硬件DPR2；截图方法与例外作为可追溯技术条件留档。
- 首轮独立文字审稿：本机Claude Sonnet 5.5，仅Read/Grep/Glob、dontAsk、no-session-persistence；输出review-text-claude.txt。未启用Bash/Write/Edit，不改用户文件。
- 内容修改：保留sources/draft-before-review.txt作为内部历史版本；当前发布正文不含Markdown或来源行。
- 最终跨模型审查：review-final-claude.txt逐张打开16图；review-fixes-claude.txt复核3张改图，两项SHOULD_FIX已解决，最终PASS。模型均为Claude Sonnet 5.5，工具仍仅Read/Grep/Glob。
- 2026-10-21，17张images/*.png按oxipng -o 4 --strip safe无损压缩：7,299,049→6,267,741字节，减少14.13%；未使用-a，sources中的取证原件未压缩。压缩后生成offsite.tsv，共35个本地原件；OpenList远端上传留待用户“已发布”。
