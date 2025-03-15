use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use colored::*;

use crate::Tool;
use super::Generator;

pub struct PythonGenerator {
    project_name: String,
    _tool: Tool,
    project_path: PathBuf,
}

impl Generator for PythonGenerator {
    fn new(project_name: &str, tool: &Tool) -> Self {
        let project_path = PathBuf::from(project_name);
        
        Self {
            project_name: project_name.to_string(),
            _tool: tool.clone(),
            project_path,
        }
    }
    
    fn generate(&self) -> Result<()> {
        // Create the project directory
        self.create_directories()?;
        
        // Create project files
        self.create_files()?;
        
        // Initialize package manager
        self.init_package_manager()?;
        
        // Initialize git
        self.init_git()?;
        
        Ok(())
    }
    
    fn create_directories(&self) -> Result<()> {
        // Create main directory
        fs::create_dir(&self.project_path)
            .context(format!("Failed to create project directory: {}", self.project_path.display()))?;
        
        // No need for subdirectories with the new structure - all code is in the main server.py file
        
        Ok(())
    }
    
    fn create_files(&self) -> Result<()> {
        // Create pyproject.toml
        self.create_pyproject_toml()?;
        
        // Create requirements.txt
        self.create_requirements_txt()?;
        
        // Create .gitignore
        self.create_gitignore()?;
        
        // Create main server file
        self.create_server_file()?;
        
        // Create README
        self.create_readme()?;
        
        Ok(())
    }
    
    fn init_package_manager(&self) -> Result<()> {
        // Create virtual environment using uv
        println!("📦 Creating Python virtual environment with uv...");
        
        // Use uv to create the virtual environment
        let venv_result = Command::new("uv")
            .args(["venv"])
            .current_dir(&self.project_path)
            .output()
            .context("Failed to create virtual environment with uv venv")?;
        
        if !venv_result.status.success() {
            let error = String::from_utf8_lossy(&venv_result.stderr);
            eprintln!("⚠️ Warning: Failed to create virtual environment: {}", error);
            eprintln!("Please run 'uv venv' manually in the project directory");
        } else {
            println!("✅ Virtual environment created successfully");
        }

        println!("\n{} 📦 Python virtual environment created!", "Success:".green().bold());
        println!("\n{}", "Next steps:".blue().bold());
        println!("1. Activate the virtual environment:");
        println!("   {}  source .venv/bin/activate  {}", "$".bold(), "# On Windows: .venv\\Scripts\\activate".dimmed());
        println!("2. Install dependencies:");
        println!("   {}  uv pip install -r requirements.txt", "$".bold());
        println!("3. Run the server in test mode to verify it's working:");
        println!("   {}  python server.py --test", "$".bold());
        println!("\n{}", "Note:".yellow().bold());
        println!("If you run the server without --test, it will appear to hang. This is normal!");
        println!("The server is waiting for MCP protocol messages on stdin and is designed to be");
        println!("used with Claude for Desktop or other MCP clients.");
        println!("\nSee the README.md for more information on how to set up with Claude for Desktop.");

        Ok(())
    }
    
    fn init_git(&self) -> Result<()> {
        Command::new("git")
            .args(["init"])
            .current_dir(&self.project_path)
            .output()
            .context("Failed to initialize git repository")?;
        
        Ok(())
    }
}

