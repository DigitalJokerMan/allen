use crate::{check_alc_error, sys::*, AllenError, AllenResult, Context};
use std::{
    ffi::{CStr, CString},
    ptr,
};

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

    pub fn is_extension_present(&self, name: &CStr) -> AllenResult<bool> {
        let result = unsafe { alcIsExtensionPresent(self.handle, name.as_ptr()) };
        check_alc_error(self.handle)?;
        Ok(result != 0)
    }

    pub fn check_alc_extension(&self, name: &CStr) -> AllenResult<()> {
        if self.is_extension_present(name)? {
            Ok(())
        } else {
            Err(AllenError::MissingExtension(
                // This seemed to be the best non error-prone way to convert &CStr to String.
                name.to_string_lossy().to_string(),
            ))
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { alcCloseDevice(self.handle) };
        if let Err(err) = check_alc_error(self.handle) {
            println!("WARNING: Device drop failed! {}", err);
        }
    }
}
