# 1111 期 B 组取证：AWS 模拟器“完全由智能体写成”

执行：Sonnet 5.5（`claude-sonnet-5-5`）。取证时刻 2026-11-12 17:53–18:15（北京时间，本机时区 UTC+8）。存档前缀 `b-`，位于 `docs/2611/1111/sources/`；抓取与截图日志见 `capture-log-b.md`。图片在 `docs/2611/1111/images/`：本组区间 13–24 用尽，溢出用 37–44（编号区间之外，由 Claude 渲染时重编；A、C 组本次都未超出 36，故 37–44 暂无冲突）。

时间一律北京时间（UTC+8），括号保留原时区。“已找到”只表示“该文字/数字确在该一手来源上”，不表示对应说法本身成立。

## 一、先说结论（给 Claude 的要点）

1. 官方文与 README 都在，原句逐字核到：“The emulator was written entirely by autonomous agents. Our team hasn’t written or reviewed any of the code.”（Matthew Goldberg，亚马逊新闻站，11 月 11 日）。这句话说的是**代码**没人写、没人审。设计、指令、护栏、流水线仍是人做的，官方自己也这么写（见 B2）。
2. 扫描说法与原文不符之处见“扫描说法勘误”。最大的一条：“数千个智能体、每天修数千个 issue”官方文里并存三种说法——导语与要点写“Thousands of autonomous AI agents wrote every line of code”，正文写 “thousands of issues fixed every day as hundreds of agents concurrently work”（数百个智能体同时工作）；SiliconANGLE 转述为 “dozens of autonomous agents”（数十个）。
3. 官方“保真度”分数是模拟器自己的及格线：v1.0.322 发布时附带的 `scores.json.gz` 里，我自己数（口径见 B4-3）是 42,937 个测试单元，24,243 通过（56.5%）、18,614 失败、80 跳过。这是官方文件、我的汇总，不是官方给出的总分；官方只按服务逐个发布“k of n”。
4. 公开仓库里的提交，抽样 700 条（共 29,684 条）全是 IssueFinderAgent / IssueFixerAgent；首条与 `vendor/` 的“更新核心组件”提交作者是 “Scaffolder”（Producer: scaffold）。`vendor/`（框架，Apache-2.0）是怎么写成的，官方没说（见 B5-5）。评测套件仓库的发布提交署的是个人姓名，不是机器人（见 B2-5）。
5. 官方文承认：GitHub 上的历史只是发布流程最后一步才推送的副本，主仓库是 CodeCommit；“我们让智能体建过很多版本”，Python 解释器那个是早期版本（B3）。

## 二、逐条取证表

### B1 “written entirely by autonomous agents … hasn't written or reviewed any of the code”

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B1-1 官方文正文 | L1（作者 Matthew Goldberg，署名 “Sr. Principal Engineer”，Amazon 官方新闻站） | https://www.aboutamazon.com/news/aws/how-autonomous-ai-agents-built-aws-emulator | “Today, we released AWS Emulator, a local emulator of AWS services that you can run on your machine to build and test your AWS applications. The emulator was written entirely by autonomous agents. Our team hasn’t written or reviewed any of the code. We could do this because the guardrails we’ve put in place gave the agents space to work independently and us the trust that they don’t color outside these lines.” | `13-aws-official-claim.png` | 发布 2026-11-12 01:25（2026-11-11 17:25:18Z，页面 JSON-LD datePublished；阅读 5 分钟）。页面 `<title>` 与 og:title 为 “AWS Emulator: a local emulator built 100% by autonomous AI agents”，页面显示的标题（H1）为 “How we built the agents that built AWS Emulator”。存档 `b-official-post.html/.txt/.headers` | 已找到 |
| B1-2 导语与要点 | L1 | 同上 | 导语：“Thousands of autonomous AI agents wrote every line of code in AWS Emulator, a new local emulator for AWS now available for download.” 要点：“AWS Emulator is a local emulator of AWS services, 100% written by autonomous AI agents.”“Thousands of autonomous agents write tests, find issues, and fix code without human review.”“The source code is available on GitHub under a CC0 license and is constantly being updated.” | `13-aws-official-claim.png` | 导语/要点写“数千个”，正文写“数百个同时”（见 B6） | 已找到 |
| B1-3 AWS 新闻博客（另一篇官方发布，署名 Daniel Abib） | L1 | https://aws.amazon.com/blogs/aws/test-your-applications-against-aws-locally-with-aws-emulator/ | “AWS Emulator is built and maintained by agents from the AWS API models using Amazon Bedrock AgentCore Runtime Instances, AWS CodePipeline, and AWS CodeCommit, so it stays current with AWS as those models change.” / “Because the code is written by agents, an independent security agent reviews changes adversarially, both while the fix is being made and again in the release pipeline as a final check before anything ships.” | 无（存档 `b-aws-blog.html/.txt`） | 发布 2026-11-12 03:00（2026-11-11 11:00:44 PST，页面 article:published_time）。这篇没有“没人写、没人审”这种句子，只说“由智能体构建和维护” | 已找到 |
| B1-4 Vogels/Swami 博客 | L1（Werner Vogels 个人博客，Amazon CTO 署名；正文是 Swami 的客座文章） | https://www.allthingsdistributed.com/2026/11/aws-emulator-and-the-engineers-job-when-agents-write-the-code.html | “All of AWS Emulator’s code was written and tested by agents, and it is built from more than 400 API models covering more than 200 AWS services.” | `18-vogels-spec-tests.png` | 发布 2026-11-11 22:30（2026-11-11 06:30 PST，页面元数据，官方未给更细时刻；日期 November 11, 2026，1671 words）。“被指认的作者”：Vogels 开头写 “I am honoured that he wanted to write a guest post” | 已找到 |
| B1-5 README/文档对“由谁写”的表述 | L1 | https://docs.aws.amazon.com/emulator/latest/userguide/what-is-service.html（存档 `b-doc-whatis.txt`）；https://raw.githubusercontent.com/aws/aws-emulator/HEAD/.github/CONTRIBUTING.md | 文档：“AI agents keep the emulator up to date with AWS. The emulator improves continuously, with updates many times a day, so it keeps up as AWS services change and grow.” CONTRIBUTING：“We don’t accept pull requests: AI agents keep the emulator up to date with AWS, with regular updates.” 仓库 README 本身没有“agents wrote”的句子 | `39-docs-limitations.png` | README 不提智能体写代码；仓库根 `AGENTS.md` 写 “this EmulatorCore-generated emulator project”（见 B5-5） | 已找到 |

