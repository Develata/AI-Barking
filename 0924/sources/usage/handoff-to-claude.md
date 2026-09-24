# 可粘贴回 Claude 的取证交接

0924 取证整理已交付，未改任何原有文件，未写正文或编辑结论；未 commit/push，X 全程只读。

新增核心文件：

- `0924/sources/usage-evidence.md`：A/B/C/D、偏差与未验证事项。
- `0924/sources/usage/`：x_raw.jsonl、x_coded.jsonl、x_excluded.jsonl、x_blind20.jsonl；协议/请求日志、逐条决策表、统计程序与数据、三个结构化摘录、原始网页/CSV、核验结果。全部新增文件逐项列于 `new-files.txt`。
- `0924/images/30-arena-text.png`、`30-arena-webdev.png`、`31-openrouter.png`、`32-frontiermath.png`。图源与限制见 `0924/sources/usage/README.md`；原 `images/README.md` 未改。

A/B/C 数字摘要（300 字以内）：Arena WebDev max：Opus 1818±21、Astra 1792±12；Text 页仍标9月13日。OpenRouter month普通流量：Opus 2218亿、Astra 3.092万亿token（窗口口径，非严格自发布累计）。X原始1414、去重1122、纳入310；直接比较73帖中，偏Opus43、偏Astra12、混合18；Wilson区间与情感/任务分组已列全，不能外推总体用户。

指定 Git Bash 命令的真实输出（另存 git-bash-validation.txt）：

```text
   1122 x_raw.jsonl
    310 x_coded.jsonl
     20 x_blind20.jsonl
   1452 total
jsonl ok
0
A  0924/doc_0924.md
AM AGENTS.md
?? .gitignore
?? 0924/doc_0924_publish.txt
?? 0924/images/
?? 0924/sources/
?? EDITORIAL.md
?? brand/
?? templates/
```

额外验证：77 个原有文件 SHA-256 全部不变，修改/删除为0、范围外新增为0；33个 Wilson 区间用独立二次方程根公式复核；JSONL 分区、原文短摘、盲审字段及固定种子均通过。

问题/假设/未完成项：X第8组top遇429，已停止；每批上限100，不是穷尽搜索。310帖是单一编码者第一轮结果（含285账号），尚无Claude一致率；原始简介均空，仅官方来源确认的Boris Cherny标employee=true，其余false为未知身份。请Claude先只读x_blind20.jsonl独立编码，再对照编码表。OpenRouter未取得严格按发布时刻累计，报告保留month窗代理及batch分列。Epoch误差为SE，不是95% CI；Epoch/MathArena未找到Opus5.5，不以旧版替代。MathArena误差置信水平与评测日期未确认。数学成果分别标明内部Astra、未命名内部模型、GPT Pro等；未独立复验论文/Lean工程。所有缺口均已保留，没有填补推测数据。
