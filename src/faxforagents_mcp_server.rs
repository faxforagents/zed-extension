//! Zed context server extension for Fax for Agents, <https://faxforagents.com>.
//!
//! The extension installs the `faxforagents-mcp` npm package into Zed's extension work directory and
//! starts it over stdio with Zed's own Node binary. Nothing is bundled. The API key comes from the
//! `faxforagents-mcp-server` context server settings and is passed to the server process as the
//! `FAXFORAGENTS_API_KEY` environment variable, which the server sends to api.faxforagents.com only.

use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const CONTEXT_SERVER_ID: &str = "faxforagents-mcp-server";
const PACKAGE_NAME: &str = "faxforagents-mcp";
const PACKAGE_VERSION: &str = "0.3.1";
const SERVER_PATH: &str = "node_modules/faxforagents-mcp/dist/index.js";

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct FaxForAgentsSettings {
    /// Your Fax for Agents API key. `ffa_test_...` reaches the sandbox at no cost, `ffa_live_...`
    /// reaches the carrier and spends the prepaid wallet. Keys are created at https://app.faxforagents.com.
    api_key: Option<String>,
}

/// `value` unless it is missing or blank.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}

struct FaxForAgentsExtension;

impl zed::Extension for FaxForAgentsExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        if context_server_id.as_ref() != CONTEXT_SERVER_ID {
            return Err(format!(
                "unknown context server: {}",
                context_server_id.as_ref()
            ));
        }

        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let settings: FaxForAgentsSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|e| e.to_string())?,
            None => FaxForAgentsSettings::default(),
        };

        let server = env::current_dir()
            .map_err(|e| format!("failed to read the extension directory: {e}"))?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        // Without a key the server still starts and serves get_guide, the guide resources and the
        // prompts. Every other tool answers ok=false, error_code=invalid_token with how to get a key.
        let mut env = Vec::new();
        if let Some(key) = non_empty(settings.api_key) {
            env.push(("FAXFORAGENTS_API_KEY".to_string(), key));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        if context_server_id.as_ref() != CONTEXT_SERVER_ID {
            return Ok(None);
        }
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(FaxForAgentsSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(FaxForAgentsExtension);
