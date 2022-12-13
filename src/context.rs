use crate::{get_string, sys::*, AllenResult, Buffer, Device, Listener, Source};
use lazy_static::lazy_static;
use std::{
    ffi::CString,
    ptr,
    sync::{Arc, Mutex, MutexGuard},
};

lazy_static! {
    static ref SINGLE_CONTEXT_LOCK: Mutex<()> = Mutex::new(());
}

pub(crate) struct ContextInner {
    handle: *mut ALCcontext,
    device: Device,
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        unsafe { alcDestroyContext(self.handle) };
        // TODO: Warn on drop fail.
    }
}

/// An OpenAL context.
#[derive(Clone)]
pub struct Context {
    inner: Arc<ContextInner>,
}

impl Context {
    pub(crate) fn new(device: Device) -> AllenResult<Context> {
        let handle = unsafe { alcCreateContext(device.inner.handle, ptr::null()) }; // TODO: support the attrlist parameter.

        if handle == ptr::null_mut() {
            Err(device.check_alc_error().expect_err("handle is null"))
        } else {
            Ok(Self {
                inner: Arc::new(ContextInner { handle, device }),
            })
        }
    }

    /// Locks the current context into self for the entire thread (if not possible, entire process).
    pub fn make_current(&self) -> Option<MutexGuard<()>> {
        // Try for thread first.
        let function: PFNALCSETTHREADCONTEXTPROC = unsafe {
            let name = CString::new("alcSetThreadContext").unwrap();

            std::mem::transmute(alcGetProcAddress(
                ptr::null_mut(),
                name.as_ptr() as *const ALCchar,
            ))
        };

        if let Some(function) = function {
            unsafe {
                function(self.inner.handle);
            }
            None
        } else {
            // Plan B: Just use alcMakeContextCurrent.
            // alcMakeContextCurrent should NOT return false.
            assert_eq!(true as i8, unsafe {
                alcMakeContextCurrent(self.inner.handle)
            });
            Some(SINGLE_CONTEXT_LOCK.lock().unwrap())
        }
    }

    pub fn is_current(&self) -> bool {
        let current_context = {
            // Try for thread first.
            let function: PFNALCGETTHREADCONTEXTPROC = unsafe {
                let name = CString::new("alcGetThreadContext").unwrap();

                std::mem::transmute(alcGetProcAddress(
                    ptr::null_mut(),
                    name.as_ptr() as *const ALCchar,
                ))
            };

            if let Some(function) = function {
                unsafe { function() }
            } else {
                // Plan B: Just use alcGetCurrentContext.
                unsafe { alcGetCurrentContext() }
            }
        };

        current_context == self.inner.handle
    }

    // These functions exist on context because they require a valid context to work.

    pub fn vendor(&self) -> &'static str {
        let _lock = self.make_current();
        get_string(AL_VENDOR)
    }

    pub fn version(&self) -> &'static str {
        let _lock = self.make_current();
        get_string(AL_VERSION)
    }

    pub fn renderer(&self) -> &'static str {
        let _lock = self.make_current();
        get_string(AL_RENDERER)
    }

    pub fn extensions(&self) -> &'static str {
        let _lock = self.make_current();
        get_string(AL_EXTENSIONS)
    }

    pub fn listener(&self) -> Listener {
        Listener::new(self.clone())
    }

    pub fn new_buffer(&self) -> AllenResult<Buffer> {
        Buffer::new(self.clone())
    }

    pub fn new_source(&self) -> AllenResult<Source> {
        Source::new(self.clone())
    }

    pub fn suspend(&self) -> AllenResult<()> {
        let _lock = self.make_current();
        unsafe {
            alcSuspendContext(self.inner.handle);
        }
        self.inner.device.check_alc_error()?;
        Ok(())
    }

    pub fn process(&self) -> AllenResult<()> {
        let _lock = self.make_current();
        unsafe {
            alcProcessContext(self.inner.handle);
        }
        self.inner.device.check_alc_error()?;
        Ok(())
    }
}
