use mvc_lib::auth::auth_role_json_file_dbset::JsonAuthRole;

pub struct IndexViewModel {
    pub roles: Vec<JsonAuthRole>
}

impl IndexViewModel {
    pub fn new(roles: Vec<JsonAuthRole>) -> Self {
        Self { roles: roles }
    }
}
