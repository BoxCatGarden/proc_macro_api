#![doc = include_str!("../README.md")]

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
        $crate::proc_macro_api_top! {
            * [ $($at)* ] [ $($al)? ]
            $([ :: ] $seg_0 $($rest_0)*)?
            $([] $seg_1 $($rest_1)*)?
            $([] { $($seg_2)* } $($rest_2)*)?
        }
        $crate::proc_macro_api_err_syn_gt_one! {
            $([ :: $seg_0 ])? $([ $seg_1 ])? $([ { $($seg_2)* } ])?
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
    [ $($prv:tt)* ] [ $($last:tt)? $(; $to_prv:tt)? $(;)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [ $($doc)* [ doc $($arg_doc)* ] ] $other $proc
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg ;)? $($last)? ]
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
    [ $($seg:tt)? $(, $rest:tt)* ]
    [ $($prv:tt)* ] [ $($last:tt)? $(; $to_prv:tt)? $(;)? ]
    [ [ $($bg_proc:tt)? ] [ $($bg_doc:tt)* ] [ $($bg_oth:tt)? ] $path:tt ]
    ) => {
        $crate::proc_macro_api_parse_seg! {
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg ;)? $($last)? ]
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
        $crate::proc_macro_api_err_attr_override! {
            $($bg_oth)? $([ $($other)+ ])?
            [ $($prv)* $($to_prv)? $($last)? $($seg)? $($rest)* ]
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
    [ $($prv:tt)* ] [ $($last:tt)? $(; $to_prv:tt)? $(;)? ]
    $bag:tt
    ) => {
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
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg ;)? $($last)? ]
            $bag
        }
        $crate::proc_macro_api_err_attr_shadow! {
            [ $($proc)? ]
            $([proc_macro$($arg_fn_0)*])?
            $([fn$($arg_fn)*])?
            $([proc_macro_attribute$($arg_at_0)*])?
            $([at$($arg_at)*])?
            $([proc_macro_derive$($arg_dr_0)*])?
            $([dr$($arg_dr)*])?
            [ $($prv)* $($to_prv)? $($last)? $($seg)? $($rest)* ]
        }
    };

    // [`any`]
    (
    [ $any:tt $(, $at:tt)* ]
    $doc:tt [ $($other:tt)* ] $proc:tt
    [ $($seg:tt)? $(, $rest:tt)* ]
    [ $($prv:tt)* ] [ $($last:tt)? $(; $to_prv:tt)? $(;)? ]
    $bag:tt
    ) => {
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] $doc [ $($other)* $any ] $proc
            [ $($rest),* ] [ $($prv)* $($to_prv)? ] [ $($seg ;)? $($last)? ]
            $bag
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_seg {
    () => {};
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
macro_rules! proc_macro_api_fn {
    ($bag:tt) => {
        $crate::proc_macro_api_err!([] $seg $api);
    };

    ([ proc_macro_attribute ] $bag:tt) => {
        $crate::proc_macro_api_fn! {
            ( args, item ) [ proc_macro_attribute ] $seg $api
        }
    };

    ([ $(proc_macro)? $(fn)? ] $seg:tt $api:ident) => {
        $crate::proc_macro_api_fn! {
            ( input ) [ proc_macro ] $seg $api
        }
    };

    ([ $(proc_macro_derive)? $(dr)? ( $($drv:tt)* ) ] $seg:tt $api:ident) => {
        $crate::proc_macro_api_fn! {
            ( item ) [ proc_macro_derive ( $($drv)* ) ] $seg $api
        }
    };

    ($attr:tt $seg:tt $api:ident) => {
        $crate::proc_macro_api_err!($attr $seg $api);
    };

    // api_fn
    (( $($arg:ident),* $(,)? ) $attr:tt
        [ $($seg:ident)* ] $api:ident
    ) => {
        #$attr
        pub fn $api (
            $($arg: proc_macro::TokenStream),*
        ) -> proc_macro::TokenStream {
            $($seg::)* $api ( $($arg),* )
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
