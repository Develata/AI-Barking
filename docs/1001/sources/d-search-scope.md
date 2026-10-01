# 搜索与失败边界（1001）

本次原站获取使用 agent-reach 路由检查、OpenCLI 浏览器/Reddit、HN Algolia API、web 搜索。搜索引擎只作发现线索，事实摘句取本地原站档案。执行期按实时时钟换算为北京时间2026-10-02凌晨；早期搜索没有逐次秒级日志，不能伪造。页面档案时刻由capture-log.md给出。

## 官方回应和发布时刻

- `Moonshot Kimi response OpenAI September 30 2026 distillation`
- `月之暗面 回应 OpenAI 蒸馏 2026 10月1日`
- `site:moonshot.ai OpenAI distillation September 2026`；此前亦检索moonshot.cn。
- `site:weibo.com Kimi智能助手 OpenAI 蒸馏 回应 2026`：搜索服务提示weibo.com被robots限制；没有把这当成“官方无回应”。
- `site:mp.weixin.qq.com 月之暗面 OpenAI 回应 2026 9月30`：未定位相关官方公众号原文；没有完整公众号历史访问。
- `site:x.com/OpenAI "distillation" "September 30, 2026"`：未定位官方事件帖。
- `site:x.com/AnthropicAI "What work can robots do" "2026"`：未定位官方事件帖。
- GoogleDeepMind官方Argon帖定位成功，直接读取time datetime；原文显示受浏览器自动翻译影响，仅采用时间/互动信息，不把翻译作英文引语。
- Kimi、OpenAI、Anthropic官方X主页公开卡片已存档；只覆盖页面加载出的样本。`opencli twitter search`超时；X浏览器search页两次无article，不据此证明全站不存在。
- CNBC、The Register、CyberScoop分别读取原站置评措辞，详见B5/B6。没有联系记者/公司，没有发送消息。

## 社区热度

- Reddit：`Gemini 4 Argon`，sort top，time week；另对`OpenAI Moonshot`、`Anthropic robots`用relevance。广泛查询中有不相干结果，保留原JSON，但只统计相关帖子。
- HN：Algolia story查询`Gemini 4 Argon`、`distillation`、机器人相关词及URL片段`what-work-can-robots-do`；具体params在各d-hn-*.json。C组URL查询nbHits=0不等于全站无讨论。
- LinkedIn：`site:linkedin.com/posts anthropic "robots" "0.3%"`及相关公开搜索；结果不对应本研究，没有据此报互动数，未登录。
- Techmeme：直接读取260930/p41；仅计该簇主来源+More链接，不计页面其他新闻或社交讨论。

## 夸大实例

- A：Reddit真实“solved hallucinations”帖；中文鼓狮原站标题“虽碾压OpenAI”已存档。正文有限量开放限定，不能删掉限定来制造夸大。
- B：Register原站标题和ic.work原站数字误读已存档；三家指定英文媒体没有核到“16000成功”。
- C：AI Job Risk原站0.3%分母接为physical tasks的句子已存档；Yahoo正文数字/条件准确。中文关键词公开搜索未找到本次可确认的指定误读原站实例；未逐平台穷尽公众号、知乎及用户列出的媒体，不是全量审计。

## 工具与边界

- 普通shell联网脚本、curl、Invoke-WebRequest受到自动审批拦截（approval required by policy；AskForApproval Never）；没有请求不允许的提权。原因没有细分到网络目标或命令内容。后改用获准的原站浏览器和credentials:omit资源下载。
- `a-capture.py`和`b-browser-capture.py`是早期未执行的方案文件，不是运行收据；实际使用d-browser-tools.ps1。
- 一次浏览器定位操作失败后读取opencli-autofix指导；本任务未改适配器，也未提交上游issue。
- Bloomberg原站正文付费墙只读可见导语；没有从转载站反补付费墙正文。
- Methodology浏览器DOM提取为空，原因是PDF；实际取得原PDF并用pdftotext/pdftoppm处理。
- Yahoo首次state抛TypeError，后续extract成功；与整页失败区分。
- 网页截图重排/懒加载产生的错误候选移到sources/d-rejected-*，保留诊断且不作证据。最终C组使用官方PDF原页。AA18最终使用完整概览图；AA17是完整两agent比较表。Model Variants横向隐藏列的截图不交作完整表。
- 无社交平台截图；公开卡片档案只取帖子文本/time/status链接，不取导航、回复框或抓取者头像。交付检查未读到账号切换器身份字段，因而没有声称完成对未知handle的精确字符串匹配；另做已知维护者标识和常见凭据字段扫描，公开发布者署名/头像不属于抓取者信息。

补充：从Moonshot主页的实际链接进入 https://www.kimi.ai/blog/ ，存档b-kimi-blog-extract.json；列表当前最新日期2026-07-16，未见本事件回应。a-bloomberg-layout.json仅移除了元数据深链中的utm追踪参数，正文及时间字段未改。

