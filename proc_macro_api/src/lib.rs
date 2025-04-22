#![doc = include_str!("../doc.md")]

/*
checklist:
 tt, ident, expr ;
 doc_hidden macro_export `pma` ;
 path $crate::`pma`! ;
 path $crate::__private ;
*/

extern crate proc_macro;

#[doc(hidden)]
pub mod __private;

mod err_syn;
mod fmt;

/// See the [crate documentation][self].
#[macro_export]
macro_rules! proc_macro_api {
    ($($tt:tt)+) => {
        $crate::proc_macro_api_parse_seg! {
            [/*rest*/] [ { $($tt)+ } ] [/*prv*/]
            [
                [/*[proc]*/] [/*[doc]*/] [/*[[other]+[proc]]*/]
                [ [/*cc*/] [/*al*/] ]
            ]
        }
    };
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_mul {
    // [first] [second]
    ($msg:expr , [ $($note:expr),* $(,)? ] , $plural_s:expr =>
        [ $first_0:tt $($first:tt)* ] [ $second_0:tt $($second:tt)* ]
        $path:tt
    ) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            $msg,
            "\n/ #", $crate::__private::stringify!($first_0),
            $("\n| #", $crate::__private::stringify!($first),)*
            "\n|_^ the first attribute", $plural_s,
            "\n...",
            "\n/ #", $crate::__private::stringify!($second_0),
            $("\n| #", $crate::__private::stringify!($second),)*
            "\n|_^ the second attribute", $plural_s,
            "\n\n  ", $crate::proc_macro_api_fmt_path!($path),
            "\n  ^\n",
            $("\n= note: ", $note,)*
        ));
    };
}

#[cfg(not(feature = "deny_shadow"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_shadow {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "deny_shadow")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_shadow {
    ([] $_0:tt $_1:tt) => {};

    // [[proc]] [cover] [path]
    ($proc:tt $cover:tt $path:tt) => {
        $crate::proc_macro_api_err_attr_mul! {
            "multiple proc-macro attributes are applied together",
            [
                "feature `deny_shadow` is enabled",
                "disabling the feature to \
leave the possible error to the compiler",
            ],
            "" =>
            $proc [ $cover ] $path
        }
    };
}

