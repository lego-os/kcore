use core::ptr::NonNull;


pub struct Semaphore<T:?Sized>{
    data:NonNull<T>
}