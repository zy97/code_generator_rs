use std::{ptr::null_mut, ffi::CStr, };

use code_generator::Entity;
use interoptopus::{
    ffi_function, ffi_type,
    patterns::{string::AsciiPointer},
};
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
// #[ffi_type]
// #[repr(C)]
// pub struct CustomService {
//     // pub is_custom: FFIOption<bool>,
//     pub is_custom: Option<FFIBool>,
// }
#[ffi_type]
#[repr(C)]
pub struct ExceptionInfo {
    pub excetpion_name: *const u8,
    pub excetpion_code:  *const u8,
    pub excetpion_displayname:  *const u8,
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
pub extern "C" fn create_createorupdatedto(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.create_createorupdatedto().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_ef_repository(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.create_ef_repository().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_exception(
    context_ptr: Option<&mut Generator>,
    exception: ExceptionInfo,
) -> FFIError {
    let context = context_ptr.unwrap();
    let exception_name;
    let excetpion_code;
    let excetpion_displayname;
   unsafe{
     exception_name =CStr::from_ptr(exception.excetpion_name as *const i8).to_str().ok().map(|s|s.to_owned());
     excetpion_code = CStr::from_ptr(exception.excetpion_code as *const i8).to_str().ok().map(|s|s.to_owned());
     excetpion_displayname = CStr::from_ptr(exception.excetpion_displayname as *const i8).to_str().ok().map(|s|s.to_owned());
   } 
   
    context
        .generator
        .create_exception(exception_name, excetpion_code, excetpion_displayname)
        .unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_iservice(
    context_ptr: Option<&mut Generator>,
    custom: Option<&bool>,
) -> FFIError {
    let context = context_ptr.unwrap();
   let custom = match custom {
       Some(x) =>*x,
       None => false,
   };
    context.generator.create_iservice(custom).unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_manager(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.create_manager().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_mancreate_pagedandsortedandfilterresultdtoager(
    context_ptr: Option<&mut Generator>,
) -> FFIError {
    let context = context_ptr.unwrap();
    context
        .generator
        .create_pagedandsortedandfilterresultdto()
        .unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_repository_interface(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.create_repository_interface().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn create_service(
    context_ptr: Option<&mut Generator>,
    custom: Option<&bool>,
) -> FFIError {
    let context = context_ptr.unwrap();
    let custom = match custom {
        Some(x) =>*x,
        None => false,
    };
    context.generator.create_service(custom).unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn insert_mapper(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.insert_mapper().unwrap();
    FFIError::Ok
}
#[ffi_function]
#[no_mangle]
pub extern "C" fn format_all(context_ptr: Option<&mut Generator>) -> FFIError {
    let context = context_ptr.unwrap();
    context.generator.format_all();
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
