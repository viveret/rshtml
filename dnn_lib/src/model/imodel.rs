use std::rc::Rc;

use crate::node::inode::INode;

pub trait INodeData {
    fn get_size(&self) -> usize;
    
    fn as_vec(&self) -> Vec<f32>;
    fn as_2d_vec(&self, w: usize) -> Vec<Vec<f32>>;
    fn as_3d_vec(&self, w: usize, h: usize) -> Vec<Vec<Vec<f32>>>;
    fn as_4d_vec(&self, w: usize, h: usize, h: usize) -> Vec<Vec<Vec<Vec<f32>>>>;
}

pub trait ITrainingData {
    fn get_input(&self) -> Vec<Rc<dyn INodeData>>;
    fn get_output(&self) -> Vec<Rc<dyn INodeData>>;

    fn get_epoch_size(&self) -> usize;
    fn get_batch_size(&self) -> usize;
    fn get_batch_count(&self) -> usize;

    fn set_epoch_size(&self, value: usize);
    fn set_batch_size(&self, value: usize);
    fn set_batch_count(&self, value: usize);
}

pub trait IModelStreamReader {

}

pub trait IModelStreamWriter {

}

pub trait IModelDataStreamReader {

}

pub trait IModelDataStreamWriter {

}

pub trait IModel: INode {
    fn get_path(&self) -> String;
    fn set_path(&self, value: String); 

    fn load_model(&self, stream: Rc<dyn IModelStreamReader>);
    fn save_model(&self, stream: Rc<dyn IModelStreamWriter>);

    fn load_data(&self, stream: Rc<dyn IModelDataStreamReader>);
    fn save_data(&self, stream: Rc<dyn IModelDataStreamWriter>);

    fn train(&self, training_data: Rc<dyn ITrainingData>);
    fn predict(&self, input: Rc<dyn INodeData>) -> Rc<dyn INodeData>;
}