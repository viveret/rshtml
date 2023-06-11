use crate::core::type_info::TypeInfo;

use super::{imodel_property::IModelProperty, ihaz_attributes::IHazAttributes};



pub struct ReflectedProperty {
    pub name: String,
    typeinfo: Box<TypeInfo>,
}

impl ReflectedProperty {
    pub fn new(name: String, typeinfo: Box<TypeInfo>) -> Self {
        Self {
            name: name,
            typeinfo: typeinfo,
        }
    }
}

impl IModelProperty for ReflectedProperty {
    fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> Box<TypeInfo> {
        self.typeinfo.clone()
    }

    fn get_type_string(&self) -> String {
        self.typeinfo.to_string()
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_type_string())
    }

    fn get_value(&self) -> String {
        self.to_string()
    }

    fn get_value_as_string(&self) -> String {
        todo!()
    }

    fn get_value_as_str(&self) -> &str {
        todo!()
    }

    fn get_value_as_bool(&self) -> bool {
        todo!()
    }

    fn get_value_as_i32(&self) -> i32 {
        todo!()
    }

    fn get_value_as_i64(&self) -> i64 {
        todo!()
    }

    fn get_value_as_f32(&self) -> f32 {
        todo!()
    }

    fn get_value_as_f64(&self) -> f64 {
        todo!()
    }

    fn get_value_as_char(&self) -> char {
        todo!()
    }

    fn get_value_as_u8(&self) -> u8 {
        todo!()
    }

    fn get_value_as_u16(&self) -> u16 {
        todo!()
    }

    fn get_value_as_u32(&self) -> u32 {
        todo!()
    }

    fn get_value_as_u64(&self) -> u64 {
        todo!()
    }

    fn get_value_as_usize(&self) -> usize {
        todo!()
    }

    fn get_value_as_i8(&self) -> i8 {
        todo!()
    }

    fn get_value_as_i16(&self) -> i16 {
        todo!()
    }

    fn set_value(&mut self, value: String) {
        todo!()
    }

    fn set_value_as_string(&mut self, value: String) {
        todo!()
    }

    fn set_value_as_str(&mut self, value: &str) {
        todo!()
    }

    fn set_value_as_bool(&mut self, value: bool) {
        todo!()
    }

    fn set_value_as_i32(&mut self, value: i32) {
        todo!()
    }

    fn set_value_as_i64(&mut self, value: i64) {
        todo!()
    }

    fn set_value_as_f32(&mut self, value: f32) {
        todo!()
    }

    fn set_value_as_f64(&mut self, value: f64) {
        todo!()
    }

    fn set_value_as_char(&mut self, value: char) {
        todo!()
    }

    fn set_value_as_u8(&mut self, value: u8) {
        todo!()
    }

    fn set_value_as_u16(&mut self, value: u16) {
        todo!()
    }

    fn set_value_as_u32(&mut self, value: u32) {
        todo!()
    }

    fn set_value_as_u64(&mut self, value: u64) {
        todo!()
    }

    fn set_value_as_usize(&mut self, value: usize) {
        todo!()
    }

    fn set_value_as_i8(&mut self, value: i8) {
        todo!()
    }

    fn set_value_as_i16(&mut self, value: i16) {
        todo!()
    }
}

impl IHazAttributes for ReflectedProperty {
    fn get_attributes(&self) -> Vec<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }

    fn get_attribute(&self, typeinfo: &crate::core::type_info::TypeInfo) -> Option<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }
}