### B2 规格与判定标准：Smithy 模型、对真实 AWS 跑的保真测试、门禁；人做了什么

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B2-1 规格 = AWS API 模型，参照实现 = 真实 AWS | L1 | 官方文（同 B1-1） | “AWS has both of those; the AWS API Models are our spec and the actual AWS cloud is our reference implementation.” | `14-aws-kiro-prompt.png` | 起因是 Anthropic 研究者“今年早些时候演示智能体自主造出 C 编译器”（官方文原句 “researchers at Anthropic demonstrated that AI agents could autonomously build a C compiler”） | 已找到 |
| B2-2 最初的人类指令（原型） | L1 | 官方文 | “I gave Kiro the simplest prompt that I could think of without any expectation that this would work: 1. Write tests based on the AWS Command Line Interface (CLI) 2. Iterate on those tests until they pass against real AWS 3. Build an in-memory implementation that those same tests pass against” / “Kiro came back after 15 minutes with a passable first version of an Amazon Simple Queue Service (Amazon SQS). I refined my prompt and had Kiro build me the rest of AWS over the weekend. And it did. It obviously had gaps” | `14-aws-kiro-prompt.png` | 原型用的是 Kiro（亚马逊自家编码智能体）；官方文没说生产流水线里的智能体用哪家模型（B7 里 HN 有人猜是 Claude，仅 L6 猜测） | 已找到 |
| B2-3 测试由谁写、对照什么 | L1 | 官方文 | “Agents devise these tests from publicly released AWS API Models and verify each one against real AWS before submitting. We have tens of thousands of fidelity tests across all of AWS” / “These tests are one of our most fundamental guardrails in our system because they provide an unambiguous goal that the agents need to meet and can be rerun repeatedly without calling AWS again.” | `15-aws-three-teams-tests.png`（含官方架构图：Smithy API models = “the spec”，Real AWS = “the reference”，三队：Test writers / Issue finders / Fixers） | 测试由智能体写、由智能体对真实 AWS 验证；人定的是“用什么当规格和标准”。评测套件 README：“Every fidelity test was verified against at least two sandboxed AWS accounts” / “Behavior that no fidelity test covers is not verified.”（`37-evalsuite-readme.png`） | 已找到 |
| B2-4 门禁 | L1 | 官方文 | “This gate obviously blocks changes that don’t compile. But it also includes agents that review against coding standards, verify the code is secure, and ensure that the change did not regress fidelity. If any of those checks fail, the change is sent back to the agent for rework; if they all pass, it is merged to mainline.” / “Without this merge gate, we wouldn’t trust what our agents are building.” | `16-aws-merge-gate.png` | 门禁里的“审查”也是智能体；Vogels 另写 “deterministic checks that stop agents from touching code they are not supposed to”（`19-vogels-engineer-job.png`） | 已找到 |
| B2-5 人在流水线里做什么（官方原文） | L1 | Vogels/Swami 博客；官方文 | Vogels 博客：“When agents write the code and check their own work, the engineer’s job is to give them room to work on their own and to build the specs, tests, and merge gates that decide when that work is correct.” / “An engineer built the scaffolding that runs them.” / “We expect agents to build and check most software, with engineers deciding what gets built and when the work is correct.” 官方文：“Turning that into a robust product took more than a weekend. … In the case of AWS Emulator, it required building guardrails that ensure the agents color within our lines.” | `19-vogels-engineer-job.png`；`14-aws-kiro-prompt.png` | 原文把“人的活”说成：定规格、搭脚手架、设计测试与合并门禁、决定做什么与何时算对。**评测套件仓库 `aws/aws-emulator-evaluation-suite` 的 README 写 “AI agents author, manage, and maintain it”，但它在 GitHub 上只有 2 次提交：2026-10-26 22:23Z “Initial commit”（amazon-ospo-app[bot]）与 2026-11-11 16:16:17Z “AWS emulator evaluation suite: fidelity tests and the fidelity-tests-runner”（作者署个人姓名，见 `b-gh-evalsuite-commits-api.json`）**，即公开发布这一步是署名个人提交；是否人工操作官方没说 | 已找到 |

### B3 早期智能体自写 Python 解释器

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B3-1 出处与原句 | L1 | 官方文，“How AWS Emulator ships to GitHub” 一节最后一段 | “We’ve had our agents build many versions of this emulator as we’ve refined the building machine. My personal favorite was from an earlier version, when they decided to build their own Python interpreter to support AWS Lambda functions. We’ve since given them guidance to reuse existing libraries, something that may seem obvious to someone who doesn’t need a coloring book.” | `17-aws-python-interpreter.png` | 说的是**早期版本**，不是公开仓库里的版本；公开仓库的提交历史只从 2026-10-21 开始（首条 “Initial emulator scaffold”，作者 Scaffolder）。当前版本的 Lambda 实现有无自写解释器，我没有查证：仓库 issues 目录里有大量 “the Lambda Python runtime has no boto3 / crippled stdlib” 之类条目（例：`issues/things-that-do-not-work/20261105T081644Z-lambda-s3-stepfunctions-the-lambda-python-runtime-ships.md`），说明当前版本的 Python 运行时仍有明显缺口，但这些条目没说它是怎么实现的 | 已找到 |

