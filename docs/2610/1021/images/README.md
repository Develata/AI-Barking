# 1021 配图说明

标题：claude也出BOT；安全问题扎堆

封面二选一：`00-cover.png` 用于小红书、抖音（3:4）；`00-cover-wide.png` 用于微信公众号（约2.35:1）。同一平台只上传一张封面，之后按下表顺序上传，共9张。正文独立可读，不含插图占位。

## 正式配图（封面之后）

| 顺序 | 文件 | 对应正文 | 图注与来源 |
|---|---|---|---|
| 1 | 00-tldr.png | 省流 | 三条已核官方事实，文字摘自正文 |
| 2 | 00-roundup.png | 今日速览 | 三条主帖标题＋三条补充速览，来源见fact-check |
| 3 | 02-duet-controls.png | 第一条 | Anthropic：每隔数小时检查，部分行为即便获批仍禁止；同页非连续截图拼接，中间省略 |
| 4 | 17-litellm-intro.png | 第二条 | LiteLLM：1.107.0发行物含窃密恶意代码 |
| 5 | 14-php.png | 第二条 | PHP官方强烈建议立即升级 |
| 6 | 15-node.png | 第二条 | Node.js三条版本线提供安全更新 |
| 7 | 16-rust.png | 第二条 | Rustup1.29.2仅更新两项证书依赖 |
| 8 | 03-ads.png | 第三条 | OpenAI：广告选择与回答生成在不同服务中运行；同页非连续截图拼接，中间省略 |

## 备用图

| 文件 | 用途 |
|---|---|
| 02-duet-controls-raw.png | 批注底图 |
| 17-litellm-intro-raw.png | 批注底图 |
| 14-php-raw.png | 批注底图 |
| 15-node-raw.png | 批注底图 |
| 16-rust-raw.png | 批注底图 |
| 03-ads-raw.png | 批注底图 |
| 01-duet-raw.png | 候补资格和套餐原文，备用取证 |
| 13-litellm-raw.png | 受影响发行物长图，备用取证 |

## 制作与核验

批注由仓库cards.toml及barking模板渲染。荧光标记和中文译注为本号后加；不改原文像素。原始来源、北京时间、限制条件见sources/fact-check.md；裁切拼接可追溯至sources/a-shots-log.jsonl与b-shots-log.jsonl。网页截图采集日期为2026-10-22。

封面为imagegen生成的纸艺办公室示意图，使用仓库通用看板娘参考角色，不是新闻现场。竖版生成2候选，选第1（布局更简洁）；横版生成4候选，选第4（标题与全身角色在中间方形裁切区内）。主题为主动工作与审批，以打卡机、钥匙锁辅助表达，不暗示特定平台已经失窃。标题逐字核查：AI主动上班，谁来把关？双封面保持两只正常手、完整双脚、狗耳、蓝发和尾巴。

完整生成提示词见sources/cover-prompts.md。独立视觉复核结果以sources/fact-check.md为准，机械渲染完成不代替人工或跨模型看图。

最终尺寸：竖封面1086×1448，横封面1919×820；信息卡均1080×1440。横封面中心820×820裁切已实际查看，标题和全身角色保留。最终独立视觉复核通过，未模拟平台二次压缩。

用户随后要求主代理自审：省流卡补齐四项安全事件，广告图改高亮独立服务分工，两图已重渲并由主代理实际看图；其他6张信息卡也重新查看。前述独立视觉复核是更改前版本记录，最新自审及边界见sources/self-review.md。

文风修订：第二主帖小标题改为“安全问题扎堆，处置要分清”，速览图同一句同步重渲并实际开图检查。封面、省流和批注文字本轮不变。当前正文850字，定点复核见sources/style-review-astra.md。


## 最终发布封面：用户指定标题修订

用户已确认发布，最终标题为“claude也出BOT；安全问题扎堆”，正文856字。上文旧标题、旧尺寸与独立复核属于先前版本的制作历史。

本次竖版、横版各生成2个候选，竖版选第1个（构图简洁），横版选第2个（中心方形裁切余量更足）。最终竖版1086×1448，横版1923×818；主代理已实际查看两图及横版中心818×818裁切，核对标题、分号、人物双手双脚和裁切范围。没有对这次新封面另做独立模型复核，也未模拟平台二次压缩。旧封面在sources中以pre-title-cover前缀保留。

机器人仅象征主动助手，独立的锁和警示图案对应安全聚合，不表示Claude遭到攻击。封面是AI生成的编辑示意图，不是新闻现场。

### 最终竖版完整提示词

```text
Edit the provided editorial cover, replacing the old headline completely with the user's new EXACT headline. Keep the cream paper-craft aesthetic, navy/muted sage and orange palette, and familiar full-body blue-haired dog-eared maid mascot with whale tail and circular AI吠点 badge, TWO normal bare human hands and two shoes. Reference 1 is the existing cover to edit; reference 2 is the mascot identity. New exact headline must be in two lines: line 1 'claude也出BOT；' (lowercase claude, uppercase BOT, Chinese semicolon); line 2 '安全问题扎堆'. No question mark, no old headline, no extra subtitle. At the lower left retain one friendly paper robot representing a proactive assistant; at lower right a separate padlock with a small orange warning triangle represents the distinct software security stories. Keep large clean negative space, crisp Chinese typography. This is an editorial illustration, don't depict the Claude assistant itself infected or malicious. No logos beyond mascot badge, no additional text. Portrait 3:4 cover, target 1080x1440. Headline largest in top 35%, two lines, ample safe margins, no clipping. Mascot full-body lower right, robot lower left, one small padlock warning between, simple composition.
```

### 最终横版完整提示词

```text
Edit the provided editorial cover, replacing the old headline completely with the user's new EXACT headline. Keep the cream paper-craft aesthetic, navy/muted sage and orange palette, and familiar full-body blue-haired dog-eared maid mascot with whale tail and circular AI吠点 badge, TWO normal bare human hands and two shoes. Reference 1 is the existing cover to edit; reference 2 is the mascot identity. New exact headline must be in two lines: line 1 'claude也出BOT；' (lowercase claude, uppercase BOT, Chinese semicolon); line 2 '安全问题扎堆'. No question mark, no old headline, no extra subtitle. At the lower left retain one friendly paper robot representing a proactive assistant; at lower right a separate padlock with a small orange warning triangle represents the distinct software security stories. Keep large clean negative space, crisp Chinese typography. This is an editorial illustration, don't depict the Claude assistant itself infected or malicious. No logos beyond mascot badge, no additional text. Native landscape cover 2.35:1, target 1920x816. CRITICAL square-thumbnail safety: headline and FULL mascot both within middle 38% of entire canvas. Headline SMALLER than portrait, only 30% of full canvas width (x~670..1250), two lines at top center, thick legible letters with lots of blank space on both sides. Whole mascot centered below headline inside x800..1150. Robot far left and warning padlock far right outside central safe area. Keep central 816x816 crop clean with 80px margins around headline. Recompose natively for landscape, do not crop portrait. Alternative candidate: make headline line 1 visibly 15 percent narrower than the existing reference title, small enough to leave at least 75 pixels of margin on each side inside the center square. Headline can be medium size; never stretch across canvas.
```

