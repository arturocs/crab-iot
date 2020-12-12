use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Deserialize, Serialize)]
static STATUS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

#[repr(C)]
#[derive(Deserialize)]
pub struct VecPieces {
    data: usize,
    size: usize,
}

#[derive(Deserialize, Serialize)]
struct Error {
    error: String,
}

#[no_mangle]
pub extern "C" fn get_status(_: *const Value) -> *mut Result<Value, &'static str> {
    let on = STATUS.load(Ordering::Relaxed);
    let status = Box::new(Ok(json!({
        "on": on ,
    })));
    Box::into_raw(status)
}

fn vec_to_pieces(mut v: Vec<u8>) -> VecPieces {
    VecPieces {
        data: v.as_mut_ptr() as usize,
        size: v.len(),
    }
}

/// # Safety
///
/// status_data should come from from calling vec_to_pieces() on a Vec<u8>.

#[no_mangle]
pub unsafe extern "C" fn set_status(status_data: VecPieces) -> VecPieces {
    let data = Vec::from_raw_parts(
        status_data.data as *mut u8,
        status_data.size,
        status_data.size,
    );
    let status: Result<Status, _> = bincode::deserialize(&data);
    match status {
        Ok(s) => {
            STATUS.set(s.get());
            vec_to_pieces(data)
        }
        Err(e) => {
            let err = Error {
                error: e.to_string(),
            };
            let data = bincode::serialize(&err).unwrap();
            vec_to_pieces(data)
        }
    }
}