### B4 覆盖范围：服务/API 数量与未实现部分

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B4-1 官方给的数字（并存多种） | L1 | 官方文；Vogels 博客；AWS 新闻博客；What’s New | 官方文：“it generated an empty web server that could handle requests for the 438 AWS API Models but did nothing”（起点是 438 个 API 模型，且当时“什么也没实现”）；Vogels：“built from more than 400 API models covering more than 200 AWS services”；Vogels 另写 “today AWS has over 200 services”；What’s New（https://aws.amazon.com/about-aws/whats-new/2026/11/aws-emulator-now-available/，2026-11-11，未给时刻）：“Services in AWS Emulator are generated from the API models AWS publishes … When AWS adds a service or API, it shows up in AWS Emulator, and its behavior is filled in and verified release by release.”；AWS 新闻博客：“It supports the APIs of AWS services that have an API model in the AWS API models repository on GitHub” | `15-aws-three-teams-tests.png`、`16-aws-merge-gate.png`、`18-vogels-spec-tests.png` | 官方说法是“按 API 模型生成服务骨架”（438/400+），实现程度“因服务与功能而异”（文档 Limitations：“How closely the emulator matches AWS varies by service and feature.”）。我没有在任何官方页面找到 “426 services” 这个数（只在 InfoWorld、frontword 两家媒体里出现，见勘误） | 已找到 |
| B4-2 仓库实际结构（我数的） | L1 数据，我的口径 | GitHub API：`/repos/aws/aws-emulator/git/trees/<sha>`（非递归，逐层取，存 `b-gh-issues-summary.json`） | `services/` 目录条目 438 个，其中 436 个以 `service-` 开头（每个服务一个 crate），另有 `common` 与 `.gitkeep` | 无 | 2026-11-12 17:55 查询，分支 main，提交 7753983c。“436 个服务 crate”不等于“436 个已实现的服务”：官方文说起点就是 438 个模型对应的空壳 | 已找到 |
| B4-3 保真度分数（官方发布的数据，我汇总） | L1 数据，我的口径 | https://github.com/aws/aws-emulator/releases/download/v1.0.322/scores.json.gz（1,140,628 字节，SHA-256 eca6d69bfa6e801d224f88dd47907ef1573002161e57d1b092de655db77e08cd；文件本身只存草稿目录，未入库） | 文件字段：`commit` 7753983c6e495ec3612695b1336bf4936fc7b7fd，`scoredAt` 2026-11-11T00:09:58Z，`units[]` 每项有 `outcome`（pass/fail/skipped）、`kind`（behavioral/wire）、`services[]`。官方定义（https://docs.aws.amazon.com/emulator/latest/userguide/fidelity.html）：“A fidelity score is, for each service, the number of fidelity tests that the emulator passes out of the total for that service, such as 40 of 50.” | `44-aws-fidelity-doc.png` | **我的汇总（`b-fidelity-scores-v1.0.322-summary.json`）**：units 42,937（behavioral 32,402、wire 10,535）；pass 24,243（56.5%）、fail 18,614、skipped 80；有测试单元的服务形态 355 个，其中 21 个一条都没过、12 个全过；服务级通过率中位数 60.1%。大服务例：CloudFormation 795/4,206（含 20 跳过）、Lambda 1,561/3,006、S3 1,763/2,814、EC2 958/2,470、DynamoDB 1,153/1,694、SQS 1,032/1,523。把 “unit” 当作官方说的 “fidelity test” 是我的推断（与官方文 “tens of thousands of fidelity tests” 数量级一致）；官方文档提醒 “Compare scores only when they use the same version of the fidelity tests.” 且 “A fidelity test that the emulator doesn’t pass yet is a known gap that we publish on purpose and track.” 测试集本身也是智能体写的、会增长，所以 56.5% 是“该次发布时刻、该套测试”的快照，不是对 AWS 的全面符合度 | 已找到（数字为我的汇总） |
| B4-4 未实现操作的行为 | L1 | 文档 fidelity.html；What’s New | “If it doesn’t implement an operation yet, it returns HTTP 501 with the error code NotImplemented, and a test that calls it fails right away.” What’s New：“Operations it doesn’t yet support return 501 Not Implemented.” | `44-aws-fidelity-doc.png` | InfoWorld 写的是 “Operations that have not been implemented or verified return HTTP 503 errors”（与官方 501 不符，见勘误） | 已找到 |
| B4-5 things-that-do-not-work 目录 | L1 数据，我的口径 | https://github.com/aws/aws-emulator/tree/main/issues/things-that-do-not-work | GitHub 页面提示：“Sorry, we had to truncate this directory to 1,000 files. 5,545 entries were omitted from the list.”（即共 6,545 个条目）；README：“APIs and behaviors that don’t work yet are under `issues/things-that-do-not-work/`, and ones that have since been fixed move to `issues/things-that-now-work/`.” | `38-gh-things-do-not-work.png` | 我用 GitHub trees API 数（2026-11-12 17:55，提交 7753983c，非截断）：`things-that-do-not-work/` 6,545 个条目（含 1 个 `.gitkeep`，即 6,544 个 .md）；`things-that-now-work/` 12,709 个条目（12,708 个 .md）。按文件名时间戳（即提交/建档时间）分日计数在 `b-gh-issues-summary.json`。文件名时间戳是建档时间，不是修复时间，所以不能由此推“每天修几个” | 已找到 |

