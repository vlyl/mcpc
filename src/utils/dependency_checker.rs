use anyhow::Result;
use which::which;

use crate::{Language, Tool};

pub struct Dependency {
    pub name: String,
    pub install_instructions: Option<String>,
}

/// Check if all required dependencies are installed based on the language and tool
pub fn check_dependencies(language: &Language, tool: &Tool) -> Result<(), Vec<Dependency>> {
    let mut missing_deps = Vec::new();
    
    // Check Git
    if which("git").is_err() {
        missing_deps.push(Dependency {
            name: "Git".to_string(),
            install_instructions: Some("https://git-scm.com/downloads".to_string()),
        });
    }
    
    match language {
        Language::Python | Language::Py => {
            // Check Python
            if which("python").is_err() && which("python3").is_err() {
                missing_deps.push(Dependency {
                    name: "Python 3.10+".to_string(),
                    install_instructions: Some("https://www.python.org/downloads/".to_string()),
                });
            }
            
            // Check UV if tool is UV
            if matches!(tool, Tool::Uv) && which("uv").is_err() {
                missing_deps.push(Dependency {
                    name: "uv".to_string(),
                    install_instructions: Some("pip install uv".to_string()),
                });
            }
        },
        Language::Typescript | Language::Ts => {
            // Check Node.js
            if which("node").is_err() {
                missing_deps.push(Dependency {
                    name: "Node.js 18+".to_string(),
                    install_instructions: Some("https://nodejs.org/".to_string()),
                });
            }
            
            // Check package manager
            match tool {
                Tool::Pnpm => {
                    if which("pnpm").is_err() {
                        missing_deps.push(Dependency {
                            name: "pnpm".to_string(),
                            install_instructions: Some("npm install -g pnpm".to_string()),
                        });
                    }
                },
                Tool::Yarn => {
                    if which("yarn").is_err() {
                        missing_deps.push(Dependency {
                            name: "yarn".to_string(),
                            install_instructions: Some("npm install -g yarn".to_string()),
                        });
                    }
                },
                Tool::Npm => {
                    if which("npm").is_err() {
                        missing_deps.push(Dependency {
                            name: "npm".to_string(),
                            install_instructions: Some("It comes with Node.js, please install Node.js".to_string()),
                        });
                    }
                },
                _ => {},
            }
        },
    }
    
    if missing_deps.is_empty() {
        Ok(())
    } else {
        Err(missing_deps)
    }
} 