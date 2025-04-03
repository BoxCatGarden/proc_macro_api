#![doc = include_str!("../README.md")]

//!
//! TODO!()
//! ERR - `:: ::`, `{} as al`, `proc = []`,
//!

mod err_syn;

/// See the [document at module level][self].
#[macro_export]
macro_rules! proc_macro_api {
    ($($tt:tt)+) => {
        $crate::proc_macro_api_top! {
            [ $($tt)+ ]
        }
    };
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_top {
    (* [ $($at:tt)* ] [ $($al:tt)? ] $cc:tt $seg:tt $($rest:tt)*) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [/*[doc]*/] [/*[other]+[proc]*/] [/*[proc]*/]
            [ $($rest),* ] [/*prv*/] [/*last*/ $seg ]
            [
                [/*[proc]*/] [/*[doc]*/] [/*[[other]+[proc]]*/]
                [ $cc [/*seg*/] $($al)? ]
            ]
        }
    };

    (* [] []) => { /* empty comma (`,,`) and trailing comma (`... ,`) */ };

    ([$(
        $(# $at:tt)*
        $(:: $seg_0:tt $(:: $rest_0:tt)*)?
        $($seg_1:ident $(:: $rest_1:tt)*)?
        $({ $($seg_2:tt)* } $(:: $rest_2:tt)*)?
        $(as $al:ident)?
    ),*]) => {$(
        $crate::proc_macro_api_err_syn_gt_one! {
            $([ :: $seg_0 ])? $([ $seg_1 ])? $([ { $($seg_2)* } ])?
        }
        $crate::proc_macro_api_top! {
            * [ $($at)* ] [ $($al)? ]
            $([ :: ] $seg_0 $($rest_0)*)?
            $([] $seg_1 $($rest_1)*)?
            $([] { $($seg_2)* } $($rest_2)*)?
        }
    )*};

    // [[at]] [al]
    (* $at:tt $al:tt) => {
        $crate::proc_macro_api_err_top_no_seg! {
            $at $al
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_top_no_seg {
    // [at] al
    ([ $($at:tt)* ] [ $($al:tt)? ]) => {
        std::compile_error!(std::concat!(
            "expected path segments",
            "\n/",
            $("\n| #", std::stringify!($at),)*
            $("\n| as ", std::stringify!($al),)?
            "\n|\n|_^ expected path segments",
        ));
    };

    ($($tt:tt)*) => {
        $crate::proc_macro_api_err_unknown!(
            mac: proc_macro_api_err_top_no_seg,
            tt: [ $($tt)* ],
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_mul {
    // [first] [second]
    ($msg:expr , [ $($note:expr),* $(,)? ] , $plural_s:expr =>
        [ $first_0:tt $($first:tt)* ] [ $second_0:tt $($second:tt)* ]
        [ $seg:tt $($rest:ident)* $({ $($_0:tt)* } $($_1:tt)*)? ]
    ) => {
        std::compile_error!(std::concat!(
            $msg,
            "\n/ #", std::stringify!($first_0),
            $("\n| #", std::stringify!($first),)*
            "\n|_^ the first attribute", $plural_s,
            "\n...",
            "\n/ #", std::stringify!($second_0),
            $("\n| #", std::stringify!($second),)*
            "\n|_^ the second attribute", $plural_s,
            "\n\n... ", std::stringify!($seg),
            $("::", std::stringify!($rest),)*
            " ...\n",
            $("\n= note: ", $note,)*
        ));
    };

    ($($tt:tt)*) => {
        $crate::proc_macro_api_err_unknown!(
            mac: proc_macro_api_err_attr_mul,
            tt: [ $($tt)* ],
        );
    };
}

#[cfg(not(feature = "no_shadow"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_shadow {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "no_shadow")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_shadow {
    ([] $_0:tt $_1:tt) => {};

    // [[proc]] [cover] [path]
    ($proc:tt $cover:tt $path:tt) => {
        $crate::proc_macro_api_err_attr_mul! {
            "multiple proc_macro attributes on one piece of a path",
            [
                "feature `no_shadow` is enabled",
                "disabling the feature to \
leave the possible error to the compiler",
            ],
            "" =>
            $proc [ $cover ] $path
        }
    };

    ($($tt:tt)*) => {
        $crate::proc_macro_api_err_unknown!(
            mac: proc_macro_api_err_attr_shadow,
            tt: [ $($tt)* ],
        );
    };
}

#[cfg(not(feature = "no_override"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_override {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "no_override")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_override {
    ($_0:tt $($_1:tt)?) => {};

    // [[old]] [[new]] [path]
    ($old:tt $new:tt $path:tt) => {
        $crate::proc_macro_api_err_attr_mul! {
            "attributes are overridden inside one path",
            [ "feature `no_override` is enabled" ],
            "(s)" =>
            $old $new $path
        }
    };

    ($($tt:tt)*) => {
        $crate::proc_macro_api_err_unknown!(
            mac: proc_macro_api_err_attr_override,
            tt: [ $($tt)* ],
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_attr {
    // [doc]
    (
    [ [ doc $($arg_doc:tt)* ] $(, $at:tt)* ]
    [ $($doc:tt)* ] $other:tt $proc:tt
    [ $($seg:tt)? $(, $rest:tt)* ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [ $($doc)* [ doc $($arg_doc)* ] ] $other $proc
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
            $bag
        }
    };

    // -> out
    (
    [] [ $($doc:tt)* ] [ $($($other:tt)+)? ] [
        $([ proc_macro $($arg_fn_0:tt)* ])?
        $([ fn $($arg_fn:tt)* ])?
        $([ proc_macro_attribute $($arg_at_0:tt)* ])?
        $([ at $($arg_at:tt)* ])?
        $([ proc_macro_derive $($arg_dr_0:tt)* ])?
        $([ dr $($arg_dr:tt)* ])?
    ]
    [ $($seg:tt),* ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    [ [ $($bg_proc:tt)? ] [ $($bg_doc:tt)* ] [ $($bg_oth:tt)? ] $path:tt ]
    ) => {
        $crate::proc_macro_api_err_attr_override! {
            $($bg_oth)? $([ $($other)+ ])?
            [ $($prv)* $($to_prv)? $last $($seg)* ]
        }
        $crate::proc_macro_api_parse_seg! {
            [ $($seg)* ] [ $last ] [ $($prv)* $($to_prv)? ]
            [
                [
                    $([ proc_macro $($arg_fn_0)* ] ;)?
                    $([ proc_macro $($arg_fn)* ] ;)?
                    $([ proc_macro_attribute $($arg_at_0)* ] ;)?
                    $([ proc_macro_attribute $($arg_at)* ] ;)?
                    $([ proc_macro_derive $($arg_dr_0)* ] ;)?
                    $([ proc_macro_derive $($arg_dr)* ] ;)?
                    $($bg_proc)?
                ] [
                    $($bg_doc)*
                    $($doc)*
                ] [
                    $([ $($other)+ ] ;)?
                    $($bg_oth)?
                ] $path
            ]
        }
    };

    // [proc_macro]
    // [proc_macro_attribute]
    // [proc_macro_derive]
    (
    [
        $([ proc_macro $($arg_fn_0:tt)* ])?
        $([ fn $($arg_fn:tt)* ])?
        $([ proc_macro_attribute $($arg_at_0:tt)* ])?
        $([ at $($arg_at:tt)* ])?
        $([ proc_macro_derive $($arg_dr_0:tt)* ])?
        $([ dr $($arg_dr:tt)* ])?
        $(, $at:tt)*
    ]
    $doc:tt [ $($other:tt)* ] [ $($proc:tt)? ]
    [ $($seg:tt)? $(, $rest:tt)* ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_err_attr_shadow! {
            [ $($proc)? ]
            $([proc_macro$($arg_fn_0)*])?
            $([fn$($arg_fn)*])?
            $([proc_macro_attribute$($arg_at_0)*])?
            $([at$($arg_at)*])?
            $([proc_macro_derive$($arg_dr_0)*])?
            $([dr$($arg_dr)*])?
            [ $($prv)* $($to_prv)? $last $($seg)? $($rest)* ]
        }
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] $doc
            [ $($other)*
                $([ proc_macro $($arg_fn_0)* ])?
                $([ proc_macro $($arg_fn)* ])?
                $([ proc_macro_attribute $($arg_at_0)* ])?
                $([ proc_macro_attribute $($arg_at)* ])?
                $([ proc_macro_derive $($arg_dr_0)* ])?
                $([ proc_macro_derive $($arg_dr)* ])?
            ] [
                $([proc_macro$($arg_fn_0)*])?
                $([fn$($arg_fn)*])?
                $([proc_macro_attribute$($arg_at_0)*])?
                $([at$($arg_at)*])?
                $([proc_macro_derive$($arg_dr_0)*])?
                $([dr$($arg_dr)*])?
            ]
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
            $bag
        }
    };

    // [`any`]
    (
    [ $any:tt $(, $at:tt)* ]
    $doc:tt [ $($other:tt)* ] $proc:tt
    [ $($seg:tt)? $(, $rest:tt)* ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] $doc [ $($other)* $any ] $proc
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
            $bag
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_seg {
    (
    [ $seg:tt $($rest:tt)* ] [ $last:tt ] [ $($prv:tt)* ] $bag:tt
    ) => {
        $crate::proc_macro_api_parse_seg! {
            [ $($rest)* ] [ $seg ] [ $($prv)* $last ] $bag
        }
    };

    (
    [] [
        $($api:ident)?
        $({$(
            $(# $at:tt)*
            $($seg_id:ident)?
            $({ $($seg_blk:tt)* })?
            $(:: $seg_cc:tt $(:: $rest:tt)*)?
            $(as $al:tt)?
        ),*})?
    ] $prv:tt
    [ $bg_proc:tt $bg_doc:tt $bg_oth:tt [ $bg_cc:tt $bg_al:tt ] ]
    ) => {
        $(
        $crate::proc_macro_api_parse_fn! {
            $bg_proc [ $bg_doc $bg_oth $bg_cc $prv ] $bg_al $api
        }
        )?
        $($(
        $crate::proc_macro_api_parse_seg_call_attr! {
            [
                $([] $seg_id)?
                $([] { $($seg_blk)* })?
                $([ :: ] $seg_cc ; [ $($rest)* ])?
            ]
            [ $($at),* ] [ $($al)? ]
            [ $prv $bg_cc $bg_proc $bg_doc $bg_oth ]
        }
        )*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_seg_call_attr {
    (
    [
        $cc:tt $seg:tt
        $([ :: ] $seg_cc:tt ; [ $($rest_0:tt)* ])?
        $(; [ $($rest_1:tt)* ])?
    ]
    $at:tt $al:tt
    [
        $prv:tt
        [ $($bg_cc:tt)? ]
        [ $($bg_proc:tt)? $(; $_0:tt)? $(;)? ] $bg_doc:tt
        [ $($bg_oth:tt)? $(; $_1:tt)? $(;)? ]
    ]
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $at [/*[doc]*/] [/*[other]+[proc]*/] [/*[proc]*/]
            [ $($seg_cc $(, $rest_0)*)? $($($rest_1),*)? ] $prv [ $seg ]
            [
                [ $($bg_proc)? ] $bg_doc [ $($bg_oth)? ]
                [ $cc $al ]
            ]
        }
    };

    ([] [] [] $_:tt) => { /* empty comma (`,,`) and trailing comma (`... ,`) */ };

    ([] $at:tt $al:tt $_:tt) => {};

    (
    [ [] $seg_0:tt [] $seg_1:tt $($_0:tt)* ]
    $at:tt $al:tt $_1:tt
    ) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_fn_no_proc {
    ([ $($cc:tt)? ] [ $($seg:ident)* ] $api:ident $(as $alias:ident)?) => {
        std::compile_error!(std::concat!(
            "expected a proc_macro attribute for `", $(
            std::stringify!($cc),)? $(
            std::stringify!($seg),
            "::",)*
            std::stringify!($api), $(
            " as ",
            std::stringify!($alias),)?
            "`",
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_fn {
    ([] $bag:tt $al:tt $api:tt) => {
        // $crate::proc_macro_api_err_fn_no_proc!([] $seg $api);
    };

    // $bg_proc [ $bg_doc $bg_oth $bg_cc $prv ] $bg_al $api
    (
    [
        $([ proc_macro $($arg_fn:tt)* ])?
        $([ proc_macro_attribute $($arg_at:tt)* ])?
        $([ proc_macro_derive $($arg_dr:tt)* ])?
    ]
    [ $doc:tt $other:tt $cc:tt $prv:tt ] [ $($al:tt)? ] $api:tt
    ) => {
        $crate::proc_macro_api_fn! {
            $doc $other $cc $prv $api [ $($al)? $api ]
            $((input) [ $($arg_fn)* ])?
            $((args, item) [ $($arg_at)* ])?
            $((item) [ $($arg_dr)* ])?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_fn {
    (
    [ $($doc:tt)* ] [ [ $($other:tt)* ] $(; $_0:tt)? $(;)? ]
    [ $($cc:tt)? ] [ $($prv:tt)* ] $api:tt [ $name:tt $($_1:tt)? ]
    ( $($args:tt),* $(,)? ) $_2:tt
    ) => {
        $(# $doc)*
        $(# $other)*
        pub fn $name (
            $($args : proc_macro::TokenStream),*
        ) -> proc_macro::TokenStream {
            $($cc)? $($prv ::)* $api ( $($args),* )
        }
    };
}

#[cfg(test)]
mod tests {
    macro_rules! aaa {
        ($tt:tt $t1:tt $t2:tt) => {
            stringify!($tt)
        };
        (r#fn) => {
            6
        };
    }
    #[test]
    fn test() {
        println!("a = {}", aaa!(r#fn));
    }

    fn a() -> i32 {
        #[cfg(test)]
        {
            3
        }
        #[cfg(not(test))]
        6
    }
}
