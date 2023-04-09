pub trait IHostEnvironment {
    fn get_app_name(self: &Self) -> String;
    fn get_content_root_path(self: &Self) -> String;
    fn get_environment_name(self: &Self) -> String;
}

struct IHostEnvironmentExtensions {}

impl IHostEnvironmentExtensions {
    pub fn is_development(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Development")
    }

    pub fn is_environment(env: &dyn IHostEnvironment, env_name: &str) -> bool {
        env.get_environment_name() == env_name
    }

    pub fn is_production(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Production")
    }

    pub fn is_staging(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Staging")
    }
}