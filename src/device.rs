use crate::{check_alc_error, sys::*, AllenError, AllenResult, Context};
use std::{ffi::CStr, ptr, sync::Arc};

pub(crate) struct DeviceInner {
    pub(crate) handle: *mut ALCdevice,
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe { alcCloseDevice(self.handle) };
    }
}

/// An OpenAL device.
#[derive(Clone)]
pub struct Device {
    pub(crate) inner: Arc<DeviceInner>,
}

impl Device {
    /// Opens a device with the specified name. Passing `None` will open the default device.
    pub fn open(device_name: Option<&CStr>) -> Option<Self> {
        let handle =
            unsafe { alcOpenDevice(device_name.map(|s| s.as_ptr()).unwrap_or(ptr::null())) };

        if handle == ptr::null_mut() {
            None
        } else {
            Some(Device {
                inner: Arc::new(DeviceInner { handle }),
            })
        }
    }

    /// The name of the device.
    pub fn device_name(&self) -> &str {
        unsafe { CStr::from_ptr(alcGetString(self.inner.handle, ALC_DEVICE_SPECIFIER)) }
            .to_str()
            .unwrap()
    }

    /// Creates a context under the device.
    pub fn create_context(&self) -> AllenResult<Context> {
        Context::new(self.clone())
    }

    pub fn is_extension_present(&self, name: &CStr) -> AllenResult<bool> {
        let result = unsafe { alcIsExtensionPresent(self.inner.handle, name.as_ptr()) };
        check_alc_error(self.inner.handle)?;
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
