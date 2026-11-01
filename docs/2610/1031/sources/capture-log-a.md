# 1031 期 A 组抓取日志

时间为北京时间（UTC+8），2026-11-02（本机美东 EST 2026-11-01 13:32 至 13:50，换算 +13 小时）。工具缩写：barking = `tools/barking/target/release/barking`；CDP = 本组自写 `a-cdp-shot.mjs`（改自 1030 期 `b-cdp-shot.mjs`：有界面 /usr/bin/google-chrome，全新临时 profile，不读用户 Cookie，不登录，不点验证控件，`--lang=en-US`；输出渲染文本、`a`-文章 JSON、链接表、元素框 JSON、1x CSS 宽度 2x 像素的 PNG）。

## 抓取与结果

| 时间 | 对象 URL | 工具 | 结果 |
|---|---|---|---|
| 02:32 | help.openai.com/en/articles/20001625-understanding-chatgpt-and-codex-usage-limits | barking shot render（headless） | 失败：渲染图只有加载图标（与 1030 期一致），改用 CDP |
| 02:33 | 同上 | CDP | 成功。`updatedAt`=1793463468.058（2026-11-01 00:17:48 北京）；文本 `a-help-usage-rendered.txt`、`a-help-usage-article.json`、`a-help-usage-boxes.json`。第一次运行因脚本默认要求正文 >2000 字符而误判失败（页面本身只有约 1,300 字符），改 `MINLEN=300` 重跑 |
| 02:34 | 同上（再抓一次取链接表） | CDP | 成功，链接表得到 12003714、11369540、20001339 等相邻文章 ID（文件在 scratchpad，不入库） |
| 02:34 | help 9793128 Pro 页 | CDP | 第一次并发启动 3 个 Chrome，该条因端口未就绪失败；串行重跑成功。`a-rendered-pro-tiers.txt`、`…-article.json` |
| 02:34 | help 6825453 发行说明 | CDP | 成功，`a-rendered-release-notes.txt`（228 KB）、article json（581 KB） |
| 02:35 | help 11487671 | CDP | URL 猜错，得到 “Flexible pricing for the Enterprise, Edu, and Business plans” 页；只用于确认 credits 的 Business/Enterprise 规则，未存入库 |
| 02:35 | help 12003714 Business models and limits、11369540 Using Codex、chatgpt.com/pricing | CDP | 成功。`a-rendered-business-models.txt`、`a-rendered-codex-plan.txt`、`a-rendered-chatgpt-pricing.txt`（价格数字未渲染出来）。pricing 页匿名，重定向后文字为英文 |
| 02:36 | help 20001354 GPT-6 models、20001275 Work and Codex、12642688 credits、11481834 rate card | CDP | 成功。`a-rendered-gpt6-models.txt`、`a-rendered-work-and-codex.txt`、`a-rendered-credits-personal.txt`；rate card 页取了但未入库 |
| 02:35 | HN 50320588、50324512、50326057；两组 HN 搜索 | barking heat | 成功，`a-heat.json`、`a-hn-*.json`；Reddit 搜索返回登录页，失败（已记入 heat.json errors） |
| 02:36 | WebSearch 与 Bing News RSS（curl） | WebSearch、curl | 用于找报道线索，未入库 |
| 02:36–02:38 | 报道与 Codex 页 12 条 | barking fetch（`a-fetch-log.jsonl`） | ok：news-bleeping、news-cryptobriefing、news-note、news-note2、news-kucoin、codex-pricing、codex-changelog、zh-163、zh-sohu、zh-qq-reset、zh-sina-tibo、zh-qq-ithome。blocked：news-msn-likely（正文过短，需 JS；不绕过） |
| 02:37 | https://api.fxtwitter.com/status/2115588390414581916、2115927832069292214、2116609772396920937、2116625475015528759、2115957220747923952、2116365292284641662、2115934686371590410 | curl | 成功，`a-fx-<id>.json/.headers`。匿名公开 JSON，不含抓取者信息；`a-fx-2115588390414581916.json` 里的引用帖作者 Angaisb_ 是公开发言 |
| 02:38 | web.archive.org CDX（help 20001625） | curl | 失败：Internet Archive “Temporarily Offline”，返回的是错误 HTML（存成了 `a-wayback-cdx-usage.json`，内容无用）。同一来源只试一次，记失败 |
| 02:39 | platform.twitter.com/embed/Tweet.html?id=… ×3 | barking shot render（headless） | 失败：空白图。改用 CDP（无需登录，匿名） |
| 02:40 | 同上 ×3 | CDP | 成功。截图 09、10、11；`a-fx-embed-tibo.txt` 为嵌入页文本 |
| 02:42 | developers.openai.com/codex/pricing | CDP | 成功（`a-rendered-codex-pricing-rendered.txt`），截图 08 |
| 02:44 | news.qq.com/rain/a/20261101A05QJV00 | CDP | 成功（`a-rendered-ifanr-qq-rendered.txt`），截图 12；第一次限高 2400 只够截到文首，重跑限高 9000 才能截到文末句 |

