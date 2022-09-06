mod buffer;
mod context;
mod device;
#[macro_use]
mod properties;
mod source;

use std::ffi::CStr;

pub use buffer::*;
pub use context::*;
pub use device::*;
pub(crate) use properties::*;
pub use source::*;

use al_sys::*;
use thiserror::Error;

/// For whatever reason, macros which take type parameters can't accept "[f32; 3]"
pub(crate) type Float3 = [f32; 3];

#[derive(Error, Debug)]
pub enum AllenError {
    #[error("an invalid name was passed")]
    InvalidName,
    #[error("a bad device was passed")]
    InvalidDevice,
    #[error("a bad context was passed")]
    InvalidContext,
    #[error("an invalid enum value was passed")]
    InvalidEnum,
    #[error("an invalid value was passed")]
    InvalidValue,
    #[error("the requested operation is not valid")]
    InvalidOperation,
    #[error("out of memory")]
    OutOfMemory,
    #[error("unknown OpenAL error: `{0}`")]
    Unknown(i32),
}

pub(crate) type AllenResult<T> = Result<T, AllenError>;

pub(crate) fn check_al_error() -> AllenResult<()> {
    let error = unsafe { alGetError() };

    if error == AL_NO_ERROR {
        Ok(())
    } else {
        Err(match error {
            AL_INVALID_NAME => AllenError::InvalidName,
            //AL_INVALID_DEVICE => AllenError::InvalidDevice,
            //AL_INVALID_CONTEXT => AllenError::InvalidContext,
            AL_INVALID_ENUM => AllenError::InvalidEnum,
            AL_INVALID_VALUE => AllenError::InvalidValue,
            AL_INVALID_OPERATION => AllenError::InvalidOperation,
            AL_OUT_OF_MEMORY => AllenError::OutOfMemory,
            e => AllenError::Unknown(e),
        })
    }
}

pub(crate) fn check_alc_error(device: *mut ALCdevice) -> AllenResult<()> {
    let error = unsafe { alcGetError(device) };

    if error == ALC_NO_ERROR {
        Ok(())
    } else {
        Err(match error {
            //ALC_INVALID_NAME => AllenError::InvalidName,
            ALC_INVALID_DEVICE => AllenError::InvalidDevice,
            ALC_INVALID_CONTEXT => AllenError::InvalidContext,
            ALC_INVALID_ENUM => AllenError::InvalidEnum,
            ALC_INVALID_VALUE => AllenError::InvalidValue,
            ALC_INVALID_OPERATION => AllenError::InvalidOperation,
            ALC_OUT_OF_MEMORY => AllenError::OutOfMemory,
            e => AllenError::Unknown(e),
        })
    }
}

pub(crate) fn get_string(param: ALenum) -> &'static str {
    unsafe { CStr::from_ptr(alGetString(param)) }
        .to_str()
        .unwrap() // Unwrap is justified because from what I understand, this SHOULD be a valid string.
}
