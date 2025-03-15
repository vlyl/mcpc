use clap::{Parser, ValueEnum};

pub mod generators;
pub mod utils;

/// Supported programming languages
#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Py,
    Python,
    Ts,
    Typescript,
}

/// Supported package manager tools
#[derive(Debug, Clone, ValueEnum)]
pub enum Tool {
    Uv,
    Pnpm,
    Yarn,
    Npm,
}

/// CLI arguments for the mcpc command
#[derive(Parser, Debug)]
#[command(name = "mcpc")]
#[command(about = "Generate MCP server project templates", long_about = None)]
pub struct Cli {
    /// Name of the project
    pub project_name: String,

    /// Programming language to use
    #[arg(short, long, value_enum, default_value = "typescript")]
    pub language: Language,

    /// Package manager tool to use
    #[arg(short, long, value_enum)]
    pub tool: Option<Tool>,
}

/// Get the default tool for a language
pub fn get_default_tool(language: &Language) -> Tool {
    match language {
        Language::Python | Language::Py => Tool::Uv,
        Language::Typescript | Language::Ts => Tool::Pnpm,
    }
} 