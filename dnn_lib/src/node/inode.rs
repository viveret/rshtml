use std::{any::TypeId, rc::Rc};



pub trait INode {
    fn can_read(&self) -> bool;
    fn can_write(&self) -> bool;
    fn can_update(&self) -> bool;

    fn get_input_node_width(&self) -> usize;
    fn get_input_node_height(&self) -> usize;
    fn get_input_node_depth(&self) -> usize;
    fn get_input_node_size(&self) -> usize;

    fn get_input_node_at_index(&self, i: usize) -> Box<dyn INode>;
    fn get_input_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode>;

    fn get_input_value_width(&self) -> usize;
    fn get_input_value_height(&self) -> usize;
    fn get_input_value_depth(&self) -> usize;
    fn get_input_value_size(&self) -> usize;

    fn get_input_value_at_index(&self, i: usize) -> f32;
    fn get_input_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32;

    fn get_input(&self) -> f32;
    fn set_input(&self, value: f32);

    fn get_output_node_width(&self) -> usize;
    fn get_output_node_height(&self) -> usize;
    fn get_output_node_depth(&self) -> usize;
    fn get_output_node_size(&self) -> usize;

    fn get_output_node_at_index(&self, i: usize) -> Box<dyn INode>;
    fn get_output_node_at_xyz(&self, x: usize, y: usize, z: usize) -> Box<dyn INode>;

    fn get_output_value_width(&self) -> usize;
    fn get_output_value_height(&self) -> usize;
    fn get_output_value_depth(&self) -> usize;
    fn get_output_value_size(&self) -> usize;

    fn get_output_value_at_index(&self, i: usize) -> f32;
    fn get_output_value_at_xyz(&self, x: usize, y: usize, z: usize) -> f32;

    fn get_output(&self) -> f32;
    fn set_output(&self, value: f32);

    fn forward(&self);
    fn backward(&self);
    fn update(&self);
    
    fn get_delta(&self) -> f32;
    fn get_gradient(&self);
    fn get_parameter(&self);
    fn get_parameter_gradient(&self);
    fn get_parameter_size(&self) -> usize;
    fn get_parameter_gradient_size(&self) -> usize;
    fn get_parameter_data(&self);

    fn set_delta(&self, value: f32);
    fn set_gradient(&self);
    fn set_parameter(&self);
    fn set_parameter_gradient(&self);
    fn set_parameter_size(&self);
    fn set_parameter_gradient_size(&self);
    fn set_parameter_offset(&self);
    fn set_parameter_gradient_offset(&self);
    fn set_parameter_data(&self);

    fn get_name(&self) -> String;
    fn get_type_id(&self) -> TypeId;
    fn get_type_name(&self) -> &str;
    fn get_id(&self) -> usize;
    fn get_index(&self) -> usize;
    fn get_parent(&self) -> Option<Rc<dyn INode>>;
    fn get_children(&self) -> Vec<Rc<dyn INode>>;
    fn get_parents(&self) -> Vec<Rc<dyn INode>>;
    fn get_children_size(&self) -> usize;
    fn get_parents_size(&self) -> usize;

    fn check_validity(&self) -> std::result::Result<usize, usize>;
    fn is_valid(&self) -> bool;
    
    fn get_node_at_index(&self, i: usize) -> Rc<dyn INode>;

    fn set_name(&self, name: &String);
    fn set_id(&self, id: usize);
    fn set_index(&self, index: usize);
    fn set_parent(&self, parent: Option<Rc<dyn INode>>);
    fn set_children(&self, children: Vec<Rc<dyn INode>>);
    fn set_parents(&self, parents: Vec<Rc<dyn INode>>);
    fn set_children_size(&self, size: usize);
    fn set_parents_size(&self, size: usize);

}