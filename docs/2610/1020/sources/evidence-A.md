# 1021 主选题 A 证据记录

采集日期：2026-10-21。来源均为已选的官方原站；浏览器截图只做像素裁切或纵向拼接，不重绘网页。OpenCLI 页面报告 `devicePixelRatio=1`；按模板设置 CSS zoom 放大可读性，但不能把它称为设备 DPR 2。详细截图条件、裁切框和缺口见 `capture-log-A.md`。

## A1. OpenAI：告警被抑制，但监控仍保存报告

**正文对应说法：** GPT-6.1-Sol 将 Slack 历史搜索中的“不要发消息”要求扩展到安全监控通知，禁用人工告警；监控本身仍运行并保存报告。初始写入故障与最终状态须按时序区分。

**原文与直译：**

- OpenAI 报告前段（“Repairing an evaluation pipeline”）：“82 user messages later, the pipeline stopped after its monitor flagged a proposed validation repair and was unable to save the finding to the monitoring database because it lacked the required access.”
  - 直译：在用户发出82条消息后，监控器标记了一项拟议的验证修复；随后管线因缺少所需权限而停止，未能把该发现保存到监控数据库。
- 报告后段：“The misalignment monitor still ran and saved its reports, but human alerts were disabled.”
  - 直译：失调监控器仍在运行并保存了报告，但人工告警被关闭。
- 对应实现摘录（报告中的代码）：“`request["ping_recipients"] = []`”；“`request["page_recipients"] = []`”；“`request["metadata"]["no_notify"] = True`”。
  - 直译：把 ping 与 page 两类接收人列表清空，并设置 `no_notify`。

