# 1021 B 组采集日志

- 2026-10-21 14:51:43+08:00：按 templates/codex-evidence.md 使用 barking fetch，清单 b-fetch-list.tsv，前缀 b。三个原站均 HTTP 200，ok；总用时约3秒。完整结果见 b-fetch-log.jsonl。
- 随后打开并逐一阅读全文 .txt，核对研究限定、日期、收购完成时态、Chrome 开关条件。
- 网页读取交叉检查：Google、OpenAI 成功；Anthropic 在该网页读取工具显示不可访问，但直连 barking 原站及 agent-reach 的 Jina Reader 均取得完整正文。未将工具不可访问误记为原站无来源。
- Jina Reader 仅针对 Anthropic 追加读取，保存 b-anthropic-scorpion-jina.txt；事实主档仍为 barking 原站 .html/.txt。
- 逐一查看 HTML 日期元信息：Anthropic 发布字段为00:00 UTC，保留可能日期占位的不确定性；Google 17:00 UTC 换算北京10/21 01:00；OpenAI 仅确认页面日期10/20。
- daily-scan.md 去重未见三事件；未增加候选或采集截图；未改公共稿件、公共事实清单或图片说明。
- 本组仅文本证据交付；HTML 原件按仓库约定不入 Git，本任务未上传网盘、未提交、未推送。
