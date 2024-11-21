//! abi_lab
//! A Rust library for exploring advanced Application Binary Interface (ABI) concepts,
//! focusing on interoperability with other languages and FFI.

#[repr(C)]
pub struct SimpleData {
    pub value: i32,
    pub flag: bool,
}

#[no_mangle]
pub extern "C" fn modify_simple_data(data: *mut SimpleData) {
    if !data.is_null() {
        unsafe {
            (*data).value += 1; // Increment value
            (*data).flag = true; // Set flag to true
        }
    }
}

#[repr(C)]
pub struct CallbackData {
    pub value: i32,
    pub callback: Option<extern "C" fn(i32)>,
}

#[no_mangle]
pub extern "C" fn register_callback(data: *mut CallbackData, callback: extern "C" fn(i32)) {
    if !data.is_null() {
        unsafe {
            (*data).callback = Some(callback);
        }
    }
}

#[no_mangle]
pub extern "C" fn trigger_callback(data: *mut CallbackData) {
    if !data.is_null() {
        unsafe {
            if let Some(callback) = (*data).callback {
                callback((*data).value);
            }
        }
    }
}

#[repr(C)]
pub struct ComplexData {
    pub id: u32,
    pub name: *const u8,
    pub name_len: usize,
    pub values: [f32; 5],
    pub callback: Option<extern "C" fn(*const ComplexData)>,
}

#[no_mangle]
pub extern "C" fn process_complex_data(data: *mut ComplexData) {
    if !data.is_null() {
        unsafe {
            if let Some(callback) = (*data).callback {
                callback(data);
            }
        }
    }
}
