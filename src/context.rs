use al_sys::*;

use crate::get_string;

pub struct Context {
    handle: *mut ALCcontext,
}

impl Context {
    pub(crate) fn from_handle(handle: *mut ALCcontext) -> Context {
        Self { handle }
    }

    pub fn make_current(&self) {
        // alcMakeContextCurrent should NOT return false.
        assert_eq!(true as i8, unsafe { alcMakeContextCurrent(self.handle) });
    }

    pub fn is_current(&self) -> bool {
        let current_context = unsafe { alcGetCurrentContext() };
        current_context == self.handle
    }

    // These functions exist on context because they require a valid context to work.

    pub fn vendor(&self) -> &'static str {
        get_string(AL_VENDOR)
    }

    pub fn version(&self) -> &'static str {
        get_string(AL_VERSION)
    }

    pub fn renderer(&self) -> &'static str {
        get_string(AL_RENDERER)
    }

    pub fn extensions(&self) -> &'static str {
        get_string(AL_EXTENSIONS)
    }

    // TODO: alcProcessContext, alcSuspendContext
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { alcDestroyContext(self.handle) };
    }
}