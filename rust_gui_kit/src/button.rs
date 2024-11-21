// Module for button components

pub struct Button {
    pub label: String,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            label: label.to_string(),
        }
    }

    pub fn on_click<F: Fn() + 'static>(&self, callback: F) {
        println!("Button clicked: {}", self.label);
        // Code to register the click event
        callback();
    }
}
