# 1107 配图说明

标题：用Claude点“不再询问”，密码公开？

本期期号1107（上期1106+1）；主帖新闻日期为2026-11-06至11-08（北京），速览含 CBC 11-05，卡片日期写11月8日；取证与截图在北京 11-09 00:48 起。主帖顺序：Claude Code 配置文件 → DoubleAI。

封面二选一：小红书/抖音用 `publish/00-cover.png`（3:4），公众号用 `publish/00-cover-wide.png`（约2.35:1）。封面后按下表上传；小红书、抖音共13张（封面＋12），公众号不插两张事件卡与两张吠点卡。

Apiiro 原文页未截图（只有内置浏览器能读，无法落盘），事实段的数字没有对应批注图。

## 封面

封面抽签：画风=名画戏仿；底色=亮色；标题=手写字；看板娘=半身；动作=旁观

- 主题对应：名画“两手相触”对应点下批准那一下，黄铜钥匙飞向告示墙对应密钥被公开。
- 生成：Codex CLI（gpt-6-astra，medium，workspace-write）内置图像生成，附看板娘参考图。A、B 两组各出竖版、横版各 1 张（旧标题）；选 A，因名画戏仿画面更抓眼、主体更清楚，B 粉笔版落选。正文标题改定后按 A 重出一轮（竖版、横版累计各 3 张，未超 5 张）。
- 验收：逐字核对主副标题；看板娘两手、两耳、一尾，保持二次元；横版居中 1:1 裁切，标题与看板娘都在框内。
- 未比对 1104、1105 竖版封面（原图不在本机）；1106 为黑白漫画，一眼不同；抽签已排除前几期取值。
- 风控：不写政党、政府；主标题为问句，副标题写明能否使用没验证。

A 组提示词：

> 竖版：画面：文艺复兴湿壁画戏仿，构图取《创世纪》两只手即将相触的姿态，明亮的天蓝与金色亮底。一只古典油画质感的手伸向一枚发光的大圆按钮，按钮上没有字；按钮下方，一把巨大的黄铜钥匙正从打开的文件夹里飞出去，飞向远处一面贴满纸张的公开告示墙。看板娘半身出现在画面右下角，捧着一把瓜子在旁边吃瓜，斜眼看着那把钥匙。主标题用粗大的手写毛笔感字样横写在画面上部天空，副标题用小一号手写字贴在其下。
>
> 横版：画面：文艺复兴湿壁画戏仿的横幅，明亮的天蓝与金色亮底。左侧一只古典油画质感的手伸向一枚发光的无字圆按钮，右侧远处是贴满纸张的公开告示墙，一把巨大的黄铜钥匙从打开的文件夹飞向告示墙。看板娘半身在中央偏下，捧瓜子旁观。主标题用粗大手写字放在中央安全区，副标题小一号紧贴其下。

B 组提示词：

> 竖版：画面：深色黑板满底，白色和彩色粉笔涂鸦。主体是一张粉笔画的公告栏，上面用图钉钉着一把大钥匙和一个打开的文件夹，几条粉笔箭头从文件夹指向公告栏。看板娘完整全身站在公告栏前，戴保安帽、拿手电筒，扮成巡夜的保安照向那把钥匙，身体保持彩色二次元。主标题在顶部用粗粉笔大字横排，副标题小一号在标题下方。
>
> 横版：画面：深色黑板横幅，粉笔涂鸦。左侧粉笔公告栏上钉着大钥匙和打开的文件夹。看板娘完整全身站在中央，戴保安帽、拿手电筒照向钥匙。主标题用粗粉笔大字横排在中央安全区上方，副标题小一号紧贴其下。

（硬性要求按 `templates/codex-cover.md` 逐字照抄，主标题「用Claude点“不再询问”，密码公开？」，副标题「能否使用没验证」。）

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | publish/01-tldr.png | 省流 | 两条事实，逐字摘自省流 |
| 2 | publish/02-roundup.png | 今日速览 | 两条主帖小标题＋Anthropic、OpenAI、微软、图书馆、2.1.319、V4-Flash |
| 3 | publish/03-event-1.png | 第一条事实段 | 事件卡（Claude Code），逐字取自正文事实段；公众号不插 |
| 4 | publish/04-bark-1.png | 第一条吠点 | 吠点卡（Claude Code），逐字取自正文吠点段；公众号不插 |
| 5 | publish/05-settings-doc.png | 第一条吠点③ | Claude Code 官方文档，批注；来源 code.claude.com 官方文档截图（2026-11-09 存档） |
| 6 | publish/06-cherny-reply.png | 第一条事实段、吠点② | Cherny 回复，批注；来源 news.ycombinator.com 原文截图（2026-11-09 存档） |
| 7 | publish/07-issue-5544.png | 第一条吠点④ | GitHub issue #5544，批注；来源 github.com issue 截图（2026-11-09 存档） |
| 8 | publish/08-event-2.png | 第二条事实段 | 事件卡（DeepSeek），逐字取自正文事实段；公众号不插 |
| 9 | publish/09-bark-2.png | 第二条吠点 | 吠点卡（DeepSeek），逐字取自正文吠点段；公众号不插 |
| 10 | publish/10-fig10-probe.png | 第二条事实段 | DoubleAI Fig.10，批注；来源 doubleai.com 原文截图（2026-11-09 存档） |
| 11 | publish/11-probe-sampling.png | 第二条吠点① | 探针工作方式，批注；来源 doubleai.com 原文截图（2026-11-09 存档） |
| 12 | publish/12-probe-midway.png | 第二条吠点② | 半途探针，批注；来源 doubleai.com 原文截图（2026-11-09 存档） |

## 备用图

| 文件 | 用途 |
|---|---|
| settings-doc-raw.png | 批注底图 |
| cherny-reply-raw.png | 批注底图 |
| issue-5544-raw.png | 批注底图 |
| fig10-probe-raw.png | 批注底图 |
| probe-sampling-raw.png | 批注底图 |
| probe-midway-raw.png | 批注底图 |

| 01-doc-settings-local.png | 取证截图 |
| 02-doc-permission-prompt.png | 取证截图 |
| 03-hn-cherny-reply.png | 取证截图 |
| 08-gh-11982-header.png | 取证截图 |
| 09-gh-91270-header.png | 取证截图 |
| 10-gh-5544-header.png | 取证截图 |
| 11-gh-13106-header.png | 取证截图 |
| 12-hn-thread-header.png | 取证截图 |
| 13-fig10-chart.png | 取证截图 |
| 14-ds-probe-code.png | 取证截图 |
| 15-ds-knowing.png | 取证截图 |
| 16-qwen-fig11.png | 取证截图 |
| 17-qwen-gadgets-code.png | 取证截图 |
| 18-qwen-evolve.png | 取证截图 |
| 19-exp2-fig6.png | 取证截图 |
| 20-fig7-clrs.png | 取证截图 |
| 21-fig8-clrs-curve.png | 取证截图 |
| 22-wy163-title.png | 取证截图 |
| 23-wy163-three-groups.png | 取证截图 |
| 24-wy163-opening-rollback.png | 取证截图 |
| 25-wy163-code-image.png | 取证截图 |

取证截图 01–03、08–25 留在 `images/`，详见 `sources/capture-log-a.md`、`sources/capture-log-B.md`。`20-fig7-clrs.png` 动画只画出灰线，不作证据。
