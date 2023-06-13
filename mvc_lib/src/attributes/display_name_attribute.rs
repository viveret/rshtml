use crate::core::type_info::TypeInfo;
use crate::model_binder::imodel_attribute::IAttribute;


pub struct DisplayNameAttribute {
    pub name: String,
}

impl DisplayNameAttribute {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

impl IAttribute for DisplayNameAttribute {
    fn get_type_info(&self) -> Option<Box<TypeInfo>> {
        Some(Box::new(TypeInfo::of::<Self>()))
    }

    fn get_name(&self) -> String {
        nameof::name_of_type!(DisplayNameAttribute).to_string()
    }

    fn get_contents(&self) -> String {
        self.name.clone()
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_contents())
    }
}