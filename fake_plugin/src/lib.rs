use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    ffi::CString,
    os::raw::c_char,
    sync::atomic::{AtomicBool, Ordering},
};
#[derive(Deserialize)]
struct Status {
    on: AtomicBool,
}
impl Status {
    fn get(&self) -> bool {
        *&self.on.load(Ordering::Relaxed)
    }
    fn set(&self, new_state: bool) {
        &self.on.store(new_state, Ordering::Relaxed);
    }
}
static STATUS: Lazy<Status> = Lazy::new(|| Status {
    on: AtomicBool::new(false),
});

#[no_mangle]
pub extern "C" fn get_status() -> *mut c_char {
    CString::new(format!(
        r#"{{
            "type": "switch",
            "data": {{ "on" : {} }}
            }}"#,
        STATUS.get()
    ))
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn set_status(status_str: *mut c_char) -> *mut c_char {
    let status_cstr = unsafe { CString::from_raw(status_str) };
    let status_struct: Result<Status, serde_json::Error> =
        serde_json::from_str(status_cstr.to_str().unwrap());
    match status_struct {
        Ok(s) => {
            STATUS.set(s.get());
            CString::new(format!(
                r#"{{
                    "type": "switch",
                    "data": {{ "on" : {} }}
                    }}"#,
                STATUS.get()
            ))
            .unwrap()
            .into_raw()
        }
        Err(e) => CString::new(format!(
            r#"{{
                    "Error": "status recived as argument is invalid",
                    "Extra_information": "{}"
                    }}"#,
            e
        ))
        .unwrap()
        .into_raw(),
    }
}
