# Zed Git Context Server

This extension provides a connection to a Git Model Context Server for Zed's AI assistant. It enables the assistant to understand and interact with Git repositories, providing contextual information about version control.

## Features

The Git context server provides information about:
- Repository status
- Commit history
- File changes
- Branch information
- And more through the MCP server

## Installation

1. Install the extension in Zed
2. Ensure you have Node.js installed (for uvx package manager)
3. Install the Git MCP server:
   ```bash
   npm install -g @modelcontextprotocol/server-git
   ```

## Configuration

Add to your Zed `settings.json` (located at `~/.config/zed/settings.json`):

```json
{
  "context_servers": {
    "zed-context-server": {
      "settings": {
        "repository": "/absolute/path/to/your/git/repository"
      }
    }
  }
}
```

> **Important:** The repository path must:
> 1. Be an absolute path (e.g., `/Users/username/projects/repo`)
> 2. Point to an existing Git repository (containing a `.git` directory)
> 3. Have proper read permissions

For example:
```json
{
  "context_servers": {
    "zed-context-server": {
      "settings": {
        "repository": "/Users/d/Projects/opensource/zed-context-server"
      }
    }
  }
}
```

## Usage

The Git context server provides several capabilities through Zed's AI assistant:

- Get repository status
- View commit history
- Check file changes
- Examine branch information

The assistant will automatically use this context when answering questions about your Git repository.

## Development

This extension is built using:
- Rust with zed_extension_api
- Model Context Protocol (MCP)
- mcp-server-git for Git integration

## Troubleshooting

1. Ensure the repository path exists and is a valid Git repository
2. Check that mcp-server-git is installed globally
3. Verify the repository path in settings.json is correct
