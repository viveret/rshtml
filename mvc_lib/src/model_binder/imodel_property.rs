use crate::core::type_info::TypeInfo;

use super::ihaz_attributes::IHazAttributes;


pub trait IModelProperty: IHazAttributes {
    fn get_name(&self) -> String;
    fn get_type(&self) -> Box<TypeInfo>;
    fn get_type_string(&self) -> String;
    fn to_string(&self) -> String;


    fn get_value(&self) -> String;
    fn get_value_as_string(&self) -> String;
    fn get_value_as_str(&self) -> &str;
    fn get_value_as_bool(&self) -> bool;
    fn get_value_as_i32(&self) -> i32;
    fn get_value_as_i64(&self) -> i64;
    fn get_value_as_f32(&self) -> f32;
    fn get_value_as_f64(&self) -> f64;
    fn get_value_as_char(&self) -> char;
    fn get_value_as_u8(&self) -> u8;
    fn get_value_as_u16(&self) -> u16;
    fn get_value_as_u32(&self) -> u32;
    fn get_value_as_u64(&self) -> u64;
    fn get_value_as_usize(&self) -> usize;
    fn get_value_as_i8(&self) -> i8;
    fn get_value_as_i16(&self) -> i16;

    fn set_value(&mut self, value: String);
    fn set_value_as_string(&mut self, value: String);
    fn set_value_as_str(&mut self, value: &str);
    fn set_value_as_bool(&mut self, value: bool);
    fn set_value_as_i32(&mut self, value: i32);
    fn set_value_as_i64(&mut self, value: i64);
    fn set_value_as_f32(&mut self, value: f32);
    fn set_value_as_f64(&mut self, value: f64);
    fn set_value_as_char(&mut self, value: char);
    fn set_value_as_u8(&mut self, value: u8);
    fn set_value_as_u16(&mut self, value: u16);
    fn set_value_as_u32(&mut self, value: u32);
    fn set_value_as_u64(&mut self, value: u64);
    fn set_value_as_usize(&mut self, value: usize);
    fn set_value_as_i8(&mut self, value: i8);
    fn set_value_as_i16(&mut self, value: i16);


}