### B5 不用于生产、不测安全；遥测；不接受 PR；许可

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B5-1 不用于生产、不用于测安全 | L1 | https://github.com/aws/aws-emulator（README）；文档 security.html | README：“AWS Emulator is for building and testing. Don’t use it for production workloads or to test security.” 文档：“Don’t use AWS Emulator to test whether your application is secure. Passing tests in the emulator don’t show that your permissions are correct or safe.” / “The emulator never denies a request signed with its own key.” | `21-gh-readme-production.png`；`39-docs-limitations.png` | 存档 `b-gh-readme-raw.txt`、`b-doc-security.txt` | 已找到 |
| B5-2 文档列出的局限 | L1 | https://docs.aws.amazon.com/emulator/latest/userguide/what-is-service.html | “Policies – The emulator doesn’t evaluate AWS Identity and Access Management (IAM) policies and resource-based policies the way AWS does.” “One Region and one account – … it doesn’t keep resources separate by Region or account.” “Verified in one Region – We check the emulator’s behavior against AWS in the US West (Oregon) Region (us-west-2) only, except for global services, which we check in the US East (N. Virginia) Region (us-east-1).” “Test runs, not long-running use – We haven’t tested the emulator for long-running use or load testing.” | `39-docs-limitations.png`（页面为浅色） | 对照 What’s New 的 “AWS Emulator doesn’t depend on an AWS Region.” 两处说法角度不同（前者说运行时不依赖地区，后者说对照真实 AWS 只在 us-west-2/us-east-1 验证），不算冲突 | 已找到 |
| B5-3 遥测默认开启与关闭方式 | L1 | README；文档 telemetry.html | “AWS Emulator is configured to collect and report usage data to AWS, to gain insight into how AWS Emulator is used. Telemetry is on by default.” “Start the emulator with the `--telemetry` option (short form `-t`) set to `off`.” “Set `EMULATOR_TELEMETRY` to `off` before you start the emulator.” 另有配置文件 `[telemetry] enabled = false` 与嵌入式 API 两种关法。文档：“The emulator sends nothing before it prints this notice.”（启动时打印一行提示）；收集内容含“which AI coding assistant drives the emulator, if any”、每个操作的调用次数与结果（含 501），保留最多 90 天，“We don’t share it with third parties.” | `24-gh-readme-telemetry.png` | 存档 `b-doc-telemetry.txt`。HN 有人因此弃用（见 B7） | 已找到 |
| B5-4 不接受 PR；许可证 | L1 | README；`.github/CONTRIBUTING.md`；LICENSE；`vendor/README.md` | README：“This repository does not take PRs.” CONTRIBUTING：“We don’t accept pull requests: AI agents keep the emulator up to date with AWS, with regular updates. To report a problem or ask for a change, please open an issue instead.” README：“This software is licensed under the CC0 1.0 Universal (CC0-1.0) License, except the code in the `vendor/` directory, which is under the Apache License 2.0.” `vendor/NOTICE`：“Copyright 2026 Amazon.com, Inc. or its affiliates. All Rights Reserved.” | `23-gh-readme-license-pr.png` | 仓库根目录没有 CONTRIBUTING.md（404，见失败清单），在 `.github/CONTRIBUTING.md`。GitHub API 许可识别为 CC0-1.0 | 已找到 |
| B5-5 `vendor/` 框架与 “Scaffolder” | L1 数据，我的口径 | 仓库 `AGENTS.md`；`vendor/README.md`；GitHub commits API（`?path=vendor`，存 `b-gh-path-commits.json`） | AGENTS.md：“Developer and agent guide for this EmulatorCore-generated emulator project for `aws` services.”“**Emulator Core**: v0.1.0”；目录树：“vendor/ # vendored, agent-immutable framework (path deps)”；README 目录树：“vendor/ # framework crates — do not edit (replaced on update)”。`vendor/` 的全部 9 次提交作者都是 “Scaffolder”，提交说明 “chore: update core components / Producer: scaffold”；`.github` 的唯一提交、`AGENTS.md` 的两次提交作者也都是 Scaffolder | `20-gh-repo-files.png`（目录页，`vendor` 一行显示 “chore: update core components”） | `vendor/` 下有 engine、build-support、smithy-macros、emulator-telemetry、server 等 8 个子目录。**这套 EmulatorCore 框架是不是智能体写的、谁写的，官方没有说**。“100% 由智能体写成”的说法，对 `vendor/` 没有一手依据，我也没有反证 | 已找到（官方未说框架由谁写，属“未找到一手来源”） |

### B6 “数千个智能体、每天修数千个 issue”等规模说法

| 说法 | 级 | 一手来源 URL | 原文摘句（原语言，逐字） | 截图文件 | 条件/口径/时区 | 状态 |
|---|---|---|---|---|---|---|
| B6-1 智能体数量 | L1 | 官方文 | 导语：“Thousands of autonomous AI agents wrote every line of code in AWS Emulator”；要点：“Thousands of autonomous agents write tests, find issues, and fix code without human review.”；正文：“We’ve seen thousands of issues fixed every day as hundreds of agents concurrently work on the codebase.” | `13-aws-official-claim.png`、`16-aws-merge-gate.png` | “数千个智能体”（导语/要点）与“数百个同时工作”（正文）并存，官方文未解释二者关系；正文数字指**同时**在跑的数量，导语未说累计或并发 | 已找到 |
| B6-2 “每天修数千个 issue” | L1 | 官方文；文档 | 官方文（上行）；文档：“updates many times a day” | `16-aws-merge-gate.png` | 仓库旁证（我的口径，不推算）：GitHub 主分支 29,684 次提交（commits API `per_page=1` 的最后一页页码，2026-11-12 17:55 查询）；首条 2026-10-21 20:54:17Z（Scaffolder, “Initial emulator scaffold”），最新（查询时）2026-11-10 20:39:05Z（IssueFinderAgent）；抽样 7 页×100 条（页 1、50、100、150、200、250、296）共 700 条，作者只有 IssueFinderAgent 357 条、IssueFixerAgent 343 条（`b-gh-commits-sample.json`）；`things-that-now-work/` 12,708 个 .md；v1.0.322 发布于 2026-11-11 00:32:13Z（发布账号 aws-emulator-ci[bot]），共 12 个公开发行版（v1.0.2xx–3xx） | 已找到（官方数字）；旁证见左 |
| B6-3 官方对“发到 GitHub 的是什么”的说明 | L1 | 官方文 | “Once merged to mainline, the changes flow through a release pipeline built entirely on AWS Developer Tools, with AWS CodeCommit as the agents’ primary code repository. … Only at the end of the release process do we also publish the code to GitHub.” | `17-aws-python-interpreter.png` | 公开的 GitHub 历史是发布副本，不是智能体的主仓库；官方文说 “over thousands of commits” | 已找到 |

