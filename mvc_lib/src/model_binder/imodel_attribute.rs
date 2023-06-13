use crate::core::type_info::TypeInfo;



pub trait IAttribute {
    fn get_name(&self) -> String;
    fn get_contents(&self) -> String;
    fn get_type_info(&self) -> Option<Box<TypeInfo>>;
    fn to_string(&self) -> String;
}