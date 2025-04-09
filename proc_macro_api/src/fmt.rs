#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_fmt_path {
    ([ $seg:tt $($rest:ident)* $({ $($_0:tt)* } $($_1:tt)*)? ]) => {
        core::concat!(
            core::stringify!($seg),
            $("::", core::stringify!($rest),)*
        )
    };
}
