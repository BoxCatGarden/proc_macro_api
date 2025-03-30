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
            [] $($tt)+
        }
    };
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_top {
    ([ $($line:tt)* ]
        $(# $at:tt)*
        $seg:ident $(:: $rest:tt)* $(as $al:ident)? $(,
            $($tt:tt)*
        )?
    ) => {
        $crate::proc_macro_api_top! {
            [
                $($line)*
                [ [ $($at)* ] [ $($al)? ] [] $seg $($rest)* ]
            ]
            $($($tt)*)?
        }
    };

    ([ $($line:tt)* ]
        $(# $at:tt)*
        $(:: $seg:tt)+ $(as $al:ident)? $(,
            $($tt:tt)*
        )?
    ) => {
        $crate::proc_macro_api_top! {
            [
                $($line)*
                [ [ $($at)* ] [ $($al)? ] [ :: ] $($seg)+ ]
            ]
            $($($tt)*)?
        }
    };

    ([ $([ [ $($at:tt)* ] $al:tt $cc:tt $($seg:tt)* ])* ]) => {$(
        $crate::proc_macro_api_parse_attr! {
            [ [ $al $cc ] [ [/*doc*/] [/*other;proc*/] ] [/*seg*/] ]
            [ [/*doc*/] [/*other*/] [/*proc*/] $($at)* ]
            [ [/*prev*/] [/*last*/] $($seg),* ]
        }
    )*};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_api_parse_attr {
    ($bag:tt
    [ [ $($doc:tt)* ] $other:tt $proc:tt [ doc $($doc_arg:tt)* ] $($at:tt)* ]
    [ [ $($prv:tt)* ] [ $($last:tt)? $(; $to_prv:tt)? $(;)? ] $($seg:tt)? $(, $rest:tt)* ]
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
            [ [ $($doc)* [ doc $($doc_arg)* ] ] $other $proc $($at)* ]
            [ [ $($prv)* $($to_prv)? ] [ $($seg ;)? $($last)? ] $($rest),* ]
        }
    };

    ($bag:tt
        $doc:tt [ $($other:tt)* ] $proc:tt
        [] $($at:tt)*
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
                $doc
                [ $($other)* [] ]
                $proc
            $($at)*
        }
    };

    ($bag:tt
        $doc:tt $other:tt [ $($proc:tt)? ]
        [ $(proc_macro)? $(fn)? ] $($at:tt)*
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
                $doc
                $other
                [[ proc_macro ]]
            $($at)*
        }
        $(
        #[cfg(not(feature = "allow_shadow"))]
        $crate::proc_macro_api_err! { 'shadow =>
            $bag $proc proc_macro fn
        }
        )?
    };

    ($bag:tt
        $doc:tt $other:tt [ $($proc:tt)? ]
        [ $(proc_macro_attribute)? $(at)? ] $($at:tt)*
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
                $doc
                $other
                [[ proc_macro_attribute ]]
            $($at)*
        }
        $(
        #[cfg(not(feature = "allow_shadow"))]
        $crate::proc_macro_api_err! { 'shadow =>
            $bag $proc proc_macro_attribute at
        }
        )?
    };

    ($bag:tt
        $doc:tt $other:tt [ $($proc:tt)? ]
        [ $(proc_macro_derive)? $(dr)? ( $($drv:tt)* ) ] $($at:tt)*
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
                $doc
                $other
                [[ proc_macro_derive ( $($drv)* ) ]]
            $($at)*
        }
        $(
        #[cfg(not(feature = "allow_shadow"))]
        $crate::proc_macro_api_err! { 'shadow =>
            $bag $proc proc_macro_derive dr
        }
        )?
    };

    ($bag:tt
        $doc:tt [ $($other:tt)* ] $proc:tt
        $any:tt $($at:tt)*
    ) => {
        $crate::proc_macro_api_parse_attr! {
            $bag
                $doc
                [ $($other)* $any ]
                $proc
            $($at)*
        }
    };

    ($bag:tt $doc:tt $other:tt $proc:tt) => {

    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        println!("a = {}", a());
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
