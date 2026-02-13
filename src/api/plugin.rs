use std::any::Any;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Arc;
use crate::api::error::CADError;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn author(&self) -> &str;
    fn description(&self) -> &str;
    fn dependencies(&self) -> Vec<&str>;
    fn initialize(&mut self, context: &dyn PluginContext) -> Result<(), CADError>;
    fn execute(&self, command: &str, args: &[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError>;
    fn shutdown(&self);
    fn get_commands(&self) -> Vec<PluginCommand>;
    fn get_menus(&self) -> Vec<PluginMenu>;
    fn get_toolbars(&self) -> Vec<PluginToolbar>;
}

pub trait PluginContext: Send + Sync {
    fn register_command(&self, command: PluginCommand) -> Result<(), CADError>;
    fn unregister_command(&self, name: &str) -> Result<(), CADError>;
    fn invoke_command(&self, name: &str, args: &[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError>;
    fn get_document(&self) -> Option<&crate::data_structure::Document>;
    fn get_selection_manager(&self) -> Option<&crate::selection::SelectionManager>;
    fn emit_event(&self, event: &dyn Any);
    fn log_message(&self, message: &str, level: LogLevel);
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub category: String,
    pub handler: Arc<dyn Fn(&[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError> + Send + Sync>,
    pub parameters: Vec<CommandParameter>,
}

impl PluginCommand {
    pub fn new(
        name: &str,
        description: &str,
        handler: impl Fn(&[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            icon: None,
            category: "General".to_string(),
            handler: Arc::new(handler),
            parameters: Vec::new(),
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn with_parameters(mut self, parameters: Vec<CommandParameter>) -> Self {
        self.parameters = parameters;
        self
    }
}

#[derive(Clone)]
pub struct CommandParameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub description: String,
    pub is_required: bool,
}

#[derive(Debug, Clone)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Point,
    Line,
    Circle,
    Arc,
    Ellipse,
    Polyline,
    Entity,
    Layer,
    Block,
    Selection,
    FilePath,
    Color,
    Enum(Vec<String>),
}

#[derive(Clone)]
pub struct PluginMenu {
    pub name: String,
    pub items: Vec<MenuItem>,
    pub position: MenuPosition,
}

#[derive(Clone)]
pub enum MenuItem {
    Command {
        name: String,
        command: String,
        icon: Option<String>,
        shortcut: Option<String>,
    },
    SubMenu {
        name: String,
        items: Vec<MenuItem>,
    },
    Separator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuPosition {
    Top,
    Bottom,
    Context,
}

#[derive(Clone)]
pub struct PluginToolbar {
    pub name: String,
    pub items: Vec<ToolbarItem>,
    pub position: ToolbarPosition,
}

#[derive(Clone)]
pub enum ToolbarItem {
    Command {
        name: String,
        command: String,
        icon: String,
        tooltip: String,
    },
    Separator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolbarPosition {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub path: std::path::PathBuf,
    pub is_loaded: bool,
    pub commands: Vec<PluginCommand>,
    pub menus: Vec<PluginMenu>,
    pub toolbars: Vec<PluginToolbar>,
}

impl PluginInfo {
    pub fn new(name: &str, version: &str, author: &str, description: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            description: description.to_string(),
            path: path.to_path_buf(),
            is_loaded: false,
            commands: Vec::new(),
            menus: Vec::new(),
            toolbars: Vec::new(),
        }
    }
}

pub trait PluginManager: Send + Sync {
    fn load_plugin(&mut self, path: &Path) -> Result<PluginInfo, CADError>;
    fn unload_plugin(&mut self, name: &str) -> Result<(), CADError>;
    fn reload_plugin(&mut self, name: &str) -> Result<PluginInfo, CADError>;
    fn get_plugin(&self, name: &str) -> Option<&dyn Plugin>;
    fn get_plugin_info(&self, name: &str) -> Option<&PluginInfo>;
    fn list_plugins(&self) -> Vec<&str>;
    fn list_commands(&self) -> Vec<&PluginCommand>;
    fn find_command(&self, name: &str) -> Option<&PluginCommand>;
    fn execute_command(&self, name: &str, args: &[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError>;
    fn register_hooks(&self, hook_type: HookType, hook: HookFunction) -> Result<(), CADError>;
    fn unregister_hooks(&self, hook_type: HookType);
}

pub type HookFunction = Arc<dyn Fn(&dyn Any) + Send + Sync>;

#[derive(Debug, Clone, PartialEq)]
pub enum HookType {
    PreCommand,
    PostCommand,
    DocumentChanged,
    SelectionChanged,
    LayerChanged,
    ViewChanged,
    EntityCreated,
    EntityModified,
    EntityDeleted,
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_infos: HashMap<String, PluginInfo>,
    commands: HashMap<String, PluginCommand>,
    hooks: HashMap<HookType, Vec<HookFunction>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_infos: HashMap::new(),
            commands: HashMap::new(),
            hooks: HashMap::new(),
        }
    }

    pub fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> Result<(), CADError> {
        let name = plugin.name().to_string();
        if self.plugins.contains_key(&name) {
            return Err(CADError::PluginError(format!("Plugin '{}' already registered", name)));
        }

        self.plugins.insert(name.clone(), Box::new(plugin));
        Ok(())
    }

    pub fn unregister_plugin(&mut self, name: &str) -> Result<(), CADError> {
        self.plugins.remove(name);
        self.commands.retain(|_, cmd| !cmd.name.starts_with(&format!("{}.", name)));
        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    pub fn register_command(&mut self, command: PluginCommand) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn find_command(&self, name: &str) -> Option<&PluginCommand> {
        self.commands.get(name)
    }

    pub fn list_commands(&self) -> Vec<&PluginCommand> {
        self.commands.values().collect()
    }

    pub fn execute_command(&self, name: &str, args: &[Box<dyn Any>]) -> Result<Box<dyn Any>, CADError> {
        if let Some(command) = self.commands.get(name) {
            (command.handler)(args)
        } else {
            Err(CADError::PluginError(format!("Command '{}' not found", name)))
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PluginLoader {
    search_paths: Vec<std::path::PathBuf>,
    registry: PluginRegistry,
}

impl PluginLoader {
    pub fn new() -> Self {
        let mut search_paths = Vec::new();
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                search_paths.push(parent.to_path_buf());
                search_paths.push(parent.join("plugins").to_path_buf());
            }
        }
        
        #[cfg(unix)]
        {
            search_paths.push(std::path::PathBuf::from("/usr/local/lib/cad/plugins"));
            search_paths.push(std::path::PathBuf::from(format!("{}/.cad/plugins", std::env::home_dir().unwrap_or_default().display())));
        }
        
        #[cfg(windows)]
        {
            if let Ok(app_data) = std::env::var("APPDATA") {
                search_paths.push(std::path::PathBuf::from(format!("{}\\CAD\\plugins", app_data)));
            }
        }

        Self {
            search_paths,
            registry: PluginRegistry::new(),
        }
    }

    pub fn add_search_path(&mut self, path: &Path) {
        self.search_paths.push(path.to_path_buf());
    }

    pub fn discover_plugins(&self) -> Vec<std::path::PathBuf> {
        let mut plugins = Vec::new();
        
        for path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if self.is_plugin_file(&path) {
                        plugins.push(path);
                    }
                }
            }
        }
        
        plugins
    }

    fn is_plugin_file(&self, path: &Path) -> bool {
        #[cfg(target_os = "windows")]
        {
            path.extension().and_then(|e| e.to_str()) == Some("dll")
        }
        #[cfg(target_os = "macos")]
        {
            path.extension().and_then(|e| e.to_str()) == Some("dylib") || path.file_name().map(|n| n.to_string_lossy().starts_with("lib")) == Some(true)
        }
        #[cfg(target_os = "linux")]
        {
            path.extension().and_then(|e| e.to_str()) == Some("so")
        }
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
