use std::cell::RefCell;

use crate::core::type_info::TypeInfo;

use super::idbset::{IDbSet, IDbSetAny};


pub struct VecDbSet<T> {
    entities: RefCell<Vec<T>>,
}

impl<T> VecDbSet<T> {
    pub fn new() -> VecDbSet<T> {
        VecDbSet {
            entities: RefCell::new(Vec::new()),
        }
    }
}

impl<T> IDbSet<T> for VecDbSet<T> where T: 'static + Clone + PartialEq {
    fn add(self: &Self, item: &T) {
        // create new id if it doesn't exist
        // if the item does have an id, then throw an error
        self.entities.borrow_mut().push(item.clone());
    }

    fn add_range(self: &Self, items: Vec<T>) {
        self.entities.borrow_mut().extend(items);
    }

    fn attach(self: &Self, item: &T) {
        // if the item doesn't have an id, then throw an error
        // if the item does have an id, then add it to the list
        self.entities.borrow_mut().push(item.clone());
    }

    fn create(self: &Self) -> T {
        todo!()
    }

    fn find(self: &Self) -> Vec<T> {
        vec![]
    }

    fn get_all(self: &Self) -> Vec<T> {
        self.entities.borrow().clone()
    }

    fn remove(self: &Self, item: &T) {
        self.entities.borrow_mut().retain(|x| x != item);
    }

    fn remove_range(self: &Self, item: Vec<T>) {
        self.entities.borrow_mut().retain(|x| !item.contains(x));
    }

    fn upcast(self: &Self) -> &dyn IDbSetAny {
        self
    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<T>()
    }

    fn entity_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(T)
    }
}

impl<T> IDbSetAny for VecDbSet<T> where T: 'static + Clone + PartialEq {
    fn add_any(self: &Self, _item: Box<dyn std::any::Any>) {
        todo!()
    }

    fn add_range_any(self: &Self, _items: Vec<Box<dyn std::any::Any>>) {
        todo!()
    }

    fn attach_any(self: &Self, _item: Box<dyn std::any::Any>) {
        todo!()
    }

    fn create_any(self: &Self) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn find_any(self: &Self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    fn get_all_any(self: &Self) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    fn remove_any(self: &Self, _item: Box<dyn std::any::Any>) {
        todo!()
    }

    fn remove_range_any(self: &Self, _item: Vec<Box<dyn std::any::Any>>) {
        todo!()
    }

    fn as_any(self: &Self, _type_info: TypeInfo) -> &dyn std::any::Any {
        todo!()
    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<T>()
    }

    fn entity_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(T)
    }

    fn save_changes(self: &Self) {
        
    }
}
