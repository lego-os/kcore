use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct OnceLock<T> {
    init_start: AtomicBool,
    init_complete: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
    _marker: PhantomData<T>,
}

impl<T> OnceLock<T> {
    pub const fn new() -> OnceLock<T> {
        OnceLock {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            _marker: PhantomData,
            init_start: AtomicBool::new(false),
            init_complete: AtomicBool::new(false),
        }
    }

    #[inline]
    pub fn get(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_initialized() {
            Some(unsafe { self.get_unchecked_mut() })
        } else {
            None
        }
    }

    #[inline]
    pub fn wait(&self) -> &T {
        while !self.is_initialized() {}
        unsafe { self.get_unchecked() }
    }

    #[inline]
    fn is_initialized(&self) -> bool {
        self.init_complete.load(Ordering::Acquire)
    }

    #[cold]
    fn initialize<F, E>(&self, value: T) {
        loop {
            match self
                .init_start
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => {
                    unsafe { (&mut *self.value.get()).write(value) };
                    self.init_complete.store(true, Ordering::Release);
                    break;
                }
                Err(_) => {
                    if self.is_initialized() {
                        break;
                    }
                }
            }
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self) -> &T {
        unsafe { (&*self.value.get()).assume_init_ref() }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        unsafe { (&mut *self.value.get()).assume_init_mut() }
    }
}

unsafe impl<T: Sync + Send> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceLock<T> {}
impl<T: UnwindSafe> UnwindSafe for OnceLock<T> {}

impl<T> Default for OnceLock<T> {
    #[inline]
    fn default() -> OnceLock<T> {
        OnceLock::new()
    }
}

impl<T: PartialEq> PartialEq for OnceLock<T> {
    #[inline]
    fn eq(&self, other: &OnceLock<T>) -> bool {
        self.get() == other.get()
    }
}
impl<T: Eq> Eq for OnceLock<T> {}
impl<T> Drop for OnceLock<T> {
    #[inline]
    fn drop(&mut self) {
        if self.is_initialized() {
            // SAFETY: The cell is initialized and being dropped, so it can't
            // be accessed again. We also don't touch the `T` other than
            // dropping it, which validates our usage of #[may_dangle].
            unsafe { (&mut *self.value.get()).assume_init_drop() };
        }
    }
}
