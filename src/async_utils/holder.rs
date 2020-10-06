// Say fuck no to Mutex, use Holder today!
// Btw this is very unsafe

//use std::cell::UnsafeCell;

// TODO: Remove this
#[repr(transparent)]
pub struct UnsafeHolder<T: ?Sized> {
    value: T
}

impl<T> UnsafeHolder<T> {

    pub fn new(value: T) -> UnsafeHolder<T> {
        UnsafeHolder {
            value
        }
    }


    pub unsafe fn get(&self) -> *mut T {
        self as *const UnsafeHolder<T> as *const T as *mut T
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.get()
    }

}
