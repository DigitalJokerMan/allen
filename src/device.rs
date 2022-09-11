use crate::{check_alc_error, sys::*, AllenResult, Context};
use std::{ffi::CStr, ptr};

/// An OpenAL device.
pub struct Device {
    handle: *mut ALCdevice,
}

impl Device {
    /// Opens a device with the specified name. Passing `None` will open the default device.
    pub fn open(device_name: Option<&CStr>) -> Option<Self> {
        let handle =
            unsafe { alcOpenDevice(device_name.map(|s| s.as_ptr()).unwrap_or(ptr::null())) };

        if handle == ptr::null_mut() {
            None
        } else {
            Some(Device { handle })
        }
    }

    /// The name of the device.
    pub fn device_name(&self) -> &str {
        unsafe { CStr::from_ptr(alcGetString(self.handle, ALC_DEVICE_SPECIFIER)) }
            .to_str()
            .unwrap()
    }

    /// Creates a context under the device.
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
