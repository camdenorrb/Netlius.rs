// Say fuck no to Mutex, use Holder today!
// Btw this is very unsafe

#[repr(transparent)]
pub struct Holder<T: ?Sized> {
    value: T
}

impl<T> Holder<T> {

    pub fn new(value: T) -> Holder<T> {
        Holder {
            value
        }
    }


    pub unsafe fn get(&self) -> *mut T {
        self as *const Holder<T> as *const T as *mut T
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.get()
    }

}