impl PythonGenerator {
    fn create_pyproject_toml(&self) -> Result<()> {
        let pyproject_toml = format!(r#"[build-system]
requires = ["setuptools>=61.0"]
build-backend = "setuptools.build_meta"

[project]
name = "{}"
version = "0.1.0"
description = "MCP (Model Context Protocol) Weather Server"
authors = [
    {{name = "Your Name", email = "your.email@example.com"}},
]
readme = "README.md"
requires-python = ">=3.10"
classifiers = [
    "Programming Language :: Python :: 3",
    "License :: OSI Approved :: MIT License",
    "Operating System :: OS Independent",
]
dependencies = [
    "mcp[cli]>=1.2.0",
    "httpx>=0.24.0",
]

[tool.setuptools]
py-modules = []
"#, self.project_name);
        
        fs::write(
            self.project_path.join("pyproject.toml"),
            pyproject_toml,
        ).context("Failed to create pyproject.toml")?;
        
        Ok(())
    }
    
    fn create_requirements_txt(&self) -> Result<()> {
        let requirements = r#"mcp[cli]>=1.2.0
httpx>=0.24.0
"#;
        
        fs::write(
            self.project_path.join("requirements.txt"),
            requirements,
        ).context("Failed to create requirements.txt")?;
        
        Ok(())
    }
    
    fn create_gitignore(&self) -> Result<()> {
        let gitignore = r#"# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*$py.class

# C extensions
*.so

# Distribution / packaging
dist/
build/
*.egg-info/
.env

# Virtual environments
.venv/
venv/
ENV/

# Unit test / coverage reports
htmlcov/
.tox/
.nox/
.coverage
.coverage.*
.cache
nosetests.xml
coverage.xml
*.cover
.hypothesis/
.pytest_cache/

# Jupyter Notebook
.ipynb_checkpoints

# Environment variables
.env
.env.*

# IDE
.idea/
.vscode/
*.swp
*.swo
.DS_Store

# MCP specific
*.log
mcp_debug_output/
claude_config_backup.json

# Python development
.python-version
.mypy_cache/
.pytest_cache/
.ruff_cache/

# Local development
local_test/
temp/
notes/
"#;
        
        fs::write(
            self.project_path.join(".gitignore"),
            gitignore,
        ).context("Failed to create .gitignore")?;
        
        Ok(())
    }
    
    fn create_server_file(&self) -> Result<()> {
        let server_code = r#"#!/usr/bin/env python3
from typing import Any
import httpx
import sys
import json
from mcp.server.fastmcp import FastMCP

# Initialize FastMCP server
mcp = FastMCP("weather")

# Constants
NWS_API_BASE = "https://api.weather.gov"
USER_AGENT = "weather-app/1.0"

async def make_nws_request(url: str) -> dict[str, Any] | None:
    """Make a request to the NWS API with proper error handling."""
    headers = {
        "User-Agent": USER_AGENT,
        "Accept": "application/geo+json"
    }
    async with httpx.AsyncClient() as client:
        try:
            response = await client.get(url, headers=headers, timeout=30.0)
            response.raise_for_status()
            return response.json()
        except Exception as e:
            print(f"Error making request to {url}: {e}", file=sys.stderr)
            return None

def format_alert(feature: dict) -> str:
    """Format an alert feature into a readable string."""
    props = feature["properties"]
    return f"""
Event: {props.get('event', 'Unknown')}
Area: {props.get('areaDesc', 'Unknown')}
Severity: {props.get('severity', 'Unknown')}
Description: {props.get('description', 'No description available')}
Instructions: {props.get('instruction', 'No specific instructions provided')}
"""

@mcp.tool()
async def get_alerts(state: str) -> str:
    """Get weather alerts for a US state.

    Args:
        state: Two-letter US state code (e.g. CA, NY)
    """
    url = f"{NWS_API_BASE}/alerts/active/area/{state}"
    data = await make_nws_request(url)

    if not data or "features" not in data:
        return "Unable to fetch alerts or no alerts found."

    if not data["features"]:
        return "No active alerts for this state."

    alerts = [format_alert(feature) for feature in data["features"]]
    return "\n---\n".join(alerts)

@mcp.tool()
async def get_forecast(latitude: float, longitude: float) -> str:
    """Get weather forecast for a location.

    Args:
        latitude: Latitude of the location
        longitude: Longitude of the location
    """
    # First get the forecast grid endpoint
    points_url = f"{NWS_API_BASE}/points/{latitude},{longitude}"
    points_data = await make_nws_request(points_url)

    if not points_data:
        return "Unable to fetch forecast data for this location."

    # Get the forecast URL from the points response
    forecast_url = points_data["properties"]["forecast"]
    forecast_data = await make_nws_request(forecast_url)

    if not forecast_data:
        return "Unable to fetch detailed forecast."

    # Format the periods into a readable forecast
    periods = forecast_data["properties"]["periods"]
    forecasts = []
    for period in periods[:5]:  # Only show next 5 periods
        forecast = f"""
{period['name']}:
Temperature: {period['temperature']}°{period['temperatureUnit']}
Wind: {period['windSpeed']} {period['windDirection']}
Forecast: {period['detailedForecast']}
"""
        forecasts.append(forecast)

    return "\n---\n".join(forecasts)

async def test_mode():
    """Run in test mode to see if the API works without Claude."""
    print("🧪 Running in test mode to verify functionality")
    print("Test 1: Getting weather alerts for CA")
    alerts = await get_alerts("CA")
    print(alerts)
    
    print("\nTest 2: Getting forecast for New York City (40.7128, -74.0060)")
    forecast = await get_forecast(40.7128, -74.0060)
    print(forecast)
    
    print("\n✅ Tests completed. If you see weather data above, the server is working correctly.")
    print("To use with Claude for Desktop, follow the instructions in README.md")

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--test":
        # Run in test mode
        import asyncio
        asyncio.run(test_mode())
    else:
        # Normal MCP server mode
        print("Starting MCP server in stdio mode...")
        print("⚠️  Note: The server will appear to hang, waiting for MCP protocol messages.")
        print("⚠️  This is normal. Use Ctrl+C to exit.")
        print("💡 To test functionality without Claude, run: python server.py --test")
        mcp.run(transport='stdio')
"#;
        
        let file_path = self.project_path.join("server.py");
        fs::write(&file_path, server_code)
            .context("Failed to create server.py")?;
        
        // Make the file executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path)?.permissions();
            perms.set_mode(0o755);  // rwxr-xr-x
            fs::set_permissions(&file_path, perms)
                .context("Failed to make server.py executable")?;
        }
        
        Ok(())
    }
    
