use al_sys::*;

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

    // TODO: alcProcessContext, alcSuspendContext
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { alcDestroyContext(self.handle) };
    }
}
