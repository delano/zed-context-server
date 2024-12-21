use serde::Deserialize;
use std::path::Path;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const SERVER_PACKAGE: &str = "mcp-server-git";

#[derive(Debug)]
struct ZedModelContextExtension;

#[derive(Debug, Deserialize)]
struct GitContextServerSettings {
    repository: String,
}

impl GitContextServerSettings {
    fn validate(&self) -> Result<()> {
        let repo_path = Path::new(&self.repository);
        env_logger::init();

        log::info!("Checking repository path permissions");

        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(metadata) = std::fs::metadata(repo_path) {
                let mode = metadata.mode();
                log::info!("Permissions: {:o}", mode);
            }
        }
        log::info!("Checking repository path: {:?}", repo_path);
        log::info!("Path exists? {}", repo_path.exists());
        log::info!("Is absolute? {}", repo_path.is_absolute());
        log::info!(
            "Current dir: {:?}",
            std::env::current_dir().unwrap_or_default()
        );

        if !repo_path.exists() {
            log::error!("Repository path does not exist3: {}", self.repository);
            return Err(format!("Repository path does not exist4: {}", self.repository).into());
        }

        if !repo_path.join(".git").exists() {
            log::error!("Not a git repository: {}", self.repository);
            return Err(format!("Not a git repository: {}", self.repository).into());
        }

        log::info!("Validated git repository: {}", self.repository);
        Ok(())
    }
}

impl zed::Extension for ZedModelContextExtension {
    fn new() -> Self {
        if let Ok(()) = env_logger::try_init() {
            log::info!("Git context server initialized");
        }
        ZedModelContextExtension
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        log::debug!(
            "Getting settings for context server: {:?}",
            context_server_id
        );

        let settings = ContextServerSettings::for_project("zed-context-server", project)?;

        let Some(settings) = settings.settings else {
            log::error!("Missing repository setting");
            return Err(
                "Missing repository setting. Please configure the repository path in settings.json"
                    .into(),
            );
        };

        let settings: GitContextServerSettings = serde_json::from_value(settings).map_err(|e| {
            log::error!("Failed to parse settings: {}", e);
            e.to_string()
        })?;

        settings.validate()?;

        // Find uvx in PATH
        let uvx_path = which::which("uvx").map_err(|e| {
            log::error!("Failed to find uvx in PATH: {}", e);
            "uvx command not found. Please install it with: npm install -g uvx".to_string()
        })?;

        log::info!(
            "Starting git context server for repository: {}",
            settings.repository
        );

        Ok(Command {
            command: uvx_path.to_string_lossy().into_owned(),
            args: vec![
                SERVER_PACKAGE.to_string(),
                "--repository".to_string(),
                settings.repository,
            ],
            env: vec![
                ("MCP_DEBUG".to_string(), "1".to_string()),
                ("RUST_LOG".to_string(), "debug".to_string()),
            ],
        })
    }
}

zed::register_extension!(ZedModelContextExtension);
