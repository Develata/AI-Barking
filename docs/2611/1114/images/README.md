# 1114 配图说明

上传：全选 `images/publish/`，按名称排序即上传顺序（封面两张在最前）。小红书、抖音按全部顺序上传；公众号同序但不插事件卡与吠点卡（03、04、10、11）。卡片内容见 `cards.toml`，由 `barking card` 渲染。

## 封面

封面抽签：画风=技术图版；底色=深色；标题=压在下方；看板娘=全身；动作=扮演

- `cover-draw` 按期号定种子，A、B 两组未拼取值。主题对应：技术图版取“机场安检 X 光屏”，深蓝近黑底：一座积木与对话气泡拼成的小房子代表“用 AI 拼出来的网站”，后门敞开没有锁，一本护照从门里滑出；看板娘扮安检员、拿手电照向护照。主标题亮黄粗字压在下方传送带上。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write，`-C` 为草稿目录）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张，共 4 张。选 A：护照从敞开的门里滑出，一眼对上“谁都能查”；B（柠檬黄系统弹窗、护照与掉落的锁、看板娘探头捂嘴）也清楚，但“锁掉在地上”不如“护照从房子里滑出来”贴题，故事性不如 A。
- 验收：逐字核对主标题「不写代码用AI建站，护照号谁都能查」与胸牌「AI 吠点」，放大看无多字、错字或乱码；对话气泡里只有圆点；看板娘两只手（一手挥、一手持手电；横版一手持手电、一手扶腰）、两只垂耳、一条尾巴，没有手套。积木是通用凸点积木，不带品牌标志。无副标题。

A 组提示词（选中）：

> 竖版：技术图版风：机场安检 X 光机的屏幕画面，深蓝近黑的底色，青白色发光线条与半透明 X 光质感。主体：一座用积木和对话气泡拼成的小房子（代表“用 AI 拼出来的网站”），在 X 光下透视可见：房子的后门大开、没有锁，一本打开的护照和几张聊天气泡正从门里滑出来，外面的人随手就能拿到。房子上方飘着几个发光的提示词对话框（只有光点，不能有字）。看板娘全身入画，站在 X 光机旁，换装成机场安检员：戴安检员帽子、手持安检手电照向护照，表情惊讶。主标题用粗壮的亮黄色字压在画面下部、叠在 X 光机传送带上。只保留一个主体（房子与护照），不堆杂物。
>
> 横版：技术图版风横幅：机场安检 X 光机的宽屏画面，深蓝近黑底，青白色发光线条与 X 光透视质感，左右是延伸的传送带。画面正中：一座积木和对话气泡拼成的小房子在 X 光下透视，后门敞开没有锁，一本打开的护照和几个聊天气泡从门里滑出。看板娘全身站在房子右侧，扮成安检员，戴安检帽、举着手电照向护照，表情惊讶。主标题用粗壮亮黄色字压在房子下方的传送带上。

B 组提示词（落选）：

> 竖版：拟物版面风：整幅高饱和柠檬黄底，画面中央是一个巨大的、通用样式的电脑系统弹窗（圆角窗口、顶部三个小圆点按钮，不仿任何真实品牌界面）。弹窗的标题栏位置就是主标题，用粗黑字嵌在弹窗里，像系统警告标题。弹窗内容区里是一本摊开的护照（只画图形，不能有可读文字或数字），护照旁边一扇小门开着、门锁掉在地上。看板娘从弹窗右下角后面探出半个身子，只露出头、垂耳、一只手和鲸鱼尾巴尖，双手捂嘴一脸震惊。撞色点缀用一点亮蓝与橙红。只保留一个主体（弹窗与护照）。
>
> 横版：拟物版面风横幅：整幅高饱和柠檬黄底，正中一个宽大的通用系统弹窗（圆角、三个小圆点按钮，不仿真实品牌），主标题用粗黑字嵌在弹窗标题栏。弹窗里是一本摊开的护照（只画图形，不能有可读文字或数字）和一扇敞开的小门、掉在地上的门锁。看板娘从弹窗右侧边框后探出头和一只手，捂嘴震惊，鲸鱼尾巴从弹窗下方露出。

