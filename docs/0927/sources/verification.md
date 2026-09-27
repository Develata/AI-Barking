# 0927 取证验收记录

本记录仅验收交付与引文一致性，不是论文独立复现或对 D 组的最终编辑结论。

- 事实表覆盖 A1–A8、B1–B7、C1–C7、D-A/D-B/D-C，另拆 D-A2 记录未找到的“38万个 agent”说法。
- 26 条主表记录：23 条已找到、2 条部分支持（A7、B6）、1 条未找到一手来源（D-A2）。没有用“与说法不符”掩盖复合说法的部分缺口。
- 90 处 `<q>` 引文逐一匹配原始存档：先 HTML 实体解码，再折叠空白；未替换词语。比对语料仅为原站提取文本、PDF normalized 文本和 Reddit JSON，排除 evidence.md、日志、脚本自身，0 处未匹配。
- 12 张 images/ PNG，文件集合与 crop-manifest.json 完全相同；全部宽度 ≤1400。
- 将每张原始截图按 manifest 坐标重新在内存裁剪/拼接，与最终 PNG 做像素差分，12/12 无差异。多块拼接处均为 2px RGB(128,128,128) 灰线。
- 最终 12 图均目视检查：正文没有截断，关键段落可见；新浪推广弹窗已关闭后再截图；Reddit 仅保留英文原文区；截图字号测量与字形像素边界的区别见 capture-log.md。
- a-arxiv-paper.pdf：pdfinfo 成功解析，31 页、786319 bytes。
- b-technical-report.pdf：续传后 pdfinfo 成功解析，40 页、21011275 bytes；pdftotext -enc UTF-8 -layout 完整提取。
- PDF 文本实际分页定位：搜索 campaign 数字 p3；E. coli 小 RNA 实验 p8；功能/活性限制 p13；Campaign accounting 与 token 分项 p30。未执行生成草案内的其他页码不得当作已验收信息。
- SHA-256：a-arxiv-paper.pdf = `649edc87e70b8e57f9515360b15ed5576c6c8afb3c46a3763e83c22a85a660e2`。
- SHA-256：b-technical-report.pdf = `d43c2fa66aa1d255aba37a0499390bcd86e5b2cafa1f8d249d728e998a4b3b27`。

末次检查 HEAD：`ded0f347a04e5158bdf984be36ee71bc90323334`；`git diff --stat` 无输出。

`git status --short`：

```text
?? .handoff/2026-09-27-0927b-evidence.md
?? docs/0927/
```

handoff 在开始任务时已存在，本轮未读写。本代理新增 76 个文件，仅位于 docs/0927/sources/（64 个）和 docs/0927/images/（12 个）。未 commit、push、删除、安装、改动基线文件，未生成发布正文、images/README.md 或 ZIP。

最终更细的目录检查发现并发新增 `docs/0927/doc_0927_publish.txt`（2173 bytes；主机显示修改时间 2026-09-27 13:10，对应北京时间约 2026-09-28 00:10）。本代理没有创建、读取或修改它；未确认具体创建进程。开始时 docs/0927 不存在，因此不能声称最终整个目录只有本代理文件。该文件不计入本轮新增清单，也未因超出本代理范围而删除。

未完成或未验证：A7 的成功逃逸陈述；B6 自身 DOI/平台编号/明确论文发表日期；D-A2 预设标题；穷尽中文社交平台搜索；科学结果的独立复现。预设标题未找到不等于全网不存在。
