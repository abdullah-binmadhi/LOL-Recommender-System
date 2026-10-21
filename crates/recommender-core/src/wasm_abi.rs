use crate::Engine;
use serde_json::Value;
use std::{cell::RefCell, mem, slice};

const MAX_INPUT_BYTES: usize = 1024 * 1024;

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
    static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static ERROR: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Allocate a byte buffer in this module's linear memory for a JS caller to fill.
#[no_mangle]
pub extern "C" fn scoring_alloc(length: usize) -> *mut u8 {
    let mut bytes = vec![0_u8; length].into_boxed_slice();
    let pointer = bytes.as_mut_ptr();
    mem::forget(bytes);
    pointer
}

/// Release a buffer previously returned by `scoring_alloc`.
///
/// # Safety
/// The pointer and length must be the exact pair returned by `scoring_alloc`, once only.
#[no_mangle]
pub unsafe extern "C" fn scoring_dealloc(pointer: *mut u8, length: usize) {
    if !pointer.is_null() {
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            pointer, length,
        )));
    }
}

/// Score a UTF-8 JSON answers object. Returns 0 on success; otherwise inspect
/// `scoring_error_ptr/len`. Both result and error buffers live until the next call.
///
/// # Safety
/// For nonzero length, the pointer must identify `length` readable bytes in this
/// module's linear memory. The input is copied/parsed before the function returns.
#[no_mangle]
pub unsafe extern "C" fn scoring_recommend(pointer: *const u8, length: usize) -> u32 {
    OUTPUT.with(|output| output.borrow_mut().clear());
    ERROR.with(|error| error.borrow_mut().clear());

    if length > MAX_INPUT_BYTES {
        set_error("Questionnaire input exceeds the 1 MiB limit");
        return 1;
    }
    if pointer.is_null() && length != 0 {
        set_error("Questionnaire input buffer is null");
        return 1;
    }
    let input = if length == 0 {
        &[]
    } else {
        slice::from_raw_parts(pointer, length)
    };
    let answers: Value = match serde_json::from_slice(input) {
        Ok(value) => value,
        Err(_) => {
            set_error("Questionnaire input must be valid JSON");
            return 1;
        }
    };

    let result = ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if engine.is_none() {
            *engine = Some(Engine::reference().map_err(|_| "Scoring model could not initialize")?);
        }
        engine
            .as_ref()
            .expect("initialized scoring engine")
            .recommend(&answers)
            .map_err(|_| "Questionnaire could not be scored")
    });

    let result = match result {
        Ok(result) => result,
        Err(message) => {
            set_error(message);
            return 2;
        }
    };
    let encoded = match serde_json::to_vec(&result) {
        Ok(encoded) => encoded,
        Err(_) => {
            set_error("Scoring output could not be encoded");
            return 3;
        }
    };
    OUTPUT.with(|output| *output.borrow_mut() = encoded);
    0
}

#[no_mangle]
pub extern "C" fn scoring_result_ptr() -> *const u8 {
    OUTPUT.with(|output| output.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn scoring_result_len() -> usize {
    OUTPUT.with(|output| output.borrow().len())
}

#[no_mangle]
pub extern "C" fn scoring_error_ptr() -> *const u8 {
    ERROR.with(|error| error.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn scoring_error_len() -> usize {
    ERROR.with(|error| error.borrow().len())
}

fn set_error(message: &str) {
    ERROR.with(|error| *error.borrow_mut() = message.as_bytes().to_vec());
}
