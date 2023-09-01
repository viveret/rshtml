use std::{rc::Rc, borrow::BorrowMut, cell::{RefCell, Ref}, any::TypeId};

use crate::{layer::ilayer::ILayer, node::inode::INode};



pub trait ILinearSequence: INode {
    fn get_layer(&self, i: usize) -> Rc<dyn ILayer>;
    fn get_layer_size(&self) -> usize;

    fn add_layer(&self, layer: Rc<dyn ILayer>);
    fn remove_layer(&self, i: usize);
    fn insert_layer(&self, i: usize, layer: Rc<dyn ILayer>);
}

pub struct LinearSequence {
    pub name: RefCell<String>,
    pub id: usize,
    pub layers: RefCell<Vec<Rc<dyn ILayer>>>,
}

impl LinearSequence {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            id: 0,
            layers: RefCell::new(Vec::new()),
        }
    }
}

impl ILinearSequence for LinearSequence {
    fn get_layer(&self, i: usize) -> Rc<dyn ILayer> {
        self.layers.borrow()[i].clone()
    }

    fn get_layer_size(&self) -> usize {
        self.layers.borrow().len()
    }

    fn add_layer(&self, layer: Rc<dyn ILayer>) {
        self.layers.borrow_mut().push(layer);
    }

    fn remove_layer(&self, i: usize) {
        self.layers.borrow_mut().remove(i);
    }

    fn insert_layer(&self, i: usize, layer: Rc<dyn ILayer>) {
        self.layers.borrow_mut().insert(i, layer);
    }
}

impl INode for LinearSequence {
    fn can_read(&self) -> bool {
        self.layers.borrow().iter().all(|layer| layer.can_read())
    }

    fn can_write(&self) -> bool {
        self.layers.borrow().iter().all(|layer| layer.can_write())
    }

    fn can_update(&self) -> bool {
        self.layers.borrow().iter().all(|layer| layer.can_update())
    }

    fn get_input_node_width(&self) -> usize {
        self.layers.borrow()[0].get_input_node_width()
    }

    fn get_input_node_height(&self) -> usize {
        self.layers.borrow()[0].get_input_node_height()
    }

    fn get_input_node_depth(&self) -> usize {
        self.layers.borrow()[0].get_input_node_depth()
    }

    fn get_input_node_size(&self) -> usize {
        self.layers.borrow()[0].get_input_node_size()
    }

    fn get_input_node_at_index(&self, i: usize) -> Box<dyn INode> {
        self.layers.borrow()[0].get_input_node_at_index(i)
    }

    fn get_input_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode> {
        self.layers.borrow()[0].get_input_node_at_xyz(x, y, z)
    }

    fn get_input_value_width(&self) -> usize {
        self.layers.borrow()[0].get_input_value_width()
    }

    fn get_input_value_height(&self) -> usize {
        self.layers.borrow()[0].get_input_value_height()
    }

    fn get_input_value_depth(&self) -> usize {
        self.layers.borrow()[0].get_input_value_depth()
    }

    fn get_input_value_size(&self) -> usize {
        self.layers.borrow()[0].get_input_value_size()
    }

    fn get_input_value_at_index(&self, i: usize) -> f32 {
        self.layers.borrow()[0].get_input_value_at_index(i)
    }

    fn get_input_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32 {
        self.layers.borrow()[0].get_input_value_at_xyz(x, y, z)
    }

    fn get_input(&self) -> f32 {
        self.layers.borrow()[0].get_input()
    }

    fn set_input(&self, value: f32) {
        self.layers.borrow()[0].set_input(value)
    }

    fn get_output_node_width(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_width()
    }

    fn get_output_node_height(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_height()
    }

    fn get_output_node_depth(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_depth()
    }

    fn get_output_node_size(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_size()
    }

    fn get_output_node_at_index(&self, i: usize) -> Box<dyn INode> {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_at_index(i)
    }

    fn get_output_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode> {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_node_at_xyz(x, y, z)
    }

    fn get_output_value_width(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_width()
    }

    fn get_output_value_height(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_height()
    }

    fn get_output_value_depth(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_depth()
    }

    fn get_output_value_size(&self) -> usize {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_size()
    }

    fn get_output_value_at_index(&self, i: usize) -> f32 {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_at_index(i)
    }

    fn get_output_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32 {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output_value_at_xyz(x, y, z)
    }

    fn get_output(&self) -> f32 {
        self.layers.borrow()[self.layers.borrow().len() - 1].get_output()
    }

    fn set_output(&self, value: f32) {
        self.layers.borrow()[self.layers.borrow().len() - 1].set_output(value)
    }

    fn forward(&self) {
        self.layers.borrow()[0].forward();
    }

    fn backward(&self) {
        self.layers.borrow()[self.layers.borrow().len() - 1].backward();
    }

    fn update(&self) {
        for layer in self.layers.borrow().iter() {
            layer.update();
        }
    }

    fn get_delta(&self) -> f32 {
        todo!()
    }

    fn get_gradient(&self) {
        todo!()
    }

    fn get_parameter(&self) {
        todo!()
    }

    fn get_parameter_gradient(&self) {
        todo!()
    }

    fn get_parameter_size(&self) -> usize {
        todo!()
    }

    fn get_parameter_gradient_size(&self) -> usize {
        todo!()
    }

    fn get_parameter_data(&self) {
        todo!()
    }

    fn set_delta(&self, value: f32) {
        todo!()
    }

    fn set_gradient(&self) {
        todo!()
    }

    fn set_parameter(&self) {
        todo!()
    }

    fn set_parameter_gradient(&self) {
        todo!()
    }

    fn set_parameter_size(&self) {
        todo!()
    }

    fn set_parameter_gradient_size(&self) {
        todo!()
    }

    fn set_parameter_offset(&self) {
        todo!()
    }

    fn set_parameter_gradient_offset(&self) {
        todo!()
    }

    fn set_parameter_data(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    fn get_type_id(&self) -> std::any::TypeId {
        TypeId::of::<LinearSequence>()
    }

    fn get_type_name(&self) -> &str {
        nameof::name_of_type!(LinearSequence)
    }

    fn get_id(&self) -> usize {
        todo!()
    }

    fn get_index(&self) -> usize {
        todo!()
    }

    fn get_parent(&self) -> Option<Rc<dyn INode>> {
        todo!()
    }

    fn get_children(&self) -> Vec<Rc<dyn INode>> {
        todo!()
    }

    fn get_parents(&self) -> Vec<Rc<dyn INode>> {
        todo!()
    }

    fn get_children_size(&self) -> usize {
        todo!()
    }

    fn get_parents_size(&self) -> usize {
        todo!()
    }

    fn get_node_at_index(&self, i: usize) -> Rc<dyn INode> {
        todo!()
    }

    fn set_name(&self, name: &String) {
        todo!()
    }

    fn set_id(&self, id: usize) {
        todo!()
    }

    fn set_index(&self, index: usize) {
        todo!()
    }

    fn set_parent(&self, parent: Option<Rc<dyn INode>>) {
        todo!()
    }

    fn set_children(&self, children: Vec<Rc<dyn INode>>) {
        todo!()
    }

    fn set_parents(&self, parents: Vec<Rc<dyn INode>>) {
        todo!()
    }

    fn set_children_size(&self, size: usize) {
        todo!()
    }

    fn set_parents_size(&self, size: usize) {
        todo!()
    }

    fn check_validity(&self) -> std::result::Result<usize, usize> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}