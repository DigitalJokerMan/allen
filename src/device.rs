use crate::{check_alc_error, AllenResult, Context};
use al_sys::*;
use std::{ffi::CStr, ptr};

pub struct Device {
    handle: *mut ALCdevice,
}

impl Device {
    pub fn open(device_name: Option<&CStr>) -> Option<Self> {
        let handle =
            unsafe { alcOpenDevice(device_name.map(|s| s.as_ptr()).unwrap_or(ptr::null())) };

        if handle == ptr::null_mut() {
            None
        } else {
            Some(Device { handle })
        }
    }

    pub fn device_name(&self) -> &str {
        unsafe { CStr::from_ptr(alcGetString(self.handle, ALC_DEVICE_SPECIFIER)) }
            .to_str()
            .unwrap()
    }

    pub fn create_context(&self) -> AllenResult<Context> {
        let handle = unsafe { alcCreateContext(self.handle, ptr::null()) }; // TODO: support the attrlist parameter.

        if handle == ptr::null_mut() {
            Err(check_alc_error(self.handle).expect_err("handle is null"))
        } else {
            Ok(Context::from_handle(handle))
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { alcCloseDevice(self.handle) };
    }
}
