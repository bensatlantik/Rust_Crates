use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Model {
    pub layers: Vec<Layer>,
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    pub input_size: usize,
    pub output_size: usize,
    pub weights: Vec<f32>,
    pub biases: Vec<f32>,
}

pub fn load_model(file_path: &str) -> Result<Model, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let model: Model = serde_json::from_reader(reader)?;
    Ok(model)
}
