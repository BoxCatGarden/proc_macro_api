#![no_implicit_prelude]
#![no_std]

extern crate proc_macro;

/// ```no_run
/// # macro_rules! a {
/// #     ($vis:vis) => {
/// $vis fn b(_) -> _ {}
/// $vis fn c(_, _) -> _ {}
/// $vis mod a {
///     pub fn b(_) -> _ {}
///     pub fn c(_, _) -> _ {}
///     pub mod a { ... }
/// }
/// #     };
/// # }
/// ```
/// Nesting the above `mod a`.
/// The number of the `mod`s equals the number of the input `tt`s.
/// The innermost `mod` is empty.
///
/// * When `b()` is called with a non-empty `TokenStream`,
/// it always generates a `const D_NUM: usize`.
/// * When `b()` is called with an empty `TokenStream`,
/// it always returns a `usize`.
/// * `c()` always replaces the input item with a `const NUM: usize`.
///
/// Those `usize` values are respectively equal to the
/// nest-depths of the functions that output them.
/// The nest-depth of the outermost function is 0.
#[macro_export]
macro_rules! dummy_api {
    ($vis:vis mod $($count_down:tt)*) => {
        $crate::dummy_api_inner! {
            proc_macro [] $vis mod $($count_down)*
        }
    };

    ($vis:vis pm2 mod $($count_down:tt)*) => {
        $crate::dummy_api_inner! {
            proc_macro2 [] $vis mod $($count_down)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! dummy_api_inner {
    ($pm:ident [ $($count:tt)* ] $vis:vis mod $tt:tt $($count_down:tt)*) => {
        #[allow(dead_code)]
        #[inline(always)]
        $vis fn b(input: ::$pm::TokenStream) -> ::$pm::TokenStream {
            use ::quote::quote;
            use ::core::convert::Into as _;

            const NUM: usize = const { (&[ $($count),* ] as &[u8]).len() };

            if input.is_empty() {
                quote! { #NUM }.into()
            } else {
                quote! {
                    const D_NUM: usize = #NUM;
                }.into()
            }
        }

        #[allow(dead_code)]
        #[inline(always)]
        $vis fn c(
            _: ::$pm::TokenStream,
            _: ::$pm::TokenStream,
        ) -> ::$pm::TokenStream {
            use ::quote::quote;
            use ::core::convert::Into as _;

            const NUM: usize = const { (&[ $($count),* ] as &[u8]).len() };

            quote! {
                const NUM: usize = #NUM;
            }.into()
        }

        #[allow(dead_code)]
        $vis mod a {
            $crate::dummy_api_inner! {
                $pm [ $($count)* 0u8 ] pub mod $($count_down)*
            }
        }
    };

    ($($tt:tt)*) => {};
}

pub mod pm2 {
    // 11 non-empty `mod`
    dummy_api!(pub pm2 mod ,,,,,,,,,,,,);
}

// 11 non-empty `mod`
dummy_api!(pub mod ,,,,,,,,,,,,);
