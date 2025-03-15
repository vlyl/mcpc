pub mod python;
pub mod typescript;

use anyhow::Result;
use crate::Tool;

/// Trait for project generators
pub trait Generator {
    /// Creates a new generator for the specified project
    fn new(project_name: &str, tool: &Tool) -> Self;
    
    /// Generates the project scaffold
    fn generate(&self) -> Result<()>;
    
    /// Initialize git repository
    fn init_git(&self) -> Result<()>;
    
    /// Create project directories
    fn create_directories(&self) -> Result<()>;
    
    /// Create project files
    fn create_files(&self) -> Result<()>;
    
    /// Initialize package manager
    fn init_package_manager(&self) -> Result<()>;
} 