#[cfg(not(feature = "deny_override"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_override {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "deny_override")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_attr_override {
    ($_0:tt $($_1:tt)?) => {};

    // [[old]] [[new]] [path]
    ($old:tt $new:tt $path:tt) => {
        $crate::proc_macro_api_err_attr_mul! {
            "attributes are overridden inside one path",
            [ "feature `deny_override` is enabled" ],
            "(s)" =>
            $old $new $path
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_attr {
    // [doc]
    (
    [ [ doc $($arg_doc:tt)* ] $(, $at:tt)* ]
    [ $($doc:tt)* ] $other:tt $proc:tt
    [ $($seg:tt $($rest:tt)*)? ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [ $($doc)* [ doc $($arg_doc)* ] ] $other $proc
            [ $($($rest)*)? ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
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
    [ $($seg:tt)* ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    [
        [ $($bg_proc:tt)? ] [ $($bg_doc:tt)* ] [ $($bg_oth:tt)? ]
        [ $($bg_prv:tt)* ] $path:tt
    ]
    ) => {
        $crate::proc_macro_api_err_attr_override! {
            $($bg_oth)? $([ $($other)+ ])?
            [ $($prv)* $($to_prv)? $last $($seg)* ]
        }
        $crate::proc_macro_api_parse_seg! {
            [ $($seg)* ] [ $last ] [ $($bg_prv)* $($prv)* $($to_prv)? ]
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
    [ $($seg:tt $($rest:tt)*)? ]
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
            [ $($prv)* $($to_prv)? $last $($seg $($rest)*)? ]
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
            [ $($($rest)*)? ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
            $bag
        }
    };

    // [`any`]
    (
    [ $any:tt $(, $at:tt)* ]
    [ $($doc:tt)* ] $other:tt $proc:tt
    [ $($seg:tt $($rest:tt)*)? ]
    [ $($prv:tt)* ] [ $last:tt $($to_prv:tt)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [ $($doc)* $any ] $other $proc
            [ $($($rest)*)? ] [ $($prv)* $($to_prv)? ] [ $($seg)? $last ]
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
        $({
            $(# $at_0:tt)*
            $($seg_0:ident)?
            $(:: $seg_cc_0:ident $(:: $rest_0:ident)*)?
            $(:: { $($seg_blk_cc_0:tt)* })?
            $({ $($seg_blk_0:tt)* })?
            $(as $al_0:tt)?
            $(,
                $(# $at:tt)*
                $($seg:ident)?
                $(:: $seg_cc:ident $(:: $rest:ident)*)?
                $(:: { $($seg_blk_cc:tt)* })?
                $({ $($seg_blk:tt)* })?
                $(as $al:tt)?
            )*
        })?
    ] $prv:tt
    [ $bg_proc:tt $bg_doc:tt $bg_oth:tt [ $bg_cc:tt $bg_al:tt ] ]
    ) => {
        $(
        $crate::proc_macro_api_parse_fn! {
            $bg_al $bg_proc [ $bg_doc $bg_oth $bg_cc $prv ] $api
        }
        )?
        $(
        $crate::proc_macro_api_err_seg_blk_al! {
            $bg_al
        }
        $crate::proc_macro_api_parse_seg_call_attr! {
            [
                $(0 $bg_cc [] $seg_0)?
                $(1 [ :: ] [ :: ] $seg_cc_0 2 [ $($rest_0)* ])?
                $(3 [ :: ] [ :: ] { $($seg_blk_cc_0)* })?
                $(4 $bg_cc [] { $($seg_blk_0)* })?
            ]
            [ $($at_0),* ] [ $($al_0)? ]
            [ $prv $bg_cc $bg_proc $bg_doc $bg_oth ]
        }
        $(
        $crate::proc_macro_api_parse_seg_call_attr! {
            [
                $(0 $bg_cc [] $seg)?
                $(1 [ :: ] [ :: ] $seg_cc 2 [ $($rest)* ])?
                $(3 [ :: ] [ :: ] { $($seg_blk_cc)* })?
                $(4 $bg_cc [] { $($seg_blk)* })?
            ]
            [ $($at),* ] [ $($al)? ]
            [ $prv $bg_cc $bg_proc $bg_doc $bg_oth ]
        }
        )*
        )?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_seg_blk_al {
    ([]) => {};

    ([ $al:tt ]) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "no rules expected `as`",
            "\n  {...} as ",
            $crate::__private::stringify!($al),
            "\n        ^^",
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_seg_call_attr {
    (
    [
        $_i:tt $cc:tt $cc_tag:tt $seg:tt
        $(1 [ :: ] [ :: ] $seg_cc:tt)?
        $(2 [ $($rest:tt)* ])?
        $(3 [ :: ] [ :: ] $seg_blk_cc:tt)?
    ]
    $at:tt $al:tt
    [
        $prv:tt $bg_cc:tt
        [ $($bg_proc:tt)? $(; $_0:tt)? $(;)? ] $bg_doc:tt
        [ $($bg_oth:tt)? $(; $_1:tt)? $(;)? ]
    ]
    ) => {
        $crate::proc_macro_api_err_seg_inner_cc! {
            $cc_tag $bg_cc $prv [ $seg ]
        }
        $crate::proc_macro_api_err_seg_gp_attr! {
            $at [ $_i $($seg_blk_cc)? ] [
                $cc_tag $_i $seg $(1 $seg_cc)? $(2 [ $($rest)* ])?
            ]
        }
        $crate::proc_macro_api_parse_attr! {
            $at [/*[doc]*/] [/*[other]+[proc]*/] [/*[proc]*/]
            [ $($seg_cc)? $($($rest)*)? $($seg_blk_cc)? ] [/*prv*/] [ $seg ]
            [
                [ $($bg_proc)? ] $bg_doc [ $($bg_oth)? ]
                $prv [ $cc $al ]
            ]
        }
    };

    ([] [] [] $_:tt) => { /* empty comma (`,,`) and trailing comma (`... ,`) */ };

    // err: no seg
    ([] $at:tt $al:tt $_:tt) => {
        $crate::proc_macro_api_err_seg_no_seg! {
            $at $al
        }
    };

    // err: multiple seg
    (
    [
        $(0 $cc_0:tt [] $seg:tt)?
        $(1 [ :: ] [ :: ] $seg_cc:tt)?
        $(2 [ $($rest:tt)* ])?
        $(3 [ :: ] [ :: ] $seg_blk_cc:tt)?
        4 $cc_1:tt [] $seg_blk:tt
    ]
    $at:tt $al:tt $_:tt
    ) => {
        $crate::proc_macro_api_err_syn_gt_one! {
            "" =>
            [
                $($seg)? $(:: $seg_cc)? $($(:: $rest)*)?
                $(:: $seg_blk_cc)?
            ]
            [ $seg_blk ]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_seg_inner_cc {
    ([] $_0:tt $_1:tt $_:tt) => {};
    ([ :: ] [] [] $_:tt) => {};

    // [path]
    ([ :: ] $_0:tt $_1:tt $path:tt) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "leading `::` is in the middle of a path",
            "\n  :: ",
            $crate::proc_macro_api_fmt_path!($path),
            "\n  ^^",
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_seg_no_seg {
    // [at] al
    ([ $($at:tt)* ] [ $($al:tt)? ]) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "expected path segments",
            "\n/",
            $("\n| #", $crate::__private::stringify!($at),)*
            $("\n| as ", $crate::__private::stringify!($al),)?
            "\n|\n|_^ expected path segments",
        ));
    };
}

#[cfg(not(feature = "deny_group_attr"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_seg_gp_attr {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "deny_group_attr")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_seg_gp_attr {
    ([] $_:tt $path:tt) => {};
    ($at:tt [ $(0)? $(1)? ] $path:tt) => {};

    (
        [ $at_0:tt $(, $at:tt)* ] $_:tt
        [
            [ $($cc:tt)? ]
            $(0 $seg:tt)?
            $(1 $seg_cc:tt)?
            $(2 [ $($rest:tt)* ])?
            $(3 $seg_blk_cc:tt)?
            $(4 $seg_blk:tt)?
        ]
    ) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "attributes are applied to a path group",
            "\n/ #", $crate::__private::stringify!($at_0),
            $("\n| #", $crate::__private::stringify!($at),)*
            "\n| ",
            $crate::proc_macro_api_fmt_str_tt! {
                $($cc)? $($seg ::)? $($seg_cc ::)? $($($rest ::)*)? { ... }
            },
            "\n|_^",
            "\n= note: feature `deny_group_attr` is enabled",
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_fn_no_proc {
    ([ $($cc:tt)? ] [ $($prv:tt)* ] [ $($al:tt)? ] $api:tt) => {
        $crate::__private::compile_error!($crate::__private::concat!(
            "expected a proc-macro attribute",
            "\n/ ", $($crate::__private::stringify!($cc),)?
            $($crate::__private::stringify!($prv), "::",)*
            $crate::__private::stringify!($api),
            $("\n| as ", $crate::__private::stringify!($al),)?
            "\n|_^",
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_fn {
    ([ _ ] $proc:tt $bag:tt $api:tt) => { /* underscore alias */ };

    ($al:tt [] [ $_0:tt $_1:tt $cc:tt $prv:tt ] $api:tt) => {
        $crate::proc_macro_api_err_fn_no_proc!($cc $prv $al $api);
    };

    // $bg_al $bg_proc [ $bg_doc $bg_oth $bg_cc $prv ] $api
    (
    [ $($al:tt)? ]
    [
        $([ proc_macro $($arg_fn:tt)* ])?
        $([ proc_macro_attribute $($arg_at:tt)* ])?
        $([ proc_macro_derive $($arg_dr:tt)* ])?
        $(; $_0:tt)? $(;)?
    ]
    [ $doc:tt [ $other:tt $(; $_1:tt)? $(;)? ] $cc:tt $prv:tt ]
    $api:tt
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
    [ $($doc:tt)* ] [ $($other:tt)* ]
    [ $($cc:tt)? ] [ $($prv:tt)* ] $api:tt [ $name:tt $($_0:tt)? ]
    ( $($args:tt),* $(,)? ) $_1:tt
    ) => {
        $(# $doc)*
        $(# $other)*
        pub fn $name (
            $($args : $crate::__private::TokenStream),*
        ) -> $crate::__private::TokenStream {
            $crate::proc_macro_api_call! {
                ( $($args),* ) $($cc)? $($prv ::)* $api
            }
        }
    };
}

#[cfg(not(feature = "auto_transform"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_call {
    ($args:tt $($path:tt)*) => {
        $($path)* $args
    };
}

#[cfg(feature = "auto_transform")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_call {
    (( $($args:tt),* ) $($path:tt)*) => {
        $crate::__private::From::from(
            $($path)* (
                $($crate::__private::From::from($args),)*
            ),
        )
    };
}
