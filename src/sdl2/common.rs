use std::ffi::{CString, NulError};
use std::ops::Add;

/// A given integer was so big that its representation as a C integer would be
/// negative.
#[derive(Debug)]
pub enum IntegerOrSdlError {
    IntegerOverflows(&'static str, u32),
    SdlError(String)
}

/// Validates and converts the given u32 to a positive C integer.
pub fn validate_int(value: u32, name: &'static str)
        -> Result<::libc::c_int, IntegerOrSdlError> {
    use self::IntegerOrSdlError::*;
    // Many SDL functions will accept `int` values, even if it doesn't make sense 
    // for the values to be negative.
    // In the cases that SDL doesn't check negativity, passing negative values 
    // could be unsafe.
    // For example, `SDL_JoystickGetButton` uses the index argument to access an 
    // array without checking if it's negative, which could potentially lead to 
    // segmentation faults.
    if value >= 1 << 31 { 
        Err(IntegerOverflows(name, value))
    } else { 
        Ok(value as ::libc::c_int)
    }
}

#[cfg(test)]
mod test {
}
