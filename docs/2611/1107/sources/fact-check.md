# 1107 期事实清单

时间一律北京时间。证据细节见 `evidence-a.md`（Claude Code 密码外泄）、`evidence-B.md`（DoubleAI）、`evidence-c.md`（速览）；截图在 `images/`。Apiiro 页面只能经内置浏览器读取（要点转述见 `a-apiiro-browser-notes.md`），自动化取证被 Cloudflare 拦下，复核者可用 `a-thn.txt`、`a-itwire.txt` 的转述交叉核对。

## 标题与省流

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 标题：用Claude点“不再询问”，密码公开？ | — | ✅ | 见 A2、A3、吠点② | 问号；“公开”需文件被传上公开仓库，事实段写明 |
| 封面副标题：能否使用没验证 | L3 | ✅ | 见 A3、吠点① | 对应省流 |
| 省流：Apiiro 称，公开网上1122份 Claude Code 设置文件存着疑似密码或密钥，能否使用没验证 | L3 | ✅ | 见 A3、吠点① | Apiiro 自报；“公开网上”指公开 GitHub 副本 |
| 省流：DoubleAI 称 DeepSeek-V4.1-Flash 不开思考模式做竞赛题，自加探针后正确率从64%升到71%，“改写架构”这组原文没写 | L1 | ✅ | 见 B3、吠点① | 公司自述 |
| 卡片日期：11月8日 | — | ✅ | 两份 README | 主帖新闻日期11/6–11/8（北京），速览含 CBC 11/5，沿用往期按最新事件标注 |
| 批注图页脚：2026-11-09 存档 | — | ✅ | `capture-log-a.md`、`capture-log-B.md` | 取证于北京 11-09 |
| 批注标题：2025 年 8 月的 GitHub issue | L1 | ✅ | `a-gh-issues-meta.json` | #5544 创建于北京2025-08-11 22:25 |

