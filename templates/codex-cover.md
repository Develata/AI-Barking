# 封面生成模板（默认 Codex，支持其他参考图生图工具）

规则见 `EDITORIAL.md`“封面”与 `docs/workflow/harness.md` 第 5 节。流程：抽签 → 按 A、B 两组各写提示词 → 各出一组竖版与横版候选（默认跑 Codex CLI，也可用当前 agent 自带的支持参考图生图工具）→ 挑一组 → 记录。每种尺寸累计最多 5 张。

## 1. 抽签

```bash
cargo run --release --manifest-path tools/barking/Cargo.toml -- cover-draw docs/<YYMM>/<MMDD>
```

输出 A、B 两组取值（画风、底色、标题、看板娘、动作）、每个值的提示，以及两行现成的 `封面抽签：…`。两组都配不上当期内容时可以重抽一次（再跑一遍），把理由写进 `images/README.md`。不要在两组之间拼取值。

## 2. 写提示词

每组写一段竖版、一段横版。提示词分两部分：

- **画面**：每组从头写，不从上一期或另一组复制句子。先定主体：当期新闻里的具体物件或场景（不是通用玩具机器人），优先画“说法 vs 实际”里能画出来的那个东西，占画面大头，可左右分屏（EDITORIAL.md 封面“主体”，2026-11-18 起）。然后把抽到的五个值逐个落到画面上：画风在桶里挑最贴主题的细分；底色；标题放在哪里、写成什么字；看板娘怎么出镜、在干什么（只作配角：探头、小人至多占画面一成；抽到角标时画面里不画她，动作不画）。
- **硬性要求**：下面这段逐字照抄，只填 `{…}`。看板娘只作配角（1117 期起）：
  - 抽到“探头”“小人”：照抄整段，并在末尾加一句「看板娘只占画面很小一部分，不超过画面面积的十分之一；画面主体是{主体}」。
  - 抽到“角标”：删去这段里“看板娘胸前圆牌上的「AI 吠点」”和从“看板娘必须和附带的参考图是同一个角色”到“一条尾巴。”的句子，换成「画面里不画本号看板娘（蓝发、垂耳、鲸尾的女仆少女）。{右下｜左下｜右上｜左上}角留出直径约为画面短边六分之一的圆形空位，不放文字或物件」，选一个不压标题和主体的角；横版要求里的“主标题与看板娘完整落在画面正中的方形区域内”改成“主标题完整落在画面正中的方形区域内”。生成时不附参考图：下面两条命令都删掉 `-i …mascot-reference.png` 那一项。新闻里的人物与模型吉祥物照 EDITORIAL.md 封面“角色”一条画。角标由主执行方选定后叠加（第 4 步）。

> 画面里只允许出现这些文字：主标题，逐字为「{主标题}」；{副标题，逐字为「…」；关键数字，逐字为「…」；——按 EDITORIAL.md 封面“文字”一条决定加不加，不加就删掉这半句}看板娘胸前圆牌上的「AI 吠点」。不要其他文字、数字、字母、logo、国旗、品牌标志或真人。看板娘必须和附带的参考图是同一个角色、同一种画法：Q版二头身到二头半身的萌系二次元少女，大头、大圆眼、脸颊红晕，赛璐璐平涂上色和清晰描边，不要写实或半写实、不要成人身材比例。外形照参考图：深蓝到浅蓝渐变长卷发、头顶一根呆毛、白色荷叶边女仆头饰、浅蓝蝴蝶结发饰；两只向下垂的深蓝色鲸鱼鳍状垂耳（内侧白色绒边），不是竖起的猫耳或狼耳；深蓝白色女仆装、白围裙上有小鲸鱼图案；一条带白色圆点的深蓝鲸鱼尾巴；胸前「AI 吠点」圆牌；两只正常的人手（不戴手套）。抽到的画风只用于场景与标题，看板娘本人保持参考图画风（2026-10-27 起）。她只有两只手、两只耳朵、一条尾巴。主标题在手机缩略图下清楚可读。{竖版：3:4 竖版，1080×1440 或同等比例。｜横版：2.35:1 横版，约 1920×816，独立横构图；主标题与看板娘完整落在画面正中的方形区域内，便于再裁出 1:1 缩略图。}

两组的四段提示词都记进 `images/README.md`（选中组在前，落选组注明“落选”）。

## 3. 运行

`{OUT}` 为草稿目录下的新子目录（如 `cover-A/`、`cover-B/`），候选图不放进仓库；`<仓库根>` 见 `docs/workflow/hosts.md`。两组可以同时在后台跑。

Linux / macOS（沙箱只开 `{OUT}` 可写）：

