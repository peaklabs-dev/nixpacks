use crate::nixpacks::{
    app::App,
    environment::{Environment, EnvironmentVariables},
    nix::NixConfig,
    phase::SetupPhase,
};
use anyhow::Result;

// pub mod deno;
// pub mod go;
// pub mod npm;
pub mod rust;
// pub mod yarn;

pub trait Provider {
    fn name(&self) -> &str;
    fn detect(&self, app: &App, _env: &Environment) -> Result<bool>;
    fn setup(&self, _app: &App, _env: &Environment) -> Result<Option<SetupPhase>> {
        Ok(None)
    }

    // fn nix_config(&self, app: &App, _env: &Environment) -> Result<NixConfig>;
    fn install_cmd(&self, _app: &App, _env: &Environment) -> Result<Option<String>> {
        Ok(None)
    }
    fn suggested_build_cmd(&self, _app: &App, _env: &Environment) -> Result<Option<String>> {
        Ok(None)
    }
    fn suggested_start_command(&self, _app: &App, _env: &Environment) -> Result<Option<String>> {
        Ok(None)
    }
    fn get_environment_variables(
        &self,
        _app: &App,
        _env: &Environment,
    ) -> Result<EnvironmentVariables> {
        Ok(EnvironmentVariables::default())
    }
}
