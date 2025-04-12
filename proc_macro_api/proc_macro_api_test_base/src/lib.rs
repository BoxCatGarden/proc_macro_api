/// ```ignore
/// $vis fn b(_) -> _ {}
/// $vis fn c(_, _) -> _ {}
/// $vis mod a {
///     pub fn b(_) -> _ {}
///     pub fn c(_, _) -> _ {}
///     pub mod a { ... }
/// }
/// ```
/// Nesting the above `mod a`.
/// The number of the `mod`s equals the number of the input `tt`s minus one.
/// The innermost `mod` doesn't have an inner `pub mod a`.
#[macro_export]
macro_rules! dummy_api {
    ($vis:vis mod $tt:tt $($($count_down:tt)+)?) => {
        #[allow(dead_code)]
        $vis fn b(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
            proc_macro::TokenStream::new()
        }

        #[allow(dead_code)]
        $vis fn c(_: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream {
            proc_macro::TokenStream::new()
        }

        $(
        #[allow(dead_code)]
        $vis mod a {
            $crate::dummy_api!(pub mod $($count_down)+);
        }
        )?
    };

    ($vis:vis mod) => {};
}

extern crate proc_macro2 as proc_macro;

// 11 levels
dummy_api!(pub mod ,,,,,,,,,,,,);
