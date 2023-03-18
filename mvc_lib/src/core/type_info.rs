use std::any::TypeId;
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_name: Cow<'static, str>,
}

impl TypeInfo {
    pub fn new(type_id: TypeId, type_name: Cow<'static, str>) -> Self {
        Self { type_id: type_id, type_name: type_name }
    }

    pub fn of<T: 'static + ?Sized>() -> Self {
        Self::new(TypeId::of::<T>(), Cow::Borrowed(std::any::type_name::<T>()))
    }

    pub fn box_of<T: 'static + ?Sized>() -> Box<Self> {
        Box::new(Self::of::<T>())
    }

    pub fn rc_of<T: 'static + ?Sized>() -> Rc<Self> {
        Rc::new(Self::of::<T>())
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_name)
    }
}