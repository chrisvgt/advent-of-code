#[allow(unused)]
macro_rules! dbgprint {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::print!($($rest)*)
    }
}
#[allow(unused)]
pub(crate) use dbgprint;

#[allow(unused)]
macro_rules! dbgprintln {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}
#[allow(unused)]
pub(crate) use dbgprintln;
