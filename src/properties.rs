use crate::AllenResult;

pub(crate) trait PropertiesContainer<T> {
    fn get(&self, param: i32) -> AllenResult<T>;
    fn set(&self, param: i32, value: T) -> AllenResult<()>;
}

/// Automatic `bool` PropertiesContainer for those that have `i32` properties.
impl<T> PropertiesContainer<bool> for T
where
    T: PropertiesContainer<i32>,
{
    fn get(&self, param: i32) -> AllenResult<bool> {
        Ok(PropertiesContainer::<i32>::get(self, param)? != 0)
    }

    fn set(&self, param: i32, value: bool) -> AllenResult<()> {
        PropertiesContainer::<i32>::set(self, param, value as i32)
    }
}

#[macro_export]
macro_rules! getter {
    ($func:ident, $ty:ident, $al_param:ident) => {
        pub fn $func(&self) -> crate::AllenResult<$ty> {
            self.get($al_param)
        }
    };
}

#[macro_export]
macro_rules! setter {
    ($func:ident, $ty:ident, $al_param:ident) => {
        pub fn $func(&self, value: $ty) -> crate::AllenResult<()> {
            self.set($al_param, value)
        }
    };
}

#[macro_export]
macro_rules! getter_setter {
    ($get_func:ident, $set_func:ident, $ty:ident, $al_param:ident) => {
        getter!($get_func, $ty, $al_param);
        setter!($set_func, $ty, $al_param);
    };
}
