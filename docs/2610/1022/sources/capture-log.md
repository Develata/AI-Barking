# 1022 采集与制作日志

- 2026-10-23：保留用户1710文字扫描及1802 HTML扫描，使用barking html2txt生成文字副本。输入均标为线索。
- 19:18北京时间：barking fetch批量7条，6条ok、Canva官网403 blocked；日志preflight-fetch-log.jsonl。403文件保留为失败记录，不作原文。
- OpenCLI doctor：daemon正常，Extension未连接。按取证模板直接转barking shot，无浏览器登录/凭据采集。
- A组Luna：Anthropic根页面原图与SoftBank移动响应式原图，坐标见a-capture.md，动作见a-shots-log.jsonl。空白锚点和截断桌面页均未采用。
- B组Astra：Canva官网正文经网页阅读工具取得，原页截图遇验证页；转官方开发者社区截图，另存独立榜单现场局部作内部核对。见b-evidence.md。未把局部榜单作为完整benchmark发布。
- 19:20北京时间：HN热度查询，结果preflight-heat.json；日期参数首次格式错误后改用Unix秒，记录未冒充成功。
- 主代理：逐张打开原始底图、卡片；省流初版溢出82px后缩短逐字摘句重渲；Claude运行条件OCR字母低置信度后改手工逐行坐标。
- Claude跨家族审读尝试：本地设置规则无效，使用更窄只读工具和仅用户设置后OAuth过期，未完成；失败输出reverse-review-sonnet.txt。
- Astra独立模型只读文本审读通过，来源索引MINOR已采纳。记录不冒称跨模型家族核验或数学证明复跑。
- 最终竖封面2候选选A，横封面4候选选D；主代理与Astra实际查看横版及819×819居中预览，文字/胸牌/正常双手/中央裁切均通过。两轮视觉审读均无阻断项。
- 重新以当前仓库源码构建barking并运行lint：标题12/20字、全文936/1000字，0错误、0警告。git diff --check无输出；工作区只有新增docs/2610/1022，未改旧期或根目录索引。待入库文本均未超过1MB。

以上为首次交付时的记录，当时尚未更新已发索引、提交推送或上传OpenList。

## 后续文案审读与发布确认

- 用户要求“自行review并优化一下文案”后，主代理完成自审，正文调整为标题17/20字、全文900/1000字，省流卡同步更新；lint为0错误、0警告。修改依据及前版留存见self-review.md、doc-1022-before-self-review.txt；本次自审未冒称新增独立复核。
- Develata随后在对话中确认“已发布”。按docs/workflow/publish.md启动本期发布后流程，登记1022索引、主帖去重摘要与三条非主帖速览；期号仍为1022，新闻与取证日期仍为2026-10-23。
