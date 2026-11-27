# Google Workspace CLI (gwscli) - User Guide

Welcome to the Google Workspace CLI (`gwscli`). This tool provides command-line
access to Google Workspace services (Docs, Sheets, Gmail, Drive, etc.).

This document is designed to help users and AI agents (LLMs) understand how to
use and interact with the CLI safely and effectively.

---

## 1. Important Security Consideration: Indirect Prompt Injection Risk

Exposing any language model to untrusted data creates the risk of an [indirect
prompt injection attack](https://en.wikipedia.org/wiki/Prompt_injection).
Agentic tools have access to a wide array of tools and APIs.

This tool, given to an agent, grants it the ability to read, create, and
modify your Google Workspace data.

*   **Never** use this tool with untrusted agents or workflows.
*   **Never** include untrusted inputs into the model context. This includes
    asking an AI agent to process mail, documents, or other resources from
    unverified or untrusted sources.
*   Untrusted inputs may contain hidden instructions that could hijack your
    session. Attackers can then leverage this to modify, steal, or destroy your
    data.
*   Always carefully review actions taken by the agent on your behalf to ensure
    they are correct and align with your intentions.

---

## 2. Dynamic Discovery & Agent Skills (Recommended for AI Agents)

If you are an AI agent or LLM using this CLI, you should **first install the
packaged agent skills**. These skills provide detailed context, safety warnings,
and domain-specific guidelines that help you use various CLI commands
correctly (e.g., managing files, reading mail).

### Installing Agent Skills
Install the skills to your agent's discovery directory immediately after
downloading the CLI:

```bash
# Note: ~/.agents/skills is just provided as an example directory.
gwscli skills install --dir ~/.agents/skills
```

**Note:** To update existing skills, run the install command with the
`--overwrite` flag. However, **do not blindly overwrite skills** as they may
contain important local changes. For AI agents, they MUST consult the user
before overwriting.

```bash
gwscli skills install --dir ~/.agents/skills --overwrite
```

To review changes before overwriting, you (or your agent) can install the new
skills to a temporary directory and diff them:

```bash
# On Linux/macOS:
TEMP_DIR=$(mktemp -d)
gwscli skills install --dir "$TEMP_DIR"
diff -ru ~/.agents/skills "$TEMP_DIR"
rm -rf "$TEMP_DIR"

# On Windows (CMD):
gwscli skills install --dir %TEMP%\new_skills
fc /s %USERPROFILE%\.agents\skills\* %TEMP%\new_skills\*
rmdir /s /q %TEMP%\new_skills
```

Always check for skill updates after updating gwscli.

### Self-Exploration & Help
You can also discover all available commands, subcommands, and flags
dynamically by using the `--help` flag.

#### Command Structure

```bash
gwscli <service> <optional resource type> <action> [flags]
```

#### Discovery Commands

*   **List all available services:**

    ```bash
    gwscli --help
    ```

*   **List actions for a specific service (e.g., `drive`):**

    ```bash
    gwscli drive --help
    ```

*   **List flags for a specific action (e.g., `drive list_recent_files`):**

    ```bash
    gwscli drive list_recent_files --help
    ```

*Note: Running `--help` on domain-specific commands dynamically queries the
backend to return the most up-to-date parameter details and schemas.*

---

## 3. Authentication & Headless Safety Guidelines

`gwscli` uses OAuth 2.0 to authenticate requests to Google Workspace APIs.
Authentication is granular and scope-specific.

There are two ways to authenticate the CLI:

1.  **Bring Your Own OAuth Client**: Call `auth setup` to configure an OAuth
    client ID and secret, allowing the CLI to generate and manage its own tokens
    via an interactive login flow.
2.  **Bring Your Own OAuth Token**: Pass `--oauth_token` to the CLI on every
    command, taking care of generating and refreshing OAuth tokens yourself.

### Option 1: Bring Your Own OAuth Client

If you want the CLI to manage your authentication state and refresh tokens, you
must configure it with a Google Cloud OAuth client. You can use an existing
client or create a new one.

**Quick Setup (Personal Mode Shortcut):** To create a personal mode OAuth client
ID in a single step, use the
[Google Cloud Console personal mode setup](https://console.cloud.google.com/auth/overview/personal?apis=ws,wsmcp,tasksmcp&clientType=desktop&isAgent=true).
It creates or selects a personal mode project, enables the required Workspace
and MCP APIs, and pre-selects **Desktop app** and **Use this client for an
AI-powered agent**. Download the client JSON file (`client_secret.json`) and
skip directly to Step 3.

For manual setup or an existing Google Cloud project, follow Steps 1–3 below:

#### Step 1: Enable the APIs

In your [Google Cloud project](https://console.cloud.google.com), enable the
required APIs. `gwscli` reaches Workspace through the MCP services. Each command
group maps to one of them:

*   `gwscli gmail` — `gmailmcp.googleapis.com`
*   `gwscli drive` — `drivemcp.googleapis.com`
*   `gwscli docs` — `docsmcp.googleapis.com`
*   `gwscli sheets` — `sheetsmcp.googleapis.com`
*   `gwscli slides` — `slidesmcp.googleapis.com`
*   `gwscli calendar` — `calendarmcp.googleapis.com`
*   `gwscli chat` — `chatmcp.googleapis.com`
*   `gwscli people` — `people.googleapis.com`
*   `gwscli tasks` — `tasksmcp.googleapis.com`

Each MCP service also needs the underlying Workspace API it fronts
(`drivemcp.googleapis.com` needs `drive.googleapis.com`, and so on).

To enable everything at once:

```bash
gcloud services enable --project=PROJECT_ID \
    gmail.googleapis.com drive.googleapis.com docs.googleapis.com \
    sheets.googleapis.com slides.googleapis.com calendar-json.googleapis.com \
    chat.googleapis.com people.googleapis.com tasks.googleapis.com

gcloud services enable --project=PROJECT_ID \
    gmailmcp.googleapis.com drivemcp.googleapis.com docsmcp.googleapis.com \
    sheetsmcp.googleapis.com slidesmcp.googleapis.com \
    calendarmcp.googleapis.com chatmcp.googleapis.com \
    tasksmcp.googleapis.com
```

**Note:** Some `gwscli chat` commands (e.g. `send_message`) additionally require
a Chat app to be configured in your project. Please refer to the
[Google Chat API configuration guide](https://developers.google.com/workspace/chat/configure-overview)
to configure a Chat app.

#### Step 2: Create a Desktop App OAuth Client

If you are not using the
[Google Cloud Console personal mode setup](https://console.cloud.google.com/auth/overview/personal?apis=ws,wsmcp,tasksmcp&clientType=desktop&isAgent=true)
shortcut above (which creates or uses a personal mode project and enables the
Workspace and MCP APIs for you), follow the
[Google Workspace developers guide](https://developers.google.com/workspace/guides/create-credentials#oauth-client-id)
to set up your OAuth consent screen and create an OAuth client ID in the project
from Step 1.

**Important details for `gwscli`:**

*   **Application Type:** You must choose **Desktop app** when creating the
    OAuth Client ID. `gwscli` starts a temporary HTTP listener on a random free
    port for the OAuth callback, which is only supported by the "Desktop app"
    client type.
*   **AI-Powered Agent Client:** You **must** check **Use this client for an
    AI-powered agent** when creating the OAuth Client ID.
*   **Test Users:** If your consent screen is marked as "External", ensure you
    add your own email address to the "Test users" list, or consent will be
    refused.

Important: Download the client JSON file securely to your machine
(`client_secret.json`).

#### Step 3: Register the client with `gwscli`

```bash
gwscli auth setup --oauth_client_file=/path/to/client_secret.json
```

The CLI reads the JSON exactly as the Cloud console exports it.

**This is destructive.** `auth setup` logs you out before saving, discarding any
OAuth tokens you have already granted. It asks for confirmation on stdin first.
Answer `Y` to proceed.

#### Interactive Login

Once the OAuth client is registered, you authorize individual commands or apps
via a browser prompt:

```bash
gwscli auth login <app> <command>
```

```bash
gwscli auth login <app>
```

*Example:*

```bash
gwscli auth login chat search_messages
```

### Option 2: Bring Your Own OAuth Token

If you already have an OAuth access token from another source, you can skip both
`auth setup` and `auth login` and pass it directly with the global
`--oauth_token` flag:

```bash
gwscli --oauth_token="$ACCESS_TOKEN" chat search_messages
```

The token must already carry the scopes the command needs; `gwscli` does not
request consent or refresh the token in this mode.

You can verify what scopes the command requires in advance via the help command
or flag.

*   **SSH / Remote Machine Hook**: The OAuth flow runs a temporary server on
    `localhost` to listen for the redirect callback. Therefore, the OAuth link
    must be opened on the same machine that executed the `gwscli auth login`
    command. If you are logged into a remote machine via SSH, you should run the
    login flow locally, or set up SSH port forwarding back to your local machine
    for the dynamic port printed by the command.

### Safety Rules for Headless/Automated Execution (For AI Agents)

*   **DO NOT RUN LOGIN OR SETUP COMMANDS IN SCRIPTS**: `gwscli auth login` is
    strictly interactive and requires a browser and user interaction, and
    `gwscli auth setup` blocks on a stdin confirmation prompt. Attempting to run
    either in a headless script or automated background process will deadlock
    the execution.
*   **Human-in-the-Loop Hand-off**: If you encounter an authentication error you
    must immediately pause and ask the human user to complete the step in their
    local terminal, for example:

    ```text
    I need to access Google Workspace, but I am unauthenticated.
    Please follow the "Option 1: Build Your Own OAuth Client" setup in this README, then
    run `gwscli auth login drive list_recent_files` in your local terminal to authorize my access.
    Alternatively, you can provide an OAuth Token directly.
    ```

### Revoking Access and Reporting Abuse

#### Revoking OAuth Tokens & Deleting Client Credentials

*   **Local CLI logout and reset**:
    *   Run `gwscli auth logout` to revoke your cached refresh or access token
        with Google's OAuth revocation endpoint and delete the locally cached
        token file.
    *   Run `gwscli auth reset` to log out and delete the locally stored OAuth
        client credentials registered with `gwscli auth setup`.
*   **Google Account connections**: Review and revoke third-party or
    self-authorized app access at
    [Google Account Third-Party Connections](https://myaccount.google.com/connections)
    (also accessible at https://myaccount.google.com/permissions).
*   **Google Cloud OAuth client**: To rotate or permanently delete your Desktop
    App OAuth client ID and secret (for example, if a secret is exposed), open
    [Google Cloud Console Credentials](https://console.cloud.google.com/apis/credentials),
    select your project, and delete or reset the OAuth 2.0 Client ID.
*   **Workspace Administrators**: Org admins can restrict third-party or custom
    OAuth app access across their domain in the
    [Google Workspace Admin Console](https://admin.google.com) under **Security >
    Access and data control > API controls**.

#### Reporting Suspected Abuse

*   **Google Cloud Platform / OAuth client abuse**: Report suspected abuse of a
    Google Cloud project or OAuth client via the
    [Google Cloud Platform Abuse Report form](https://support.google.com/code/contact/cloud_platform_report).
*   **Google Workspace abuse**: Report suspected Workspace domain or service
    abuse using the guidance in
    [Report abuse on Google Workspace](https://support.google.com/a/answer/91535)
    or [Report Content for Legal Reasons](https://support.google.com/legal/answer/3110420).

---

<!-- disableFinding(LINE_OVER_80) -->

## 4. Output Formatting & Parsing

*   **Output Behavior**: `gwscli` writes command output directly to `stdout`.
*   **Checking Output Format (`--help`)**: Run `gwscli <app> <tool> --help` to
    inspect the high-level `Output Format` summary for any command.
*   **Inspecting Detailed Structure (`--schema`)**: Run
    `gwscli <app> <tool> --schema` to print the full JSON schema / response
    structure and field definitions without executing a backend call.
*   **Output Configuration Flags**: Some commands provide flags that customize
    the response payload (such as `--excludeContentSnippets` or export format
    options). Consult `gwscli <app> <tool> --help` for available flags.
*   **Structured JSON Output**: Commands returning resource entities or
    paginated collections (such as `drive search_files` or
    `calendar list_events`) output valid JSON. You can safely pipe their output
    to `jq`:
    *   *Example:*

        ```bash
        gwscli drive search_files --query="title contains 'Report'" | jq '.files[] | {title: .title, id: .id}'
        ```

*   **Markdown & Plain Text Output**: Commands that return document content
    (such as `docs read_doc`) output raw Markdown strings directly. Do not pipe
    these commands to `jq`. Static utility commands (`version`, `skills list`,
    etc.) return plain text strings.
