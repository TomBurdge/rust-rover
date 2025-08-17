mod error;
use rover::return_coordinates;
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};

/// The result interface to go.
/// Since both rust and go both implement errors as return values, structuring the interface like
/// this can be particularly nice. There remains responsibility, on the go side (which I have done)
/// to parse the result from the FFI call into (value, err), where if err_str != "" =>
/// (,err_st).
///
/// Notice how we are using serde here to put this itself into a String, so that it can be parsed
/// via a json interface on the go side. In this [writeup by arcjet](https://blog.arcjet.com/webassembly-on-the-server-compiling-rust-to-wasm-and-executing-it-from-go/)
/// they comment that the overhead is relatively small for this jsonification.
///
/// The benefit of this is being able to give results like this, while avoiding messing with
/// boilerplating C struct types. The disadvantage is the Rust/Go boilerplating required.
#[derive(Serialize, Deserialize, Debug)]
struct CoordinatesResult {
    /// UTF-8 string with the result, or empty if there was an error.
    result: String,
    /// UTF-8 string with the error message, or empty on success.
    error: String,
}

/// # Safety
///
/// This function can be called from the C FFI via any language. Currently implemented is with go.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extern_return_coordinates(
    top_right: *const libc::c_char,
    instructions: *const libc::c_char,
) -> *const libc::c_char {
    fn c_err<S: Into<String>>(msg: S) -> *const libc::c_char {
        let coordinate_result = CoordinatesResult {
            result: "".to_string(),
            error: msg.into(),
        };
        CString::new(serde_json::to_string(&coordinate_result).unwrap())
            .unwrap()
            .into_raw()
    }

    if top_right.is_null() {
        return c_err(error::CError::NullPointerTopRight);
    }

    if instructions.is_null() {
        return c_err(error::CError::NullPointerInstruction);
    }

    let top_right = unsafe {
        CStr::from_ptr(top_right)
            .to_str()
            .map_err(|_| error::CError::InvalidUTF8TopRight)
            .map(|s| s.to_owned())
    };
    let instructions = unsafe {
        CStr::from_ptr(instructions)
            .to_str()
            .map_err(|_| error::CError::InvalidUTF8Instruction)
            .map(|s| s.to_owned())
    };

    let (top_right, instructions) = match (top_right, instructions) {
        (Ok(tr), Ok(ins)) => (tr, ins),

        // It's a little fiddly to match both at once
        (Err(e), _) | (_, Err(e)) => return c_err(e),
    };

    match return_coordinates(top_right, instructions) {
        Ok(vec_of_lines) => {
            let output = vec_of_lines.join("\n");
            // This would fail when there is an interior NUL, the possibility of which is pretty
            // small, since I wrote the code.
            // It would be possible to catch an error here too and adding another variation to the
            // err enum.
            CString::new(
                serde_json::to_string(&CoordinatesResult {
                    result: output,
                    error: "".to_string(),
                })
                .unwrap(),
            )
            .unwrap()
            .into_raw()
        }
        Err(e) => {
            let msg = e.to_string();
            CString::new(
                serde_json::to_string(&CoordinatesResult {
                    result: "".to_string(),
                    error: msg,
                })
                .unwrap(),
            )
            .unwrap()
            .into_raw()
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;
    use std::ffi::CString;

    // TODO: check the output type
    #[test]
    fn simulated_main_function() {
        let top_right = CString::new("5 5").unwrap().into_raw();
        let instructions = CString::new(
            "1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM",
        )
        .unwrap()
        .into_raw();
        extern_return_coordinates(top_right, instructions);
    }
}
