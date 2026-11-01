# C 组 capture-log（北京时间，UTC+8）

机器日志：`sources/c-fetch-log.jsonl`（`barking fetch`，含 `time_bj`、字节数、SHA-256）、`sources/c-shots-log.jsonl`（`barking shot`，含 `time_bj`）、`sources/c-heat.json`。下表只记批次、工具与例外。本机时区 EST，北京时间 = 本机 +13 小时；开始 02:31，交回前约 03:00。

## 批量存档（barking fetch）

| 批次 | 内容 | 开始时刻 | 结果 |
|---|---|---|---|
| c1 | 仓库页；raw：README、LICENSE、SETUP.md、docs/000、002、006；GitHub API：repo、commits、contributors；deepseek.com/en/harness、/zh/harness | 02:33:14 | 12 ok（contributors 返回 `[]`；仓库页 html 含静态渲染的 README） |
| c2 | The Next Web；MarkTechPost；LINUX DO；DEV（paullovelace） | 02:34:35 | tnw、devto ok；**marktechpost、linuxdo blocked**（Cloudflare 403 质询，未重试，未用镜像） |
| c3 | 新浪财经 10-31 稿 | 02:34:53 | ok |
| c4 | SCMP；Bloomberg | 02:35 | **scmp blocked**（Cloudflare 403）；bloomberg ok（200，页面只含标题与前两段，正文其余未取） |
| c5 | 新浪新闻自媒体稿（k.sina.cn）；网易转载智东西稿；新浪科技转载智东西稿 | 02:35:36 | 3 ok |
| c6 | docs/003、005、007（raw）；DeepSeek Harness 主仓库 README（先 main 分支 404，改 master） | 02:41 | doc003/005/007 ok；dsh-readme 404（`main`）→ 重取 master ok |
| c7 | 微信公众号原文（Bloomberg 页面里的链接） | 02:46:44 | ok（200，3.7 MB html，只存本地；`.txt` 约 40 KB） |

## 自写/一次性请求

- GitHub 仓库 `git clone`（会话临时目录，不入库），取 `git log`；提交、包元数据、`src/policy.ts` 存档为 `c-git-log.txt`、`c-package.json`、`c-src-policy.ts.txt`。
- `curl --max-time 60`：`api.github.com/repos/.../dsh-superagent-prototype`（取样 02:31、02:33、02:43）；`api.github.com/repos/deepseek-ai/deepseek-harness`（只看默认分支名）；fxtwitter JSON：官方 X 长文帖 2116857257245987236（`c-article-x-status.json`）、Pandaily 2116713246535086563（`c-fx-pandaily.json`）、AGTP 2116631973519372495（`c-fx-agtp.json`）。fxtwitter JSON 是公开接口返回，不含抓取者信息。
- `barking heat`：HN 条目 50316116、50321277、50326523（`c-hn-*.json`）与查询 “dsh-superagent”“deepseek superagent”“Exploring Super-agent Design”（02:33:52）。
- `sources/c-xcard.ts`：X 官方嵌入页的匿名 headless Chrome 卡片截图脚本（全新临时 user-data-dir，不读本机 Cookie；`/usr/bin/google-chrome --headless=new --no-sandbox`）。
- WebSearch（发现 URL 与转载；发现之后回到一手页面核）：约 7 次。未用 Exa、firecrawl、opencli。

## 例外与失败（如实）

