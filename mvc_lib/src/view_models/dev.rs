use std::rc::Rc;

use crate::view::iview::IView;


pub struct IndexViewModel {
    pub views: Vec<Rc<Box<dyn IView>>>,
}

impl IndexViewModel {
    pub fn new(views: Vec<Rc<Box<dyn IView>>>) -> Self {
        Self { views: views }
    }
}


pub struct ViewsViewModel {
    pub views: Vec<Rc<dyn IView>>,
}

impl ViewsViewModel {
    pub fn new(views: Vec<Rc<dyn IView>>) -> Self {
        Self { views: views }
    }
}

pub struct ViewDetailsViewModel {
    pub view: Rc<dyn IView>,
}

impl ViewDetailsViewModel {
    pub fn new(view: Rc<dyn IView>) -> Self {
        Self { view: view }
    }
}