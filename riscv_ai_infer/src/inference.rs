use crate::model_loader::Model;
use nalgebra::{DMatrix, DVector};

pub fn run_inference(model: &Model, input_data: Vec<f32>) -> Vec<f32> {
    let input_vector = DVector::from_vec(input_data);
    let mut output_vector = input_vector;

    for layer in &model.layers {
        let weight_matrix = DMatrix::from_vec(layer.output_size, layer.input_size, layer.weights.clone());
        let bias_vector = DVector::from_vec(layer.biases.clone());
        output_vector = forward_pass(&output_vector, &weight_matrix, &bias_vector);
    }
    output_vector.data.as_vec().clone()
}

fn forward_pass(inputs: &DVector<f32>, weights: &DMatrix<f32>, biases: &DVector<f32>) -> DVector<f32> {
    (weights * inputs) + biases
}
