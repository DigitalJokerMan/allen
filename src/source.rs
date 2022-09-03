use crate::{check_al_error, AllenResult, Buffer};
use al_sys::*;
use std::{ffi::c_void, mem::size_of};

/// NOTE: Sources are bound to a context.
pub struct Source {
    handle: u32,
}

#[allow(dead_code)]
impl Source {
    fn get_f(&self, param: i32) -> f32 {
        let result = unsafe {
            let mut value = 0.0;
            alGetSourcef(self.handle, param, &mut value);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn get_3f(&self, param: i32) -> [f32; 3] {
        let result = unsafe {
            let mut value = [0.0, 0.0, 0.0];
            alGetSource3f(
                self.handle,
                param,
                &mut value[0],
                &mut value[1],
                &mut value[2],
            );
            value
        };

        check_al_error().unwrap();

        result
    }

    fn get_i(&self, param: i32) -> i32 {
        let result = unsafe {
            let mut value = 0;
            alGetSourcei(self.handle, param, &mut value);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn get_3i(&self, param: i32) -> [i32; 3] {
        let result = unsafe {
            let mut value = [0, 0, 0];
            alGetSource3i(
                self.handle,
                param,
                &mut value[0],
                &mut value[1],
                &mut value[2],
            );
            value
        };

        check_al_error().unwrap();

        result
    }

    fn set_f(&self, param: i32, value: f32) -> AllenResult<()> {
        unsafe { alSourcef(self.handle, param, value) };
        check_al_error()
    }

    fn set_3f(&self, param: i32, value: [f32; 3]) -> AllenResult<()> {
        unsafe { alSource3f(self.handle, param, value[0], value[1], value[2]) };
        check_al_error()
    }

    fn set_i(&self, param: i32, value: i32) -> AllenResult<()> {
        unsafe { alSourcei(self.handle, param, value) };
        check_al_error()
    }

    fn set_3i(&self, param: i32, value: [i32; 3]) -> AllenResult<()> {
        unsafe { alSource3i(self.handle, param, value[0], value[1], value[2]) };
        check_al_error()
    }
}

macro_rules! getter_f {
    ($func:ident, $al_ty:ident) => {
        pub fn $func(&self) -> f32 {
            self.get_f($al_ty)
        }
    };
}

macro_rules! setter_f {
    ($func:ident, $al_ty:ident) => {
        pub fn $func(&self, value: f32) -> AllenResult<()> {
            self.set_f($al_ty, value)
        }
    };
}

macro_rules! getter_3f {
    ($func:ident, $al_ty:ident) => {
        pub fn $func(&self) -> [f32; 3] {
            self.get_3f($al_ty)
        }
    };
}

macro_rules! setter_3f {
    ($func:ident, $al_ty:ident) => {
        pub fn $func(&self, value: [f32; 3]) -> AllenResult<()> {
            self.set_3f($al_ty, value)
        }
    };
}
// TODO: Streaming

impl Source {
    pub fn new() -> AllenResult<Self> {
        let mut handle = 0;
        unsafe { alGenSources(1, &mut handle) };

        check_al_error()?;

        Ok(Self { handle })
    }

    pub(crate) fn handle(&self) -> u32 {
        self.handle
    }

    getter_f!(pitch, AL_PITCH);
    setter_f!(set_pitch, AL_PITCH);

    getter_f!(gain, AL_GAIN);
    setter_f!(set_gain, AL_GAIN);

    getter_f!(min_gain, AL_MIN_GAIN);
    setter_f!(set_min_gain, AL_MIN_GAIN);

    getter_f!(max_gain, AL_MAX_GAIN);
    setter_f!(set_max_gain, AL_MAX_GAIN);

    getter_f!(max_distance, AL_MAX_DISTANCE);
    setter_f!(set_max_distance, AL_MAX_DISTANCE);

    getter_f!(rolloff_factor, AL_ROLLOFF_FACTOR);
    setter_f!(set_rolloff_factor, AL_ROLLOFF_FACTOR);

    getter_f!(cone_outer_gain, AL_CONE_OUTER_GAIN);
    setter_f!(set_cone_outer_gain, AL_CONE_OUTER_GAIN);

    getter_f!(cone_inner_angle, AL_CONE_INNER_ANGLE);
    setter_f!(set_cone_inner_angle, AL_CONE_INNER_ANGLE);

    getter_f!(cone_outer_angle, AL_CONE_OUTER_ANGLE);
    setter_f!(set_cone_outer_angle, AL_CONE_OUTER_ANGLE);

    getter_f!(reference_distance, AL_REFERENCE_DISTANCE);
    setter_f!(set_reference_distance, AL_REFERENCE_DISTANCE);

    getter_f!(playback_position, AL_SEC_OFFSET);
    setter_f!(set_playback_position, AL_SEC_OFFSET);

    getter_f!(playback_position_in_samples, AL_SAMPLE_OFFSET);

    getter_3f!(position, AL_POSITION);
    setter_3f!(set_position, AL_POSITION);

    getter_3f!(velocity, AL_VELOCITY);
    setter_3f!(set_velocity, AL_VELOCITY);

    getter_3f!(direction, AL_DIRECTION);
    setter_3f!(set_direction, AL_DIRECTION);

    pub fn is_looping(&self) -> bool {
        self.get_i(AL_LOOPING) != 0
    }

    pub fn set_looping(&self, value: bool) -> AllenResult<()> {
        self.set_i(AL_LOOPING, value as i32)
    }

    // Buffer should be set only.

    pub fn set_buffer(&self, buffer: &Buffer) -> AllenResult<()> {
        self.set_i(AL_BUFFER, buffer.handle() as i32)
    }
    // TODO: AL_SOURCE_RELATIVE, AL_CONE_INNER_ANGLE, AL_CONE_OUTER_ANGLE, AL_SOURCE_STATE (should be read-only?)

    pub fn play(&self) -> AllenResult<()> {
        unsafe { alSourcePlay(self.handle) };
        check_al_error()
    }

    pub fn pause(&self) -> AllenResult<()> {
        unsafe { alSourcePause(self.handle) };
        check_al_error()
    }

    pub fn stop(&self) -> AllenResult<()> {
        unsafe { alSourceStop(self.handle) };
        check_al_error()
    }

    pub fn rewind(&self) -> AllenResult<()> {
        unsafe { alSourceRewind(self.handle) };
        check_al_error()
    }

    // TODO: alSourceQueueBuffers, alSourceUnqueueBuffers
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe { alDeleteSources(1, &self.handle) }
    }
}
