> This page is for Postman Docs, version Latest (v12) (default).
> For other versions, use one of these documentation indexes:
> - Latest (v12) (default): https://learning.postman.com/latest-v-12/llms.txt
> - v11: https://learning.postman.com/v11/llms.txt

> For clean Markdown content of this page, append .md to this URL. For the complete documentation index, see https://learning.postman.com/llms.txt.

# Install the Postman plugin for a specific agent

> Install the Postman plugin into Antigravity, Claude Code, Codex, GitHub Copilot CLI, Cursor, Factory, Kimi Code, OpenCode, or Pi using each agent's own marketplace or command.

Install the plugin into a single coding agent using that agent's own method. Check that your agent meets the [requirements](/docs/postman-plugin/install/overview/#requirements) before you start.

> **Info**
>
> To install into more than one agent at once, see [Install the Postman plugin into detected agents](/docs/postman-plugin/install/detected-agents/).

## Antigravity

Install the plugin for Antigravity using the following command:

```bash showLineNumbers={false}
agy plugin install https://github.com/postmanlabs/postman-plugin
```

To update the plugin, run the same command again. To remove it, run `agy plugin uninstall postman`. Restart Antigravity for the change to take effect.

You can also install the plugin from the Antigravity app. Go to **Customizations**, select **Plugins**, then select **Install from URL** and enter the repository URL.

## Claude Code

Before installing, run an interactive `claude` session at least once to register the official marketplace.

Install the plugin for Claude Code using [Claude's plugin marketplace](https://claude.com/plugins/postman):

```bash showLineNumbers={false}
claude plugin marketplace add anthropics/claude-plugins-official
claude plugin install postman@claude-plugins-official
```

## Codex

Install the plugin for Codex from the app, a direct link, or the command line.

#### App

1. Open the Codex app, and click **Plugins**.
2. Search for "Postman".
3. Select the Postman plugin, and click **Install**.

#### Direct link

Go to [chatgpt.com/plugins/postman](https://chatgpt.com/plugins/postman) and click **Install**. This prompts you to open the Codex app to finish the installation.

#### Command line

```bash showLineNumbers={false}
codex plugin marketplace add postmanlabs/postman-plugin --ref main
codex plugin marketplace upgrade postman
codex plugin add postman@postman
```

These commands track the `main` branch. The `marketplace upgrade` step records the revision so Codex doesn't fetch the plugin again the next time it starts.

> **Info**
>
> On Codex, the plugin adds a session-start hook that guides Codex to Postman's skills. How the hook is handled depends on how you install:
>
> * **App or command line** — The hook installs untrusted. The next time Codex starts, it shows "Hooks need review." Approve the Postman hook there. Codex asks you again after an update changes the hook.
> * **Direct link (ChatGPT)** — The published plugin has no hook, so Codex doesn't prompt you.
>
> Postman's skills and MCP server work either way, with or without the hook. The hook makes Codex reach for Postman's skills more reliably.
>
> If you install with the detected-agents installer instead, it approves the hook for you, or tells you to review it if it can't. See [Codex session-start hook](/docs/postman-plugin/install/detected-agents/#codex-session-start-hook).

## GitHub Copilot CLI

Install the plugin for GitHub Copilot CLI using the following commands:

```bash showLineNumbers={false}
copilot plugin marketplace add postmanlabs/postman-plugin
copilot plugin install postman@postman
```

To update, run `copilot plugin marketplace update postman`, then `copilot plugin update postman`. To sign in to Postman's MCP server, run `/mcp` inside a Copilot CLI session. Restart Copilot CLI, or run `/restart`, for the change to take effect.

## Cursor

Inside Cursor, install the plugin from the [Cursor marketplace](https://cursor.com/marketplace/postman):

```text showLineNumbers={false}
/add-plugin postman
```

## Factory

Install the plugin for Factory using the following commands:

```bash showLineNumbers={false}
droid plugin marketplace add https://github.com/postmanlabs/postman-plugin.git
droid plugin install postman@postman-plugin --scope user
```

To update, run `droid plugin marketplace update postman-plugin`, then `droid plugin update postman@postman-plugin --scope user`. To sign in to Postman's MCP server, run `/mcp` inside a Droid session.

## Kimi Code

Inside a Kimi Code session, install the plugin using the full repository URL:

```text showLineNumbers={false}
/plugins install https://github.com/postmanlabs/postman-plugin/tree/main
```

To update, run the same command again, then `/new`. To remove the plugin, run `/plugins remove postman`. To sign in to Postman's MCP server, run `/mcp-config login plugin-postman:postman`.

## OpenCode

Install the plugin for OpenCode using the command for your version.

OpenCode 2.0.4 or later:

```bash showLineNumbers={false}
opencode plugin add github:postmanlabs/postman-plugin
```

OpenCode 1.14.33 or later:

```bash showLineNumbers={false}
opencode plugin --global github:postmanlabs/postman-plugin
```

You need Git for either command. Restart OpenCode after installing.

> **Note**
>
> If you installed the plugin for OpenCode by cloning the repository and adding a loader file, remove that install to avoid registering each skill twice. Delete `~/.config/opencode/plugins/postman.ts` and the `~/.config/opencode/postman-plugin` directory.

To remove the plugin on OpenCode 2.0.4 or later, run `opencode plugin remove github:postmanlabs/postman-plugin`. On OpenCode 1.14.33 or later, delete the `plugin` entry from `~/.config/opencode/opencode.json` or `opencode.jsonc`, then restart OpenCode.

## Pi

Install the plugin for Pi from [Pi's package gallery](https://pi.dev/packages/@postman/postman-plugin):

```bash showLineNumbers={false}
pi install npm:@postman/postman-plugin
```

To update, run `pi update npm:@postman/postman-plugin`. Then, inside a Pi session, sign in to Postman's MCP server with `/mcp login postman`. Signing in from the shell with `pi mcp login` doesn't load extensions, so it reports no server named `postman`.