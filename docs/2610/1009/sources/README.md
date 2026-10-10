# 1009 来源入口

事实核验见 [fact-check.md](fact-check.md)；取证清单见 `evidence-a.md`（Anthropic 越界报告）、`evidence-b.md`（Google 拟购 Spirit 数据）、`evidence-c.md`（紫外线全天图）、`evidence-d.md`（速览）；抓取日志见 `capture-log-a.md` … `capture-log-d.md`。派工单 `../../../../.handoff/2026-10-09-1009-evidence.md`。PDF 与 HTML 原件不入库，存 OpenList（大小与 SHA-256 记入 `offsite.tsv`）。

## 主帖一：Anthropic 越界报告

- 报告：https://www.anthropic.com/research/investigating-unintended-model-actions （`a-anthropic-report.txt`；published_time 2026-10-09T16:09Z，modified 22:04Z）
- 费城警方声明：原件未在 phillypolice.com / phila.gov 找到；6abc 全文转刊（`a-6abc.txt`、`a-6abc-browser.txt`）；CBS https://www.cbsnews.com/news/philadelphia-police-anthropic-ai-false-homicide-tip/ （`a-cbs.txt`）
- 背景（未入正文）：Anthropic 7/30（`a-jul30.txt`）、8/31（`a-aug31.txt`）、9/9（`a-sep9.txt`）三篇
- 媒体（L4，未入正文）：Reuters（`a-reuters-tip.txt`）、TechCrunch（`a-tc.txt`）、NBC10、Fox29、AFP、NYPost；《华盛顿邮报》付费墙（`a-wapo-paywall.txt`）；《纽约时报》经《西雅图时报》授权转载（`a-nyt-seattletimes.txt`，“20 份签证申请”的出处，匿名信源，不用）
- 夸大实例：Startup Fortune（`a-startupfortune.txt`，把 OpenAI 9 月事件写进 Anthropic 报告）
- 热度：HN 50027118（89–91 分 / 77 评，北京 10-10 11:46）

## 主帖二：Google 拟购 Spirit 内部数据

- Reuters 10/8：https://www.reuters.com/world/lawmakers-raise-alarm-google-plan-acquire-spirit-airlines-data-ai-models-2026-10-08/ （`b-reuters.txt`；published 2026-10-08T19:49:26Z，modified 22:13:51Z）
- 议员联名信（2026-10-08，121 人署名）：Horsford 官网 PDF（`b-letter.txt`）；Horsford 新闻稿（`b-horsford-pr.txt`）；AFA-CWA 同文（`b-afacwa.txt`）
- 破产案卷（Spirit，案号 25-11897-shl）：Dkt 1463 拍卖结果通知与资产清单（`b-dkt1463-auction-results.txt`，PDF 第 18 页）；Dkt 1489 空乘工会异议（`b-dkt1489-afa-objection.txt`）；Dkt 1581 隐私监察员 9/8 报告（`b-dkt1581-cpo-report-sep8.txt`）；Dkt 1594 Google 初步回应（`b-dkt1594-google-prelim-response.txt`）；Dkt 1684 监察员 10/5 补充报告（`b-dkt-ombudsman-oct5.txt`）；Dkt 1677（`b-epiq-1677-epic-amicus.txt`）
- 早期报道（8 月）：Bloomberg Law（`b-bloomberglaw-0817.txt`）、CNN（`b-cnn-0818.txt`）
- 夸大实例：CNN 标题、24/7 Wall St、Cybernews、赢政天下、PYMNTS（见 `evidence-b.md` 第四节）

## 主帖三：紫外线全天图

- Anthropic 文章：https://www.anthropic.com/research/the-missing-map-of-the-sky （`c-anthropic-map.txt`；published 2026-10-08T20:59Z）
- 作者技术页：https://menard.pha.jhu.edu/uvmap/ （`c-menard-uvmap.txt`；手稿文本 `c-uvmap-manuscript.md`；PDF 原件 403 未取）
- JHU 系网站短讯：https://physics-astronomy.jhu.edu/2026/10/09/brice-menard-works-with-anthropics-claude-science-to-produce-first-complete-uv-light-map-of-the-sky/ （`c-jhu-news.txt`）
- 此前全天 UV 图：MAST UV-BKGD（`c-mast-uvbkgd.txt`）、FIMS/SPEAR（`c-mast-fims.txt`）
- 夸大实例：NewsBytes、Lifeboat、The Decoder、新智元（`c-*.txt`，见 `evidence-c.md`）

## 速览

- Epoch InnovationEval：https://epoch.ai/publications/innovationeval （`d-d1-epoch.txt`）
- Codex Composer predictions：https://help.openai.com/en/articles/20001601-composer-predictions-in-codex （`m-codex-composer.md`）；release notes https://help.openai.com/en/articles/6825453-chatgpt-release-notes （10 月 9 日条，未单独存档）
- Microsoft-Decision-1：https://commandline.microsoft.com/microsoft-decision-1-model-foundry/ （`d-d2-msdecision.txt`；HTML 原件因 Cloudflare 未存）
- Hales 客座文章：https://terrytao.wordpress.com/2026/10/09/what-mathematicians-should-know-about-the-lean-theorem-proverquestions-of-reliability-and-ai/ （`d-d3-tao.txt`）
- Harvey LAB-AA v1.1：https://artificialanalysis.ai/evaluations/harvey-lab-aa （`d-d4-harvey.txt`）
- Arena Alignment Index：https://arena.ai/blog/ai-alignment-index （`d-d5-arena.txt`）
- Jesse Waites 档案检索：https://jessewaites.com/blog/post/i-pointed-ai-at-400-years-of-archives/ （`d-d6-waites.txt`）
- 未收：Deno 加入 Cloudflare（`d-d7-deno.txt`、`d-d7-cfblog.txt`）
- 热度：`d-heat.json`

## 脚本

- `m-raw-crops.py`：从取证截图裁切/拼接批注底图（Claude）
- 各组抓取与截图脚本：`a-*.py`/`a-browser.mjs`、`b-*`、`c-browser.mjs`、`d-html2txt.py`、`d-heat.py`
