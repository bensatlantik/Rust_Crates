// Module for window management

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Window {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub fn show(&self) {
        println!("Showing window: {} [{}x{}]", self.title, self.width, self.height);
        // Platform-specific code for creating and displaying the window
    }
}
