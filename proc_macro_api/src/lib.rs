#![doc = include_str!("../README.md")]

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err {
    ([ $($cc:tt)? ] [ $($seg:ident)* ] $api:ident $(as $alias:ident)?) => {
        std::compile_error!(std::concat!(
            "expected a proc_macro annotation for `", $(
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
    ([] $seg:tt $api:ident) => {
        $crate::proc_macro_api_err!([] $seg $api);
    };

    ([ $(proc_macro_attribute)? $(at)? ] $seg:tt $api:ident) => {
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

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse {
    // full-path begin without attr
    ($api_or_seg:ident $($body:tt)?) => {
        $crate::proc_macro_api_parse! {
            #[] => [] $api_or_seg $($body)?
        }
    };

    // full- or sub-path with attr: collect path segments
    (# $attr:tt $(# $_old_attr:tt)? =>
        [ $($pre:ident)* ] $seg:ident $body:tt
    ) => {
        $crate::proc_macro_api_parse! {
           #$attr => [$($pre)* $seg] $body
        }
    };

    // full- or sub-path with attr: parse its sub-path
    (# $attr:tt => $seg:tt {$(
        $(# $new_attr:tt)?
        $api_or_seg:ident $(:: $body:tt)?
    ),* $(,
    )?}) => {$(
        $crate::proc_macro_api_parse! {
            $(#$new_attr)? #$attr =>
            $seg $api_or_seg $($body)?
        }
    )*};

    // raw api: into api_fn
    (# $attr:tt $(# $_omit_attr:tt)? => $seg:tt $api:ident) => {
        $crate::proc_macro_api_fn! {
            $attr $seg $api
        }
    };
}

/// See the [document at module level][self].
#[macro_export]
macro_rules! proc_macro_api {
    ($($tt:tt)+) => {
        $crate::proc_macro_api_top! {
            [ $($tt)+ ] []
        }
    };
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_top {
    ([ $(# $at:tt)*
        $seg:ident $(:: $rest:tt)* $(as $al:ident)? $(, $($tt:tt)*)?
    ] [ $($line:tt)* ]
    ) => {
        $crate::proc_macro_api_top! {
            [ $($($tt)*)? ]
            [ $($line)* [ [ $($at)* ] [ $($al)? ] [] $seg $($rest)* ] ]
        }
    };

    ([ $(# $at:tt)*
        { $($inner:tt)* } $(, $($tt:tt)*)?
    ] [ $($line:tt)* ]
    ) => {
        $crate::proc_macro_api_top! {
            [ $($($tt)*)? ]
            [ $($line)* [ [ $($at)* ] [] [] { $($inner)* } ] ]
        }
    };

    ([ $(# $at:tt)*
        $(:: $seg:tt)+ $(as $al:ident)? $(, $($tt:tt)*)?
    ] [ $($line:tt)* ]
    ) => {
        $crate::proc_macro_api_top! {
            [ $($($tt)*)? ]
            [ $($line)* [ [ $($at)* ] [ $($al)? ] [ :: ] $($seg)+ ] ]
        }
    };

    ([] [ $([ [ $($at:tt)* ] [ $($al:tt)? ] $cc:tt $($seg:tt)* ])* ]) => {$(
        $crate::proc_macro_api_parse_attr! {
            [ $($at),* ] [/*[doc]*/] [/*[other]+[proc]*/] [/*[proc]*/]
            [ $($seg),* ] [/*prv*/] [/*last*/]
            [
                [/*[proc]*/] [/*[doc]*/] [/*[[other]+[proc]]*/]
                [ $cc [/*seg*/] $($al)? ]
            ]
        }
    )*};
}

#[cfg(not(feature = "no_shadow"))]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_shadow {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "no_shadow")]
#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_err_shadow {
    ([] $($tt:tt)*) => {};
    ([ $proc:tt ] $cover:tt
    [ $seg:tt $($rest:ident)* $({ $($_0:tt)* } $($_1:tt)*)? ]
    ) => {
        std::compile_error!(std::concat!(
            "multiple proc_macro attributes on one sub-path:\n",
            "      #", std::stringify!($proc), " <---- the first annotation\n",
            "      #", std::stringify!($cover), " <---- the second annotation\n",
            "  ... ", std::stringify!($seg),
            $("::", std::stringify!($rest),)*
            " ...",
        ));
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
        $crate::proc_macro_api_err_shadow! {
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
