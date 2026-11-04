# issue 14370 评论 6618575621（ronthedrummer，NONE，2026-11-03T23:56:23Z）
来源：gh api，原文逐字；URL https://github.com/postmanlabs/postman-app-support/issues/14370#issuecomment-6618575621

Same path on a later build. Today I opened an existing signed-in Postman on macOS 26.5.2 (25F84). The app updated from 12.31.3 to 12.35.1 on that launch, then `auto-existing-upgrade` installed the CLI and the Claude Code and Codex plugins. Cursor was already installed.

The desktop install date in the log is 2026-03-20. The fresh-download checks logged `skipped-not-fresh`. The existing-user checks are what ran.

From `~/Library/Application Support/Postman/logs/main.log`, 2026-11-03T22:58:31Z through 22:59:29Z:

```text
PostmanCLIService~checkInstalled: not found
{"code":"ENOENT"}

PostmanCLIService~installCli: completed
{"degraded":false,"source":"auto-existing-upgrade"}

Main: autoInstall.cli.existing-user finished
{"outcome":"installed","platform":"darwin","exposed":true,"signedIn":true,"flagTimedOut":false}

PostmanAgentPluginService~install: script completed
{"source":"auto-existing-upgrade","results":["claude-code:installed","codex:installed","cursor:already-installed"],"installer":"script"}

Main: autoInstall.plugin.existing-user finished
{"outcome":"installed","platform":"darwin","exposed":true,"signedIn":true,"flagTimedOut":false,"useScript":true,"installer":"script","scriptResults":"claude-code:installed,codex:installed,cursor:already-installed"}
```

Checked against the 12.33.4 report above:

- CLI: `~/.local/bin/postman` appeared in that window. `postman --version` prints 1.85.0. It was not installed at the start of the launch.
- Claude Code: `postman@claude-plugins-official` is enabled in `~/.claude/settings.json`. `~/.claude/plugins/installed_plugins.json` records version 2.0.1, user scope, `installedAt` and `lastUpdated` both `2026-11-03T22:58:41.007Z`, commit `c6352ef5605cd46d3f817e15c601eeb9a56c08f7`.
- Codex: `~/.codex/config.toml` has `[marketplaces.postman]` set to `https://github.com/postmanlabs/postman-plugin.git` and `[plugins."postman@postman"]` with `enabled = true`. That file's mtime is 2026-11-03T22:59:29Z. The cached plugin is 1.3.0. Its `hooks/codex-hooks.json` registers a SessionStart hook for `startup|clear|compact` that prints `hooks/session-start-context.md`. `mcp.codex.json` points at `https://mcp.postman.com/mcp`. `config.toml` has no `hooks.state` or `trusted_hash` entry.
- Cursor: this run recorded `cursor:already-installed`. The installed marketplace plugin is 2.2.0, and its cache predates this launch, so this run did not install it. The same session-start text is injected into Cursor agent sessions, with the `postman:` skill prefix removed. Cursor's Postman MCP server is configured and still asks for authentication.
- Copilot CLI, Factory Droid, OpenCode, Pi, Kimi, Gemini, and Antigravity are not installed here. `~/.agents` has no Postman config. This run's result list names Claude Code, Codex, and Cursor.

This confirms the install and the startup instruction. I do not have evidence that data was uploaded.

