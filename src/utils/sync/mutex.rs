pub struct Mutex<T> {
    t: T,
}

impl<T> Mutex<T> {
    pub fn lock(&self) {}

    pub fn try_lock(&self) {}
}
