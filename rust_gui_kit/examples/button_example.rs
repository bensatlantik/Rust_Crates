use rust_gui_kit::{window::Window, button::Button};

fn main() {
    // Initialize the library (if needed)
    rust_gui_kit::initialize();

    // Create a window
    let window = Window::new("Button Example", 800, 600);
    window.show();

    // Create a button and define its click event
    let button = Button::new("Click Me");
    button.on_click(|| {
        println!("Button was clicked!");
    });
}
