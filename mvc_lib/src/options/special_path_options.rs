

#[derive(Clone, Debug)]
pub enum SpecialPathName {
    CWD,
    EXE,
    CargoPath,
}


#[derive(Clone, Debug)]
pub struct SpecialPathOptions {
    pub use_cwd: bool,
    pub use_exe_path: bool,
    pub use_cargo_path: bool,
    pub use_default_path_order: bool,
    pub path_order: Vec<SpecialPathName>,
}

impl SpecialPathOptions {
    pub const SPECIAL_PATH_NAMES: [SpecialPathName; 3] = [SpecialPathName::CargoPath, SpecialPathName::CWD, SpecialPathName::EXE];

    pub fn new() -> Self {
        Self {
            use_cwd: true,
            use_exe_path: true,
            use_cargo_path: true,
            use_default_path_order: true,
            path_order: vec![],
        }
    }

    pub fn get_path_order(&self) -> Vec<SpecialPathName> {
        if self.use_default_path_order {
            Self::SPECIAL_PATH_NAMES.to_vec()
        } else {
            self.path_order.to_vec()
        }
    }

    pub fn get_special_paths(&self) -> Vec<String> {
        let crate_root = std::env::var("CARGO_MANIFEST_DIR").ok();
        let exe_path = std::env::current_exe().ok().map(|x| x.to_str().map(|x| x.to_string())).flatten();
        let cwd_path = std::env::current_dir().ok().map(|x| x.to_str().map(|x| x.to_string())).flatten();
        
        let order = self.get_path_order();
        order.iter().filter(|x| 
            match x {
                SpecialPathName::CWD => self.use_cwd,
                SpecialPathName::EXE => self.use_exe_path,
                SpecialPathName::CargoPath => self.use_cargo_path,
            }
        ).filter_map(|x|
            match x {
                SpecialPathName::CWD => cwd_path.clone(),
                SpecialPathName::EXE => exe_path.clone(),
                SpecialPathName::CargoPath => crate_root.clone(),
            }
        )
        .collect()
    }
}