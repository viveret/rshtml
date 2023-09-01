use std::{any::TypeId, rc::Rc, borrow::BorrowMut, cell::RefCell};

use syn::token::Ref;

use crate::node::inode::INode;

use super::imodel::IModel;




pub struct GenericModel {
    pub name: RefCell<String>,
    pub id: usize,   
    sequence: RefCell<Option<Rc<dyn IModel>>>,
}

impl GenericModel {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            id: 0,
            sequence: RefCell::new(None),
        }
    }
}

impl INode for GenericModel {
    fn can_read(&self) -> bool {
        self.sequence.borrow().as_ref().unwrap().can_read()
    }

    fn can_write(&self) -> bool {
        self.sequence.borrow().as_ref().unwrap().can_write()
    }

    fn can_update(&self) -> bool {
        self.sequence.borrow().as_ref().unwrap().can_update()
    }

    fn get_input_node_width(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_node_width()
    }

    fn get_input_value_width(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_value_width()
    }

    fn get_input_node_height(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_node_height()
    }

    fn get_input_value_height(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_value_height()
    }

    fn get_input_node_depth(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_node_depth()
    }

    fn get_input_value_depth(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_value_depth()
    }

    fn get_output_node_width(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_node_width()
    }

    fn get_output_value_width(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_value_width()
    } 

    fn get_output_node_height(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_node_height()
    }

    fn get_output_value_height(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_value_height()
    }

    fn get_output_node_depth(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_node_depth()
    }

    fn get_output_value_depth(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_value_depth()
    }

    fn get_input_node_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_node_size()
    }

    fn get_input_value_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_input_value_size()
    }

    fn get_output_node_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_node_size()
    }

    fn get_output_value_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_output_value_size()
    }

    fn forward(&self) {
        self.sequence.borrow().as_ref().unwrap().forward()
    }

    fn backward(&self) {
        self.sequence.borrow().as_ref().unwrap().backward()
    }

    fn update(&self) {
        self.sequence.borrow().as_ref().unwrap().update()
    }

    fn get_output(&self) -> f32 {
        self.sequence.borrow().as_ref().unwrap().get_output()
    }

    fn get_input(&self) -> f32 {
        self.sequence.borrow().as_ref().unwrap().get_input()
    }

    fn get_delta(&self) -> f32 {
        self.sequence.borrow().as_ref().unwrap().get_delta()
    }

    fn get_gradient(&self) {
        self.sequence.borrow().as_ref().unwrap().get_gradient()
    }

    fn get_parameter(&self) {
        self.sequence.borrow().as_ref().unwrap().get_parameter()
    }

    fn get_parameter_gradient(&self) {
        self.sequence.borrow().as_ref().unwrap().get_parameter_gradient()
    }

    fn get_parameter_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_parameter_size()
    }

    fn get_parameter_gradient_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_parameter_gradient_size()
    }

    fn get_parameter_data(&self) {
        self.sequence.borrow().as_ref().unwrap().get_parameter_data()
    }

    fn set_output(&self, value: f32) {
        self.sequence.borrow().as_ref().unwrap().set_output(value)
    }

    fn set_input(&self, value: f32) {
        self.sequence.borrow().as_ref().unwrap().set_input(value)
    }

    fn set_delta(&self, value: f32) {
        self.sequence.borrow().as_ref().unwrap().set_delta(value)
    }

    fn set_gradient(&self) {
        self.sequence.borrow().as_ref().unwrap().set_gradient()
    }

    fn set_parameter(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter()
    }

    fn set_parameter_gradient(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_gradient()
    }

    fn set_parameter_size(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_size()
    }

    fn set_parameter_gradient_size(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_gradient_size()
    }

    fn set_parameter_offset(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_offset()
    }

    fn set_parameter_gradient_offset(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_gradient_offset()
    }

    fn set_parameter_data(&self) {
        self.sequence.borrow().as_ref().unwrap().set_parameter_data()
    }

    fn get_name(&self) -> String {
        self.name.borrow().clone()
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<GenericModel>()
    }

    fn get_type_name(&self) -> &str {
        nameof::name_of_type!(GenericModel)
    }

    fn get_id(&self) -> usize {
        todo!()
    }

    fn get_index(&self) -> usize {
        self.id
    }

    fn get_parent(&self) -> Option<Rc<dyn INode>> {
        todo!()
    }

    fn get_children(&self) -> Vec<Rc<dyn INode>> {
        self.sequence.borrow().as_ref().unwrap().get_children()
    }

    fn get_parents(&self) -> Vec<Rc<dyn INode>> {
        todo!()
    }

    fn get_children_size(&self) -> usize {
        self.sequence.borrow().as_ref().unwrap().get_children_size()
    }

    fn get_parents_size(&self) -> usize {
        0
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

    fn get_node_at_index(&self, i: usize) -> std::rc::Rc<dyn INode> {
        todo!()
    }

    fn get_input_node_at_index(&self, i: usize) -> Box<dyn INode> {
        todo!()
    }

    fn get_input_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode> {
        todo!()
    }

    fn get_input_value_at_index(&self, i: usize) -> f32 {
        todo!()
    }

    fn get_input_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32 {
        todo!()
    }

    fn get_output_node_at_index(&self, i: usize) -> Box<dyn INode> {
        todo!()
    }

    fn get_output_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode> {
        todo!()
    }

    fn get_output_value_at_index(&self, i: usize) -> f32 {
        todo!()
    }

    fn get_output_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32 {
        todo!()
    }

    fn check_validity(&self) -> std::result::Result<usize, usize> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

impl IModel for GenericModel {
    fn get_path(&self) -> String {
        todo!()
    }

    fn set_path(&self, value: String) {
        todo!()
    }

    fn load_model(&self, stream: Rc<dyn super::imodel::IModelStreamReader>) {
        todo!()
    }

    fn save_model(&self, stream: Rc<dyn super::imodel::IModelStreamWriter>) {
        todo!()
    }

    fn load_data(&self, stream: Rc<dyn super::imodel::IModelDataStreamReader>) {
        todo!()
    }

    fn save_data(&self, stream: Rc<dyn super::imodel::IModelDataStreamWriter>) {
        todo!()
    }

    fn train(&self, training_data: Rc<dyn super::imodel::ITrainingData>) {
        todo!()
    }

    fn predict(&self, input: Rc<dyn super::imodel::INodeData>) -> Rc<dyn super::imodel::INodeData> {
        todo!()
    }
}