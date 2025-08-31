use core::{
    cell::UnsafeCell,
    hint,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub fn into_inner(self) -> UnsafeCell<T> {
        self.data
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn lock(&self) -> SpinGuard<T> {
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            } else {
                hint::spin_loop()
            }
        }
    }

    pub fn try_lock(&self) -> Option<SpinGuard<T>> {
        match self
            .lock
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
        {
            Ok(_) => Some(SpinGuard {
                lock: &self.lock,
                data: NonNull::new(self.data.get()).unwrap(),
            }),
            Err(_) => None,
        }
    }

    #[inline]
    pub fn locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}

unsafe impl<T: ?Sized + Sync + Send> Send for SpinLock<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for SpinLock<T> {}

pub struct SpinGuard<'a, T: ?Sized> {
    lock: &'a AtomicBool,
    data: NonNull<T>,
}

impl<'a, T: ?Sized> Deref for SpinGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<'a, T: ?Sized> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl<'a, T: ?Sized> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}

unsafe impl<'a, T: ?Sized + Sync + Send> Send for SpinGuard<'_, T> {}
unsafe impl<'a, T: ?Sized + Sync + Send> Sync for SpinGuard<'_, T> {}
