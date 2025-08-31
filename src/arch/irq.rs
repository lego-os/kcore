pub trait Interrupt {
    fn disable() {}
    fn enable() {}
    fn disabled() -> bool {
        false
    }
}
