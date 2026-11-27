# 读者稿检查模板

规则见 `EDITORIAL.md`“文风”与“核验流程”第 4 步，四家模型家族互斥表见 `docs/workflow/harness.md`。正文定稿后、与第一轮反向核验并行跑一次。检查者必须与主执行方不同模型家族、不带本期上下文（不开任何工具，不给它仓库路径和事实清单）。它只读，不改稿，不核事实；采纳与否由主执行方逐条判断，按“文风”一节修改。

## 命令

`{OUT}` 为草稿目录下的新子目录。把下面 `---` 之间的提示词填好（`{标题候选}` 取自 `sources/titles.md`，每行一个；`{正文}` 是 `doc_<MMDD>_publish.txt` 全文，去掉第一行标题）写入 `{OUT}/prompt.md`：

**GPT 复核（Codex CLI，默认最新 Sol；主执行方为 Claude / Gemini / DeepSeek 时使用）**：

```bash
codex exec -m gpt-6.2-sol -c 'model_reasoning_effort="medium"' -s read-only --skip-git-repo-check --ephemeral -C "{OUT}" -o "{OUT}/report.md" - < "{OUT}/prompt.md"
```

这是 Linux / macOS 写法；Windows 把 `-s read-only` 换成 `-s danger-full-access`（Windows 原生 Codex 沙箱不可靠，见 `docs/workflow/hosts.md`）。

**Claude 复核（Claude CLI，默认最新 Opus；主执行方为 GPT / Gemini / DeepSeek 时使用，在 `{OUT}` 里运行）**：

```bash
claude -p --model opus --effort high --tools "" --strict-mcp-config --no-session-persistence < "{OUT}/prompt.md" > "{OUT}/report.md"
```

**Gemini / DeepSeek 复核（备选）**：把 `{OUT}/prompt.md` 全文交给对应跨家族、无工具、无本期上下文的独立会话，输出原样保存为 `{OUT}/report.md`。

约 2 分钟。把 `{OUT}/report.md` 存为本期 `sources/style-review.md`，在末尾补一节“采纳”：逐条写采纳或不采纳及理由。

## prompt.md

---

你是一个刷小红书的普通读者：对 AI 有兴趣，但不是从业者，不认识行业术语。下面是一篇待发的图文正文和几个标题候选。只凭下面的文字回答，不要上网，不要读任何文件，不要改写全文，不要核对事实。

请输出四节，每条都引用原句；某节没有问题就写“无”。

1. 标题：假设你在信息流里看到这些标题，你会点哪个？排出前三，各用一句话说为什么。哪些标题你看不出在说什么事？只看你排第一的那个标题、不看正文，你以为发生了什么？
2. 读不懂：逐句列出你卡住的地方——看不懂的词、要读两遍的句子、不知道主语是谁、数字不知道在比什么。
3. 像 AI 写的：哪些句子一读就像 AI 生成的？说清像在哪里，例如“A 不等于 B”式的告诫、段末一句道理、对仗的小标题、空洞的建议、营销腔的“你”。不要泛泛评价文风。
4. 会划走的地方：读到哪一段你就不想看了？哪条“吠点”你不在乎？读完以后，你知道这件事跟你有什么关系、你能做什么吗？

=== 标题候选 ===
{标题候选}

=== 正文 ===
{正文}

---
