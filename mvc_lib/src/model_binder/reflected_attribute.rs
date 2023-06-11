use crate::core::type_info::TypeInfo;

use super::imodel_attribute::IAttribute;



// this struct implements IAttribute and is used to store attributes in the model binder.
// attributes are the same as in rust, e.g. #[attribute_name(value)]
pub struct ReflectedAttribute {
    pub name: String,
    pub typeinfo: Option<Box<TypeInfo>>,
    pub contents: String,
}

impl ReflectedAttribute {
    pub fn new(name: String, contents: String, typeinfo: Option<Box<TypeInfo>>) -> Self {
        Self {
            name: name,
            contents: contents,
            typeinfo: typeinfo,
        }
    }
}

impl IAttribute for ReflectedAttribute {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        self.typeinfo.clone().unwrap_or(Box::new(TypeInfo::of::<Self>()))
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.name, self.contents)
    }

    fn get_contents(&self) -> String {
        self.contents.clone()
    }
}