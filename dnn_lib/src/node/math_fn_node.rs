use std::{rc::Rc, cell::RefCell, any::TypeId};

use super::inode::INode;

pub trait IMathFn {
    fn math_fn(&self, v: f32) -> f32;
    fn math_fn_name(&self) -> &str;
}

pub trait IMathFnNode: INode {
    fn math_fn(&self) -> Rc<dyn IMathFn>;
}

pub struct MathFnNode {
    math_fn_to_use: Rc<dyn IMathFn>,
    input_value: RefCell<f32>,
    output_value: RefCell<f32>,
    delta_value: RefCell<f32>,

    next_node: RefCell<Option<Rc<dyn INode>>>,
}

impl MathFnNode {
    pub fn sin() -> Self {
        Self {
            math_fn_to_use: Rc::new(SinFn {}),
            input_value: RefCell::new(0.0),
            output_value: RefCell::new(0.0),
            delta_value: RefCell::new(0.0),
            next_node: RefCell::new(None),
        }
    }
}

impl INode for MathFnNode {
    fn can_read(&self) -> bool {
        true
    }

    fn can_write(&self) -> bool {
        true
    }

    fn can_update(&self) -> bool {
        true
    }

    fn get_input_node_width(&self) -> usize {
        1
    }

    fn get_input_value_width(&self) -> usize {
        1
    }

    fn get_input_node_height(&self) -> usize {
        1
    }

    fn get_input_value_height(&self) -> usize {
        1
    }

    fn get_input_node_depth(&self) -> usize {
        1
    }

    fn get_input_value_depth(&self) -> usize {
        1
    }

    fn get_output_node_width(&self) -> usize {
        1
    }

    fn get_output_value_width(&self) -> usize {
        1
    }

    fn get_output_node_height(&self) -> usize {
        1
    }

    fn get_output_value_height(&self) -> usize {
        1
    }

    fn get_output_node_depth(&self) -> usize {
        1
    }

    fn get_output_value_depth(&self) -> usize {
        1
    }

    fn get_input_node_size(&self) -> usize {
        1
    }

    fn get_input_value_size(&self) -> usize {
        1
    }

    fn get_output_node_size(&self) -> usize {
        1
    }

    fn get_output_value_size(&self) -> usize {
        1
    }

    fn forward(&self) {
        match self.next_node.borrow().as_ref() {
            Some(next_node) => {
                next_node.as_ref().set_input(*self.output_value.borrow());
            }
            None => {
                self.update();
            }
        }
    }

    fn backward(&self) {
        todo!()
    }

    fn update(&self) {
        let original_value = *self.output_value.borrow();
        let new_value = self.math_fn_to_use.math_fn(*self.input_value.borrow());
        *self.output_value.borrow_mut() = new_value;
        // not sure if this is correct use of delta
        *self.delta_value.borrow_mut() = original_value - new_value;
    }

    fn get_output(&self) -> f32 {
        *self.input_value.borrow()
    }

    fn get_input(&self) -> f32 {
        *self.output_value.borrow()
    }

    fn get_delta(&self) -> f32 {
        *self.delta_value.borrow()
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

    fn set_output(&self, output: f32) {
        self.output_value.replace(output);
    }

    fn set_input(&self, input: f32) {
        self.input_value.replace(input);
    }

    fn set_delta(&self, delta: f32) {
        self.delta_value.replace(delta);
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
        TypeId::of::<MathFnNode>()
    }

    fn get_type_name(&self) -> &str {
        nameof::name_of_type!(MathFnNode)
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

    fn get_node_at_index(&self, i: usize) -> Rc<dyn INode> {
        todo!()
    }

    fn check_validity(&self) -> std::result::Result<usize, usize> {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

impl IMathFnNode for MathFnNode {
    fn math_fn(&self) -> Rc<dyn IMathFn> {
        self.math_fn_to_use.clone()
    }
}


pub struct SinFn {

}

impl IMathFn for SinFn {
    fn math_fn(&self, v: f32) -> f32 {
        v.sin()
    }

    fn math_fn_name(&self) -> &str {
        "sin"
    }
}
