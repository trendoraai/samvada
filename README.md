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

Create a `.env` file in your project root or within ~/.samvada/:

```bash
touch ~/.samvada/.env
OPENAI_API_KEY=your_api_key_here
```

## Usage

### Creating a New Chat

```bash
# Create a new chat file
samvada chat create my_chat

# Create a chat file in a custom directory
samvada chat create my_chat --dir /path/to/directory
```

### Validating Chat Files

```bash
# Lint a single chat file
samvada chat lint my_chat.md

# Lint all chat files in a directory
samvada chat lint ./chats
```

### Chatting with AI

```bash
# Start or continue a chat
samvada chat ask my_chat.md
```

Pass api key as an option (this will create a `.env` file in ~/.samvada/ to store the api key):
```bash
samvada chat ask my_chat.md --api-key your_api_key_here
```

## Chat File Format

Chat files use markdown with YAML frontmatter:

```markdown
---
title: My Chat
system: You are a helpful assistant.
model: gpt-4
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

### File References

To reference external files in your chat:

```markdown
user: Please review this code:
[[src/main.rs]]
```

Note: The filename should be on a separate line, enclosed in double square brackets.

## Logging

Samvada automatically generates log files alongside your chat files, capturing all interactions and system events for easy tracking and debugging.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).
