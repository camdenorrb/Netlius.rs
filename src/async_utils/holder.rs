// Say fuck no to Mutex, use Holder today!

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

    pub fn get(&self) -> *mut T {
        self as *const Holder<T> as *const T as *mut T
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.get() }
    }

}