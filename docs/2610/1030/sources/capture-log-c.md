# C 组抓取与截图日志（1030 期）

执行：claude-sonnet-5-5；时间北京时间（UTC+8）2026-10-31。机器可读日志：`c-fetch-log.jsonl`（barking fetch）、`c-shots-log.jsonl`（barking shot）、`c2-heat.json`（barking heat；`c-heat.json` 是第一次运行，搜索窗口取错、0 条，作废）。

## 取证页面（barking fetch，HTTP 状态均为 200 除注明）

| 时刻 | 名称/URL | 工具 | 结果 |
|---|---|---|---|
| 16:49 | 苹果新闻稿 https://www.apple.com.cn/newsroom/2026/10/apple-intelligence-features-are-coming-to-users-in-china/ | fetch | ok，正文完整；元数据 pubDate 2026-10-30T20:59:25Z |
| 16:49 | 苹果新闻稿 RSS https://www.apple.com.cn/newsroom/rss-feed.rss | fetch | ok |
| 16:49 | Apple 智能中国页 https://www.apple.com.cn/apple-intelligence/ | fetch | ok（含千问两段与脚注） |
| 16:52 | 同花顺转南方都市报 http://news.10jqka.com.cn/20261031/c681321335.shtml；21财经转上海证券报；澎湃号（日照日报）；IT之家 1/020/392；Global Times 1373351；China Daily；9to5Mac | fetch | 均 ok |
| 16:57 | 新浪转文汇报、都市快报、21世纪经济报道；凤凰网；AIbase；钛媒体；21经济网（库克 10-15） | fetch | 均 ok |
| 16:58 | Reuters 库克 10-15 | fetch | blocked（HTTP 401），未重试，用 TechNode、21经济网代替 |
| 16:58 | TechNode；Global Times 1372342 | fetch | ok |
| 16:59 | IT之家 0/977/109（7 月）；站长之家 7 月 | fetch | ok |
| 17:03 | 新浪科技（TechWeb）；新浪微博页 2 个（用户发帖页，见待删除） | fetch | ok |
| 17:03–17:04 | 新浪转 IT之家；IT之家 1/020/389（05:02） | fetch | ok |

## 阿里官方渠道（未取到）

- alibabagroup.com 新闻页：16:51 headless Chrome 渲染成功，最新条目 2026-09-28，无苹果稿（页面已目视确认）。
- alibabanews.com（16:52）、alizila.com（16:53）：headless Chrome 返回 Cloudflare 拦截/验证页（curl 同为 403），按规定不绕过，失败。
- 阿里微信公众号原稿：公众平台页不公开抓取，未尝试。

## 截图（barking shot render + cropbox，headless Chrome，DPR 2，700 CSS px；页面均为浅色底，29 为深色底）

25–35 号图的来源 URL、像素框见 `c-shots-log.jsonl`。逐张目视确认：25、26、27、28、29、30、31、32、33、35 文字清楚、范围正确；30 顶端有一小截苹果 logo；34 末行被切，弃用。未目视逐张核对但同页：中间渲染图 ab-news、abn-home、alizila-home（阿里站）、sjb-alibaba（21财经移动页被“点击查看完整内容”截断，未入 images）、ith-july。

## 社区热度（barking heat）

- HN：5 组查询，自 2026-10-14 起；命中与本事件相关 2 条（50115269 2 分 0 评 10-16；50169113 1 分 0 评 10-21）；10-31 发布的新闻未见 HN 帖。
- Reddit：2 个请求（第一次运行）被 old.reddit 登录页拦截，失败，未重试。
- 微博/小红书：无稳定公开页，未取数。

## 待删除文件（精确路径，均在 E:\gitclone\AI-Barking\docs\2610\1030\）

- sources\c-heat.json（作废的首次运行）
- sources\c-sina-weibo-quanmian-launch.html、.txt、.headers（微博个人用户发帖页的新浪镜像，含私人账号昵称，不入库）
- sources\c-sina-siri-lai-le.html、.txt、.headers（同上，新浪微博页，未使用）
- images\34-sina21-alibaba-statement.png（末行被切，已由 35 取代）

## 假设

- 发布时刻以苹果页面 JSON-LD `pubDate` 为准（正文无时刻）。
- “IT之家 05:02 版”取自 IT之家原站 1/020/389（页面标 2026/10/31 5:02:33）。
