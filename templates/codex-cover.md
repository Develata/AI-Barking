# Codex 封面生成模板

由 Claude 填写 `{…}` 后写入草稿目录的 `prompt.md`，用 Codex CLI 生成。两段提示词直接取自本期 `images/README.md` 的“3:4 竖版提示词”“2.35:1 横版提示词”，不另写一份。

每次运行出竖版、横版各 1 张；每种尺寸先凑满 2 张候选（即运行 2 次），挑不出再改提示词重跑，每种尺寸累计最多 5 张。

## 命令

`{OUT}` 为草稿目录下的新子目录（如 `cover-1/`、`cover-2/`），候选图不放进仓库。

```bash
codex exec -m gpt-6-astra -c 'model_reasoning_effort="medium"' -s danger-full-access --skip-git-repo-check --ephemeral -C "E:/gitclone/AI-Barking" -i "E:/gitclone/AI-Barking/common_images/profile_picture.png" -o "{OUT}/report.md" - < "{OUT}/prompt.md"
```

在后台运行，约 3 分钟。`image_generation` 是 Codex CLI 的内置功能（0.156 起为 stable，默认开启）。

## prompt.md

---

你是图像生成执行者。用你的图像生成能力生成两张封面图，附带的参考图是账号看板娘头像（务必保持其外观一致：蓝色长发、狗耳、鲸鱼尾巴、女仆装、白色肉垫手套、胸前“AI 吠点”圆牌）。

输出：
- `{OUT}/00-cover.png`（竖版 3:4）与 `{OUT}/00-cover-wide.png`（横版 2.35:1，不支持则取最接近的横版比例并说明）。
- 生成工具若存到别处，复制到上述路径。只写入 `{OUT}/`，不改动其他文件，不 commit。
- 回报两张图的实际像素尺寸与保存路径。

=== 图 1：3:4 竖版 ===
{竖版提示词}
=== 图 2：2.35:1 横版 ===
{横版提示词}

---

## 挑选与验收（Claude）

1. 逐字核对画面里的每个字与数字：必须与提示词引号内文字、正文数字一致；多字、漏字、错字、乱码、logo、真人脸即淘汰。
2. 横版：按居中 1:1（宽 = 图高）裁切检查，主标题与看板娘必须完整留在框内。
3. 竖版：手机缩略图下主标题可读；主体不拥挤（EDITORIAL.md“构图：做减法”）。
4. 选中的图复制为本期 `images/00-cover.png`、`images/00-cover-wide.png`；在 `images/README.md` 封面节记录：生成方式、候选数、选中哪张及理由、落选原因、逐字核对结果。
