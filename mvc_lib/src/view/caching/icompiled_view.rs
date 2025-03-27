pub trait ICompiledView: Send + Sync {
    fn get_file_path(&self) -> String;
    fn get_index_in_file(&self) -> usize;
    fn get_hash(&self) -> u64;
    fn get_has_source_code(&self) -> bool;
    fn get_has_output_code(&self) -> bool;
    fn get_source_code(&self) -> String;
    fn get_output_code(&self) -> String;
    fn try_get_source_code(&self) -> Option<String>;
    fn try_get_output_code(&self) -> Option<String>;
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CompiledView {
    pub file_path: String,
    pub index_in_file: usize,
    pub hash: u64,
    pub source_code: Option<String>,
    pub output_code: Option<String>,
}

impl ICompiledView for CompiledView {
    fn get_file_path(&self) -> String {
        self.file_path.clone()
    }

    fn get_index_in_file(&self) -> usize {
        self.index_in_file
    }

    fn get_hash(&self) -> u64 {
        self.hash
    }

    fn get_has_source_code(&self) -> bool {
        self.source_code.is_some()
    }

    fn get_source_code(&self) -> String {
        self.source_code.clone().unwrap_or_else(|| String::new())
    }

    fn try_get_source_code(&self) -> Option<String> {
        self.source_code.clone()
    }
    
    fn try_get_output_code(&self) -> Option<String> {
        self.output_code.clone()
    }
    
    fn get_output_code(&self) -> String {
        self.output_code.clone().unwrap_or_else(|| String::new())
    }
    
    fn get_has_output_code(&self) -> bool {
        self.output_code.is_some()
    }
}

impl CompiledView {
    // Creates a new CompiledView instance
    pub fn new(file_path: String, index_in_file: usize, hash: u64, source_code: Option<String>, output_code: Option<String>) -> Self {
        CompiledView {
            file_path,
            index_in_file,
            hash,
            source_code,
            output_code
        }
    }

    // Builder pattern: set file path
    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = file_path;
        self
    }

    // Builder pattern: set index in file
    pub fn with_index_in_file(mut self, index: usize) -> Self {
        self.index_in_file = index;
        self
    }

    // Builder pattern: set hash
    pub fn with_hash(mut self, hash: u64) -> Self {
        self.hash = hash;
        self
    }

    // Builder pattern: set source code
    pub fn with_source_code(mut self, source_code: Option<String>) -> Self {
        self.source_code = source_code;
        self
    }

    // Builder pattern: set output code
    pub fn with_output_code(mut self, output_code: Option<String>) -> Self {
        self.output_code = output_code;
        self
    }
}