```bash
codex exec -m gpt-6.2-sol -c 'model_reasoning_effort="medium"' -s workspace-write --skip-git-repo-check --ephemeral -C "{OUT}" -i "<仓库根>/common_images/mascot-reference.png" -o "{OUT}/report.md" - < "{OUT}/prompt.md"
```

Windows：

```bash
codex exec -m gpt-6.2-sol -c 'model_reasoning_effort="medium"' -s danger-full-access --skip-git-repo-check --ephemeral -C "E:/gitclone/AI-Barking" -i "E:/gitclone/AI-Barking/common_images/mascot-reference.png" -o "{OUT}/report.md" - < "{OUT}/prompt.md"
```

约 3 分钟。`image_generation` 是 Codex CLI 的内置功能（0.156 起为 stable，默认开启）。Windows 拦下 `danger-full-access` 时同样改用 `-s workspace-write`，并以 `{OUT}` 为工作区（1024 期如此）。Linux / macOS 的 `workspace-write` 生成失败或图没落到 `{OUT}` 时，退回 `danger-full-access`（`-C` 用本机仓库根），并在 `images/README.md` 封面节记一笔。若改用当前 agent 自带的支持参考图生图工具（见 `docs/workflow/harness.md` 第 5 节），按同份竖版/横版提示词与参考图输出到 `{OUT}/00-cover.png`、`{OUT}/00-cover-wide.png`，同样走第 4 步验收。

### prompt.md

---

你是图像生成执行者。用你的图像生成能力生成两张封面图。

输出：
- `{OUT}/00-cover.png`（竖版 3:4）与 `{OUT}/00-cover-wide.png`（横版 2.35:1，不支持则取最接近的横版比例并说明）。
- 生成工具若存到别处，复制到上述路径。只写入 `{OUT}/`，不改动其他文件，不 commit。
- 回报两张图的实际像素尺寸与保存路径。

=== 图 1：3:4 竖版 ===
{竖版提示词}
=== 图 2：2.35:1 横版 ===
{横版提示词}

---

## 4. 挑选与验收（主执行方）

1. 逐字核对画面里的每个字与数字：必须与提示词引号内文字、正文一致；多字、漏字、错字、乱码、logo、真人脸即淘汰。小字裁出放大约 4 倍再看。
2. 看板娘（出镜时）：数手、耳朵、尾巴；看她是不是保持二次元可爱风（2026-10-27 起 Develata 决定，不再随场景画风重画），且姿势、动作按本期抽签重新画，不是参考图原样贴进来；估一下她占画面多大，超过一成就淘汰（1117 期起）。抽到角标的，看留出的那个角是否空着。
3. 与前几期比：用看图/读文件工具打开前 3 期的封面（`images/00-cover.png`，1029 期起 `images/publish/00-cover.png`），和候选一起看。缩略图一眼像同一系列（同样的底色、同样的标题字和位置、同样的看板娘姿势）就淘汰。
4. 横版：按居中 1:1（宽 = 图高）裁切检查，主标题（与出镜的看板娘）必须完整留在框内。竖版：手机缩略图下主标题可读，主体不拥挤。
5. 抽到角标的，在复制前叠加角标：把 `common_images/profile_picture.png` 裁成圆形，直径为画面短边的六分之一（竖版、横版都按短边同比例缩放），放进留出的那个角（`{输出}` 用新文件名，不覆盖候选；scale2ref 里 `iw`、`ih` 指画面，`mdar` 指头像；下面的命令放右下角；左下把 `overlay=` 的 x 改成 `min(W\,H)/30`，上方把 y 改成 `min(W\,H)/30`）。叠好后再看一遍角标没压住标题和主体。

   ```bash
   ffmpeg -y -i "{候选}" -i common_images/profile_picture.png -filter_complex "[1][0]scale2ref=w='min(iw,ih)/6':h='ow/mdar'[av][bg];[av]format=rgba,geq=r='r(X,Y)':g='g(X,Y)':b='b(X,Y)':a='if(lte(hypot(X-W/2,Y-H/2),W/2),255,0)'[avc];[bg][avc]overlay=W-w-min(W\,H)/30:H-h-min(W\,H)/30:format=rgb" "{输出}"
   ```

6. 选中的图复制为本期 `images/publish/00-cover.png`、`images/publish/00-cover-wide.png`（1029 期起；`images/publish/` 不存在就先建；1028 及更早期次是 `images/00-cover.png`、`images/00-cover-wide.png`）。`{OUT}` 草稿目录里的候选与落选图不放进 `publish/`。在 `images/README.md` 封面节记录：`封面抽签：…` 一行（选中组，整行照抄）、抽签种子、生成方式、候选数、选中哪组及理由、落选原因、逐字核对结果。
