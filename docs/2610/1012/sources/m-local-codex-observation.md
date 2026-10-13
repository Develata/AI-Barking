# 本号机器上的 Codex 守护进程目录（Claude 只读观察，体感、样本小）

观察时间：2026-10-13 07:2x EDT（北京 10/13 19:2x）。只读命令（`ps`、`ls`、`du`、`codex --version`），没有启动、停止或修改任何 Codex 进程与文件。用户名与绝对路径已改写为 `~`。

## WSL Debian（Codex CLI 0.159.3，bun 全局安装）

`~/.codex/packages/app-server-daemon/`：

| 项 | 内容 |
|---|---|
| `releases/0.156.1-x86_64-unknown-linux-musl` | 370M，目录时间 Sep 25 17:03 |
| `releases/0.157.0-x86_64-unknown-linux-musl` | 374M，Sep 25 17:09 |
| `releases/0.157.1-x86_64-unknown-linux-musl` | 374M，Sep 25 21:16 |
| `releases/0.159.2-x86_64-unknown-linux-musl` | 424M，Sep 30 10:04 |
| `releases/0.159.3-x86_64-unknown-linux-musl` | 425M，Sep 30 19:11 |
| `current` | 软链 → `releases/0.159.3-…` |
| `auto-update-version` | 33 字节，Sep 30 19:16 |
| `install.lock` | 0 字节，Oct 9 09:54 |
| 合计（`du -sh`） | 2.0G |

观察时刻 WSL 内没有 `app-server` 守护进程在运行（WSL 本次开机于 2026-10-13 02:06 EDT；当时只有另一会话的 `codex exec` 只读审查进程）。目录时间为 WSL 本地时间（EDT）。

## Windows（Scoop 安装的 Codex，0.162.1）

在跑的 `codex.exe … app-server …` 进程属于 Codex 桌面应用（父进程为桌面应用），与 CLI 的默认守护进程不是一回事，不作证据。

## 用途与边界

- 只说明“在本号这台机器上，旧版本副本没有被清掉，累计约 2G”；不能外推到所有用户、所有平台。
- 旧副本是否本应被清理、0.156.1 为何已有副本，须以 A 组源码取证为准。
