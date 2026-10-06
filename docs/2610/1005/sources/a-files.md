# 1005 A组新增文件清单

共70个本组文件：sources 63个、images 7个。下列所有路径相对于`docs/2610/1005/`。JSON/TXT成对表示分别新增两份文件；不表示只存摘要。没有ZIP、发布正文、公共README或其他组文件。

## 交付说明（3）

- sources/evidence-a.md
- sources/capture-log-a.md
- sources/a-files.md

## 原站正文与元数据（40，每行两份）

| JSON | TXT |
|---|---|
| sources/a-wikimedia.json | sources/a-wikimedia.txt |
| sources/a-wdqs.json | sources/a-wdqs.txt |
| sources/a-collusion.json | sources/a-collusion.txt |
| sources/a-transluce.json | sources/a-transluce.txt |
| sources/a-rubyhack.json | sources/a-rubyhack.txt |
| sources/a-metr.json | sources/a-metr.txt |
| sources/a-cryptobriefing.json | sources/a-cryptobriefing.txt |
| sources/a-hn.json | sources/a-hn.txt |
| sources/a-aihot.json | sources/a-aihot.txt |
| sources/a-aihot-oct6.json | sources/a-aihot-oct6.txt |
| sources/a-aihot-item.json | sources/a-aihot-item.txt |
| sources/a-openai-timeline.json | sources/a-openai-timeline.txt |
| sources/a-openai-report.json | sources/a-openai-report.txt |
| sources/a-openai-timeline-en.json | sources/a-openai-timeline-en.txt |
| sources/a-openai-report-en.json | sources/a-openai-report-en.txt |
| sources/a-openai-alignment.json | sources/a-openai-alignment.txt |
| sources/a-verge.json | sources/a-verge.txt |
| sources/a-reuters.json | sources/a-reuters.txt |
| sources/a-icwork.json | sources/a-icwork.txt |
| sources/a-pollar.json | sources/a-pollar.txt |

## CSV、元数据及社区摘录（13）

- sources/a-wikimedia-edits.csv
- sources/a-csv-statistics.json
- sources/a-revisions-en.wikipedia.org.json
- sources/a-revisions-test.wikipedia.org.json
- sources/a-revisions-test2.wikipedia.org.json
- sources/a-revisions-www.mediawiki.org.json
- sources/a-revisions-commons.wikimedia.org.json
- sources/a-revisions-simple.wikipedia.org.json
- sources/a-revisions-incubator.wikimedia.org.json
- sources/a-revisions-meta.wikimedia.org.json
- sources/a-revisions-bg.wikipedia.org.json
- sources/a-hn-selected.json
- sources/a-reddit-selected.json

## 采集日志与脚本（7）

- sources/a-fetch-log.jsonl：每次原站/公共API抓取URL、时间、工具与结果。
- sources/a-shots-log.jsonl：实际截图区域、DPR、时刻；失败图不能因该机械日志标capture就当验收通过。
- sources/a-archive.mjs：已执行的公开页面存档脚本。
- sources/a-csv-audit.mjs：已执行的54修订元数据与统计脚本。
- sources/a-shots.mjs：初始截图尝试，部分失败；见capture-log-a.md。
- sources/a-capture.mjs：单次CDP截图脚本，最终用于03/04/05/07；参数坐标先由只读browser eval实测。
- sources/a-build-report.py：**自动审批拒绝，未执行**；不作为交付事实或验证回执。报告由直接文件编辑完成。

## 图片（7）

| 文件 | 尺寸 | 验收 |
|---|---|---|
| images/01-a-wikimedia-opening.png | 1400×1384 | 可用：标题、署名、日期、前两段 |
| images/02-a-wikimedia-no-compromise.png | 1400×1656 | 不可发布：裁切/缩放异常，按不删除要求保留 |
| images/03-a-wikimedia-summary.png | 1400×2540 | 可用：未发现攻破/协调段与三条完整列表 |
| images/04-a-wdqs-summary.png | 1400×1300 | 可用：完整Summary表、爬虫开头 |
| images/05-a-wdqs-timeline.png | 1400×2580 | 可用：完整时间线首尾 |
| images/06-a-wikimedia-context.png | 1400×2400 | 不可发布：裁切/缩放异常，按不删除要求保留 |
| images/07-a-wdqs-scrapers.png | 1400×2300 | 可用：爬虫、图表图例、1/128采样及漏抓段 |

实际验证：32份JSON可解析；54条CSV元数据全部解析；7图PNG头均宽1400；有效5图已逐张目视。没有任何文件超过1MB。CSV截图缺失；02/06不得当正式图上传。未commit、push、删除。