（硬性要求按 `templates/codex-cover.md` 逐字照抄：主标题「不写代码用AI建站，护照号谁都能查」，无副标题。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字取自正文事实段（不放吠点） |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋谷歌 /teamwork 形式化韦伊猜想、Project Zero 披露期限、Gemini × Wallet、DeepSeek 开放 App 能力 API、EFF 旁观者隐私 |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（Lovable），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（Lovable），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-crissy-numbers.png | 第一条事实段 | 研究原文的样本与数字（159,041 个项目、24,664 个加载即发记录、7,403 个 30%），批注；来源 crissyfield.de 原文截图（2026-11-15 存档） |
| 6 | publish/06-crissy-limits.png | 第一条吠点① | Limitations 段（邮箱可能是公开联系地址或测试数据；30% 不应读作已确认泄露率），批注；来源同上 |
| 7 | publish/07-crissy-cases.png | 第一条事实段 | 论坛 8 万资料与 20 万聊天消息、服务商 1000 多条证件记录（含护照号）（两段拼接），批注；来源同上 |
| 8 | publish/08-hn-igor.png | 第一条事实段 | HN 评论：自称 Lovable 安全负责人称根因是行级安全不足，批注；来源 news.ycombinator.com 原文截图（2026-11-15 存档） |
| 9 | publish/09-lovable-terms.png | 第一条吠点② | Lovable 服务条款 No Sensitive Data 与 Your Responsibilities（三段拼接，深色页面），批注；来源 lovable.dev 原文截图（2026-11-15 存档） |
| 10 | publish/10-event-2.png | 第二条事实段 | 事件卡（Deepnote），逐字取自正文事实段；公众号不插 |
| 11 | publish/11-bark-2.png | 第二条吠点 | 吠点卡（Deepnote），逐字取自正文吠点段；公众号不插 |
| 12 | publish/12-dn-silence.png | 第二条事实段、吠点② | “When to speak and when to stay silent” 图（Gemini 23/40、10/10）与图注 “Message delivery only”，批注；来源 deepnote.com 原文截图（2026-11-15 存档） |
| 13 | publish/13-dn-ranks.png | 第二条吠点①② | 排名段（does not separate them；Fable 5.2 inside Sol’s）与 27、18 次超时句（两栏拼成一栏），批注；来源同上 |
| 14 | publish/14-judge-rules.png | 第二条吠点② | 公开仓库评审提示第 7、8 条与超时规则段（两段拼接，深色 raw 视图），批注；来源 github.com 原文截图（2026-11-15 存档） |
| 15 | publish/15-xzy-title.png | 第二条吠点① | 新智元（网易号）标题与发布时刻，批注；来源 163.com 原文截图（2026-11-15 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| crissy-numbers-raw.png | 批注底图 |
| crissy-limits-raw.png | 批注底图 |
| crissy-cases-raw.png | 批注底图 |
| hn-igor-raw.png | 批注底图 |
| lovable-terms-raw.png | 批注底图 |
| dn-silence-raw.png | 批注底图 |
| dn-ranks-raw.png | 批注底图 |
| judge-rules-raw.png | 批注底图 |
| xzy-title-raw.png | 批注底图 |
| hn-title-raw.png | HN 标题（“passport details and more than 200K private messages”）；吠点删去后未用 |
| 01-crissy-title.png | 取证截图（A 组） |
| 02-crissy-numbers.png | 取证截图（A 组） |
| 03-crissy-limitations.png | 取证截图（A 组） |
| 04-crissy-supabase-only.png | 取证截图（A 组） |
| 05-crissy-forum-case.png | 取证截图（A 组） |
| 06-crissy-passport-case.png | 取证截图（A 组） |
| 07-crissy-responsibility.png | 取证截图（A 组） |
| 08-lovable-blog-title.png | 取证截图（A 组） |
| 09-lovable-blog-still-need-coming.png | 取证截图（A 组） |
| 10-hn-title-author.png | 取证截图（A 组） |
| 11-hn-neathack-4000.png | 取证截图（A 组） |
| 12-hn-lovable-igor.png | 取证截图（A 组） |
| 13-page-title.png | 取证截图（B 组） |
| 14-method-run.png | 取证截图（B 组） |
| 15-leaderboard-intervals.png | 取证截图（B 组） |
| 16-not-separate-text.png | 取证截图（B 组） |
| 17-silence-chart.png | 取证截图（B 组） |
| 18-silence-text.png | 取证截图（B 组） |
| 19-results-table.png | 取证截图（B 组） |
| 20-timeouts-three-ways.png | 取证截图（B 组） |
| 21-cost-text.png | 取证截图（B 组） |
| 23-zh-163-title-date.png | 取证截图（B 组） |
| 24-x-founder-post.png | 取证截图（B 组） |
| 25-quick-findings.png | 取证截图（B 组） |
| 26-limitations.png | 取证截图（B 组） |
| 27-repo-judge-prompt-silence-timeout.png | 取证截图（B 组） |
| 28-zh-163-1p8x.png | 取证截图（B 组） |
| 29-zh-163-16x-430x.png | 取证截图（B 组） |
