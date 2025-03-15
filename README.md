# mcpc

[MCP](https://modelcontextprotocol.io/) (Model Context Protocol) Server Template Generator

## Introduction

`mcpc` is a command-line tool that generates scaffolding for MCP (Model Context Protocol) server projects. It helps you quickly set up a properly structured project with all necessary files and configurations.

## Features

- Generate MCP server templates for TypeScript or Python
- Support for multiple package managers (pnpm, yarn, npm for TypeScript; uv for Python)
- Automatic dependency installation
- System dependency validation
- Git repository initialization
- Complete project structure following official MCP documentation
- Python test mode for verifying server functionality without Claude
- Configuration files and best practices included

## Installation

### Prerequisites
- Rust and Cargo (can be installed via [rustup](https://rustup.rs/))

### Build and Install
```bash
cargo install --path .
```

## Usage

Basic usage:
```bash
mcpc project_name [options]
```

Options:
- `-l, --language`: Programming language to use (py/python, ts/typescript, default: typescript)
- `-t, --tool`: Package manager tool to use:
  - For TypeScript: pnpm (default), yarn, npm
  - For Python: uv (default)


Examples:
```bash
# Create a TypeScript project named 'weather-api' using pnpm
mcpc weather-api -l ts -t pnpm

# Create a Python project named 'mcp-server' using uv
mcpc mcp-server -l py -t uv
```

## Generated Project Structure

### TypeScript Project
```
project_name/
├── .gitignore
├── .prettierignore
├── .prettierrc
├── package.json
├── README.md
├── tsconfig.json
├── build/
└── src/
    └── index.ts
```

### Python Project
```
project_name/
├── .gitignore
├── pyproject.toml
├── README.md
├── requirements.txt
├── server.py
└── .venv/
```

## Using Generated Projects

### Python
```bash
# Activate virtual environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies (if not already installed)
uv pip install -r requirements.txt

# Test server functionality without Claude
python server.py --test

# Run server for Claude integration
python server.py
```

### TypeScript
```bash
# Install dependencies (if not already installed)
pnpm install  # or: yarn / npm install

# Build the project
pnpm run build  # or: yarn build / npm run build

# Run server
node build/index.js
```

## Claude for Desktop Integration

To integrate with Claude for Desktop, configure your server in Claude's configuration file:

```json
{
  "mcpServers": {
    "weather": {
      "command": "uv",  // or "node" for TypeScript
      "args": [
        "--directory",
        "/ABSOLUTE/PATH/TO/PROJECT",
        "run",
        "server.py"  // or "build/index.js" for TypeScript
      ]
    }
  }
}
```

## Development

### Build from Source
```bash
git clone https://github.com/yourusername/mcpc.git
cd mcpc
cargo build --release
```

### Running Tests
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 