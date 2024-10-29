pub const FRONTMATTER_TEMPLATE: &str = r"---
title: {title}
system: {system}
model: {model}
api_endpoint: {api_endpoint}
created_at: {created_at}
updated_at: {updated_at}
tags: {tags}
summary: {summary}
---";

pub const ADD_OPENAI_KEY_MESSAGE: &str = "OpenAI API key not found! Please set it using one of these methods:\n\
1. Run the command with your API key using --api-key=your-api-key-here\n\
2. Set it in your .env file\n\
3. Set it as an environment variable:\n\
   - Windows (Command Prompt): set OPENAI_API_KEY=your-api-key-here\n\
   - Windows (PowerShell): $env:OPENAI_API_KEY='your-api-key-here'\n\
   - Mac/Linux: export OPENAI_API_KEY=your-api-key-here";
