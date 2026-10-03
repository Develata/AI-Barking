# 搜索与方法范围

本轮 A/B/C 取证按指定网站进行；使用agent-reach平台路由、OpenCLI浏览器、web搜索与公开HN Algolia API。没有登录、读取/导出Cookie或使用镜像/代理域名。用户指定允许的现有浏览器会话用于只读页面。

C组web搜索：Cloudflare Clef The Register October 2026 training data；Cloudflare Clef 39 毫秒 全开源 Jev；site.theregister.com/2026/10/01/ clef Cloudflare。共3个查询、2次工具调用，用于发现原站URL，最终事实回读原站。OpenCLI Reddit一次（Clef OR Jev、LocalLLaMA/top/month），X一次（Clef from:Cloudflare），均Failed to fetch；没有为修复工具而越过仅本期目录的写入范围。依opencli-autofix检查过范围约束，未改适配器、未创建上游Issue。A/B及补充搜索关键词与结果见各组日志/搜索存档。

C组HN Algolia搜索各一次：Clef、Jev；另读取Clef帖子评论树一次。评论points=null不作0分。Jev Reddit首发未定位，解释贴分开记录。

OpenCLI可用；agent-reach实际PATH入口可用，技能示例conda dl不存在。check-update返回v1.5.0已最新。未更新任何软件。

若干包含整页通用脚本的执行被自动审批以approval required拒绝，未产生抓取；后续改为对已确认公开原页的具体标题、正文、表格进行范围更窄的只读提取成功。未请求或获取额外权限。两次社区榜原生CDP大区域captureScreenshot超时/中止，改为原生viewport截图；最终结果以实际图及验收记录为准。

所有截图为原站像素，DPR2；宽限制依用户要求先读的模板按CSS像素计，最大1400CSS=2800物理像素。未用插值放大冒充DPR2。HuggingFace顶部会话头像最初在预览中发现，最终重新截取公开内容区域，不保留该顶栏。原站HTML无Cookie HTTP抓取；登录态浏览器仅存公开正文范围，未存header/account/avatar。
