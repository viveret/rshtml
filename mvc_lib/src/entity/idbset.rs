use std::any::Any;

use crate::core::type_info::TypeInfo;

// this trait represents a database set of entities. it is used to store and retrieve rows of data.
pub trait IDbSetAny {
    // add an item to the database set
    fn add_any(self: &Self, item: Box<dyn Any>);
    // add a range of items to the database set
    fn add_range_any(self: &Self, items: Vec<Box<dyn Any>>);
    // attach an item to the database set
    fn attach_any(self: &Self, item: Box<dyn Any>);
    // create a new item in the database set
    fn create_any(self: &Self) -> Box<dyn Any>;
    // find an item in the database set
    fn find_any(self: &Self) -> Vec<Box<dyn Any>>;
    // get all items in the database set
    fn get_all_any(self: &Self) -> Vec<Box<dyn Any>>;
    // remove an item from the database set
    fn remove_any(self: &Self, item: Box<dyn Any>);
    // remove a range of items from the database set
    fn remove_range_any(self: &Self, item: Vec<Box<dyn Any>>);
    // cast the database set to a generic Any type
    fn as_any(self: &Self, type_info: TypeInfo) -> &dyn Any;
    // get the type info for the database set
    fn entity_type_info(self: &Self) -> TypeInfo;
    // get the type name for the database set entity type
    fn entity_type_name(self: &Self) -> &'static str;
    // save changes of database set to underlying data store
    fn save_changes(self: &Self);
}

// this trait represents a database set of entities. it is used to store and retrieve rows of data.
// TEntity is the type of entity that is stored in the database set.
pub trait IDbSet<TEntity> where TEntity: 'static + Clone + PartialEq {
    // add an item to the database set
    fn add(self: &Self, item: &TEntity);
    // add a range of items to the database set
    fn add_range(self: &Self, items: Vec<TEntity>);
    // attach an item to the database set
    fn attach(self: &Self, item: &TEntity);
    // create a new item in the database set
    fn create(self: &Self) -> TEntity;
    // find an item in the database set
    fn find(self: &Self) -> Vec<TEntity>;
    // get all items in the database set
    fn get_all(self: &Self) -> Vec<TEntity>;
    // remove an item from the database set
    fn remove(self: &Self, item: &TEntity);
    // remove a range of items from the database set
    fn remove_range(self: &Self, item: Vec<TEntity>);
    // cast the database set to a generic IDbSetAny type
    fn upcast(self: &Self) -> &dyn IDbSetAny;
    // get the type info for the database set
    fn entity_type_info(self: &Self) -> TypeInfo;
    // get the type name for the database set entity type
    fn entity_type_name(self: &Self) -> &'static str;
}

// extension methods for the IDbSet trait
pub struct IDbSetExtensions {

}
