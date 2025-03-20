

pub enum SpecialPathName {
    CWD,
    EXE,
    CargoPath,
}

pub struct AppContentProviderServiceOptions {
    pub use_cwd: bool,
    pub use_exe_path: bool,
    pub use_cargo_path: bool,
    pub path_order: Vec<SpecialPathName>,
}

impl AppContentProviderServiceOptions {
    pub fn new() -> Self {
        Self {
            use_cwd: true,
            use_exe_path: true,
            use_cargo_path: true,
            path_order: vec![SpecialPathName::CargoPath, SpecialPathName::CWD, SpecialPathName::EXE],
        }
    }
}

