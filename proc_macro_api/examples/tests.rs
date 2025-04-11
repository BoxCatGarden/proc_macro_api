extern crate proc_macro;

#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

macro_rules! api {
    ($($vis:ident)? => $tt:tt $($count_down:tt)*) => {
        #[allow(dead_code)]
        $($vis)? mod a {
            use proc_macro::TokenStream;

            pub fn b(_: TokenStream) -> TokenStream {
                TokenStream::new()
            }

            pub fn c(_: TokenStream, _: TokenStream) -> TokenStream {
                TokenStream::new()
            }

            api!(pub => $($count_down)*);
        }
    };

    ($($vis:ident)? =>) => {};
}

api!(=> ,,,,,,,);

