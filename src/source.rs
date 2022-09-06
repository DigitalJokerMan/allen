use crate::{check_al_error, AllenResult, Buffer, Float3, PropertiesContainer};
use al_sys::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum SourceState {
    Initial = AL_INITIAL as isize,
    Playing = AL_PLAYING as isize,
    Paused = AL_PAUSED as isize,
    Stopped = AL_STOPPED as isize,
}

/// NOTE: Sources are bound to a context.
pub struct Source {
    handle: u32,
}

impl PropertiesContainer<f32> for Source {
    fn get(&self, param: i32) -> f32 {
        let result = unsafe {
            let mut value = 0.0;
            alGetSourcef(self.handle, param, &mut value);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn set(&self, param: i32, value: f32) {
        unsafe { alSourcef(self.handle, param, value) };
        check_al_error().unwrap();
    }
}

impl PropertiesContainer<[f32; 3]> for Source {
    fn get(&self, param: i32) -> [f32; 3] {
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

    fn set(&self, param: i32, value: [f32; 3]) {
        unsafe { alSource3f(self.handle, param, value[0], value[1], value[2]) };
        check_al_error().unwrap()
    }
}

impl PropertiesContainer<i32> for Source {
    fn get(&self, param: i32) -> i32 {
        let result = unsafe {
            let mut value = 0;
            alGetSourcei(self.handle, param, &mut value);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn set(&self, param: i32, value: i32) {
        unsafe { alSourcei(self.handle, param, value) };
        check_al_error().unwrap();
    }
}

impl PropertiesContainer<SourceState> for Source {
    fn get(&self, param: i32) -> SourceState {
        FromPrimitive::from_i32(PropertiesContainer::<i32>::get(self, param)).unwrap()
    }

    fn set(&self, param: i32, value: SourceState) {
        PropertiesContainer::<i32>::set(self, param, ToPrimitive::to_i32(&value).unwrap());
    }
}

impl PropertiesContainer<[i32; 3]> for Source {
    fn get(&self, param: i32) -> [i32; 3] {
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

    fn set(&self, param: i32, value: [i32; 3]) {
        unsafe { alSource3i(self.handle, param, value[0], value[1], value[2]) };
        check_al_error().unwrap()
    }
}

#[rustfmt::skip]
impl Source {
    getter_setter!(pitch, set_pitch, f32, AL_PITCH);
    getter_setter!(gain, set_gain, f32, AL_GAIN);
    getter_setter!(min_gain, set_min_gain, f32, AL_MIN_GAIN);
    getter_setter!(max_gain, set_max_gain, f32, AL_MAX_GAIN);
    getter_setter!(max_distance, set_max_distance, f32, AL_MAX_DISTANCE);
    getter_setter!(rolloff_factor, set_rolloff_factor, f32, AL_ROLLOFF_FACTOR);
    getter_setter!(reference_distance, set_reference_distance, f32, AL_REFERENCE_DISTANCE);

    getter_setter!(cone_outer_gain, set_cone_outer_gain, f32, AL_CONE_OUTER_GAIN);
    getter_setter!(cone_inner_angle, set_cone_inner_angle, f32, AL_CONE_INNER_ANGLE);
    getter_setter!(cone_outer_angle, set_cone_outer_angle, f32, AL_CONE_OUTER_ANGLE);

    getter_setter!(time_in_secs, set_time_in_secs, f32, AL_SEC_OFFSET);
    getter_setter!(time_in_samples, set_time_in_samples, i32, AL_SAMPLE_OFFSET);
    getter_setter!(time_in_bytes, set_time_in_bytes, i32, AL_BYTE_OFFSET);

    getter_setter!(position, set_position, Float3, AL_POSITION);
    getter_setter!(velocity, set_velocity, Float3, AL_VELOCITY);
    getter_setter!(direction, set_direction, Float3, AL_DIRECTION);

    getter_setter!(is_looping, set_looping, bool, AL_LOOPING);
    getter_setter!(is_relative, set_relative, bool, AL_SOURCE_RELATIVE);

    getter_setter!(state, set_state, SourceState, AL_SOURCE_STATE);

    pub fn set_buffer(&self, buffer: &Buffer) {
        self.set(AL_BUFFER, buffer.handle() as i32);
    }
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
