#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_unknown {
    (
        mac: $mac:ident ,
        tt: [ $($tt:tt)* ] $(,)?
    ) => {
        std::compile_error!(std::concat!(
            "unknown error inside of `proc_macro_api!`",
            "\n  inner macro: ", std::stringify!($mac),
            "\n  tokens: ",
            $("\n    ", std::stringify!($tt),)*
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_syn_gt_one {
    ($($_0:tt)?) => {};

    ([ $($first:tt)* ] [ $($err:tt)* ] $([ $($rest:tt)* ])*) => {
        std::compile_error!(std::concat!(
            "no rules expected `", $(std::stringify!($err),)* " ...`",
            "\n/ ", $(std::stringify!($first),)* " ...",
            "\n| ", $(std::stringify!($err),)* " ...",
            $("\n| ", $(std::stringify!($rest),)* " ...",)*
            "\n|_^",
        ));
    };

    ($($tt:tt)*) => {
        $crate::proc_macro_api_err_unknown!(
            mac: proc_macro_api_err_syn_gt_one,
            tt: [ $($tt)* ],
        );
    };
}
