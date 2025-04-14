#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_syn_gt_one {
    ($ddd:expr => $($_0:tt)?) => {};

    ($ddd:expr => [ $($first:tt)* ] [ $($err:tt)* ] $($_0:tt)*) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "no rules expected `", $($crate::__private::stringify!($err),)* "`",
            "\n/ ", $($crate::__private::stringify!($first),)* $ddd,
            "\n| ", $($crate::__private::stringify!($err),)* $ddd,
            "\n|_^ this is not expected",
        ));
    };
}