### B7 热度与社区质疑（热度用 `barking heat`，评论仅作 L6 线索）

`barking heat docs/2611/1111 b --item 50456631 --item 50456740 --item 50456319 --hn "AWS Emulator"`，取数时刻 2026-11-12 17:58:10（北京时间），`b-heat.json`、`b-hn-*.json`。

| 条目 | 标题 | 分数 / 评论 | 提交时刻 |
|---|---|---|---|
| https://news.ycombinator.com/item?id=50456631 | “AWS emulator”（github.com/aws） | 847 分 / 235 评（18:06 页面显示 848 分） | 2026-11-12 02:21（2026-11-11 18:21Z） |
| https://news.ycombinator.com/item?id=50456740 | “AWS Emulator and the engineer's job when agents write the code” | 111 分 / 2 评 | 2026-11-12 02:28 |
| https://news.ycombinator.com/item?id=50456319 | “AWS Emulator: 100% built by agents”（官方文） | 10 分 / 0 评 | 2026-11-12 01:55 |
| https://news.ycombinator.com/item?id=50462073 | “Test applications against AWS locally with AWS Emulator”（AWS 新闻博客） | 2 分 / 1 评 | 2026-11-12 09:59 |

HN 不公开评论得分，所以下面“得分”只能写回复数。Reddit 两条搜索都被重定向到登录页，未取到（记入 `b-heat.json` errors）。截图 `40-hn-50456631-title.png`（标题与 848 分、235 评；含提交者的公开用户名）。

| 评论（链接） | 原文要点（逐字摘） | 回复数 | 备注 |
|---|---|---|---|
| https://news.ycombinator.com/item?id=50457211 forgotpwd16 | “It’s funny how author talks so much about testing when they themselves also boast how they haven’t written or reviewed any of the code.” | 1（挂在 ke4qqq 回复串下） | 质疑“没人审代码却大谈测试”，这是对宣传方式的评价，不是事实 |
| https://news.ycombinator.com/item?id=50457884 Eridrus | “how can we trust the accuracy of this simulator? Agentic loops are great and all, but they can still miss plenty of things.” | 5 | 与 B4-3 的 56.5% 呼应；发布 2026-11-12 04:01（2026-11-11 20:01:56Z） |
| https://news.ycombinator.com/item?id=50462592 AdieuToLogic | “29,684 commits in a repository less than a month old does not instill confidence in this being a ‘real product.’ That is roughly 1,000 accepted commits per day.” | 2；发布 2026-11-12 11:14（2026-11-12 03:14:31Z） | 数字与我查到的 29,684 一致；“日均 1000”是他的算术，我没有核 |
| https://news.ycombinator.com/item?id=50457091 forgotpwd16 | “~30K commits in 3w; quality software for sure. Is someone trying to get promotion by tokenmaxxing?” | 0 | 讽刺与猜测，L6 |
| https://news.ycombinator.com/item?id=50456723 CharlieDigital，及回复 https://news.ycombinator.com/item?id=50456780（ke4qqq） | 前者：“Skeptical of using because it’s not clear whether this was just an exercise or a real product they are going to ship and maintain.” 后者：“It’s a real product that we plan to maintain.”；ke4qqq 随后（50457140）给出官方文链接并写 “Swami, our SVP of AI blogged about it separately and our CTO is tweeting about it” | 77（整串）；ke4qqq 回复 57 | 回复者自称在 AWS（措辞 “our SVP of AI”），HN 上无法核实身份，L6；另有 _msw_（50458268）“Disclosure: I work for Amazon. I’m not directly involved in building this product” |
| https://news.ycombinator.com/item?id=50457261 hbcdbff（引 Vogels 文） | “Claudeslop snuck in at the very end” | 2 | 对 Vogels 文末句式的讽刺，L6 |
| https://news.ycombinator.com/item?id=50457154 paavohtl | “Looks like this was ~100% generated with Claude within the past month. … they’re not using the latest models.” | 3 | **猜测**（据文风词频）。官方只说原型用 Kiro，生产流水线所用模型没说；本条无依据，不进正文 |
| https://news.ycombinator.com/item?id=50459920 scriptsmith（引 security 页）；https://news.ycombinator.com/item?id=50457726 jbmchuck | 前者引文档 “Don’t use the emulator to test … IAM policies and permissions boundaries”；后者 “there isn’t a quick way to reset state other than restarting the process” | 3 / 2 | 局限条目，与文档一致 |
| https://news.ycombinator.com/item?id=50457536 microflash | “Telemetry is on by default. They can’t help themselves, can they? I’ll stick with Floci and Testcontainers.” | 0 | 遥测默认开启，与 B5-3 一致 |

