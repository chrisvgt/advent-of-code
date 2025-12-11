macro_rules! dbgprint {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::print!($($rest)*)
    }
}
pub(crate) use dbgprint;

macro_rules! dbgprintln {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}
pub(crate) use dbgprintln;
