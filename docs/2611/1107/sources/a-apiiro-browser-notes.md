# Apiiro 原文阅读记录（内置浏览器）

- 页面：https://apiiro.com/blog/coding-agents-leaking-secrets-github-dont-ask-again
- 读取：北京 2026-11-09，由 Claude 经 Develata 批准，在桌面端内置浏览器读取（`get_page_text`）。自动化访问（headless Chrome、curl、`barking fetch`）被 Cloudflare 质询拦下，未绕过，见 `capture-log-a.md`。本文件是要点转述，不是原文存档；页面未截图。
- 级别：Apiiro 自报；页面标题栏称自家产品为 AI AppSec Agent。多家报道（iTWire、THN，见 `a-itwire.*`、`a-thn.*`）转述了大部分数字，可用于复核。原页未显示发布日，报道写 11 月 5 或 6 日。

## 数字

| 项目 | 数值 |
|---|---|
| 公开 `.claude/settings.local.json` 副本 | 104,185 份，103,617 个仓库 |
| 含“real secret”的文件 | 1,122 份（1.1%） |
| 这些文件里的不同密钥 | 1,379 个 = 服务商密钥 636 + 配公网主机的密码或令牌 743 |
| 其他位置（文件历史、共享 settings.json、对话记录等）另有 | 1,160 个 |
| 总计 | 2,539 个不同密钥，1,994 个仓库 |
| 涉及组织 | 63 个，1 家确认密钥曾有效 |
| 历史版本里、当前副本没有的密钥 | 734 个，509 个仓库 |
| 分类 | 直连数据库较多：数据库密码 379、Supabase service-role 密钥 122 |
| 留存 | 11/2 时都在默认分支；含密钥仓库的首次提交中位数早 217 天 |
| 通用扫描器 | 1,122 份里 gitleaks 标出 564、TruffleHog 643，215 份两者都没标出 |
| Claude 共同署名 | 追溯的 1,105 个含密钥仓库里，539 个有 Claude 共同署名的提交改动过该文件 |

## 机制（Apiiro 的说法）

- 命令里带密钥时，用户选“Yes, and don't ask again”，规则按命令字符串存入项目内的 `.claude/settings.local.json`。称用 Opus-5.5、Fable-5.1 复现。
- 官方防护只写进开发者的全局 git 忽略名单，不写仓库自己的 `.gitignore`；文件夹后来才成仓库、换机器克隆、文件已被提交，都不受保护。
- 文件已被 git 跟踪时，Claude Code 的调试日志有记录，但不提示用户，后续批准继续写入。
- 16 款编程工具的文档对比里，只有 Claude Code 被确认默认把完整命令写进项目文件（文档对比，不是实测）。

## 局限（Limits 一节）

- 数据来自 11 月 1–3 日只读的 GitHub 搜索；没有使用或测试任何凭据，没有联系任何主机。
- “real secret”指符合服务商格式或指向公网主机的凭据，“not one known to be valid”。
- 判定为规则过滤，加独立 LLM 复核（按书面标准，只看打码上下文）；gitleaks、TruffleHog 未开验证。
- Apiiro 称发表前已告知 Anthropic，11 月 2–3 日逐一通知了相关组织。

## 更新段（11 月 8 日）

Apiiro 转述 Claude Code 负责人 Boris Cherny 在 HN 的回应，HN 原文见 `a-hn-50407882.json` 与 `evidence-a.md` A5。先行报告：四个 GitHub issue、ironPeak、Lakera，核对结果见 `evidence-a.md` A7、A10。
