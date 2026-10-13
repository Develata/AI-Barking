# 1012 期更正与补充

发布后完成的跨模型反向核验（`sources/fact-check.md`“反向核验”一节）指出以下问题。发布稿不删不改，以评论区补充为准。

## 评论区补充（建议置顶）

补充两处：
1. 电视盒那条说得不够准：Anthropic 的复制实验里，“蠕虫”的大模型 GLM-5.2 跑在外部服务器上，它在8台模拟安卓节点上部署的是另一个8B 小模型（Llama3.1-8B），下一份蠕虫由实验框架启动，并不是模型把自己的权重复制过去。原文说的是“假如 GLM-5.2 的能力能装进8B 模型”才能自我复制。
2. Codex 那条：codex features disable daemon_auto_start 只是让以后不再自动启动，已经在跑的后台进程要另用 codex app-server daemon stop 停掉（有用户反映更新器可能还要手动结束）。这一行为只针对“符合条件的交互会话”，带 --no-daemon、--profile 等参数启动的不会拉起。

## 其他记录（未在评论区展开）

- Anthropic 页面标注 Oct 12，元数据发布时间为北京时间 10月13日 03:30。
- “真实设备未必更难攻破”原文为“可能没有难很多”（might not be dramatically more difficult）。
- “暂无 OpenAI 成员回复”应为“截至采集时，issue 内未见可识别的官方回复”。
- 速览：AA 的 Muse Wonder 指数52与2.8倍成本为最高推理档、智能指数测试任务口径；AA-3D 中 Sonnet-5.5 为最高档含回退；xAI 合并订阅为新付费套餐，第三方模型限 Premium 及以上。