另有一条值得记录：仓库 GitHub Issues 页 2026-11-12 17:55 显示 55 个未关闭 issue（API 返回 55 条，全部 author_association=NONE，即外部用户），例如 #56 “S3 Control: account-ID host prefix makes every Terraform aws_s3_bucket fail with AWS provider >= 6.2”（2026-11-12 09:57:37Z）。这是外部用户在发布一天内报的缺口，不是智能体自报的缺口。存档 `b-gh-issues-page.html/.txt`。

### B8 夸大实例（把事件写歪、或把范围写大的标题/句子）

“夸大”只按字面比较原文：官方原句是“代码没人写、没人审”。下面的“歪”在于：把“设计也由智能体完成”“无任何人工参与”“数百/数十个”“426 项服务”“503”这类官方没有的说法写进去。

| 站点 / URL | 发布时刻 | 发布方 | 原句（逐字） | 与官方的差别 | 是否已更新 |
|---|---|---|---|---|---|
| https://siliconangle.com/2026/11/11/amazons-open-source-aws-emulator-lets-developers-validate-their-code-locally-without-the-cloud/ | 2026-11-12 01:30:26（2026-11-11 17:30:26Z）；最后修改 11-12 10:56:52（02:56:52Z） | SiliconANGLE，Mike Wheatley | “AWS Senior Principal Engineer Matt Goldberg said that the entire service was designed and written exclusively by AI agents, with not even a single line of its code written or reviewed by a human engineer.”；“organizing dozens of autonomous agents running on Amazon Bedrock AgentCore into three separate teams” | 官方只说“代码由智能体写、没人写没人审”；“designed”（设计也是智能体）是 SiliconANGLE 加的，官方文里设计（规格、护栏、流水线）是人做的；“dozens”与官方 “hundreds … concurrently / thousands” 不符 | 页面元数据 dateModified 为 11-12 10:56:52（02:56:52Z）；存档与截图（18:05）即该修改后的措辞，更新前措辞我没有取到 |
| https://www.zhiding.cn/ai-infrastructure/2026/1112/3205711.shtml | 2026-11-12（页面只给日期） | 至顶网（SiliconANGLE 中文转述） | 导语：“该工具由AI代理自主设计并编写，无任何人工参与”；正文：“整个服务完全由AI智能体设计和编写，甚至没有一行代码经过人类工程师的编写或审查” | “无任何人工参与”官方没说，官方文与 Vogels 博客都写了人搭脚手架、设计护栏、给初始指令；“设计”同上 | 未见更新 |
| https://www.itmedia.co.jp/news/article/2611/12/2000003126/ | 2026-11-12 08:56（页面 “公開 2026年11月12日 09時56分”，日本时间） | ITmedia NEWS | 标题：“Amazon、AWSのAPIを手元で再現する「AWS Emulator」を無償公開　コードはすべてAIエージェントが記述”；正文：“人間の開発チームはコードを一切書いておらず、レビューもしていない。” | 与官方一致，无夸大；可作“忠实转述”对照（日文） | — |
| https://frontword.com/ai-race/aws-emulator-agent-built-local-testing/ | 2026-11-12 03:28:52（2026-11-11 19:28:52Z） | frontword，Akhil Ponnada | “supporting all 426 AWS cloud services at launch” / “Despite covering 426 services”；“built entirely by autonomous AI agents” | “426 项服务全覆盖”官方一手页面没有；官方是“按 438 个 API 模型生成，实现程度因服务而异”。同样的 426 也见 InfoWorld（见勘误） | 未见更新 |
| https://www.infoworld.com/article/4247383/aws-unveils-local-emulator-service-to-help-enterprises-accelerate-app-development-and-testing.html | 页面元数据互相不一致：2026-11-11T12:34:59+05:30（= 11-11 15:04:59 北京）与 2026-11-11T17:00:00Z（= 11-12 01:00）；页面日期 Nov 11, 2026 | InfoWorld，Anirban Ghoshal | “Currently, the emulator supports all 426 AWS services”；“Operations that have not been implemented or verified return HTTP 503 errors rather than being treated as successful, it said.” | 官方文档与 What’s New 写的是 “501 Not Implemented”，不是 503；“all 426 services”无官方依据；这篇同时引 IDC 分析师 Devin Pratt（IDC 研究总监）称上线时不模拟限流、故障、快照还原、分区域可用性，也不隔离执行的代码（“does not sandbox the code it executes”），这是有价值的第三方限制，但是 L4 报道里的分析师发言 | 未见更新 |
| https://www.geekwire.com/2026/aws-gives-developers-a-free-local-stand-in-for-its-cloud-built-from-scratch-by-ai-agents/ | Nov 11, 2026 10:51 am（太平洋时间，约 2026-11-12 02:51 北京；页面本身 403 质询，时刻与正文来自 Exa 搜索摘录，不是存档） | GeekWire，Todd Bishop | “AWS says its engineers didn’t write or review a single line of code in its newest developer tool, and the company is touting that as a selling point.”；标题 “built from scratch by AI agents” | 标题的 “from scratch”（“从零造出”）是 GeekWire 的措辞；文中并引 IDC 的限制。属“忠实但加了卖点”类，不算错 | — |

英文社区对此事的“把话说大”主要在评论层，不是标题层，见 B7。没有找到“AI 自己写出整个 AWS”这类字面标题；最接近的是上表“designed … exclusively”“无任何人工参与”。

## 三、可能的吠点（原文里有、线索摘要没提或说反的条件与限制）

