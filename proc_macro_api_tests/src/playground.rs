// This file will be included by "lib.rs" when
// compiling with feature `playground`.

#[cfg(not(feature = "playground"))]
macro_rules! proc_macro_api {
    ($($tt:tt)*) => {};
}

// Local functions should be defined in this module or
// with a name starting with `play_t_`.
mod pg {}

// APIs exported here should have a name starting with `play_t_`.
proc_macro_api! {}
