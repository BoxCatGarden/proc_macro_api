#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_unknown {
    (
        mac: $mac:ident ,
        tt: [ $($tt:tt)* ] $(,)?
    ) => {
        std::compile_error!(std::concat!(
            "unknown error inside `proc_macro_api!`",
            "\n= inner macro: ", std::stringify!($mac),
            "\n= tokens: ",
            $("\n    ", std::stringify!($tt),)*
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_syn_gt_one {
    ($ddd:expr => $($_0:tt)?) => {};

    ($ddd:expr => [ $($first:tt)* ] [ $($err:tt)* ] $($_0:tt)*) => {
        std::compile_error!(std::concat!(
            "no rules expected `", $(std::stringify!($err),)* "`",
            "\n/ ", $(std::stringify!($first),)* $ddd,
            "\n| ", $(std::stringify!($err),)* $ddd,
            "\n|_^ this is not expected",
        ));
    };
}
