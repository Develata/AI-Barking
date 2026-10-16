# 1015 期 A 组抓取日志（两部门拟管 23 项 AI 技术出口）

执行方：claude-sonnet-5-5（Claude Code 子代理）。时刻为北京时间（UTC+8；本机美东 EDT = 北京 − 12 小时）。开工约 19:38，原定 60 分钟线已过（含一次网络中断与 LibreOffice 安装等待），收尾约 21:10。**fetch 日志里的时间是本机时钟换算，约 ±数分钟。**

工具：`barking fetch / shot / heat`（仓库自带）、curl（一次：官方附件下载）、firecrawl_search（共 8 次，仅用于找 URL）、opencli（1 次尝试截图，版式被放大裁切，废弃）、headless Chrome（经 `barking shot render`）、LibreOffice 26.8.1（Scoop 安装，仅用于把 `.wps` 转 PDF）、poppler `pdftotext`、`uv run --with olefile` / `--with pillow`（一次性环境，未安装到系统）。未登录任何站点，未运行读取浏览器 Cookie 的工具，未用镜像/代理站。

## 1. 抓取

| 约时刻 | URL / 对象 | 工具 | 结果 / 失败原因 | 存档 |
|---|---|---|---|---|
| 19:39 | 公告页 https://www.mofcom.gov.cn/zcfb/dwmygl/art/2026/art_61446ab4beee4f63973d4d0ab3097648.html ；答记者问 https://www.mofcom.gov.cn/zwgk/zcjd/art/2026/art_b11d5c1e9374421e93e85dc87eba6e38.html | barking fetch | 200，ok；已打开 `.txt` 确认是正文 | `a-a-notice.*`、`a-a-qa.*`（清单名带了 a，所以文件名是 `a-a-…`） |
| 19:40 | 公告附件 1 `.wps`（公告页 `.txt` 中的下载链接 `/api-gateway/jpaas-web-server/front/document/download?fileUrl=…&fileName=1.拟列入《中国禁止出口限制出口技术目录》的人工智能、生物医药、光伏技术范围（征求意见稿）.wps`） | curl | 第一次不带 Referer/UA：**403**（仅 150 字节，试验文件已用 rm 删除）；第二次带浏览器 UA 与 Referer：302；跟随跳转后 200，191,488 字节，OLE2 复合文档（Word97 格式）。跳转目标 `/cms_files/filemanager/600926013/attach/20269/4f8fdb877dd448dfab8b65ba1f340bd5.wps` | `a-annex1.wps`（原件，不入库） |
| 19:45 | 附件文本提取 | 自写 `a-wps2txt.py`（olefile，读 WordDocument 流与 piece table） | 成功，7,999 字符；表格单元格被压平，上标丢失（“10^26”显示为“1026”） | `a-annex1.txt` |
| 19:55 | 2025/2023/2020 前次修订公告与 2025 答问 | barking fetch | 全部 200 ok（`notice25-28`、`qa2025`、`notice23-57`、`notice20-38`） | `a-notice25-28.*`、`a-qa2025.*`、`a-notice23-57.*`、`a-notice20-38.*` |
| 20:00–20:02 | 媒体：Hindu、Economic Times(AFP)、Global Times、钜亨、联合早报、新华网客户端、答问第二 URL、SCMP | barking fetch | 7 条 ok；**SCMP 403 blocked**（Cloudflare 质询，cf-mitigated: challenge；按规则记失败，未绕过，保留失败页作证据） | `a-hindu.*`、`a-et.*`、`a-globaltimes.*`、`a-cnyes.*`、`a-zaobao.*`、`a-xinhua-qa.*`、`a-mofcom-qa2.*`、`a-scmp.html/.headers`（失败证据） |
| 20:02 | Bloomberg、theedgemarkets（Bloomberg 同稿）、Morningstar（Dow Jones）、Bloomberg Law | barking fetch | 200 ok；Bloomberg 页只有导语约 3 段（付费墙以上），Edge 与 Morningstar 为全文；Bloomberg Law 未打开 | `a-bloomberg.*`、`a-edge-bbg.*`、`a-morningstar-dj.*`、`a-bbglaw.*` |
| 20:03 | 公告同文第二路径 https://www.mofcom.gov.cn/zwgk/zcfb/art/2026/art_0a31a3c9ac404f7a9b0e917ce3412997.html（Bloomberg 引用）与 2020 年公告页 | barking fetch | 200 ok | `a-cl-bloomberg-url.*`、`a-notice20-38.*` |
| 20:03–20:09 | firecrawl_search ×8（找媒体与前次修订：商务部目录公告、Reuters/Bloomberg/SCMP、中文报道、夸大标题、AIHOT） | firecrawl | 成功；仅取线索。**路透社原文、第一财经首发稿未出现**；找“把拟写成已禁止”类标题两轮无果 | 无 |
| 20:04 | `barking shot render` 公告页（700 CSS px，DPR 2，1400×6400） | headless Chrome | 成功；先用 opencli 截图试过，页面按 1400 CSS px 放大 2 倍导致右侧截断且带翻译插件图标，**废弃**（临时文件在 `%TEMP%\a-shots\`，不入库，已不使用） | 中间图在 `%TEMP%\barking-shot\26101015-a\`（不入库） |
| 20:05 | `barking shot cropbox` 出 images/01、02 | barking shot | 成功，已逐张打开目视确认 | `images/01-notice-title-scope.png`、`images/02-notice-feedback-registration.png` |
| 20:06–20:07 | `barking shot render` 答记者问页（1400×10400）；`stack` 出 images/03、04 | barking shot | 成功。第一次 stack 背景自动取了页面红色（#ab0000），第二次指定 `--bg "#ffffff"` 重做；**第一版 03、04 与更早一版单段 03（自己生成）已用 rm 逐个删除**，shots-log 中保留这些动作的记录 | `images/03-qa-api-saas-local-deploy.png`、`images/04-qa-40-14-26-counts.png` |
| 20:09 | `barking heat`（HN 三组查询，自 2026-10-16 15:00 北京时间） | barking heat | HN 命中 0；Reddit 搜索被重定向到登录页（失败，已记入 errors）。另用 Algolia `search_by_date?query=China export` 手查，无相关主帖 | `a-heat.json` |
| 20:10 | AIHOT 首页与条目页 https://aihot.news 、https://aihot.news/items/p11tj593iq5nrtiotvoxllop4 | barking fetch | 200 ok；条目显示 IT之家来源 16:23、列表 16:44、评分 83 | `a-aihot.*`、`a-aihot-item.*` |
| 20:12–20:40 | LibreOffice 安装 | `scoop install libreoffice`（extras，26.8.1，下载 357 MB） | 安装用时较长（msiexec 展开）；期间 `soffice` 报 exit 99 / C++ 异常（安装未完成时调用，非文件问题），安装完成后 `soffice.exe --headless --convert-to pdf` 成功 | 见下 |
| 20:43 | 附件 `.wps` → PDF | soffice --headless（用户配置目录取默认，不带自定义 profile） | 成功，9 页，762,647 字节；PDF 为 LibreOffice 渲染，版式非官方原貌 | `a-annex1.pdf`（不入库）、`a-annex1-pdftotext.txt` |
| 21:06 | `barking shot pdf` 渲第 1、3 页（144 dpi）；`cropbox` 出 images/05、06 | barking shot | 成功；已打开目视确认。第 3 页确认“≥10^26 次浮点运算”上标 | `images/05-annex1-ai-prohibited-p1.png`、`images/06-annex1-ai-llm-restricted-p3.png` |

## 2. 图片清单（`images/`，区间 01–12，本组用 6 张；2 倍像素，宽 ≤1400）

| 文件 | 内容 | 来源 / 说明 |
|---|---|---|
| `01-notice-title-scope.png` | 公告页标题、元数据（2026-10-16 16:00）、发布文号与“一、拟修订内容”整段 | headless Chrome 渲染（700 CSS px，DPR 2），只裁切，1400×2345；浅色底；顶部页眉裁掉 |
| `02-notice-feedback-registration.png` | “二、征求意见反馈方式”与“三、出口前登记要求”（一）（二）（三） | 同上，1400×2470；含商务部公开联系方式（邮箱、传真、通信地址） |
| `03-qa-api-saas-local-deploy.png` | 答记者问页标题与“海外本地化部署、模型调用”一问一答整段（含 API/APP/SaaS 句） | 页头（面包屑、来源、标题）与问答两段竖向拼接，间隔 40 px 白底，1400×1600 |
| `04-qa-40-14-26-counts.png` | 答记者问：“拟新增技术的范围”一问一答（40/14/26） | 同 03 的页头 + 一段问答，1400×1330 |
| `05-annex1-ai-prohibited-p1.png` | 附件 1 第 1 页：“一、人工智能”禁止出口部分表头与前两行，页脚脚注“已通过开源、发表学术论文等方式进入公共领域的技术，不纳入管理范围。” | LibreOffice 渲染 PDF 第 1 页（144 dpi）裁掉页边，1000×1540；**渲染页，非官方 WPS 原貌**；第 2 行表格在页底被分页截断 |
| `06-annex1-ai-llm-restricted-p3.png` | 附件 1 第 3 页：266501X、**266504X 大模型技术（含权重参数、≥10^26、≥20,000亿、全球前10）**、266505X 开头 | 同上，1000×1540；上一页续表的行首半截（266404X 的第 3–6 点）在页顶，上下文不完整 |

未截的候选：附件第 2 页脚注 2（自动许可）、第 4–5 页智能体与算力集群；可用 `a-annex1.pdf` 补渲。

## 3. 自己生成的试验文件与清理

- 已删除（rm 精确路径）：curl 第一次 403 的 `a-annex1.wps`（150 字节错误页）与同名 `.hdr`；`images/03`、`04` 的两个旧版本（背景色错误）。
- `%TEMP%` 下的 `a-shots`、`a-lo`、`a-list*.tsv` 为临时目录，不在仓库内。
- 保留的失败证据：`a-scmp.html/.headers`（Cloudflare 质询页）。

## 4. 泄露自查

- 对本组所有文本类存档（`a-*.txt/.json/.jsonl/.headers/.py`）grep “develata”“Users\QQ”：无命中。截图均为官方页与 PDF 渲染，不含登录态，未截浏览器界面；“QQ”类字符串的二进制命中为误报（PDF、图片字节）。
- `a-aihot.*` 与 `a-hindu.*` 页面里有站点自带的登录/订阅提示文字，来自站点，不是抓取者信息。

## 5. 单文件大小

超 1 MB 的文本存档：无（最大 `a-bloomberg.html` 388 KB 属 HTML 原件，不入库；`a-annex1.pdf` 748 KB、`a-annex1.wps` 187 KB 为原件，不入库）。
