

// this trait represents the host environment.
pub trait IHostEnvironment {
    // get the application name.
    fn get_app_name(self: &Self) -> String;
    // get the application content root path.
    fn get_content_root_path(self: &Self) -> String;
    // get the application environment name.
    fn get_environment_name(self: &Self) -> String;
}

// extension methods for IHostEnvironment.
struct IHostEnvironmentExtensions {}

impl IHostEnvironmentExtensions {
    // is_development returns true if the environment name is "Development".
    pub fn is_development(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Development")
    }

    // is_environment returns true if the environment name is the specified name.
    pub fn is_environment(env: &dyn IHostEnvironment, env_name: &str) -> bool {
        env.get_environment_name() == env_name
    }

    // is_production returns true if the environment name is "Production".
    pub fn is_production(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Production")
    }

    // is_staging returns true if the environment name is "Staging".
    pub fn is_staging(env: &dyn IHostEnvironment) -> bool {
        Self::is_environment(env, "Staging")
    }
}