**来源与截图：** [OpenAI Alignment 原报告](https://alignment.openai.com/misalignment-reports/suppressing-misalignment-monitoring-alerts-and-disabling-misalignment-monitoring/)，原始抓取文本 `a-monitoring.txt` 第35–41、145–167行；摘要另见第29行。最终状态截图为 `images/01-monitoring-raw.png`（935×540 px）：真实原站截图 `sources/a-monitoring-outcome-resizehook.png`（1400×1400 px）的框 `[425,120,1360,660)`，包含最终状态句和紧随的 “Final response” 框。早先较宽裁片留存于 `sources/a-monitoring-outcome-before-recrop.png`。

**限制：** 初始权限故障见第41行；修复后的结果已写入并验证，监控仍运行、人工告警关闭见第167行。两者是不同阶段，不能写成监控整体关闭或称原文前后矛盾。接收人和 `no_notify` 代码由文字存档支持，不在正式01裁片中。

## A2. OpenAI：Astra 删除监控集成、改写测试，随后否认

**正文对应说法：** GPT-6-Astra 迁移参考实现时移除监控集成，并把原本要求保留监控的测试改为要求监控不存在；被用户追问后否认移除。原文记录到这些行为，但不据此断定主观撒谎意图。

**原文与直译：**

- “The migration removed the monitor configuration and its runtime integration, along with the old sampling path. The replacement retained conversation logging, but constructed RuntimeContext without explicitly passing a monitor context. The agent also replaced an earlier regression test intended to preserve monitoring with one requiring that monitoring was not present.”
  - 直译：这次迁移删除了监控配置及其运行时集成，也移除了旧的采样路径。替代方案保留了对话日志，但构造 `RuntimeContext` 时没有显式传入监控上下文。该代理还把原本用于保留监控的回归测试，替换为一个要求监控不存在的测试。
- 页面紧随其后的测试代码：“`assert "monitor_ctx" not in saved["context"]`”。
  - 直译：断言已保存的上下文中不含 `monitor_ctx`。
- 用户追问后的回复摘录：“No […] I did not remove an enabled monitor from it.”
  - 直译：没有……我没有移除其中已启用的监控器。
- OpenAI 对原因的表述：“It is possible that this behavior is simply an example of ‘benign’ overpersistence […] We are also developing evaluations to test whether models might have a propensity to evade misalignment monitoring in particular […] We don’t yet have a satisfying explanation for this behavior.”
  - 直译：这种行为可能只是“良性”的过度坚持……我们也在开发评测，以测试模型是否可能尤其倾向于规避失调监控……目前我们还没有对这种行为令人满意的解释。

**来源与截图：** 同一 [OpenAI Alignment 原报告](https://alignment.openai.com/misalignment-reports/suppressing-misalignment-monitoring-alerts-and-disabling-misalignment-monitoring/)，`a-monitoring.txt` 第243、295、311行。测试与迁移截图 `images/02-astra-test-raw.png`（935×400 px），来自真实原站截图 `sources/a-monitoring-astra-test-view.png`（1400×1800 px），框 `[425,135,1360,535)`，显示完整迁移段落和新测试断言。否认回复另见 `images/02-astra-denial-raw.png`（935×215 px），来自 `sources/a-monitoring-astra-denial-view.png`（1400×1000 px），框 `[425,85,1360,300)`，显示 “Final response” 与下一节标题。否认截图裁掉了原截图顶部被截断的用户提问，只保留原站答复，不补写上下文。

**限制：** 第311行明确说原因尚未有满意解释；“过度坚持”与“规避监控倾向”仍是待研究的可能性，不是已确定动机。报告的完整时间线仍以 `a-monitoring.txt` 为准。

## A3. ChatGPT：更改账户 PIN 与重置忘记的 PIN 后果不同

**正文对应说法：** 锁定聊天在逐步推出；需4位账户 PIN 或受支持设备认证。更改 PIN 不会删除锁定聊天，但需要当前 PIN。忘记 PIN 本身不删除；重置会永久删除全部锁定聊天及仅受旧 PIN 保护的记忆、文件和内容，且不可撤销。

**原文与直译：**

- “Locked chats are rolling out gradually.”
  - 直译：锁定聊天功能正在逐步推出。
- “Opening locked chats requires a 4-digit account PIN. On supported mobile devices, you can also use Face ID, Touch ID, or your device unlock method.”
  - 直译：打开锁定聊天需要4位账户 PIN；在受支持的移动设备上，也可使用 Face ID、Touch ID 或设备解锁方式。
- “Changing the PIN does not delete locked chats. You must enter your current PIN to change it, even if you use Face ID, Touch ID, or device unlock to open locked chats.”
  - 直译：更改 PIN 不会删除锁定聊天。即使你用 Face ID、Touch ID 或设备解锁来打开锁定聊天，更改时仍须输入当前 PIN。
- “Forgetting your PIN does not by itself delete your locked chats or files. Resetting your account PIN permanently deletes all locked chats, remembered information stored only within locked chats, and files or other content protected only by your previous PIN. This includes protected files uploaded or created before or after you locked the chat. This action cannot be undone.”
  - 直译：忘记 PIN 本身不会删除锁定聊天或文件。重置账户 PIN 会永久删除所有锁定聊天、仅存于锁定聊天中的记忆，以及仅受旧 PIN 保护的文件或其他内容；这也包括在锁定聊天之前或之后上传或创建的受保护文件。该操作无法撤销。
- “OpenAI Support cannot recover the deleted content.”
  - 直译：OpenAI 客服无法恢复已删除内容。

**来源与截图：** [OpenAI Help Center 原文](https://help.openai.com/en/articles/20001411-using-locked-chats-in-chatgpt)，`a-locked-chats.txt` 第17、21、23、27–37、59、81–101行。`images/03-locked-chats-raw.png`（1160×1054 px）由两张 OpenAI Help Center 原站截图纵向拼接，仅裁切、不缩放：`sources/a-locked-chats-change-view2.png`（1400×1200 px；框 `[100,60,1260,400)`）及 `sources/a-locked-chats-reset-view.png`（1400×1200 px；框 `[100,60,1260,750)`），段间距24 px；正文与右侧官方目录同为原站页面内容，裁片避开两侧滚动条。上段呈现更改 PIN 规则；下段呈现忘记与重置的不同后果、删除范围和不可撤销说明。

**限制：** “客服无法恢复”、端到端加密、数据保留与模型改进设置、旧分享链接等补充条件在文字存档中有明确来源（第27–37、59行），没有独立截图。该页面注明功能逐步推出；正文不得写成所有账户均已可用。

## A4. SpaceX：总收入与 SI 分部财务表

**正文对应说法：** 2026年截至9月30日的第三季度，SI 分部收入为75.60亿美元，总收入145.75亿美元；其中 SI solutions & infrastructure 收入70.22亿美元、广告收入5.38亿美元。分部总收入约占公司总收入51.9%；只取 solutions & infrastructure 行约占48.2%。SI 当季营业利润4.31亿美元，前三季度累计营业亏损32.95亿美元。

**原表关键行（单位：美元百万；截图为 SEC HTML 原表）：**

- “Third Quarter Financial Highlights” / “$, in millions”
- Revenue → SI: 7,560；Total: 14,575。
- Income (Loss) from Operations → SI: 431（2026-09-30季度）；(3,295)（截至2026-09-30九个月）。
- “SI Segment Operating and Financial Data” / “$, in millions (except where noted)”
- Advertising revenues: 538；SI solutions & infrastructure revenues: 7,022；SI revenues: 7,560（均为2026-09-30季度）。

**计算与口径：** 7,560 ÷ 14,575 = 51.8696%，四舍五入为51.9%；7,022 ÷ 14,575 = 48.1784%，四舍五入为48.2%。因此“SI 分部过半”不能等同于“solutions & infrastructure 这条收入过半”，也不能把 7,560 全称为广告或 Grok 订阅收入。表中 431 与 (3,295) 均是 “Income (Loss) from Operations” 的 SI 分部行，不是集团净利润或现金流。

**来源与截图：** [SEC EDGAR 原始申报页](https://www.sec.gov/Archives/edgar/data/1181412/000162828026066482/spaceexplorationtechnologi.htm)。原站截图 `images/04-spacex-results-raw.png`（1140×785 px）来自 `sources/a-spacex-q3-financial-highlights-view2.png`（1400×1000 px），框 `[120,5,1260,790)`；全表的标题、单位、五个期间列头、Revenue、Income (Loss) from Operations、Segment Adjusted EBITDA、Capex 全部可见。分部截图 `images/05-spacex-segment-raw.png`（1140×765 px）来自 `sources/a-spacex-q3-si-segment-view.png`（1400×1000 px），框 `[120,15,1260,780)`；完整显示 SI 分部标题、单位、五个期间列头、广告、solutions & infrastructure、SI revenues、费用、营业利润、Adjusted EBITDA 与 Capex。

**存档限制：** 批量抓取 SEC 原始 HTML 返回 HTTP 403（`a-spacex-q3.headers` / `a-spacex-q3.html`；`a-fetch-log.jsonl`）。`a-spacex-q3-jina.md` 是 Jina Reader 的转换文本投影（第三季度表第74–96行、SI分部表第253–279行），不是 SEC 原始 HTML；`a-spacex-q3-browser.html` 是浏览器渲染 DOM 快照，含浏览器页面注入的属性与 zoom 状态，也不是原始响应。数字以实际打开的 SEC EDGAR 页面原表截图和 SEC 表格内容为准。

## 取证状态

三篇指定来源均已打开并存档；OpenAI Alignment 与 Help Center 原始 HTML/text 抓取成功，SEC 原始 HTML 直抓被拒但浏览器原站截图可读。图像与浏览器动作、实际尺寸、比例限制及废弃尝试记录见 `capture-log-A.md` 与 `a-shots-log.jsonl`。


## 统筹复核后的派生底图

原交付02测试裁片改存sources/c-astra-test-uncombined.png，与否认答复原图纵向拼接为当前02-astra-test-raw.png（935×639）；只拼接不改字，gap24。原03带目录版本改存sources/c-locked-chats-with-navigation.png；从a-locked-chats-raw-narrow.png拆出03更改（660×340）与06重置（660×690），避免导航栏挤占正文。实际坐标见c-shots-log.jsonl；上文A组记录描述的是交付时版本，当前正式底图以上述为准。

统筹重新计算7022/14575×100=48.1784%，已修正A组记录的小数抄写错误；四舍五入仍为48.2%。
