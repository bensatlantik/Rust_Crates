use abi_lab::{SimpleData, modify_simple_data, CallbackData, register_callback, trigger_callback, ComplexData, process_complex_data};

fn main() {
    // 1. SimpleData Example
    let mut data = SimpleData {
        value: 10,
        flag: false,
    };

    println!("Before modification: Value = {}, Flag = {}", data.value, data.flag);
    modify_simple_data(&mut data as *mut _);
    println!("After modification: Value = {}, Flag = {}", data.value, data.flag);

    // 2. CallbackData Example
    extern "C" fn example_callback(val: i32) {
        println!("Callback triggered with value: {}", val);
    }

    let mut callback_data = CallbackData {
        value: 42,
        callback: None,
    };

    println!("\nRegistering and triggering a callback...");
    register_callback(&mut callback_data as *mut _, example_callback);
    trigger_callback(&mut callback_data as *mut _);

    // 3. ComplexData Example
    extern "C" fn complex_callback(data: *const ComplexData) {
        unsafe {
            println!(
                "ComplexData Callback triggered with ID: {}, Name: {:?}",
                (*data).id,
                std::str::from_utf8(std::slice::from_raw_parts((*data).name, (*data).name_len))
                    .unwrap_or("Invalid UTF-8")
            );
        }
    }

    let mut complex_data = ComplexData {
        id: 99,
        name: b"Example\0".as_ptr(),
        name_len: 7,
        values: [1.0, 2.0, 3.0, 4.0, 5.0],
        callback: Some(complex_callback),
    };

    println!("\nProcessing ComplexData...");
    process_complex_data(&mut complex_data as *mut _);
}
