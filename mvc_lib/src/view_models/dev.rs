use std::rc::Rc;

use crate::view::iview::IView;


pub struct ViewsViewModel {
    pub views: Vec<Rc<Box<dyn IView>>>,
}

impl ViewsViewModel {
    pub fn new(views: Vec<Rc<Box<dyn IView>>>) -> Self {
        Self { views: views }
    }
}