1. 官方说“没人写、没人审”，同一篇文章把设计工作放在人身上：规格选择（API 模型）、对照对象（真实 AWS）、最初的三步提示词、三队分工、合并门禁、“让智能体复用现有库”的指导。——官方文 “I refined my prompt”“it required building guardrails”“We’ve since given them guidance to reuse existing libraries”；Vogels 博客 “An engineer built the scaffolding that runs them.” 同一件事的两句话：“代码没人写没人审” ≠ “没有人参与”。
2. 测试、被测物、“审查者”都是智能体：测试由智能体写并对真实 AWS 验证，门禁里的代码审查和安全审查也是智能体（官方文 B2-3/B2-4）。官方称这是用真实 AWS 作参照的“不含糊的目标”，不是人读过代码。
3. 官方自己发布的保真度分数：v1.0.322 的 scores.json.gz 里，我数到约 56.5% 的测试单元通过（B4-3，我的汇总）；仓库里有 6,545 个“还不行”的条目与 12,709 个“已修复”条目（B4-5）；官方文档明说 “Today, the emulator matches AWS most closely for control plane APIs. Fidelity for data plane APIs, and support for infrastructure as code tools … varies by service.”（`44-aws-fidelity-doc.png`）。
4. 不用于生产、不用于测安全；不评估 IAM 与资源策略；只在 us-west-2（全球服务 us-east-1）验证；一个账号一个区域；资源只放内存；没测过长时间运行（B5-1/B5-2）。IDC 分析师另称上线时不模拟限流、故障等（InfoWorld，L4）。
5. 遥测默认开启，可用 `--telemetry off`、`EMULATOR_TELEMETRY=off`、配置文件或嵌入式 API 关闭（B5-3）。
6. 仓库不接受 PR；框架目录 `vendor/` 是 Apache-2.0、由 “Scaffolder” 更新，官方没说它怎么写成（B5-4/B5-5）。评测套件是 2 次提交的只读仓库，发布提交署个人名（B2-5）。
7. GitHub 上看到的不是智能体的主仓库：官方文 “Only at the end of the release process do we also publish the code to GitHub”，主仓库是 CodeCommit（B6-3）。公开历史自 2026-10-21 起，官方说“建过很多版本”，Python 解释器是早期版本的事（B3-1）。
8. 官方文里数量说法不一致：“数千个智能体”（导语/要点）对“数百个同时工作”（正文）；“438 个 API 模型”（官方文）对“400+ 模型、200+ 服务”（Vogels）（B4-1、B6-1）。
9. 模型没说：原型用 Kiro；生产流水线里的智能体用哪家模型，官方文与 AWS 新闻博客都没写（HN 有人按文风猜是 Claude，L6，不能用）。
10. 保真度分数的出处：README 与文档都把“每服务通过多少”交给 releases 页，但我存档的 releases 页静态文本里每个版本只有修复清单与 “Assets” 加载占位，不见分数表（较早的 5 个版本页面写 “Scores are not available for this release.”）；v1.0.322 的分数在 `scores.json.gz` 附件里（1.1 MB 压缩，我自己解压汇总）。页面上是否有 JS 渲染的分数视图，我没有看到（`b-gh-releases.txt`、`b-gh-releases-api.json`，渲染截图只作检查未存）。

## 四、扫描说法勘误（与“已知情况”不符或需补充之处）

1. “emulator was written entirely by autonomous agents. Our team hasn't written or reviewed any of the code.”——**属实**，逐字一致（B1-1）。作者是 Matthew Goldberg，署名 “Sr. Principal Engineer”；Vogels 博客写作 “Senior Principal Engineer on the AWS Developer Tools team”。
2. “覆盖‘400+ API 模型 / 200+ 服务’”——这是 **Vogels 博客**的说法（“more than 400 API models covering more than 200 AWS services”），**不是 Goldberg 官方文**；官方文的数字是 **438 个 AWS API Models**（且是“起点时空壳”的数字）。仓库实际有 436 个服务 crate。InfoWorld 与 frontword 写的 “426 services” 在任何官方一手页面上都找不到。
3. “官方称‘数千个智能体、每天修数千个 issue’”——**部分支持**：导语/要点写“数千个智能体”；正文写“数千个 issue 每天被修，数百个智能体同时工作”。“每天修数千个”确在官方文里，但是官方自述，旁证见 B6-2。
4. “规格来自 Smithy API 模型，保真度测试由智能体从公开 API 模型生成、须对着真实 AWS 跑通”——**属实**（B2-3），评测套件 README 补充“每条测试至少在两个沙箱 AWS 账号上验证过”。
5. “三道门禁（编译/规范/安全/保真）”——**属实但措辞不同**：官方文写“blocks changes that don’t compile … agents that review against coding standards, verify the code is secure, and ensure that the change did not regress fidelity”，即编译 + 三类智能体检查，共四项。
6. “早期版本智能体‘决定自己写一个 Python 解释器’支持 Lambda，后被要求复用现有库”——**属实**，原句见 B3-1；强调“earlier version”，不是公开仓库里的版本。
7. “缺口在仓库 issues/things-that-do-not-work 目录”——**属实**，6,545 个条目（B4-5）。
8. “默认开启遥测（`--telemetry off` 或 `EMULATOR_TELEMETRY`）”——**属实**，另有配置文件与嵌入式 API 两种方式（B5-3）。
9. “仓库不接受 PR；许可 CC0”——**属实但要补一句**：`vendor/` 目录是 Apache-2.0（README 与 LICENSE 都写了）；CONTRIBUTING 在 `.github/` 下，根目录没有。
10. “Werner Vogels 博客（含 Swami 客座）”——**属实**：Vogels 写开头与结尾，主体是 Swami Sivasubramanian 客座。HN 帖 50456740 提交者没有注明这一点。SiliconANGLE 写成 “a second blog post, Swami Sivasubramanian, AWS’ vice president of agentic AI”。
11. HN 数字：50456631 当时 847 分/235 评，与“842/235”基本一致（我 17:58 查）；50456740：111 分/2 评；50456319：10 分/0 评。
12. 已知情况未提到、本组新发现的：**AWS 新闻博客**（Daniel Abib 署名，2026-11-12 03:00 北京）；**What’s New**；**评测套件仓库**及其 README 写 “AI agents author, manage, and maintain it”；**IDC 分析师限制**（InfoWorld）；**官方保真度分数文件及我的汇总（56.5%）**；**提交作者统计**（只有 IssueFinderAgent / IssueFixerAgent / Scaffolder）；**HN 评论里自称 AWS 员工者的回应**（L6）。
13. 时间顺序：v1.0.322 发行版时间戳 2026-11-11 00:32:13Z（= 11-11 08:32 北京）早于官方文（11-12 01:25 北京）约 17 小时，也早于 Vogels 博客（11-11 22:30 北京）。仓库当时是否已对外公开我没有验证（API 显示 created_at 2026-09-11T19:01:12Z，pushed_at 2026-11-11T00:32:10Z），所以不能由此写“公众先拿到源码”。

