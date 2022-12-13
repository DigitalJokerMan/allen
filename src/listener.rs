use crate::{
    check_al_error, sys::*, AllenResult, Context, Float3, Orientation, PropertiesContainer,
};

/// A [`Context`]'s listener.
#[non_exhaustive]
pub struct Listener {
    context: Context,
}

impl PropertiesContainer<f32> for Listener {
    fn get(&self, param: i32) -> AllenResult<f32> {
        let _lock = self.context.make_current();

        let result = unsafe {
            let mut value = 0.0;
            alGetListenerf(param, &mut value);
            value
        };

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: f32) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alListenerf(param, value) };
        check_al_error()?;

        Ok(())
    }
}

impl PropertiesContainer<[f32; 3]> for Listener {
    fn get(&self, param: i32) -> AllenResult<[f32; 3]> {
        let _lock = self.context.make_current();

        let result = unsafe {
            let mut value = [0.0, 0.0, 0.0];
            alGetListener3f(param, &mut value[0], &mut value[1], &mut value[2]);
            value
        };

        check_al_error()?;

        Ok(result)
    }

    fn set(&self, param: i32, value: [f32; 3]) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alListener3f(param, value[0], value[1], value[2]) };
        check_al_error()?;

        Ok(())
    }
}

impl PropertiesContainer<Orientation> for Listener {
    fn get(&self, param: i32) -> AllenResult<Orientation> {
        let _lock = self.context.make_current();

        let mut value = Orientation::default();
        unsafe { alGetListenerfv(param, &mut value as *mut Orientation as *mut f32) };
        check_al_error()?;

        Ok(value)
    }

    fn set(&self, param: i32, value: Orientation) -> AllenResult<()> {
        let _lock = self.context.make_current();

        unsafe { alListenerfv(param, &value as *const Orientation as *const f32) };
        check_al_error()?;

        Ok(())
    }
}

impl Listener {
    pub(crate) fn new(context: Context) -> Self {
        Self { context }
    }

    getter_setter!(gain, set_gain, f32, AL_GAIN);

    getter_setter!(position, set_position, Float3, AL_POSITION);
    getter_setter!(velocity, set_velocity, Float3, AL_VELOCITY);
    getter_setter!(orientation, set_orientation, Orientation, AL_ORIENTATION);
}
