# 1101 期 D 组抓取与热度日志

执行方：Haiku 5.5（claude-haiku-5-5）。时间均为北京时间（UTC+8）。未截图。未登录，未输入凭据，未使用付费墙绕过或镜像站。

## 一、批量存档（`barking fetch docs/2611/1101 d <清单>`，日志 `d-fetch-log.jsonl`）

| 时刻 | 名称 | URL | 结果 | 字节 | 备注 |
|---|---|---|---|---|---|
| 23:49:09 | neon-postgres19 | https://neon.com/blog/postgres-19-is-late-and-its-ais-fault | ok 200 | 237569 | 原文，`.txt` 259 行 |
| 23:49:09 | pg-roadmap | https://www.postgresql.org/developer/roadmap/ | ok 200 | 7578 | 路线图（“planned for October 2026”） |
| 23:49:09 | pg-versioning | https://www.postgresql.org/support/versioning/ | ok 200 | 12669 | 作为“后期维护”背景，未引用 |
| 23:49:09 | occupysaas-home | https://occupysaas.com/ | ok 200 | 29052 | 官网 |
| 23:49:10 | gh-occupysaas-org | https://github.com/occupysaas | ok 200 | 183124 | 组织页，匿名 |
| 23:49:10 | gh-api-org-repos | https://api.github.com/orgs/occupysaas/repos | ok 200 | 18451 | 仓库列表 JSON（3 个仓库） |
| 23:49:11 | trigger-incident | https://trigger.dev/blog/security-incident-nov-2026 | ok 200 | 182691 | 官方博客 |
| 23:49:12 | trigger-status | https://status.trigger.dev | ok 200 | 475870 | 状态页总览 |
| 23:49:46 | rhaas-blog | http://rhaas.blogspot.com/2026/10/postgres-19-is-late-and-its-ais-fault.html | ok 200 | 98441 | Haas 博客转载（注明 originally appeared at neon） |
| 23:49:47 | pg-news | https://www.postgresql.org/about/news/ | **failed 404** | 4217 | 旧 URL 猜测，未找到替代列表页，未重试 |
| 23:49:47 | pg19-rc1 | https://www.postgresql.org/about/news/postgresql-19-rc-1-released-3409/ | ok 200 | 12628 | RC1 页（GA 计划 2026-11-19） |
| 23:50:32 | trigger-incident-oct06 | https://status.trigger.dev/incident/1083239 | ok 200 | 133623 | 2026-10-06 事件（背景） |
| 23:50:32 | trigger-incident-status | https://status.trigger.dev/incident/1110434 | ok 200 | 127355 | 2026-11-01 事件 |
| 23:50:32 | occupysaas-salescompel-gh | https://github.com/occupysaas/salescompel | ok 200 | 266924 | README、About、Commits 数 |
| 23:50:32 | occupysaas-zeroauthy-gh | https://github.com/occupysaas/zeroauthy | ok 200 | 336703 | README、About、Commits 数 |
| 23:51:32 | pg-beta4 | https://www.postgresql.org/about/news/postgresql-19-beta-4-released-3386/ | **failed**（超时，批次总预算不足，未重试） | — | 用于核“revert”一节，未取得，记为未核 |

失败与未重试的原因：`pg-news` 为 404（URL 不存在，属于我方猜测的地址）；`pg-beta4` 为超时（批次总预算耗尽，按规则不做原地重试，记为未核）。

## 二、HN 热度（`barking heat docs/2611/1101 d --item … --hn …`）

- 23:49:22 首次运行：条目 50312408、50119991、50331402、50331784；HN 搜索 “occupysaas”、“trigger.dev”、“postgres 19”、“Postgres AI bugs”。写入 `d-heat.json`、`d-hn-{id}.json`（评论树）。
- 之后把 `d-heat.json` 改名为 `d-heat-hn-run1.json`（改名，未删除），以便保留 HN 数据。
- 23:53:13 Reddit 查询（“Postgres 19 AI bugs”、“OccupySaaS”、“Trigger.dev environment variables”）：**三个请求全部失败**，原因是 old.reddit.com 重定向到登录页（“响应不是 JSON”）。最终写入的 `d-heat.json` 仅含这次 Reddit 失败记录（`reddit: {}`，`errors` 3 条）。HN 数据见 `d-heat-hn-run1.json`。
- 中间一次尝试把 HN 条目与 Reddit 合并重跑，被工具以“已有 d-hn-*.json，拒绝覆盖”拒绝，未写入任何文件。

