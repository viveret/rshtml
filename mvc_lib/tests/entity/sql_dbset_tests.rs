use mvc_lib::entity::{idbset::IDbSet, ihaz_id::IHazSqlId, sql_dbset::SqlFileDbSet};
use rusqlite::{params, ToSql};
use tempfile::NamedTempFile;
use std::path::Path;

// Test struct 1: Simple geometric point
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Point {
    id: u32,
    x: i32,
    y: i32,
}

impl IHazSqlId for Point {
    fn get_sql_id(&self) -> Box<dyn rusqlite::ToSql + '_> {
        Box::new(self.id.to_sql().unwrap())
    }
}

// Test struct 2: User data
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct SimpleUser {
    id: String,
    name: String,
    age: u8,
}

impl IHazSqlId for SimpleUser {
    fn get_sql_id(&self) -> Box<dyn rusqlite::ToSql + '_> {
        Box::new(self.id.to_sql().unwrap())
    }
}

// Test struct 3: More complex geometric shape
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Polygon {
    id: u32,
    vertices: Vec<Point>,
    name: String,
}

impl IHazSqlId for Polygon {
    fn get_sql_id(&self) -> Box<dyn rusqlite::ToSql + '_> {
        Box::new(self.id.to_sql().unwrap())
    }
}

// Helper to create a temporary SQLite database
fn temp_db() -> NamedTempFile {
    NamedTempFile::new().expect("Failed to create temp file")
}

// Common test utilities
mod test_utils {
    use rusqlite::ToSql;
    
    use super::*;
    
    pub fn point_factory() -> Point {
        Point { id: 0, x: 0, y: 0 }
    }
    
    pub fn parse_point(row: &rusqlite::Row) -> rusqlite::Result<Point> {
        Ok(Point {
            id: 0,
            x: row.get(0)?,
            y: row.get(1)?,
        })
    }
    
    pub fn point_to_params(p: &Point) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(p.x), Box::new(p.y)]
    }
    
    pub fn user_factory() -> SimpleUser {
        SimpleUser {
            id: "default".to_string(),
            name: "Anonymous".to_string(),
            age: 0,
        }
    }
    
    pub fn parse_user(row: &rusqlite::Row) -> rusqlite::Result<SimpleUser> {
        Ok(SimpleUser {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    }
    
    pub fn user_to_params(u: &SimpleUser) -> Vec<Box<dyn ToSql>> {
        vec![Box::new(u.id.clone()), Box::new(u.name.clone()), Box::new(u.age)]
    }
    
    pub fn polygon_factory() -> Polygon {
        Polygon {
            id: 0,
            vertices: vec![],
            name: "unnamed".to_string(),
        }
    }
    
    pub fn parse_polygon(row: &rusqlite::Row) -> rusqlite::Result<Polygon> {
        let vertices_str: String = row.get(2)?;
        let vertices = serde_json::from_str(&vertices_str).unwrap_or_default();
        Ok(Polygon {
            id: row.get(0)?,
            name: row.get(1)?,
            vertices,
        })
    }
    
    pub fn polygon_to_params(p: &Polygon) -> Vec<Box<dyn ToSql>> {
        let vertices_json = serde_json::to_string(&p.vertices).unwrap();
        vec![Box::new(p.name.clone()), Box::new(vertices_json)]
    }
}

// Test initialization
#[test]
pub fn db_initialization_test() {
    let db_file = temp_db();
    let point_db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "points".to_string(),
        test_utils::point_factory,
        test_utils::parse_point,
        test_utils::point_to_params,
    )
    .unwrap();
    
    assert_eq!(point_db.entity_type_name(), "Point");
}

// CRUD tests for Point

#[test]
pub fn point_crud_test() {
    let db_file = temp_db();
    let db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "points".to_string(),
        test_utils::point_factory,
        test_utils::parse_point,
        test_utils::point_to_params,
    )
    .unwrap();
    
    // Create
    let mut point = test_utils::point_factory();
    point.x = 10;
    point.y = 20;
    
    // Add
    db.add(&point);
    
    // Read
    let points = db.get_all();
    assert_eq!(points.len(), 1);
    assert_eq!(points[0], point);
    
    // Update
    point.x = 30;
    db.add(&point); // This creates a new record in current implementation
    assert_eq!(db.get_all().len(), 2);
    
    // Delete
    db.remove(&point);
    assert_eq!(db.get_all().len(), 1);
}

// CRUD tests for SimpleUser

#[test]
pub fn test_user_crud() {
    let db_file = temp_db();
    let db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "users".to_string(),
        test_utils::user_factory,
        test_utils::parse_user,
        test_utils::user_to_params,
    )
    .unwrap();
    
    let user = SimpleUser {
        id: "123".to_string(),
        name: "Alice".to_string(),
        age: 30,
    };
    
    db.add(&user);
    let users = db.get_all();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0], user);
    
    // Test find
    let found = db.find();
    assert!(!found.is_empty());
}

// CRUD tests for Polygon

#[test]
pub fn test_polygon_crud() {
    let db_file = temp_db();
    let db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "polygons".to_string(),
        test_utils::polygon_factory,
        test_utils::parse_polygon,
        test_utils::polygon_to_params,
    )
    .unwrap();
    
    let triangle = Polygon {
        id: 0,
        name: "Triangle".to_string(),
        vertices: vec![
        Point { id: 0, x: 0, y: 0 },
        Point { id: 0, x: 10, y: 0 },
        Point { id: 0, x: 5, y: 10 },
        ],
    };
    
    db.add(&triangle);
    let polygons = db.get_all();
    assert_eq!(polygons.len(), 1);
    assert_eq!(polygons[0], triangle);
}

// Batch operations tests
#[test]
pub fn test_batch_operations() {
    let db_file = temp_db();
    let db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "batch_points".to_string(),
        test_utils::point_factory,
        test_utils::parse_point,
        test_utils::point_to_params,
    )
    .unwrap();
    
    let points = vec![
    Point { id: 0, x: 1, y: 1 },
    Point { id: 0, x: 2, y: 2 },
    Point { id: 0, x: 3, y: 3 },
    ];
    
    // Test add_range
    db.add_range(points.clone());
    assert_eq!(db.get_all().len(), 3);
    
    // Test remove_range
    db.remove_range(points[0..2].to_vec());
    assert_eq!(db.get_all().len(), 1);
}

// Type conversion tests
#[test]
pub fn test_type_conversion() {
    let db_file = temp_db();
    let db = SqlFileDbSet::open(
        db_file.path().to_str().unwrap().to_string(),
        "type_users".to_string(),
        test_utils::user_factory,
        test_utils::parse_user,
        test_utils::user_to_params,
    )
    .unwrap();
    
    let user = SimpleUser {
        id: "type_test".to_string(),
        name: "Conversion".to_string(),
        age: 99,
    };
    
    // Test upcasting to IDbSetAny
    let any_db = db.upcast();
    any_db.add_any(Box::new(user.clone()));
    
    // Test get_all_any
    let all_users = any_db.get_all_any();
    let downcast_user = all_users[0].downcast_ref::<SimpleUser>().unwrap();
    assert_eq!(downcast_user, &user);
}