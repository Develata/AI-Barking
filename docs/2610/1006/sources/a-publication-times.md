# A 组时间核验

本轮核验发生在北京时间 2026-10-07 凌晨（本机仍为 2026-10-06 EDT）。期次目录仍按派工使用 1006，不把抓取时间当新闻发布时间。
只给日期、没有时区或时刻的来源不强行换算。下列“读取”指 web 工具原站抽取；搜索索引与原站读取严格分开。

| 发布方/页面 | 北京时间 UTC+8 | 原始显示 | 核验边界 |
|---|---|---|---|
| WINK News 首发 | 2026-10-01 04:22 | datePublished=2026-09-30T16:22:00-04:00 | 收尾外部补入a-wink.html第1451行，本执行方只读核对元数据；首次联网读取仍失败 |
| WINK News 更新 | 2026-10-02 08:35:13 | dateModified=2026-10-01T20:35:13-04:00 | 同HTML第1452行；不是首发时间 |
| Gulf Coast News / WBBH 首发 | 2026-10-01 12:13 | datePublished=2026-10-01T04:13:00Z | 外部补入a-gulfcoast.html meta/JSON-LD一致；旧索引曾将同时间显示为Updated |
| Gulf Coast News / WBBH 更新（元数据） | 2026-10-02 23:58:59.841 | dateModified=2026-10-02T15:58:59.841166Z | 外部补入HTML第336行，与可见更新时间不一致，保留两者 |
| Gulf Coast News / WBBH 更新（可见文字） | 2026-10-02 21:16 | Updated: 9:16 AM EDT Oct 2, 2026 | 外部补入a-gulfcoast.txt注明的页面显示；没有拿它覆盖HTML元数据 |
| TechSpot | 未能换算 | October 4, 2026, 8:37 | 原站可读，未显示时区；未取得 datePublished 元数据 |
| The Verge | 未核实 | 仅不采用的代理搜索结果给出 2026-10-05 15:00 UTC | 原站两次读取失败；23:00 北京时间仅为该未核时刻的换算，不得用于发布 |
| India Today，普通页面 | 2026-10-06 10:59 | Oct 6, 2026 08:29 IST | 原站索引和正文入口；不能证明为首次发布时间 |
| India Today，AMP 页面 | 2026-10-05 16:14 | UPDATED: Oct 5, 2026 13:44 IST | 原站 AMP 搜索索引；明确是 UPDATED。两版本不一致，均保留，不择一冒充首发 |
| Cybernews | 未能换算 | 站内列表 October 5, 2026 | 正文抽取未显时刻；不能确定首发/更新 |
| Inc. | 未能换算 | Oct 5, 2026 | 原站正文日期，无时刻/时区 |
| Futurism | 2026-10-06 08:52 | Published Oct 5, 2026 8:52 PM EDT | 原站搜索索引，open 失败；不得标为已取得完整页面 |
| Pasquale Pillitteri 英文篇 | 未能换算 | 04/10/2026 | 原站搜索索引，未给时刻/时区；open 失败 |
| IT之家 | 2026-10-05 13:00:25（按中国站默认时区解释） | 2026/10/5 13:00:25 | 原站搜索索引；页面未显时区，时区解释是一项假设；open 超时 |
| 新浪科技 | 未取得具体时刻 | URL 日期 2026-10-05 | 仅定位到标题/链接；未使用其转载正文 |
| Anthropic 隐私政策 | 官方未给时刻 | Effective September 10, 2026 | 生效日期，不是本案声明时间 |
| Anthropic 政府请求政策、执法请求说明 | 官方未给时刻 | March 16, 2026 | 页面日期，不推断最后修改的具体时刻 |
| Anthropic Usage Policy | 官方未给时刻 | Effective September 15, 2025 | 生效日期 |
| Anthropic 消费端训练开关说明 | 官方未给时刻 | August 3, 2026 | 页面日期 |
| Anthropic 个人数据训练说明 | 官方未给时刻 | March 16, 2026 | 页面日期 |
| Anthropic Interviewer 2025-12 场次说明 | 官方未给时刻 | March 26, 2026 | 页面日期 |
| Anthropic Covered Models 保留说明 | 官方未给时刻 | September 5, 2026；文内政策生效 June 9, 2026 | 两日期含义不同；不能把 covered-model/ZDR 规则直接套给普通消费端 |
| Anthropic 透明度主页 | 官方未给时刻 | Last updated July 23, 2026 | 最后更新日期 |
| Anthropic 政府请求报告 | 官方未给时刻 | July 2025–December 2025 | 统计期，不是发布日期 |
| OpenAI US privacy policy | 官方未给时刻 | Updated: September 10, 2026 | 更新日期 |
| OpenAI 帮助危机用户博文 | 官方未给时刻 | August 26, 2025 | 发布日期 |
| OpenAI 执法政策 PDF | 官方未给时刻 | v2025.12 | 版本号，非精确发布时间 |

对应原站 URL 与访问结果见 capture-log-a.md。除收尾外部补入的WINK/Gulf Coast HTML外，其余网页未取得本地完整HTML/DOM快照。补入材料的隐私问题及核验边界见a-supplement-review.md。
