use std::{cell::RefCell, rc::Rc, any::TypeId};

use super::inode::INode;



pub struct ValueNode {
    pub value: f32,
    pub name: Option<String>,
    pub id: usize,
    pub parent: Box<dyn INode>,
    pub next_node: RefCell<Option<Rc<dyn INode>>>,
}

impl ValueNode {
    pub fn new(parent: Box<dyn INode>) -> Self {
        Self {
            name: None,
            id: 0,
            parent: parent,
            value: 0.0,
            next_node: RefCell::new(None),
        }
    }
}

impl INode for ValueNode {
    fn can_read(&self) -> bool {
        true
    }

    fn can_write(&self) -> bool {
        false
    }

    fn can_update(&self) -> bool {
        false
    }

    fn get_input_node_width(&self) -> usize {
        0
    }

    fn get_input_value_width(&self) -> usize {
        0
    }

    fn get_input_node_height(&self) -> usize {
        0
    }

    fn get_input_value_height(&self) -> usize {
        0
    }

    fn get_input_node_depth(&self) -> usize {
        0
    }

    fn get_input_value_depth(&self) -> usize {
        0
    }

    fn get_output_node_width(&self) -> usize {
        0
    }

    fn get_output_value_width(&self) -> usize {
        0
    }

    fn get_output_node_height(&self) -> usize {
        0
    }

    fn get_output_value_height(&self) -> usize {
        0
    }

    fn get_output_node_depth(&self) -> usize {
        0
    }

    fn get_output_value_depth(&self) -> usize {
        0
    }

    fn get_input_node_size(&self) -> usize {
        0
    }

    fn get_input_value_size(&self) -> usize {
        0
    }

    fn get_output_node_size(&self) -> usize {
        0
    }

    fn get_output_value_size(&self) -> usize {
        0
    }

    fn forward(&self) {
        match self.next_node.borrow().as_ref() {
            Some(node) => {
                node.as_ref().set_input(self.value);
            },
            None => {}
        }
    }

    fn backward(&self) {

    }

    fn update(&self) {

    }

    fn get_output(&self) -> f32 {
        self.value
    }

    fn get_input(&self) -> f32 {
        0.0
    }

    fn get_delta(&self) -> f32 {
        0.0
    }

    fn get_gradient(&self) {

    }

    fn get_parameter(&self) {

    }

    fn get_parameter_gradient(&self) {

    }

    fn get_parameter_size(&self) -> usize {
        0
    }

    fn get_parameter_gradient_size(&self) -> usize {
        0
    }

    fn get_parameter_data(&self) {

    }

    fn set_output(&self, output: f32) {

    }

    fn set_input(&self, input: f32) {

    }

    fn set_delta(&self, delta: f32) {
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
        todo!()
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ValueNode>()
    }

    fn get_type_name(&self) -> &str {
        nameof::name_of_type!(ValueNode)
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

    fn get_node_at_index(&self, i: usize) -> Rc<dyn INode> {
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