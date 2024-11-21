use rust_gui_kit::window::Window;

fn main() {
    // Initialize the library (if needed)
    rust_gui_kit::initialize();

    // Create and show a basic window
    let window = Window::new("Basic Window", 800, 600);
    window.show();
}