## 五、截图清单（均已逐张目视确认）

| 文件 | 内容 | 备注 |
|---|---|---|
| `13-aws-official-claim.png` | 官方文标题、导语、署名与日期、Key takeaways、“Today, we released…” 段 | 4 段拼接，页面浅色底；署名旁有作者官方头像（AWS 官方新闻站公开头像） |
| `14-aws-kiro-prompt.png` | “AWS Emulator started as a weekend experiment” 全节（三步提示词、15 分钟、周末） | 单次裁切 |
| `15-aws-three-teams-tests.png` | 三队分工段、官方架构图（Smithy API models = the spec；Real AWS = the reference；Test writers / Issue finders / Fixers）、第一队测试段 | 单次裁切，含图注 |
| `16-aws-merge-gate.png` | 第二队、第三队、合并门禁段，“thousands of issues fixed every day as hundreds of agents concurrently work” | 单次裁切 |
| `17-aws-python-interpreter.png` | “How AWS Emulator ships to GitHub” 一节：CodeCommit 为主仓库、最后才推 GitHub、issues 目录、Python 解释器句 | 2 段拼接（中间省略的是一张配图，无正文），段间黑色间隔条是工具生成的 |
| `18-vogels-spec-tests.png` | Vogels 博客：Goldberg 的假设、Smithy 作规格、保真测试、“more than 400 API models covering more than 200 AWS services”、合并门禁 | 单次裁切；页面浅色 |
| `19-vogels-engineer-job.png` | Vogels 博客：“The engineer’s job when agents write the code” 全节 | 单次裁切 |
| `20-gh-repo-files.png` | GitHub 仓库首页：Issues 55、3.4k stars、最新提交作者 IssueFinderAgent、文件列表（含 vendor “chore: update core components”） | 单次裁切；匿名 headless Chrome，登录栏已裁掉；底部 “View all files” |
| `21-gh-readme-production.png` | README 开头（含 “Don’t use it for production workloads or to test security.”）与 Known issues | 2 段拼接，间隔条黑色 |
| `22-gh-readme-telemetry.png` | **废图**：第 2/3 段间有一行被切断，见“待删除” | 被 24 取代 |
| `23-gh-readme-license-pr.png` | README “Reporting issues and contributing”（does not take PRs）与 License（CC0 / vendor Apache-2.0） | 单次裁切 |
| `24-gh-readme-telemetry.png` | README 遥测节：默认开启、`--telemetry off`、`EMULATOR_TELEMETRY` | 3 段拼接，省略了 docker 与配置文件小节 |
| `37-evalsuite-readme.png` | 评测套件 README：“AI agents author, manage, and maintain it”；“verified against at least two sandboxed AWS accounts” | 单次裁切 |
| `38-gh-things-do-not-work.png` | issues/things-that-do-not-work 目录页：“truncate this directory to 1,000 files. 5,545 entries were omitted” | 右侧列是 GitHub 的加载占位条，不是缺图 |
| `39-docs-limitations.png` | AWS 文档 Limitations 全节 + “How AWS Emulator keeps improving” | 2 段拼接；页面浅色 |
| `40-hn-50456631-title.png` | HN 帖标题、848 分、235 评（18:06 取） | 含提交者公开用户名，不含抓取者信息 |
| `41-aws-fidelity-doc.png` | **废图**：底部切断一行，见“待删除” | 被 44 取代 |
| `42-news-siliconangle.png` | SiliconANGLE 标题与 “designed and written exclusively by AI agents” 段 | 单次裁切 |
| `43-news-zhiding.png` | 至顶网标题、导语 “无任何人工参与”、正文 | 2 段拼接，页面浅色 |
| `44-aws-fidelity-doc.png` | AWS 文档 fidelity 页 “What is fidelity?” 全节（501 NotImplemented、保真度分数定义、控制面/数据面） | 单次裁切 |

没有为“保真度 56.5%”截图：该数字来自我对官方文件的汇总，不是页面上看得到的数字；如要上卡，由 Claude 以 `b-fidelity-scores-v1.0.322-summary.json` 的数字自制（并标明“据官方发布的 scores.json.gz 汇总”）。

## 六、本组假设与限制

- 官方文“Published: November 11, 2026”无时刻；北京时间用页面 JSON-LD 的 17:25:18Z。
- “保真度 56.5%”把 scores.json.gz 的 `units` 当作官方所说的 “fidelity tests”，是推断（数量级与官方 “tens of thousands” 一致，但官方没有在文档里定义 “unit”）。
- GitHub 提交与 issues 的数字都是 2026-11-12 17:55–18:07 的瞬时值，仓库每天都在更新。
- HN 评论得分不公开；评论列表按 Algolia 返回的顺序，未做全面通读（208 条嵌套评论中用关键词扫描了约 40 条）。
- GeekWire 页面被 Cloudflare 质询拦住（403），未存档，时刻与引文来自 Exa 搜索摘录，已标明。
- 只做了 B 组。
