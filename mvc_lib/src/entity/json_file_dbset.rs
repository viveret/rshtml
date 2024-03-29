use std::path::Path;
use std::{any::Any, io::Write};
use std::cell::RefCell;
use std::fs::File;

use crate::core::type_info::TypeInfo;

use super::idbset::{ IDbSet, IDbSetAny };


// this implements the IDbSet trait for a json file dbset for a specific entity type.
// TEntity is the type of entity that is stored in the database set.
pub struct JsonFileDbSet<TEntity: Clone> {
    // the path to the json file
    file_path: String,
    // the items in the database set
    items: RefCell<Vec<TEntity>>,
    // the items in the database set as json
    items_json: RefCell<Vec<serde_json::Value>>,
    // the factory method to create a new TEntity
    factory_method: fn() -> TEntity,
    // the method to parse a serde_json::Value into a TEntity
    parse_item_method: fn(v: serde_json::Value) -> TEntity,
    // the method to stringify a TEntity into a serde_json::Value
    jsonify_item_method: fn(v: TEntity) -> serde_json::Value,
}

impl <TEntity: 'static + Clone> JsonFileDbSet<TEntity> {
    pub fn new(
        file_path: String,
        f: File,
        factory_method: fn() -> TEntity,
        parse_item_method: fn(v: serde_json::Value) -> TEntity,
        jsonify_item_method: fn(v: TEntity) -> serde_json::Value,
    ) -> Self {
        let my_self = Self {
            file_path: file_path.clone(),
            factory_method: factory_method,
            parse_item_method: parse_item_method,
            jsonify_item_method: jsonify_item_method,
            items: RefCell::new(vec![]),
            items_json: RefCell::new(vec![]),
        };

        let metadata = f.metadata().unwrap();
        if metadata.len() < 4096 {
            // cache since it is not that much text
            my_self.cache_file_to_memory(Some(f));
        }

        my_self
    }

    pub fn open(
        file_path: String,
        factory_method: fn() -> TEntity,
        parse_item_method: fn(v: serde_json::Value) -> TEntity,
        jsonify_item_method: fn(v: TEntity) -> serde_json::Value,
    ) -> std::io::Result<Self> {
        let path = Path::new(&file_path);
        match path.try_exists() {
            Ok(true) => {
                let f = File::open(file_path.clone())?;
                std::io::Result::Ok(Self::new(file_path, f, factory_method, parse_item_method, jsonify_item_method))
            },
            Ok(false) => {
                let f = File::create(file_path.clone())?;
                std::io::Result::Ok(Self::new(file_path, f, factory_method, parse_item_method, jsonify_item_method))
            },
            Err(e) => {
                std::io::Result::Err(e)
            }
        }
    }

    // read the json file and return the items as a Vec<serde_json::Value>
    fn read(&self) -> <Vec<serde_json::Value> as IntoIterator>::IntoIter {
        let cached_len = self.items_json.borrow().len();
        if cached_len > 0 {
            self.cache_to_memory();
        }
        self.items_json.borrow().clone().into_iter()
    }

    // parse a serde_json::Value into a TEntity
    fn parse_item(&self, v: serde_json::Value) -> TEntity {
        (self.parse_item_method)(v)
    }

    // parse a serde_json::Value into a TEntity and cache it
    fn parse_and_cache_dbset_json(&self, v: serde_json::Value) {
        let mut new_items = vec![];
        let mut new_items_json = vec![];

        if let Some(v) = v.get("rows") {
            if let Some(v) = v.as_array() {
                for it in v {
                    new_items_json.push(it.clone());
                    new_items.push((self.parse_item_method)(it.clone()));
                }
            }
        }

        self.items.replace(new_items);
        self.items_json.replace(new_items_json);
    }

    // cache the json file to memory
    // f: Option<File> the file to be cached. if f is None then it will open the file using the default file path
    // returns the json file as a serde_json::Value
    fn cache_file_to_memory(self: &Self, f: Option<File>) {
        let f = match f {
            Some(f) => f,
            None => {
                match File::open(self.file_path.clone()) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("Could not open {}: {:?}", self.file_path, e);
                        return;
                    }
                }
            },
        };

        let json = serde_json::from_reader(f);
        match json {
            Ok(json) => {
                self.parse_and_cache_dbset_json(json);
            },
            Err(e) => {
                println!("Could not parse {}: {:?}", self.file_path, e);
            }
        }
    }

    // cache the json file to memory
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

    fn write_to_file(self: &Self) {
        let mut file = File::create(self.file_path.clone()).unwrap();
        let json = serde_json::json!({
            "rows": self.items_json.borrow().clone()
        });
        let json_str = serde_json::to_string_pretty(&json).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

impl<TEntity: 'static + Clone + PartialEq> JsonFileDbSet<TEntity> {

}

impl<TEntity: 'static + Clone + PartialEq> IDbSetAny for JsonFileDbSet<TEntity> {
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

    fn save_changes(self: &Self) {
        self.write_to_file();
    }
}

impl<TEntity: 'static + Clone + PartialEq> IDbSet<TEntity> for JsonFileDbSet<TEntity> {
    fn add(self: &Self, item: &TEntity) {
        self.items.borrow_mut().push(item.clone());
        self.items_json.borrow_mut().push((self.jsonify_item_method)(item.clone()));
    }

    fn add_range(self: &Self, items: Vec<TEntity>) {
        (*self.items.borrow_mut()).extend_from_slice(&items.iter().cloned().collect::<Vec<TEntity>>());
        (*self.items_json.borrow_mut()).extend_from_slice(&items.iter().map(|x| (self.jsonify_item_method)(x.clone())).collect::<Vec<serde_json::Value>>());
    }

    fn attach(self: &Self, item: &TEntity) {
        self.items.borrow_mut().push(item.clone());
        self.items_json.borrow_mut().push((self.jsonify_item_method)(item.clone()));
    }

    fn create(self: &Self) -> TEntity {
        (self.factory_method)()
    }

    fn find(self: &Self) -> Vec<TEntity> {
        self.read().map(|x| self.parse_item(x)).collect()
    }
    
    fn get_all(self: &Self) -> Vec<TEntity> {
        // self.read()
        //     .map(|x| self.parse_item(x))
        //     .collect()
        self.items.borrow().clone()
    }

    fn remove(self: &Self, item: &TEntity) {
        let mut items = self.items.borrow_mut();
        let mut items_json = self.items_json.borrow_mut();

        let index = items.iter().position(|x| x == item).unwrap();
        items.remove(index);
        items_json.remove(index);
    }

    fn remove_range(self: &Self, item: Vec<TEntity>) {
        let mut items = self.items.borrow_mut();
        let mut items_json = self.items_json.borrow_mut();

        for it in item {
            let index = items.iter().position(|x| x == &it).unwrap();
            items.remove(index);
            items_json.remove(index);
        }
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