- **GitHub stargazers 时间序列**：`/stargazers?per_page=100&page=N`（带 `star+json`）无凭据返回 401 “Requires authentication”，9 个采样页全部失败（记录在 `c-stargazers-samples.jsonl`，内容只是错误），没有使用任何 token 或登录态；改用 `stargazers_count` 的多次采样。
- **Reddit**：`barking heat --reddit` 被重定向到 old.reddit.com 登录页，未取到（未登录）。
- **HN 评论**：三个主帖均 0 条评论，无可摘。
- **MarkTechPost、LINUX DO、SCMP**：Cloudflare 质询，记为失败，没换镜像或代理。
- **官方 X 长文文章页**（`x.com/i/article/2116829218935857152`）：`barking shot render` headless Chrome 匿名渲染失败（Chrome 无输出）；文章全文用 fxtwitter JSON（`c-article.txt` 为转存文本）。
- **微信公众号原文页**：curl 取到正文（`c-wechat-original.txt`），但 `barking shot render` 在 headless Chrome 里被“环境异常，完成验证后即可继续访问”拦下，没绕过，无截图。
- **The Next Web 渲染**：页面被 Cookie 隐私提示遮罩（整页变暗），不点接受，未截图。
- **Bloomberg**：只取到标题与前两段，未绕过付费墙。
- **commits 页截图**：时间与头像处是 GitHub 的加载占位条（页面未等到异步数据），图中时间处为空，时间以 API 为准。
- **仓库页截图**：使用 headless Chrome 匿名（未登录）渲染，GitHub 移动窄版式（700 CSS px），页头为登出态导航条（已裁掉）。
- 未使用 opencli（扩展未连接）、未登录、未接受 Cookie 提示、未发帖/点赞/评论/关注。

## 截图清单（`docs/2610/1031/images/`，C 组区间 25–36，均 2 倍像素比，已逐张打开目视确认文字清楚）

| 文件 | 来源 | 说明 |
|---|---|---|
| 25-readme-disclaimer.png | https://github.com/deepseek-ai/dsh-superagent-prototype，`barking shot render` 700 CSS px DPR 2（02:36），cropbox 0,2170–1400,3690 | README 面板：标题、免责段、“one GitHub repository”、“acts only when asked”；浅色底 |
| 26-repo-header-stars.png | 同页，cropbox 0,100–1400,1000 | 顶部有 1–2 px 黑条（导航条边），已被 35 取代；**待删除** |
| 27-authz-accept-mode.png | …/blob/main/docs/002-authorization.md，cropbox 0,8400–1400,9700 | “Accept mode and rules” 整节 |
| 28-deploy-danger-full-access.png | …/blob/main/docs/006-deployment.md，cropbox 0,1950–1400,2710 | “Permissions and isolation” 整节 |
| 29-setup-defaults-observe.png | …/blob/main/SETUP.md，cropbox 0,4965–1400,5560 | 默认值清单（observe、每小时检查、飞书只读 @） |
| 30-x-deepseekharness-article-post.png | X 官方嵌入页，`c-xcard.ts`，2116857257245987236 | 官方账号帖子卡片（只有文章链接）；含官方账号头像与 handle（官方账号，非抓取者）；嵌入页显示美东 6:37 AM Nov 1, 2026 |
| 31-sina-niuma-headline-7x24.png | https://k.sina.cn/article_5953740931_162dee083067043yzs.html，render 700 CSS px，cropbox 0,295–1400,1665（02:4x） | 标题、“11.01 09:35”、“7x24 小时在线”段；站内广告条与顶部栏已裁掉 |
| 32-sina-niuma-auto-mode.png | 同页（`--height 12000` 的整页渲染），cropbox 0,22740–1400,23770 | “得切换到 auto 模式，再自己写规则”段 |
| 33-commit-history-single.png | …/commits/main，render 700 CSS px，cropbox 0,110–1400,800 | 只有 1 个提交（时间处为加载占位条） |
| 34-x-pandaily-post.png | X 官方嵌入页，`c-xcard.ts`，2116713246535086563；cropbox 0,0–1096,540 | Pandaily 帖卡片（媒体官方账号） |
| 35-repo-header-clean.png | 仓库页，cropbox 0,112–1400,1000 | About、MIT license、6.7k stars、36 watching、667 forks（02:36） |
| 36-philosophy-acts-only-when-asked.png | …/blob/main/docs/000-philosophy.md，cropbox 0,1400–1400,2340 | “What follows for the agent” 整节 |

## 抓取者信息自查

对 `docs/2610/1031/` 里本组文件 grep 本机用户名与本机仓库路径：证据文件之外无命中。X 截图只含官方/媒体账号卡片（DeepSeek Harness、Pandaily），页面均为未登录态。
