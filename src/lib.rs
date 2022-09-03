use thiserror::Error;

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
    Unknown(u32),
}

pub(crate) type AllenResult<T> = Result<T, AllenError>;

pub(crate) fn check_al_error() -> AllenResult<()> {
    use al_sys::*;

    let error = unsafe { alGetError() } as u32;

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
    use al_sys::*;

    let error = unsafe { alcGetError(device) } as u32;

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
