use std::any::Any;

use crate::server::NgynContext;

/// `NgynInjectable` is a trait that defines the basic structure of an injectable in Ngyn.
/// It is designed to be thread-safe.
pub trait NgynInjectable: AsAny + Send {
    /// Creates a new instance of the injectable.
    /// This is for internal use only.
    fn new() -> Self
    where
        Self: Sized;

    fn inject(&mut self, _cx: &NgynContext) {}
}

/// `AsAny` is a trait that allows a type to be converted to a trait object.
/// It is designed to be thread-safe.
pub trait AsAny: Any {
    /// Returns a reference to the trait object.
    fn as_any(&self) -> &dyn Any;
}

pub(crate) trait CloneBox<T: AsAny + Clone> {
    fn clone_box(&self) -> Box<T>;
}

impl<T: AsAny + Clone> CloneBox<T> for T {
    fn clone_box(&self) -> Box<T> {
        let mut fat_ptr = self as *const T;
        unsafe {
            let data_ptr = &mut fat_ptr as *mut *const T as *mut *mut ();
            assert_eq!(*data_ptr as *const (), self as *const T as *const ());
            *data_ptr = Box::into_raw(Box::new(self.clone())) as *mut ();
        }
        unsafe { Box::from_raw(fat_ptr as *mut T) }
    }
}

impl<T: NgynInjectable> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
