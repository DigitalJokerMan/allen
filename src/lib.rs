mod buffer;
mod context;
mod device;
#[macro_use]
mod properties;
mod listener;
mod source;
pub(crate) mod sys;

use crate::sys::*;
pub use buffer::*;
pub use context::*;
pub use device::*;
pub use listener::*;
pub(crate) use properties::*;
pub use source::*;
use std::ffi::CStr;
use thiserror::Error;

/// For whatever reason, macros which take type parameters can't accept "[f32; 3]"
pub(crate) type Float3 = [f32; 3];

/// Used to define the orientation of a listener.
#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Orientation {
    pub up: Float3,
    pub at: Float3,
}

/// An OpenAL error.
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

    #[error("missing OpenAL extension: {0}")]
    MissingExtension(String),
}

pub(crate) type AllenResult<T> = Result<T, AllenError>;

pub(crate) fn check_al_error() -> AllenResult<()> {
    let error = unsafe { alGetError() };

    if error == AL_NO_ERROR {
        Ok(())
    } else {
        Err(match error {
            AL_INVALID_NAME => AllenError::InvalidName,
            AL_INVALID_ENUM => AllenError::InvalidEnum,
            AL_INVALID_VALUE => AllenError::InvalidValue,
            AL_INVALID_OPERATION => AllenError::InvalidOperation,
            AL_OUT_OF_MEMORY => AllenError::OutOfMemory,
            e => AllenError::Unknown(e),
        })
    }
}

pub(crate) fn get_string(param: ALenum) -> &'static str {
    unsafe { CStr::from_ptr(alGetString(param)) }
        .to_str()
        .unwrap() // Unwrap is justified because from what I understand, this SHOULD be a valid string.
}

pub fn is_extension_present(name: &CStr) -> AllenResult<bool> {
    let result = unsafe { alIsExtensionPresent(name.as_ptr()) };
    check_al_error()?;
    Ok(result != 0)
}

pub(crate) fn check_al_extension(name: &CStr) -> AllenResult<()> {
    if is_extension_present(name)? {
        Ok(())
    } else {
        Err(AllenError::MissingExtension(
            // This seemed to be the best non error-prone way to convert &CStr to String.
            name.to_string_lossy().to_string(),
        ))
    }
}
