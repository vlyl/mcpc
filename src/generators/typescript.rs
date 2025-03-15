use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::Tool;
use super::Generator;

pub struct TypeScriptGenerator {
    project_name: String,
    tool: Tool,
    project_path: PathBuf,
}

impl Generator for TypeScriptGenerator {
    fn new(project_name: &str, tool: &Tool) -> Self {
        let project_path = PathBuf::from(project_name);
        
        Self {
            project_name: project_name.to_string(),
            tool: tool.clone(),
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
        
        // Create subdirectories (simplified to match MCP examples)
        let dirs = vec![
            "src",
            "build",
        ];
        
        for dir in dirs {
            fs::create_dir_all(self.project_path.join(dir))
                .context(format!("Failed to create directory: {}", dir))?;
        }
        
        Ok(())
    }
    
    fn create_files(&self) -> Result<()> {
        // Create package.json
        self.create_package_json()?;
        
        // Create tsconfig.json
        self.create_tsconfig_json()?;
        
        // Create .gitignore
        self.create_gitignore()?;
        
        // Create .prettierrc and .prettierignore
        self.create_prettier_config()?;
        
        // Create main MCP server file
        self.create_server_file()?;
        
        // Create README
        self.create_readme()?;
        
        Ok(())
    }
    
    fn init_package_manager(&self) -> Result<()> {
        // Get package manager command
        let cmd = match self.tool {
            Tool::Pnpm => "pnpm",
            Tool::Yarn => "yarn",
            Tool::Npm => "npm",
            _ => "npm",
        };
        
        println!("📦 Installing dependencies with {}...", cmd);
        
        // Install runtime dependencies
        println!("Installing runtime dependencies...");
        let runtime_deps_result = match self.tool {
            Tool::Yarn => {
                Command::new(cmd)
                    .args(["add", "@modelcontextprotocol/sdk", "zod"])
                    .current_dir(&self.project_path)
                    .output()
            },
            _ => {
                Command::new(cmd)
                    .args(["install", "@modelcontextprotocol/sdk", "zod"])
                    .current_dir(&self.project_path)
                    .output()
            }
        };
        
        if let Err(e) = &runtime_deps_result {
            eprintln!("⚠️ Warning: Failed to install runtime dependencies: {}", e);
            eprintln!("Please run '{} install @modelcontextprotocol/sdk zod' manually", cmd);
        }
        
        // Install development dependencies
        println!("Installing development dependencies...");
        let dev_deps_result = match self.tool {
            Tool::Yarn => {
                Command::new(cmd)
                    .args(["add", "--dev", "@types/node", "typescript"])
                    .current_dir(&self.project_path)
                    .output()
            },
            Tool::Pnpm => {
                Command::new(cmd)
                    .args(["install", "-D", "@types/node", "typescript"])
                    .current_dir(&self.project_path)
                    .output()
            },
            _ => {
                Command::new(cmd)
                    .args(["install", "--save-dev", "@types/node", "typescript"])
                    .current_dir(&self.project_path)
                    .output()
            }
        };
        
        if let Err(e) = &dev_deps_result {
            eprintln!("⚠️ Warning: Failed to install development dependencies: {}", e);
            eprintln!("Please run '{} install --save-dev @types/node typescript' manually", cmd);
        }
        
        if runtime_deps_result.is_ok() && dev_deps_result.is_ok() {
            println!("✅ Dependencies installed successfully");
        } else {
            eprintln!("⚠️ Some dependencies may not have been installed properly.");
            eprintln!("Please check the output above and install any missing dependencies manually.");
        }
        
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

impl TypeScriptGenerator {
    fn create_package_json(&self) -> Result<()> {
        let _package_manager = match self.tool {
            Tool::Pnpm => "pnpm",
            Tool::Yarn => "yarn",
            Tool::Npm => "npm",
            _ => "npm", // Default fallback
        };
        
        let package_json = format!(
            r#"{{
  "name": "{}",
  "version": "0.1.0",
  "description": "MCP (Model Context Protocol) server",
  "type": "module",
  "main": "build/index.js",
  "bin": {{
    "{}": "./build/index.js"
  }},
  "scripts": {{
    "start": "node build/index.js",
    "dev": "nodemon --exec node --loader ts-node/esm src/index.ts",
    "build": "tsc && chmod +x build/index.js"
  }},
  "dependencies": {{
    "@modelcontextprotocol/sdk": "^1.0.0",
    "zod": "^3.22.4"
  }},
  "devDependencies": {{
    "@types/node": "^20.10.0",
    "nodemon": "^3.0.2",
    "ts-node": "^10.9.2",
    "typescript": "^5.3.2"
  }},
  "engines": {{
    "node": ">=16.0.0"
  }}
}}"#,
            self.project_name,
            self.project_name
        );
        
        fs::write(
            self.project_path.join("package.json"),
            package_json,
        ).context("Failed to create package.json")?;
        
        Ok(())
    }
    
    fn create_tsconfig_json(&self) -> Result<()> {
        let tsconfig_json = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "Node16",
    "moduleResolution": "Node16",
    "outDir": "./build",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules"]
}"#;
        
        fs::write(
            self.project_path.join("tsconfig.json"),
            tsconfig_json,
        ).context("Failed to create tsconfig.json")?;
        
        Ok(())
    }
    
