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
/// The nested `mod` are as many as the input `tt`s.
/// The innermost `mod` doesn't have an inner `pub mod a`.
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
            dummy_api!(pub mod $($count_down)+);
        }
        )?
    };

    ($vis:vis mod) => {};
}

// dummy_api!(pub mod ,,,,,,,,,,,,);