## A Claude Code 密码外泄

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| A1 让 Claude Code 连数据库时，命令里常带密码 | L3 | ✅ | `a-apiiro-browser-notes.md` 机制段；[Apiiro](https://apiiro.com/blog/coding-agents-leaking-secrets-github-dont-ask-again) | Apiiro 说法 |
| A2 弹窗选“Yes, and don't ask again” | L1 | ✅ | https://code.claude.com/docs/en/settings（图 01） | 官方文档 |
| A3 Apiiro 称，批准的规则若是完整命令，密码也存进项目的设置文件 | L3 | ✅ | 同 A1；官方 permissions；`a-thn.txt` | 按反向核验收窄：规则也可能是通配前缀 |
| A4 文件传上公开的代码网站 GitHub，谁都能看到 | L3 | ✅ | 同 A1 | 限定“公开” |
| A5 Apiiro 搜到10.4万份这类公开文件，1122份含它判定的“真实密钥” | L3 | ✅ | 同 A1；`a-thn.txt` | 104,185 份；1,122 份（1.1%）；Apiiro 自报 |
| A6 1家公司确认密钥曾有效 | L3 | ✅ | 同上 | 总数2,539因字数未入正文；原文 One confirmed its key had been live |
| A7 Claude Code 团队的 Cherny 称，一直没做清除密钥，正在想办法 | L2 | ✅ | https://news.ycombinator.com/item?id=50407882（`a-hn-50407882.json`，id 50408126） | 北京11/8 05:13（UTC 11/7 21:13）；“secret scrubbing”译作自动清除密钥；未承诺时间 |
| 吠点① “真实密钥”按格式和公网地址判定，再由 AI 复核，Apiiro 没试过能不能用 | L3 | ✅ | `a-apiiro-browser-notes.md` 局限；`a-thn.txt`、`a-itwire.txt` | 1家确认曾有效在事实段 |
| 吠点① 这1122份里的密钥近半是服务商密钥，不全是密码 | L3 | ✅ | 同上 | 636/1,379=46%；分母限定当前 1,122 份文件 |
| 吠点② Cherny 称文件是人手动建的再上传 | L2 | ✅ | 同 A7 | “the files were manually created and checked in” |
| 吠点② Apiiro 称追溯1105个项目，539个有 Claude 署名的改动碰过它，证明不了密码是 Claude 传的 | L3 | ✅ | `a-apiiro-browser-notes.md`；`a-itwire.txt` | 后半为本号判断，iTWire 同样指出不能证明 |
| 吠点③ 官方防护只在用户自己电脑上生效，不跟着项目走 | L1 | ✅ | https://code.claude.com/docs/en/settings（`a-settings.txt`） | “adds … to your global git excludes file” |
| 吠点④ 同类问题2025年8月就有人报告，Apiiro 列的4个问题帖都被机器人关闭 | L1 | ✅ | `a-gh-issues-meta.json` | #5544 创建于2025-08-11；4个均由 github-actions[bot] 关闭；“未见官方回复”按反向核验删去 |

## B DoubleAI

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| B1 DoubleAI 博文称 | L1 | ✅ | https://www.doubleai.com/research/can-llms-redesign-themselves-as-they-train（`b-doubleai.txt`） | 页面只写 Nov 6, 2026，无时刻，未换算北京时间 |
| B2 让 DeepSeek-V4.1-Flash 分析自己做的243道旧题，写出判断对错的小程序“探针” | L1 | ✅ | 同上；图 15 | 2024–2025 比赛题；公司自述 |
| B3 答完题，探针读一次内部状态，判错就重答，最多三次 | L1 | ✅ | 同上；图 14 | “at the moment it finishes a solution”“up to three times” |
| B4 不开思考模式，做 AIME、HMMT 2026两场竞赛共63题，公司自测正确率从64%升到71% | L1 | ✅ | 同上；`b-doubleai-figdata.txt`；HF 行数 | 30+33=63；合并 63.9→71.0 |
| B5 只挑最长的答案重答，是65.4% | L1 | ✅ | Fig.10 数据 | length only，both=65.4 |
| 吠点① 新智元标题写“改写 Transformer 架构”，这组原文只说“控制采样”，没写改模型本身 | L1/L5 | ✅ | `b-wy163.txt`；`b-doubleai.txt` | “this layer controls sampling”；总述称四个实验“redesigns the network”，故限定“这组” |
| 吠点② 新智元稿写“检查点当场回滚”，DoubleAI 原文是答完才读探针，半途探针分不清同题两次作答的成败 | L1/L5 | ✅ | 同上；图 15 | “without thinking mode”；半途探针“could not tell the failing one apart” |
| 吠点③ 新智元稿的“消耗相同的 token 预算”原文没有，这组也没给成本 | L1/L5 | ✅ | 同上 | 原文 grep same token、budget 无命中；2.8×/1.8× 属 Qwen 组 |
| 吠点④ DoubleAI 没披露实验重复次数 | L1 | ✅ | 同上 | B5d |
| 吠点④ 截至11月9日本号未查到第三方复现 | — | ⚠️ | `evidence-B.md` B9b | 本号检索结论，截至北京11月9日，未穷尽；范围见 evidence-B |

## 批注译注

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| Claude 请求运行 Bash 命令，你选“Yes, and don't ask again”时，Claude Code 把这次批准存成一条 allow 规则。 | L1 | ✅ | 图 01，`a-settings.txt` | 官方 settings 文档 |
| 第一次在尚未忽略它的 git 仓库写这个文件时，Claude Code 把它加进你的全局 git 排除文件，不进入你的提交。 | L1 | ✅ | 图 01 | 官方 settings 文档 |
| 如果文件是你手动创建的，且 Claude Code 还没写过它，请自己把它加进 .gitignore。 | L1 | ✅ | 图 01 | 官方 settings 文档 |
| 一直以来，Claude Code 没有做密钥清理（这方面有很多好工具），但我们正在研究能做些什么来防止这种情况。 | L2 | ✅ | 图 03，`a-hn-50407882.json` | Cherny HN 回复 |
| Claude 创建这个文件时，我们会自动把它 gitignore，所以这里的文件是手动创建并提交的。 | L2 | ✅ | 图 03 | 同上 |
| 已批准的、带凭据的 bash 命令被原样存放，凭据以明文保存在 settings.local.json 里。 | L1 | ✅ | 图 10，`a-gh-issues-meta.json` | issue #5544 标题 |
| 回答时不开思考模式。 | L1 | ✅ | 图 13 | DoubleAI Experiment 4 |
| 它的探针会让预测为错的解答重新采样，最多三次。 | L1 | ✅ | 图 13 | Fig.10 图注 |
| 探针在 DeepSeek 写完解答的那一刻读取它自己的内部状态。 | L1 | ✅ | 图 14 | DoubleAI 原文 |
| 这一层不是往网络的表示里添加内容，而是控制采样。 | L1 | ✅ | 图 14 | DoubleAI 原文 |
| 这种知识也只在答案写完之后才出现。 | L1 | ✅ | 图 15 | DoubleAI 原文 |
| 对同一道题的两次作答，半途的探针分不出哪一次会失败。 | L1 | ✅ | 图 15 | DoubleAI 原文 |

## 速览

| 文中事实 | 级 | 结果 | 一手来源 | 备注（条件、时区、口径） |
|---|---|---|---|---|
| 据彭博社报道，Anthropic IPO 潜在买方讨论估值1.2–1.3万亿美元 | L4 | ⚠️ | https://www.bloomberg.com/news/articles/2026-11-06/anthropic-ipo-valuation-expectations-pressured-after-ai-rout（`c-bloomberg-ipo.txt`） | 原文 “potential buyers are discussing about a $1.2 trillion to $1.3 trillion valuation range … according to people familiar with the matter”；另有人低至1.1万亿，未入图；买方口径，非定价；URL 日期11/6 |
| 据 TechCrunch 报道，本轮目前已融110亿美元，估值1.25万亿美元 | L4 | ⚠️ | https://techcrunch.com/2026/11/06/openai-reportedly-raises-an-additional-11-billion-at-a-1-25-trillion-valuation/（`c-techcrunch-openai.txt`） | 11/7 06:53（PST 11/6 14:53）；原文 “reportedly raised $11 billion so far”，转述彭博社，整轮未完成、或约200亿；TechCrunch 未写投前投后 |
| Langford 称微软 AI Frontiers 组已结束，他已离开 | L2 | ✅ | https://hunch.net/?p=13764006（`c-hunch.txt`） | 个人博客，作者自述：“In early September we were reorged into Microsoft AI”“So AI Frontiers is over”“left”；未见微软回应；组约50人 |
| 据 CBC 报道，大学图书馆网站被 AI 抓取机器人拖慢甚至挤垮 | L4 | ⚠️ | https://www.cbc.ca/news/world/university-libraries-ai-data-scraping-bots-9.7408287（`c-cbc-libraries.txt`） | 11/5 17:00（EST 04:00）；原文 “swarms of data-scraping bots — that are slowing or crashing sites”；北卡1.1万次芬兰音乐搜索是“去年”事件，未入图 |
| Claude Code 2.1.319 修复 worktree 沙箱逃逸等问题 | L1 | ✅ | https://raw.githubusercontent.com/anthropics/claude-code/main/CHANGELOG.md | npm 发布 11/6 08:58（UTC 00:58）；原句 “Fixed a sandbox escape when creating a worktree”；未写影响范围或是否被利用 |
| 作者自报：284B DeepSeek 压缩量化版，64GB 内存纯 CPU 能跑 | L1 | ✅ | https://github.com/synchopate/llama-modchip（`c-llama-modchip-readme.txt`） | 作者自报、非独立评测；Ryzen 9 9955HX，纯 CPU 解码约9–11 tok/s；Q2_0 量化，未打补丁的 llama.cpp 打不开；另有 CPU+GPU 配置 |

速览标签“V4-Flash”：指 llama-modchip 的 DeepSeek-V4-Flash 压缩量化模型（`c-llama-modchip-readme.txt`），与主帖的 V4.1-Flash 不同。

不收：FT 燃气机组（FT 403，未读到）；Arena 周报（榜单页 403，无周报原文）；8rank Mac 分类器（无 L1–L4）。

## 反向核验

第一轮（Codex gpt-6-astra，medium，只读）：2 个 BLOCKER、4 个 SHOULD_FIX、2 个 NEEDS_VERIFICATION，均采纳。

1. BLOCKER DeepSeek 提升数字丢失测试范围：省流、省流卡、事件卡补模型版本、不开思考模式与数学竞赛，并写明公司自测。
2. BLOCKER 速览“打补丁”隐藏压缩前提：改为“压缩量化版”，标签 V4-Flash。
3. SHOULD_FIX 泄漏机制写成必然：改为“批准的规则如果是完整命令”，公开条件写明传上公开网站。
4. SHOULD_FIX “近半”分母漂移：限定“这1122份文件里的密钥”。
5. SHOULD_FIX OpenAI 口径：加“本轮目前”。
6. SHOULD_FIX “未找到论文”标 L1：拆开，后者记为本号检索结论 ⚠️，“重复次数”改“实验重复次数”。
7. NEEDS_VERIFICATION 图面元信息：补卡片日期、存档日、issue 日期行；DoubleAI 写“页面标注11月6日”。
8. NEEDS_VERIFICATION “未见官方回复”：issue 元数据不足以排除，删去。

第二轮（Codex gpt-6-astra，medium，只读）：PASS，无 BLOCKER。3 个 SHOULD_FIX、2 个 NICE_TO_HAVE，均采纳：官方文档译注补写入条件；“半途读不出对错”收窄到这组探针与同题两次作答；吠点④点名 DoubleAI 并写明截至时间；主帖日期范围与标签映射补记。“误差范围”“长思维链”因字数删去，核验结论不变。

## 视觉复核

Codex gpt-6-astra，medium，只读，20 张图：PASS，无 BLOCKER。5 个 SHOULD_FIX 采纳：`05`、`06`、`07`、`11` 的高亮扩到译注对应的完整句；`09` 吠点卡的“中文稿、原文、这组”改为点名新智元与 DoubleAI，④补截至时间。NICE_TO_HAVE（原文区字小）不改，保留完整上下文。修改后重渲的卡片由 Claude 逐张目视复查。
