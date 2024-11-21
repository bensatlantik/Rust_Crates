/// Utility functions (placeholder for future functionality)
pub fn normalize_input(data: Vec<f32>) -> Vec<f32> {
    let max = data.iter().cloned().fold(f32::MIN, f32::max);
    data.iter().map(|x| x / max).collect()
}
