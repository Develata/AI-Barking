# 1023 速览草稿（内部，未定稿）

依据 `evidence-d.md`。字数上限 40（EDITORIAL.md“速览”），定稿前用 `barking lint` 复核。

| # | 条目草稿 | 级别 | 依据与限定 |
|---|---|---|---|
| D1 | Anthropic 称，断开应用只停新访问，Claude Duet 已学到的保留至重置记忆 | L1 | Duet 安全说明 10/23：“Disconnecting an app stops new access from that moment, but what Claude Duet already learned from it stays until you reset its memory”；研究预览；1021 已发产品，本条为新安全说明 |
| D2 | 谷歌称，合作方 Nexus AI 的结核筛查工具已辅助评估逾2.5万张胸片 | L1 | Google Research 10/23：约40家机构、逾90名临床人员；胸片数不是确诊数；工具归 Nexus AI；原文“six countries”实为三国在用、三国扩展中，不写国数 |
| D3 | Brotli 解码漏洞获 CVSS 4.0 评分9.2，OpenAI 称 curl、Python 绑定等也要查 | L1 | CVE 记录（Google CNA）9.2 CRITICAL；patchtheplanet（OpenAI）“Other software to check”列 Electron、curl/libcurl、Python/Java 绑定、字体解码；“要查”≠“已确认受影响”；“二月源码已修”未找到一手，不用；Istio 3.1 7.7 归属不清，不用 |
| 备用 | Railway 称，最多1.85%工作区（约10万个）部分环境变量可能泄露 | L1 | 初步报告 10/22：“potentially compromised”；约10万为 Railway 换算 |
