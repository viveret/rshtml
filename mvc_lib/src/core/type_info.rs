use std::any::TypeId;
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

// this is a struct that holds information about a type.
#[derive(Clone, Debug)]
pub struct TypeInfo {
    // the type id of the type
    pub type_id: TypeId,
    // the name of the type
    pub type_name: Cow<'static, str>,
}

impl TypeInfo {
    // create a new type info struct from a type id and type name.
    pub fn new(type_id: TypeId, type_name: Cow<'static, str>) -> Self {
        Self { type_id: type_id, type_name: type_name }
    }

    // create a new type info struct from a type.
    pub fn of<T: 'static + ?Sized>() -> Self {
        Self::new(TypeId::of::<T>(), Cow::Borrowed(std::any::type_name::<T>()))
    }

    // create a new type info struct from a type and wrap it in a Rc.
    pub fn rc_of<T: 'static + ?Sized>() -> Box<Self> {
        Box::new(Self::of::<Rc<T>>())
    }

    // returns true if the type info is the same as the type info of the type.
    pub fn is_same_as(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_name)
    }
}