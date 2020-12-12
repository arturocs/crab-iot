use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use utils::{error::Error, *};
static STATUS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

#[no_mangle]
pub extern "C" fn get_status(_: *const Value) -> *mut Result<Value, &'static str> {
    let on = STATUS.load(Ordering::Relaxed);
    let status = Box::new(Ok(json!({
        "on": on ,
    })));
    Box::into_raw(status)
}

fn ffi_error(error_msg: &str) -> *mut Result<Value, Error> {
    Box::into_raw(Box::new(Err(error!(error_msg))))
}

/// # Safety
///
/// status_data should come from from calling Box::into_raw() in a boxed serde_json::Value.

#[no_mangle]
pub unsafe extern "C" fn set_status(status_data: *const Value) -> *mut Result<Value, Error> {
    match status_data.as_ref() {
        Some(v) => match v.get("on") {
            Some(status) => match status {
                Value::Bool(b) => {
                    STATUS.store(*b, Ordering::Relaxed);
                    Box::into_raw(Box::new(Ok(json! ({ "on": *STATUS }))))
                }
                _ => ffi_error("'on' value is not a boolean"),
            },
            None => ffi_error("Value does not contains 'on' field"),
        },
        None => ffi_error("Unable to dereference value"),
    }
}