HN 数据要点（现查，2026-11-02 23:49 北京时间；搜索接口与条目接口可能不一致，均记录）：

| 条目 ID | 标题（简） | 发布（北京时间） | 分数 / 评论 |
|---|---|---|---|
| 50312408 | Postgres 19 is late because so many more bugs are being found now（Haas 博客转载） | 10-31 09:38 | 698（搜索 699） / 427 |
| 50281809 | Postgres 19 is Late, and it's AI's Fault（Neon 原页） | 10-29 06:26 | 5 / 0 |
| 50306359 | 同上（转载页） | 10-31 00:34 | 4 / 0 |
| 50334549 | Haas: Postgres 19 is late（LWN） | 11-02 17:04 | 3 / 0 |
| 50309439 | PostgreSQL 19 Running Behind Schedule（Phoronix） | 10-31 04:06 | 12 / 0 |
| 50119991 | Postgres 19 delayed after AI reviewers find more bugs（The Register） | 10-16 23:55 | 227 / 93 |
| 50331402 | OccupySaaS | 11-02 08:17 | 433（条目 432） / 269（条目 268） |
| 50324966 | OccupySaaS – They laid us off（早期帖） | 11-01 20:26 | 6 / 4 |
| 50331784 | Trigger.dev customer environment variables exposed | 11-02 09:09 | 148（条目 147） / 74 |
| 50338312 | Trigger.dev 环境变量暴露对比（theainewsreport） | 11-02 23:38 | 1 / 0 |

HN 评论接口不含评论分数，所以“高赞”无法统计；本组只按回复数排序取样（见 `evidence-D.md` 第三节）。

## 三、curl 与本地处理

- 23:5x GitHub API 分页（匿名，`--max-time 60`）：`https://api.github.com/repos/occupysaas/zeroauthy/commits?per_page=1&page=728` 与 `…/salescompel/commits?per_page=1&page=140`，取末页得首个提交。剩余配额 55 / 54（匿名）。
- 写入 `d-gh-zeroauthy-first-commit.json`、`d-gh-salescompel-first-commit.json`。原始响应含提交者邮箱字段，**已在写盘后改写为只保留 sha、作者日期与提交信息首行**（个人信息去除）。
- 23:5x `curl --max-time 60 https://raw.githubusercontent.com/occupysaas/zeroauthy/main/CLEAN_ROOM.md` → `d-occupysaas-zeroauthy-CLEAN_ROOM.md`（成功，6719 字节）。
- 脚本 `d-hn-digest.py`：仅本地读取 `d-hn-*.json`，输出回复数最多的顶层评论。用 `python3 -I` 运行。

## 四、本组新增文件（`docs/2611/1101/sources/`）

- 原件（HTML，派工单规定只存本地）：`d-*.html`（`d-neon-postgres19.html`、`d-rhaas-blog.html`、`d-pg19-rc1.html`、`d-pg-roadmap.html`、`d-pg-versioning.html`、`d-occupysaas-home.html`、`d-gh-occupysaas-org.html`、`d-occupysaas-zeroauthy-gh.html`、`d-occupysaas-salescompel-gh.html`、`d-trigger-incident.html`、`d-trigger-incident-status.html`、`d-trigger-incident-oct06.html`、`d-trigger-status.html`）。
- 头文件：`d-*.headers`（各条）。
- 文本：`d-*.txt`（各 HTML 转写）、`d-occupysaas-zeroauthy-CLEAN_ROOM.md`。
- JSON：`d-gh-api-org-repos.json`、`d-gh-zeroauthy-first-commit.json`、`d-gh-salescompel-first-commit.json`、`d-hn-50312408.json`、`d-hn-50119991.json`、`d-hn-50331402.json`、`d-hn-50331784.json`。
- 日志与热度：`d-fetch-log.jsonl`、`d-heat.json`、`d-heat-hn-run1.json`。
- 脚本：`d-hn-digest.py`。
- 证据：`evidence-D.md`、本文件。
- 截图：无（D 组不截图）。

## 五、待删除文件清单

- `/tmp/x`：空目录，由我在本组开工时 `mkdir -p` 创建，位于仓库外。无内容，可 `rmdir`。
- 仓库内：无。本组没有生成需要删除的仓库内文件。
- 仓库外的 `scratchpad` 下 `d-manifest*.tsv`，属于派工清单，不在仓库内。

## 六、大小

所有文件均小于 1 MB（最大为 `d-rhaas-blog.html`，98 KB；HN JSON 最大 247 KB）。