    fn create_gitignore(&self) -> Result<()> {
        let gitignore = r#"# Dependencies
node_modules/
.pnp
.pnp.js
.yarn/install-state.gz

# Build outputs
build/
dist/
out/
.next/
.nuxt/
.vuepress/dist

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# Logs
logs/
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# Testing
coverage/
.nyc_output/

# Editor directories and files
.idea/
.vscode/
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
.DS_Store

# TypeScript specific
*.tsbuildinfo
.tscache/

# MCP specific
mcp_debug_output/
claude_config_backup.json

# Local development
local_test/
temp/
notes/
.turbo
.vercel
.cache/
"#;
        
        fs::write(
            self.project_path.join(".gitignore"),
            gitignore,
        ).context("Failed to create .gitignore")?;
        
        Ok(())
    }
    
    fn create_prettier_config(&self) -> Result<()> {
        // Create .prettierrc
        let prettierrc = r#"{
  "semi": true,
  "trailingComma": "all",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2
}"#;
        
        fs::write(
            self.project_path.join(".prettierrc"),
            prettierrc,
        ).context("Failed to create .prettierrc")?;
        
        // Create .prettierignore
        let prettierignore = r#"node_modules/
dist/
build/
coverage/
.next/
"#;
        
        fs::write(
            self.project_path.join(".prettierignore"),
            prettierignore,
        ).context("Failed to create .prettierignore")?;
        
        Ok(())
    }
    
    fn create_server_file(&self) -> Result<()> {
        let server_code = r#"#!/usr/bin/env node
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";

const NWS_API_BASE = "https://api.weather.gov";
const USER_AGENT = "weather-app/1.0";

// Create server instance
const server = new McpServer({
  name: "weather",
  version: "1.0.0",
});

// Helper function for making NWS API requests
async function makeNWSRequest<T>(url: string): Promise<T | null> {
  const headers = {
    "User-Agent": USER_AGENT,
    Accept: "application/geo+json",
  };

  try {
    const response = await fetch(url, { headers });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return (await response.json()) as T;
  } catch (error) {
    console.error("Error making NWS request:", error);
    return null;
  }
}

interface AlertFeature {
  properties: {
    event?: string;
    areaDesc?: string;
    severity?: string;
    status?: string;
    headline?: string;
    description?: string;
    instruction?: string;
  };
}

// Format alert data
function formatAlert(feature: AlertFeature): string {
  const props = feature.properties;
  return [
    `Event: ${props.event || "Unknown"}`,
    `Area: ${props.areaDesc || "Unknown"}`,
    `Severity: ${props.severity || "Unknown"}`,
    `Description: ${props.description?.substring(0, 200) || "No description"}...`,
    `Instructions: ${props.instruction || "No specific instructions provided"}`,
    "---",
  ].join("\n");
}

interface ForecastPeriod {
  name?: string;
  temperature?: number;
  temperatureUnit?: string;
  windSpeed?: string;
  windDirection?: string;
  shortForecast?: string;
  detailedForecast?: string;
}

interface AlertsResponse {
  features: AlertFeature[];
}

interface PointsResponse {
  properties: {
    forecast?: string;
  };
}

interface ForecastResponse {
  properties: {
    periods: ForecastPeriod[];
  };
}

// Register weather tools
server.tool(
  "get-alerts",
  "Get weather alerts for a state",
  {
    state: z.string().length(2).describe("Two-letter state code (e.g. CA, NY)"),
  },
  async ({ state }) => {
    const stateCode = state.toUpperCase();
    const alertsUrl = `${NWS_API_BASE}/alerts/active/area/${stateCode}`;
    const alertsData = await makeNWSRequest<AlertsResponse>(alertsUrl);

    if (!alertsData) {
      return {
        content: [
          {
            type: "text",
            text: "Failed to retrieve alerts data",
          },
        ],
      };
    }

    const features = alertsData.features || [];
    if (features.length === 0) {
      return {
        content: [
          {
            type: "text",
            text: `No active alerts for ${stateCode}`,
          },
        ],
      };
    }

    const formattedAlerts = features.map(formatAlert);
    const alertsText = `Active alerts for ${stateCode}:\n\n${formattedAlerts.join("\n")}`;

    return {
      content: [
        {
          type: "text",
          text: alertsText,
        },
      ],
    };
  },
);

server.tool(
  "get-forecast",
  "Get weather forecast for a location",
  {
    latitude: z.number().min(-90).max(90).describe("Latitude of the location"),
    longitude: z.number().min(-180).max(180).describe("Longitude of the location"),
  },
  async ({ latitude, longitude }) => {
    // Get grid point data
    const pointsUrl = `${NWS_API_BASE}/points/${latitude.toFixed(4)},${longitude.toFixed(4)}`;
    const pointsData = await makeNWSRequest<PointsResponse>(pointsUrl);

    if (!pointsData) {
      return {
        content: [
          {
            type: "text",
            text: `Failed to retrieve grid point data for coordinates: ${latitude}, ${longitude}. This location may not be supported by the NWS API (only US locations are supported).`,
          },
        ],
      };
    }

    const forecastUrl = pointsData.properties?.forecast;
    if (!forecastUrl) {
      return {
        content: [
          {
            type: "text",
            text: "Failed to get forecast URL from grid point data",
          },
        ],
      };
    }

    // Get forecast data
    const forecastData = await makeNWSRequest<ForecastResponse>(forecastUrl);
    if (!forecastData) {
      return {
        content: [
          {
            type: "text",
            text: "Failed to retrieve forecast data",
          },
        ],
      };
    }

    const periods = forecastData.properties?.periods || [];
    if (periods.length === 0) {
      return {
        content: [
          {
            type: "text",
            text: "No forecast periods available",
          },
        ],
      };
    }

    // Format forecast periods
    const formattedForecast = periods.slice(0, 5).map((period: ForecastPeriod) =>
      [
        `${period.name || "Unknown"}:`,
        `Temperature: ${period.temperature || "Unknown"}°${period.temperatureUnit || "F"}`,
        `Wind: ${period.windSpeed || "Unknown"} ${period.windDirection || ""}`,
        `Forecast: ${period.detailedForecast || "No forecast available"}`,
        "---",
      ].join("\n"),
    );

    const forecastText = `Forecast for ${latitude}, ${longitude}:\n\n${formattedForecast.join("\n")}`;

    return {
      content: [
        {
          type: "text",
          text: forecastText,
        },
      ],
    };
  },
);

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("Weather MCP Server running on stdio");
}