    fn create_readme(&self) -> Result<()> {
        let readme = format!(r#"# {}

A Model Context Protocol (MCP) server implementation in Python.

## About

This project implements an MCP server that provides weather information via the National Weather Service API. It demonstrates how to create a server that can be used with MCP compatible clients like Claude for Desktop.

## Getting Started

### Prerequisites

- Python 3.10 or newer
- uv (Python package manager)

### Installation

```bash
# Create and activate virtual environment
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
uv pip install -r requirements.txt
```

### Testing the Server

To test the server functionality without Claude for Desktop:

```bash
python server.py --test
```

This will run the server in test mode and display weather alerts for California and a forecast for New York City.

### Running the Server

**Important Note:** When running in normal mode, this server is designed to be used with Claude for Desktop or other MCP clients. 
When you run `python server.py` directly, it will appear to hang because it's waiting for MCP protocol 
messages via stdin. This is expected behavior - you should not run it in this mode for interactive use.

### Integration with Claude for Desktop

To integrate with Claude for Desktop, you'll need to configure the MCP server in Claude's configuration file. 

Open `~/Library/Application Support/Claude/claude_desktop_config.json` (create it if it doesn't exist) and add:

```json
{{
  "mcpServers": {{
    "weather": {{
      "command": "uv",
      "args": [
        "--directory",
        "/ABSOLUTE/PATH/TO/{}",
        "run",
        "server.py"
      ]
    }}
  }}
}}
```

Replace `/ABSOLUTE/PATH/TO/{}` with the absolute path to your project directory.

Once configured, restart Claude for Desktop, and you should see the weather tools appear in the tools menu.

## Available Tools

This MCP server provides the following tools:

- **get_alerts**: Get active weather alerts for a US state
  - Parameters: `state` (two-letter state code)

- **get_forecast**: Get weather forecast for a location
  - Parameters: `latitude`, `longitude`

## Example Queries for Claude

After connecting your server to Claude for Desktop, you can ask questions like:

- "What are the active weather alerts in California?"
- "What's the weather forecast for New York? (coordinates: 40.7128, -74.0060)"

## Troubleshooting

- **Server appears to hang in normal mode**: This is normal. The server is waiting for MCP protocol messages on stdin.
- **No tools appear in Claude**: Make sure the paths in `claude_desktop_config.json` are correct and absolute. Restart Claude for Desktop.
- **Error in Claude's logs**: Check `~/Library/Logs/Claude/mcp*.log` for errors.
- **API errors**: If you're getting errors with the weather API, try using test mode to see detailed error messages.

## License

MIT
"#, self.project_name, self.project_name, self.project_name);
        
        fs::write(
            self.project_path.join("README.md"),
            readme,
        ).context("Failed to create README.md")?;
        
        Ok(())
    }
} 