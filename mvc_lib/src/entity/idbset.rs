use std::any::Any;
use std::cell::RefCell;
use std::fs::{self, File};

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


pub struct JsonFileDbSet<TEntity: Clone> {
    file_path: String,
    items: RefCell<Vec<TEntity>>,
    items_json: RefCell<Vec<serde_json::Value>>,
    factory_method: fn() -> TEntity,
    parse_item_method: fn(v: serde_json::Value) -> TEntity,
}

impl <TEntity: 'static + Clone> JsonFileDbSet<TEntity> {
    pub fn new(
        file_path: String,
        factory_method: fn() -> TEntity,
        parse_item_method: fn(v: serde_json::Value) -> TEntity
    ) -> Self {

        let my_self = Self {
            file_path: file_path.clone(),
            factory_method: factory_method,
            parse_item_method: parse_item_method,
            items: RefCell::new(vec![]),
            items_json: RefCell::new(vec![]),
        };

        match File::open(file_path.clone()) {
            Ok(f) => {
                let metadata = f.metadata().unwrap();
                if metadata.len() < 4096 {
                    // cache since it is not that much text
                    my_self.cache_file_to_memory(Some(f));
                }
            },
            Err(e) => {
                println!("Could not open {}: {:?}", file_path, e);
            }
        }

        my_self
    }

    fn read(&self) -> <Vec<serde_json::Value> as IntoIterator>::IntoIter {
        let cached_len = self.items_json.borrow().len();
        if cached_len > 0 {
            self.cache_to_memory();
        }
        self.items_json.borrow().clone().into_iter()
    }

    fn parse_item(&self, v: serde_json::Value) -> TEntity {
        (self.parse_item_method)(v)
    }

    fn parse_and_cache_dbset_json(&self, v: serde_json::Value) {
        let mut new_items = vec![];
        let mut new_items_json = vec![];

        for it in v.get("rows").unwrap().as_array().unwrap() {
            new_items_json.push(it.clone());
            new_items.push((self.parse_item_method)(it.clone()));
        }

        self.items.replace(new_items);
        self.items_json.replace(new_items_json);
    }

    fn cache_file_to_memory(self: &Self, f: Option<File>) {
        let f = match f {
            Some(f) => f,
            None => File::open(self.file_path.clone()).unwrap(),
        };

        let json: serde_json::Value = serde_json::from_reader(f).unwrap();
        self.parse_and_cache_dbset_json(json);
    }

    fn cache_to_memory(self: &Self) {
        match File::open(self.file_path.clone()) {
            Ok(f) => {
                self.cache_file_to_memory(Some(f));
            },
            Err(e) => {
                println!("Could not open {}: {:?}", self.file_path, e);
            }
        }
    }
}

impl<TEntity: 'static + Clone> JsonFileDbSet<TEntity> {

}

impl<TEntity: 'static + Clone> IDbSetAny for JsonFileDbSet<TEntity> {
    fn add_any(self: &Self, item: Box<dyn Any>) {
        self.add(item.downcast_ref::<TEntity>().unwrap())
    }

    fn add_range_any(self: &Self, items: Vec<Box<dyn Any>>) {
        self.add_range(items.iter().map(|x| x.downcast_ref::<TEntity>().unwrap().clone()).collect::<Vec<TEntity>>());
    }

    fn attach_any(self: &Self, item: Box<dyn Any>) {
        let item: &TEntity = item.downcast_ref().unwrap();
        self.attach(item);
    }

    fn create_any(self: &Self) -> Box<dyn Any> {
        Box::new(self.create())
    }

    fn find_any(self: &Self) -> Vec<Box<dyn Any>> {
        self.find().iter().map(|x| Box::new(x.clone()) as Box<dyn Any>).collect::<Vec<Box<dyn Any>>>()
    }

    fn get_all_any(self: &Self) -> Vec<Box<dyn Any>> {
        self.get_all().iter().map(|x| Box::new(x.clone()) as Box<dyn Any>).collect::<Vec<Box<dyn Any>>>()
    }

    fn remove_any(self: &Self, item: Box<dyn Any>) {
        let item: &TEntity = item.downcast_ref().unwrap();
        self.remove(item);
    }

    fn remove_range_any(self: &Self, items: Vec<Box<dyn Any>>) {
        self.remove_range(items.iter().map(|x| x.downcast_ref::<TEntity>().unwrap().clone()).collect())
    }

    fn as_any(self: &Self, type_info: TypeInfo) -> &dyn Any {
        assert_eq!(self.type_id(), type_info.type_id);
        self
    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        IDbSet::entity_type_info(self)
    }

    fn entity_type_name(self: &Self) -> &'static str {
        IDbSet::entity_type_name(self)
    }
}

impl<TEntity: 'static + Clone> IDbSet<TEntity> for JsonFileDbSet<TEntity> {
    fn add(self: &Self, item: &TEntity) {
        self.items.borrow_mut().push(item.clone());
    }

    fn add_range(self: &Self, items: Vec<TEntity>) {
        (*self.items.borrow_mut()).extend_from_slice(&items.iter().cloned().collect::<Vec<TEntity>>());
    }

    fn attach(self: &Self, item: &TEntity) {
        self.items.borrow_mut().push(item.clone());
    }

    fn create(self: &Self) -> TEntity {
        (self.factory_method)()
    }

    fn find(self: &Self) -> Vec<TEntity> {
        self.read().map(|x| self.parse_item(x)).collect()
    }
    
    fn get_all(self: &Self) -> Vec<TEntity> {
        self.read()
            .map(|x| self.parse_item(x))
            .collect()
    }

    fn remove(self: &Self, item: &TEntity) {

    }

    fn remove_range(self: &Self, item: Vec<TEntity>) {

    }

    fn entity_type_info(self: &Self) -> TypeInfo {
        TypeInfo::of::<TEntity>()
    }

    fn entity_type_name(self: &Self) -> &'static str {
        nameof::name_of_type!(TEntity)
    }

    fn upcast(self: &Self) -> &dyn IDbSetAny {
        self
    }
}