main().catch((error) => {
  console.error("Fatal error in main():", error);
  process.exit(1);
});
"#;
        
        fs::write(
            self.project_path.join("src/index.ts"),
            server_code,
        ).context("Failed to create src/index.ts")?;
        
        Ok(())
    }
    
    fn create_readme(&self) -> Result<()> {
        let package_manager = match self.tool {
            Tool::Pnpm => "pnpm",
            Tool::Yarn => "yarn",
            Tool::Npm => "npm",
            _ => "npm",
        };
        
        let readme = format!(r#"# {}

A Model Context Protocol (MCP) server implementation.

## About

This project implements an MCP server that provides weather information via the National Weather Service API. It demonstrates how to create a server that can be used with MCP compatible clients like Claude for Desktop.

## Getting Started

### Prerequisites

- Node.js 16 or newer
- {} (package manager)

### Installation

```bash
# Install dependencies
{} install
```

### Building the Server

```bash
# Build the TypeScript code
{} run build
```

### Running the Server

For development:
```bash
# Run in development mode with hot reload
{} run dev
```

For Claude for Desktop integration, you'll need to add the server to your Claude configuration. Open `~/Library/Application Support/Claude/claude_desktop_config.json` and add:

```json
{{
  "mcpServers": {{
    "weather": {{
      "command": "node",
      "args": [
        "/ABSOLUTE/PATH/TO/{}/build/index.js"
      ]
    }}
  }}
}}
```

Replace `/ABSOLUTE/PATH/TO/{}` with the absolute path to your project.

## Available Tools

This MCP server provides the following tools:

- **get-alerts**: Get active weather alerts for a US state
  - Parameters: `state` (two-letter state code)

- **get-forecast**: Get weather forecast for a location
  - Parameters: `latitude`, `longitude`

## Example Queries for Claude

After connecting your server to Claude for Desktop, you can ask questions like:

- "What's the weather in Sacramento?"
- "What are the active weather alerts in California?"
- "Tell me the forecast for New York (40.7128, -74.0060)"

## License

MIT
"#,
            self.project_name,
            package_manager,
            package_manager,
            package_manager,
            package_manager,
            self.project_name,
            self.project_name
        );
        
        fs::write(
            self.project_path.join("README.md"),
            readme,
        ).context("Failed to create README.md")?;
        
        Ok(())
    }
} 