use std::rc::Rc;

use crate::core::type_info::TypeInfo;

use super::ihaz_attributes::IHazAttributes;
use super::imodel_attribute::IAttribute;
use super::imodel_property::IModelProperty;


// this struct is used to represent a reflected property during execution / runtime.
pub struct ReflectedProperty {
    pub attributes: Vec<Rc<dyn IAttribute>>,
    pub name_ampersand: bool,
    pub name: String,
    pub return_type: Option<Box<TypeInfo>>,
}

impl ReflectedProperty {
    pub fn new(
        attributes: Vec<Rc<dyn IAttribute>>,
        name_ampersand: bool,
        name: String,
        return_type: Option<Box<TypeInfo>>
    ) -> Self {
        Self {
            attributes: attributes,
            name_ampersand: name_ampersand,
            name: name,
            return_type: return_type,
        }
    }
}

impl IModelProperty for ReflectedProperty {
    fn get_has_name_ampersand(&self) -> bool {
        self.name_ampersand
    }

    fn get_return_type(&self) -> Option<Box<TypeInfo>> {
        self.return_type.clone()
    }

    fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    fn get_type_info(&self) -> Box<TypeInfo> {
        Box::new(TypeInfo::of::<Self>())
    }

    fn to_string(&self) -> String {
        format!(
            "{}{}: {}", 
            self.attributes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
            self.get_name(),
            self.return_type.as_ref().map(|x| x.to_string()).unwrap_or("void".to_string())
        )
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

    fn set_value(&mut self, _value: String) {
        todo!()
    }

    fn set_value_as_string(&mut self, _value: String) {
        todo!()
    }

    fn set_value_as_str(&mut self, _value: &str) {
        todo!()
    }

    fn set_value_as_bool(&mut self, _value: bool) {
        todo!()
    }

    fn set_value_as_i32(&mut self, _value: i32) {
        todo!()
    }

    fn set_value_as_i64(&mut self, _value: i64) {
        todo!()
    }

    fn set_value_as_f32(&mut self, _value: f32) {
        todo!()
    }

    fn set_value_as_f64(&mut self, _value: f64) {
        todo!()
    }

    fn set_value_as_char(&mut self, _value: char) {
        todo!()
    }

    fn set_value_as_u8(&mut self, _value: u8) {
        todo!()
    }

    fn set_value_as_u16(&mut self, _value: u16) {
        todo!()
    }

    fn set_value_as_u32(&mut self, _value: u32) {
        todo!()
    }

    fn set_value_as_u64(&mut self, _value: u64) {
        todo!()
    }

    fn set_value_as_usize(&mut self, _value: usize) {
        todo!()
    }

    fn set_value_as_i8(&mut self, _value: i8) {
        todo!()
    }

    fn set_value_as_i16(&mut self, _value: i16) {
        todo!()
    }
}

impl IHazAttributes for ReflectedProperty {
    fn get_attributes(&self) -> Vec<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }

    fn get_attribute(&self, _typeinfo: &crate::core::type_info::TypeInfo) -> Option<std::rc::Rc<dyn super::imodel_attribute::IAttribute>> {
        todo!()
    }
}