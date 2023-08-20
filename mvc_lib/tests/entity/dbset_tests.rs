use std::{rc::Rc, fs::File, io::Write};

use mvc_lib::entity::{vec_dbset::VecDbSet, idbset::IDbSet, idbset::IDbSetAny, json_file_dbset::JsonFileDbSet};


#[derive(Clone, Debug, PartialEq)]
struct TestPerson {
    id: i32,
    name: String,
}

impl TestPerson {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }

    fn parse(x: serde_json::Value) -> TestPerson {
        todo!()
    }

    fn jsonify(self: &TestPerson) -> serde_json::Value {
        let mut x = serde_json::Map::new();
        x.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(self.id)));
        x.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        serde_json::Value::Object(x)
    }
}

struct DbSetTestSuite<T> where T: IDbSet<TestPerson> {
    dbset: Rc<T>,
}

impl<T> DbSetTestSuite<T> where T: IDbSet<TestPerson> {
    pub fn new(dbset: Rc<T>) -> Self { Self { dbset } }

    pub fn perform_tests(self: &Self, save_changes: bool) {
        // get the type info for the database set
        let type_info = self.dbset.entity_type_info();

        // get the type name for the database set entity type
        let type_name = self.dbset.entity_type_name();

        // create a new person
        let person = TestPerson::new();

        // add the person to the database set
        self.dbset.add(&person);
        if save_changes { self.dbset.upcast().save_changes(); }

        {
            // should be one person
            let people = self.dbset.get_all();
            assert_eq!(people.len(), 1);
        }

        // remove the person from the database set
        self.dbset.remove(&person);
        if save_changes { self.dbset.upcast().save_changes(); }

        {
            // should be empty
            let people = self.dbset.get_all();
            assert_eq!(people.len(), 0);
        }
    }

    pub fn perform_tests_all_options(self: &Self) {
        self.perform_tests(false);
        self.perform_tests(true);
    }
}

#[test]
fn vec_dbset_tests() {
    let dbset = Rc::new(VecDbSet::<TestPerson>::new());
    let suite = DbSetTestSuite::new(dbset);
    suite.perform_tests_all_options();
}

#[test]
fn jsonfile_dbset_tests() {
    // need to create empty json file
    let mut f = File::create("test.json").unwrap();
    f.write_all("{}".as_bytes()).unwrap();
    f.flush().unwrap();
    
    let dbset = Rc::new(JsonFileDbSet::<TestPerson>::new(
        "test.json".to_string(),
        File::open("test.json").unwrap(),
        || TestPerson::new(),
        TestPerson::parse,
        |x| TestPerson::jsonify(&x)
    ));
    let suite = DbSetTestSuite::new(dbset);
    suite.perform_tests_all_options();
}