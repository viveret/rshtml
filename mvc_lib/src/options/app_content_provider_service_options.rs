
#[derive(Clone)]
pub enum SpecialPathName {
    CWD,
    EXE,
    CargoPath,
}

#[derive(Clone)]
pub struct AppContentProviderServiceOptions {
    pub use_cwd: bool,
    pub use_exe_path: bool,
    pub use_cargo_path: bool,
    pub use_default_path_order: bool,
    pub path_order: Vec<SpecialPathName>,
}

impl AppContentProviderServiceOptions {
    pub const SPECIAL_PATH_NAMES: [SpecialPathName; 3] = [SpecialPathName::CargoPath, SpecialPathName::CWD, SpecialPathName::EXE];

    pub fn new() -> Self {
        Self {
            use_cwd: true,
            use_exe_path: true,
            use_cargo_path: true,
            use_default_path_order: false,
            path_order: Self::SPECIAL_PATH_NAMES.to_vec(),
        }
    }
}

