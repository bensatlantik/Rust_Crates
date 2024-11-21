use rust_gui_kit::{window::Window, button::Button};

#[test]
fn test_window_creation() {
    let window = Window::new("Test Window", 800, 600);
    window.show();
    assert_eq!(window.title, "Test Window");
    assert_eq!(window.width, 800);
    assert_eq!(window.height, 600);
}

#[test]
fn test_button_click() {
    let button = Button::new("Click Me");
    button.on_click(|| {
        println!("Button was clicked!");
    });
}
