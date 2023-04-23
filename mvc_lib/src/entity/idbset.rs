use std::any::Any;

use crate::core::type_info::TypeInfo;


pub trait IDbSetAny {
    fn add_any(self: &Self, item: Box<dyn Any>);
    fn add_range_any(self: &Self, items: Vec<Box<dyn Any>>);
    fn attach_any(self: &Self, item: Box<dyn Any>);
    fn create_any(self: &Self) -> Box<dyn Any>;
    fn find_any(self: &Self) -> Vec<Box<dyn Any>>;
    fn get_all_any(self: &Self) -> Vec<Box<dyn Any>>;
    fn remove_any(self: &Self, item: Box<dyn Any>);
    fn remove_range_any(self: &Self, item: Vec<Box<dyn Any>>);

    fn as_any(self: &Self, type_info: TypeInfo) -> &dyn Any;

    fn entity_type_info(self: &Self) -> TypeInfo;
    fn entity_type_name(self: &Self) -> &'static str;
}

pub trait IDbSet<TEntity> {
    fn add(self: &Self, item: &TEntity);
    fn add_range(self: &Self, items: Vec<TEntity>);
    fn attach(self: &Self, item: &TEntity);
    fn create(self: &Self) -> TEntity;
    fn find(self: &Self) -> Vec<TEntity>;
    fn get_all(self: &Self) -> Vec<TEntity>;
    fn remove(self: &Self, item: &TEntity);
    fn remove_range(self: &Self, item: Vec<TEntity>);

    fn upcast(self: &Self) -> &dyn IDbSetAny;

    fn entity_type_info(self: &Self) -> TypeInfo;
    fn entity_type_name(self: &Self) -> &'static str;
}

pub struct IDbSetExtensions {

}
