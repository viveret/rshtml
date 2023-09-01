use std::{any::TypeId, rc::Rc};

use crate::node::inode::INode;

use super::ilayer::ILayer;



pub struct GenericLayer {
    pub name: Option<String>,
    pub id: usize,
    pub parent: Box<dyn INode>,
}

impl GenericLayer {
    pub fn new(parent: Box<dyn INode>) -> Self {
        Self {
            name: None,
            id: 0,
            parent: parent,
        }
    }
}

impl INode for GenericLayer {
    fn can_read(&self) -> bool {
        todo!()
    }

    fn can_write(&self) -> bool {
        todo!()
    }

    fn can_update(&self) -> bool {
        todo!()
    }

    fn get_input_node_width(&self) -> usize {
        todo!()
    }

    fn get_input_value_width(&self) -> usize {
        todo!()
    }

    fn get_input_node_height(&self) -> usize {
        todo!()
    }

    fn get_input_value_height(&self) -> usize {
        todo!()
    }

    fn get_input_node_depth(&self) -> usize {
        todo!()
    }

    fn get_input_value_depth(&self) -> usize {
        todo!()
    }

    fn get_output_node_width(&self) -> usize {
        todo!()
    }

    fn get_output_value_width(&self) -> usize {
        todo!()
    }

    fn get_output_node_height(&self) -> usize {
        todo!()
    }

    fn get_output_value_height(&self) -> usize {
        todo!()
    }

    fn get_output_node_depth(&self) -> usize {
        todo!()
    }

    fn get_output_value_depth(&self) -> usize {
        todo!()
    }

    fn get_input_node_size(&self) -> usize {
        todo!()
    }

    fn get_input_value_size(&self) -> usize {
        todo!()
    }

    fn get_output_node_size(&self) -> usize {
        todo!()
    }

    fn get_output_value_size(&self) -> usize {
        todo!()
    }

    fn forward(&self) {
        todo!()
    }

    fn backward(&self) {
        todo!()
    }

    fn update(&self) {
        todo!()
    }

    fn get_output(&self) -> f32 {
        todo!()
    }

    fn get_input(&self) -> f32 {
        todo!()
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

    fn set_output(&self, value: f32) {
        todo!()
    }

    fn set_input(&self, value: f32) {
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
        todo!()
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<GenericLayer>()
    }

    fn get_type_name(&self) -> &str {
        todo!()
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

impl ILayer for GenericLayer {
    
}