## 截图（`images/`，2 倍像素比，宽度均 ≤1400 px）

| 文件 | 内容 | 来源页 | 备注 |
|---|---|---|---|
| 01-help-usage-full.png | 帮助页全文（标题至第三段） | help 20001625 | 页头 “Updated: yesterday” 在内 |
| 02-help-usage-start-december.png | 页头 + “Starting in December…”段 | 同上 | stack：页头 + 段落 |
| 03-help-usage-tokens-instant.png | 页头 + token 计量与 Instant 段 | 同上 | |
| 04-help-usage-limit-options.png | 页头 + “用完后”段（wait / upgrade / credits、Settings → Usage） | 同上 | |
| 05-business-instant-note.png | Business 页 “Note on unlimited access” | help 12003714 | |
| 06-models-instant-limits.png | models 页 “Instant limits” | help 20001354 | Free/Go 句；Plus/Pro 没有同句 |
| 07-credits-how-they-work.png | personal credits 页 “How credits work” | help 12642688 | 页面右侧有客服浮窗按钮，是页面自带 UI |
| 08-codex-pricing-share-usage.png | “ChatGPT Work and Codex share usage” 提示框 | developers.openai.com/codex/pricing | |
| 09-tibo-not-rolling-out.png | Tibo 帖 | X 官方嵌入渲染 | 引用帖里的配图占位（灰色块）在 stack 时省去，卡片边框在拼接处略有断开；未改像素 |
| 10-btibor-checkout-disclaimer.png | Tibor Blaho 帖 | 同上 | |
| 11-luojiuyue-exaggeration.png | luojiuyue 帖（含被引用帖） | 同上 | 原帖有 “Show more”折叠，截图保持页面原样 |
| 12-ifanr-qq-headline.png | 爱范儿稿（腾讯新闻转载）标题与首段 + 文末句 | news.qq.com | stack：标题至“Instant 聊天除外”段 + 文末“无论如何……”两段；页面左侧分享按钮在拼接处被截断 |

X 帖截图都是匿名嵌入卡片，不含抓取者头像、昵称、handle；已逐张目视检查。
全部 12 张现行图片已逐张打开目视确认文字清楚、范围正确（07 页面右侧的客服浮窗按钮为页面自带 UI）。

## 偏离与处理

- 第一版 05、06、07：页头截得太短（副标题与 “Updated” 行被截断），用 `mv` 移出仓库（位置：scratchpad `<scratchpad>/wrong/`，仓库外），重做为现行 05–07。`a-shots-log.jsonl` 里保留了这三张的旧记录，原样留着，文件名与现行图相同，以最后三条（同 URL 的 stack 记录）为准。
- 没有执行 `rm`。
- 自己生成但无用的文件（待 Claude 收尾删除）见回报中的清单。
- `a-codex-changelog.txt` 为 988,031 字节（接近 1 MB 上限，未超）；HTML 原件 `a-codex-changelog.html` 4.6 MB 仅存本地。
- 交付前对 `docs/2610/1031/` 做了一次 grep（develata、/home/、邮箱），本组文件无匹配（其他组文件有，非本组范围）。
