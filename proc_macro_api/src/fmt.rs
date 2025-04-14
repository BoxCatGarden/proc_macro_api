#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_fmt_path {
    ([ $seg:tt $($rest:ident)* $({ $($_0:tt)* } $($_1:tt)*)? ]) => {
        $crate::__private::concat!(
            $crate::__private::stringify!($seg),
            $("::", $crate::__private::stringify!($rest),)*
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_fmt_str_tt {
    ($($tt:tt)*) => {
        $crate::__private::concat!(
            $($crate::__private::stringify!($tt),)*
        )
    };
}
