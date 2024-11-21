pub mod model_loader;
pub mod inference;
pub mod optimizations;
pub mod utils;

pub use crate::model_loader::{load_model, Model};
pub use crate::inference::run_inference;
pub use crate::optimizations::optimize_for_riscv;

/// Entry function to load a model and run inference on input data
pub fn infer_from_model(model_path: &str, input_data: Vec<f32>) {
    // Load the model from a file
    if let Ok(model) = load_model(model_path) {
        // Optionally optimize the model for RISC-V
        let optimized_model = optimize_for_riscv(model);

        // Run inference on the input data
        let result = run_inference(&optimized_model, input_data);

        // Print the results
        println!("Inference result: {:?}", result);
    } else {
        println!("Failed to load the model");
    }
}
