# Samvada

Samvada is a powerful command-line tool for managing AI-assisted conversations using markdown files. It offers a structured approach to create, validate, and interact with AI chat sessions while maintaining a clean, version-controllable format.

## Features

- 📝 Create new chat files with proper frontmatter
- 🔍 Lint chat files to ensure correct structure
- 💬 Interactive chat sessions with AI assistants
- 📎 File reference support for including external content
- 📊 Detailed logging of chat interactions
- 🤖 Support for different AI models
- 🎛️ Customizable system prompts
- ⚙️ Configurable via YAML configuration file

## Installation

### Prerequisites

- Rust (latest stable version)
- OpenAI API key

### Installing Globally from Crates.io

```bash
cargo install samvada
```

### Building from Source

```bash
git clone https://github.com/yourusername/samvada
cd samvada
cargo build --release
```

## Configuration

Samvada uses a configuration file and supports multiple ways to provide your OpenAI API key.

### Configuration File (`config.yml`)

Samvada utilizes a `config.yml` file located in `~/.samvada/` to store default settings such as the system prompt, AI model, and API endpoint. If this file doesn't exist, Samvada will create it with default values when you first run the tool.

**Default `config.yml`:**

```yaml
system_prompt: "You are a helpful assistant."
model: "gpt-3.5-turbo"
api_endpoint: "https://api.openai.com/v1/chat/completions"
```

You can customize these settings by editing the `config.yml` file:

```bash
nano ~/.samvada/config.yml
```

### .env File

Generally, samvada uses local directory `.env` file to check for keys. If not found, then it will look into the global `.env` file in `~/.samvada/`.

### API Key Configuration

Samvada requires an OpenAI API key to function. The API key can be provided through several methods, with the following precedence order:

1. **Command Line Argument (`--api-key`)**
2. **`.env` File in Configuration Directory (`~/.samvada/.env`)**
3. **Environment Variable (`OPENAI_API_KEY`)**

#### 1. Command Line Argument

You can provide the API key directly when running a command:

```bash
samvada chat ask my_chat.md --api-key your_api_key_here
```

*Note:* Providing the API key via the command line will create or update the `.env` file in `~/.samvada/` with the provided key.

#### 2. `.env` File

Create a `.env` file in the Samvada configuration directory with your API key:

```bash
echo "OPENAI_API_KEY=your_api_key_here" > ~/.samvada/.env
```

#### 3. Environment Variable

Set the `OPENAI_API_KEY` environment variable in your shell:

- **Linux/macOS:**

  ```bash
  export OPENAI_API_KEY=your_api_key_here
  ```

- **Windows (Command Prompt):**

  ```cmd
  set OPENAI_API_KEY=your_api_key_here
  ```

- **Windows (PowerShell):**

  ```powershell
  $env:OPENAI_API_KEY="your_api_key_here"
  ```

### API Key Precedence Order

Samvada will check for the API key in the following order:

1. **Command Line Argument (`--api-key`)**: Highest priority.
2. **`.env` File (`~/.samvada/.env`)**: If no command line argument is provided.
3. **Environment Variable (`OPENAI_API_KEY`)**: If neither of the above is provided.

If the API key is not found, Samvada will prompt an error message indicating that the key is missing and needs to be set using one of the methods above.

## Usage

### Creating a New Chat

```bash
# Create a new chat file with default settings
samvada chat create my_chat

# Create a chat file with a custom directory
samvada chat create my_chat --dir /path/to/directory
```

When creating a new chat, Samvada will use the default configurations from `config.yml` to populate the frontmatter of the chat file.

### Validating Chat Files

```bash
# Lint a single chat file
samvada chat lint my_chat.md

# Lint all chat files in a directory
samvada chat lint ./chats
```

### Chatting with AI

```bash
# Start or continue a chat session
samvada chat ask my_chat.md
```

If you have not set your API key in the `.env` file or environment variable, you can pass it directly:

```bash
samvada chat ask my_chat.md --api-key your_api_key_here
```

*Note:* Providing the API key with `--api-key` will store it in `~/.samvada/.env` for future use.

## Chat File Format

Chat files use markdown with YAML frontmatter to define the conversation settings and history.

**Example Chat File (`my_chat.md`):**

```markdown
---
title: My Chat
system: You are a helpful assistant.
model: gpt-3.5-turbo
api_endpoint: https://api.openai.com/v1/chat/completions
created_at: 2024-01-01T12:00:00Z
updated_at: 2024-01-01T12:00:00Z
tags: []
summary:
---
user: Hello, how can you help me today?
assistant: I'm here to assist you with any questions or tasks you might have. How can I help?
user: Here's a file, summarize it.
[[src/main.rs]]
```

### Customizing the Frontmatter

You can override the default configurations from `config.yml` by specifying them in the frontmatter of your chat file. This allows you to customize settings like `system`, `model`, and `api_endpoint` on a per-chat basis.

### File References

Include external content in your chat by referencing files:

```markdown
user: Please review this code:
[[src/main.rs]]
```

*Note:* The file path should be on a separate line, enclosed in double square brackets.

## Logging

Samvada automatically generates log files alongside your chat files. These logs capture all interactions and system events, providing an audit trail for tracking and debugging.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).