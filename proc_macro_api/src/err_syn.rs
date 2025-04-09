#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_unknown {
    (
        mac: $mac:ident ,
        tt: [ $($tt:tt)* ] $(,)?
    ) => {
        core::compile_error!(core::concat!(
            "unknown error inside `proc_macro_api!`",
            "\n= inner macro: ", core::stringify!($mac),
            "\n= tokens: ",
            $("\n    ", core::stringify!($tt),)*
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_syn_gt_one {
    ($ddd:expr => $($_0:tt)?) => {};

    ($ddd:expr => [ $($first:tt)* ] [ $($err:tt)* ] $($_0:tt)*) => {
        core::compile_error!(core::concat!(
            "no rules expected `", $(core::stringify!($err),)* "`",
            "\n/ ", $(core::stringify!($first),)* $ddd,
            "\n| ", $(core::stringify!($err),)* $ddd,
            "\n|_^ this is not expected",
        ));
    };
}
