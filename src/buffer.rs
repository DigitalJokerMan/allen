use crate::{check_al_error, check_al_extension, sys::*, AllenResult, Context};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CString},
    mem::size_of,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Channels {
    /// One audio channel.
    Mono,
    /// Two audio channels; one left & one right.
    Stereo,
}

/// Container for OpenAL buffer data to be passed into [`Buffer::data`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BufferData<'a> {
    /// AL_FORMAT_*8
    I8(&'a [i8]),
    /// AL_FORMAT_*16
    I16(&'a [i16]),
    /// AL_FORMAT_*_FLOAT32; requires extension ``AL_EXT_float32``.
    F32(&'a [f32]),
    /// AL_FORMAT_*_DOUBLE_EXT; requires extension ``AL_EXT_double``.
    F64(&'a [f64]),
}

impl BufferData<'_> {
    fn ptr(&self) -> *const c_void {
        match self {
            BufferData::I8(data) => data.as_ptr() as *const c_void,
            BufferData::I16(data) => data.as_ptr() as *const c_void,
            BufferData::F32(data) => data.as_ptr() as *const c_void,
            BufferData::F64(data) => data.as_ptr() as *const c_void,
        }
    }

    fn size(&self) -> usize {
        match self {
            BufferData::I8(data) => size_of::<i8>() * data.len(),
            BufferData::I16(data) => size_of::<i16>() * data.len(),
            BufferData::F32(data) => size_of::<f32>() * data.len(),
            BufferData::F64(data) => size_of::<f64>() * data.len(),
        }
    }
}

/// Buffer of audio data.
/// NOTE: Buffers are bound to a device.
/// To ensure safety, buffers are not allowed to be cloned. There can only be one instance per-handle.
pub struct Buffer {
    handle: u32,
    context: Context,
}

impl Buffer {
    pub(crate) fn new(context: Context) -> AllenResult<Self> {
        let handle = {
            let mut handle = 0;
            unsafe {
                let _lock = context.make_current();
                alGenBuffers(1, &mut handle)
            };

            check_al_error()?;

            handle
        };

        Ok(Self { handle, context })
    }

    pub(crate) fn handle(&self) -> u32 {
        self.handle
    }

    /// Fills the buffer with data.
    pub fn data(&self, data: BufferData, channels: Channels, sample_rate: i32) -> AllenResult<()> {
        let _lock = self.context.make_current();

        let format = match data {
            BufferData::I8(_) => match channels {
                Channels::Mono => AL_FORMAT_MONO8,
                Channels::Stereo => AL_FORMAT_MONO16,
            },
            BufferData::I16(_) => match channels {
                Channels::Mono => AL_FORMAT_MONO16,
                Channels::Stereo => AL_FORMAT_STEREO16,
            },
            BufferData::F32(_) => {
                check_al_extension(&CString::new("AL_EXT_float32").unwrap())?;
                match channels {
                    Channels::Mono => AL_FORMAT_MONO_FLOAT32,
                    Channels::Stereo => AL_FORMAT_STEREO_FLOAT32,
                }
            }
            BufferData::F64(_) => {
                check_al_extension(&CString::new("AL_double").unwrap())?;
                match channels {
                    Channels::Mono => AL_FORMAT_MONO_DOUBLE_EXT,
                    Channels::Stereo => AL_FORMAT_STEREO_DOUBLE_EXT,
                }
            }
        };

        unsafe {
            alBufferData(
                self.handle,
                format,
                data.ptr(),
                data.size() as i32,
                sample_rate,
            )
        };

        check_al_error()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { alDeleteBuffers(1, &self.handle) }
        if let Err(err) = check_al_error() {
            println!("WARNING: Buffer drop failed! {}", err);
        }
    }
}
