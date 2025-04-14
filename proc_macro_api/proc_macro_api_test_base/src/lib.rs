#![no_implicit_prelude]

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
/// * When `b()` is a derive-macro, it accepts a `struct` and `impl`s it
/// with a `const NUM: usize`.
/// * When `b()` is a function-like macro, it always returns a `usize`.
/// * `c()` always replaces the input item with a `const NUM: usize`.
///
/// The value of the `const NUM: usize` and the returned `usize` equals
/// the nest-depth of the function. The nest-depth of the outermost
/// function is 0.
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
            use ::syn;
            use ::syn::parse::{Parse, Parser};
            use ::core::result::Result::Ok;
            use ::core::convert::Into as _;

            let num = (&[ $($count),* ] as &[u8]).len();

            if let Ok(ref item) = syn::ItemStruct::parse.parse2(input.into()) {
                let name = &item.ident;
                let generics = &item.generics;
                quote! {
                    impl #generics #name #generics {
                        const NUM: usize = #num;
                    }
                }.into()
            } else {
                quote! { #num }.into()
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

            let num = (&[ $($count),* ] as &[u8]).len();

            quote! {
                const NUM: usize = #num;
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

extern crate proc_macro;

// 11 non-empty `mod`
dummy_api!(pub mod ,,,,,,,,,,,,);
