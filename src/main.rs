use clap::Parser;
use std::path::PathBuf;
use std::process;
use colored::Colorize;

use mcpc::{
    Cli, 
    get_default_tool,
    generators::{Generator, python::PythonGenerator, typescript::TypeScriptGenerator},
    utils::dependency_checker::check_dependencies,
};

fn main() {
    let cli = Cli::parse();
    
    // Determine the default tool based on the selected language
    let tool = cli.tool.unwrap_or_else(|| get_default_tool(&cli.language));
    
    // Check for required dependencies
    if let Err(missing_deps) = check_dependencies(&cli.language, &tool) {
        eprintln!("{}", "❌ Missing required dependencies:".red().bold());
        for dep in missing_deps {
            eprintln!("  - {}", dep.name.yellow());
            if let Some(install_instructions) = dep.install_instructions {
                eprintln!("    {}: {}", "Install with".blue(), install_instructions.green());
            }
        }
        process::exit(1);
    }
    
    // Create the project directory
    let project_path = PathBuf::from(&cli.project_name);
    if project_path.exists() {
        eprintln!("{} Directory '{}' already exists. Please choose another project name.", 
            "❌".red().bold(), 
            cli.project_name.yellow());
        process::exit(1);
    }
    
    // Generate the project
    let result = match cli.language {
        mcpc::Language::Python | mcpc::Language::Py => {
            let generator = PythonGenerator::new(&cli.project_name, &tool);
            generator.generate()
        },
        mcpc::Language::Typescript | mcpc::Language::Ts => {
            let generator = TypeScriptGenerator::new(&cli.project_name, &tool);
            generator.generate()
        },
    };
    
    match result {
        Ok(_) => {
            println!("{} Successfully created MCP server project: {}", 
                "✅".green().bold(), 
                cli.project_name.green().bold());
            println!("{} Project location: {}", 
                "📁".blue().bold(), 
                project_path.display().to_string().blue());
            println!("{} Next steps:", "🚀".yellow().bold());
            println!("  cd {}", cli.project_name);
            
            match cli.language {
                mcpc::Language::Python | mcpc::Language::Py => {
                    println!("  {}", "# Activate virtual environment".dimmed());
                    println!("  source .venv/bin/activate  # On Windows: .venv\\Scripts\\activate");
                    println!("  {}", "# Install dependencies".dimmed());
                    println!("  uv pip install -r requirements.txt");
                    println!("  {}", "# Run the server".dimmed());
                    println!("  python server.py");
                },
                mcpc::Language::Typescript | mcpc::Language::Ts => {
                    println!("  {}", "# Install dependencies".dimmed());
                    match tool {
                        mcpc::Tool::Pnpm => println!("  pnpm install"),
                        mcpc::Tool::Yarn => println!("  yarn"),
                        mcpc::Tool::Npm => println!("  npm install"),
                        _ => {},
                    }
                    println!("  {}", "# Run the server".dimmed());
                    match tool {
                        mcpc::Tool::Pnpm => println!("  pnpm dev"),
                        mcpc::Tool::Yarn => println!("  yarn dev"),
                        mcpc::Tool::Npm => println!("  npm run dev"),
                        _ => {},
                    }
                },
            }
        },
        Err(e) => {
            eprintln!("{} Failed to create project: {}", "❌".red().bold(), e);
            process::exit(1);
        }
    }
}
