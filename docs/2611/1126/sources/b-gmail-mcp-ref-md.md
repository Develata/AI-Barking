This is an MCP server provided by Gmail API. The server provides tools for developers to build LLM applications on top of Gmail.

A [Model Context Protocol (MCP) server](https://modelcontextprotocol.io/docs/learn/server-concepts) acts as a proxy between an external service that provides context, data, or capabilities to a Large Language Model (LLM) or AI application. MCP servers connect AI applications to external systems such as databases and web services, translating their responses into a format that the AI application can understand.

### Server Setup


You must [configure the Gmail MCP server](https://developers.google.com/workspace/gmail/api/guides/configure-mcp-server.md.txt) before use. For more information about using Google and Google Cloud remote MCP servers, see [Google Cloud MCP servers overview](https://docs.cloud.google.com/mcp/overview).

### Server Endpoints

An MCP service endpoint is the network address and communication interface (usually a URL) of the MCP server that an AI application (the Host for the MCP client) uses to establish a secure, standardized connection. It is the point of contact for the LLM to request context, call a tool, or access a resource.

The Gmail MCP API MCP server offers the following MCP toolset endpoints:

- https://gmailmcp.googleapis.com/mcp/v1
- https://gmailmcp.googleapis.com/mcp/v1self

## MCP Tools

An [MCP tool](https://modelcontextprotocol.io/legacy/concepts/tools) is a function or executable capability that an MCP server exposes to a LLM or AI application to perform an action in the real world. Toolsets are a group of tools that are useful for a specific task. Using toolsets can help your agent perform better because it decreases the number of tools available to your agent.

### Toolsets

The gmailmcp.googleapis.com MCP server has the following toolsets:

| MCP Toolsets |||
| Endpoint | Description | Tools |
|---|---|---|
| /mcp/v1 | Toolset for the Gmail MCP Server. | ` - apply_sensitive_message_label - apply_sensitive_thread_label - create_draft - create_filter - create_label - forward - get_draft - get_message - get_thread - label_message - label_thread - list_drafts - list_filters - list_labels - mark_message_spam - mark_thread_spam - reply - search_threads - send_message - trash_message - trash_thread - unlabel_message - unlabel_thread - unmark_message_spam - unmark_thread_spam - untrash_message - untrash_thread - update_draft - update_label - update_message_labels ` |
| /mcp/v1self | Self-mode toolset for the Gmail MCP Server. | ` - create_draft - create_filter - create_label - get_draft - get_message - get_thread - label_message - label_thread - list_drafts - list_filters - list_labels - mark_message_spam - mark_thread_spam - search_threads - trash_message - trash_thread - unlabel_message - unlabel_thread - unmark_message_spam - unmark_thread_spam - untrash_message - untrash_thread - update_draft - update_label - update_message_labels ` |

### Get MCP tool specifications


To get the MCP tool specifications for all tools in an MCP server, use the `tools/list` method. The following example demonstrates how to use `curl` to list all tools and their specifications currently available within the MCP server.

| Curl Request |
|---|
| ```bash curl --location 'https://gmailmcp.googleapis.com/mcp/v1' \ --header 'content-type: application/json' \ --header 'accept: application/json, text/event-stream' \ --data '{ "method": "tools/list", "jsonrpc": "2.0", "id": 1 }' ``` |