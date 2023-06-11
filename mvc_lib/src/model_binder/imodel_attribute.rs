use crate::core::type_info::TypeInfo;



pub trait IAttribute {
    fn get_name(&self) -> String;
    fn get_contents(&self) -> String;
    fn get_type_info(&self) -> Box<TypeInfo>;
    // fn get_type_string(&self) -> String;
    fn to_string(&self) -> String;
}