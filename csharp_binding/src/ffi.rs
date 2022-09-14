use std::ptr::null_mut;

use code_generator::Entity;
use interoptopus::{ffi_type, ffi_function};
use interoptopus::patterns::string::AsciiPointer;
#[ffi_type(opaque)]
pub struct Generator {
    generator: Entity,
}
#[ffi_type]
#[repr(C)]
pub enum FFIError {
    Ok,
    NullPointerPassed = 10,
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create(context_ptr: Option<&mut *mut Generator>, path: AsciiPointer) -> FFIError {
    let context = context_ptr.unwrap();
    let entity = Box::new(Generator {
        generator: Entity::new(path.as_str().unwrap().to_string()).unwrap(),
    });
    *context = Box::into_raw(entity);
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_dto(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.create_dto().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn dispose(context_ptr: Option<&mut *mut Generator>) -> FFIError {
    if context_ptr.is_none() {
        return FFIError::NullPointerPassed;
    }
    let context = context_ptr.unwrap();
    {
        unsafe { Box::from_raw(*context) };
    }
    *context = null_mut();
    FFIError::Ok
}
