use crate::{check_al_error, sys::*, AllenResult, Buffer, Context, Float3, PropertiesContainer};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// The state of a [`Source`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SourceState {
    /// The initial state of a source. The source will also be placed in this state after calling [`Source::rewind`].
    Initial = AL_INITIAL as isize,
    /// The source is currently playing.
    Playing = AL_PLAYING as isize,
    /// The source was playing, and now it's paused.
    Paused = AL_PAUSED as isize,
    /// The source has been stopped.
    Stopped = AL_STOPPED as isize,
}

/// A source used to play [`Buffer`]s.
/// NOTE: Sources are bound to a context.
pub struct Source {
    handle: u32,
    context: Context,
}

impl PropertiesContainer<f32> for Source {
    fn get(&self, param: i32) -> AllenResult<f32> {
        let _lock = self.context.make_current();

        let result = unsafe {
            let mut value = 0.0;
            alGetSourcef(self.handle, param, &mut value);
            value
        };

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: f32) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSourcef(self.handle, param, value) };
        check_al_error()?;

        Ok(())
    }
}

impl PropertiesContainer<[f32; 3]> for Source {
    fn get(&self, param: i32) -> AllenResult<[f32; 3]> {
        let _lock = self.context.make_current();

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

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: [f32; 3]) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSource3f(self.handle, param, value[0], value[1], value[2]) };
        check_al_error()?;

        Ok(())
    }
}

impl PropertiesContainer<i32> for Source {
    fn get(&self, param: i32) -> AllenResult<i32> {
        let _lock = self.context.make_current();

        let result = unsafe {
            let mut value = 0;
            alGetSourcei(self.handle, param, &mut value);
            value
        };

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: i32) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSourcei(self.handle, param, value) };
        check_al_error()?;

        Ok(())
    }
}

impl PropertiesContainer<SourceState> for Source {
    fn get(&self, param: i32) -> AllenResult<SourceState> {
        let _lock = self.context.make_current();

        Ok(FromPrimitive::from_i32(PropertiesContainer::<i32>::get(self, param)?).unwrap())
    }

    fn set(&self, param: i32, value: SourceState) -> AllenResult<()> {
        let _lock = self.context.make_current();

        PropertiesContainer::<i32>::set(self, param, ToPrimitive::to_i32(&value).unwrap())
    }
}

impl PropertiesContainer<[i32; 3]> for Source {
    fn get(&self, param: i32) -> AllenResult<[i32; 3]> {
        let _lock = self.context.make_current();

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

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: [i32; 3]) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSource3i(self.handle, param, value[0], value[1], value[2]) };
        check_al_error()?;

        Ok(())
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

    // AL_SOFT_source_length
    getter!(length_in_secs, f32, AL_SEC_LENGTH_SOFT, "AL_SOFT_source_length");
    getter!(length_in_samples, i32, AL_SAMPLE_LENGTH_SOFT, "AL_SOFT_source_length");
    getter!(length_in_bytes, f32, AL_BYTE_LENGTH_SOFT, "AL_SOFT_source_length");

    // AL_EXT_SOURCE_RADIUS
    getter_setter!(source_radius, set_source_radius, f32, AL_SOURCE_RADIUS, "AL_EXT_SOURCE_RADIUS");

    pub fn set_buffer(&self, buffer: Option<&Buffer>) -> AllenResult<()> {
        self.set(
            AL_BUFFER,
            match buffer {
                Some(buffer) => buffer.handle() as i32,
                None => 0,
            },
        )
    }

    getter!(buffers_queued, i32, AL_BUFFERS_QUEUED);
    getter!(buffers_processed, i32, AL_BUFFERS_PROCESSED);
}

impl Source {
    pub(crate) fn new(context: Context) -> AllenResult<Self> {
        let mut handle = 0;
        unsafe {
            let _lock = context.make_current();
            alGenSources(1, &mut handle)
        };

        check_al_error()?;

        Ok(Self { handle, context })
    }

    pub fn play(&self) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe {
            alDistanceModel(ToPrimitive::to_i32(&self.context.get_distance_model()).unwrap());
            alSourcePlay(self.handle);
        }
        check_al_error()
    }

    pub fn pause(&self) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSourcePause(self.handle) };
        check_al_error()
    }

    pub fn stop(&self) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSourceStop(self.handle) };
        check_al_error()
    }

    pub fn rewind(&self) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alSourceRewind(self.handle) };
        check_al_error()
    }

    pub fn queue_buffers(&self, buffers: &[&Buffer]) -> AllenResult<()> {
        let _lock = self.context.make_current();

        let buffers = buffers
            .iter()
            .map(|buffer| buffer.handle())
            .collect::<Vec<_>>();

        unsafe {
            alSourceQueueBuffers(
                self.handle,
                buffers.len() as i32,
                buffers.as_ptr() as *const u32,
            )
        };

        check_al_error()
    }

    pub fn queue_buffer(&self, buffer: &Buffer) -> AllenResult<()> {
        let _lock = self.context.make_current();

        self.queue_buffers(&[buffer])
    }

    pub fn unqueue_buffers(&self, count: i32) -> AllenResult<()> {
        let _lock = self.context.make_current();

        let _buffers = vec![0u32; count as usize]; // This will be discarded.

        unsafe { alSourceUnqueueBuffers(self.handle, count, _buffers.as_ptr() as *mut u32) };

        check_al_error()
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe { alDeleteSources(1, &self.handle) }
        if let Err(err) = check_al_error() {
            println!("WARNING: Source drop failed! {}", err);
        }
    }
}
