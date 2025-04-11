extern crate proc_macro;

#[no_link]
extern crate proc_macro_api;

use proc_macro_api::proc_macro_api;

#[allow(unused)]
mod mod_a {
    use proc_macro::TokenStream;

    pub fn an_fn_api(input: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    pub mod mod_b {
        use proc_macro::TokenStream;

        pub fn an_attr_api(args: TokenStream, item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn a_derive_api(item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn test_attr_0(args: TokenStream, item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn test_derive_0(item: TokenStream) -> TokenStream {
            TokenStream::new()
        }
    }
}

proc_macro_api! {
    mod_a::{
        // alias and documentation
        /// A function-like macro.
        #[fn] an_fn_api,

        // forwarding
        #[proc_macro_attribute]
        mod_b::{
            // use the forwarded annotation
            an_attr_api as the_attr_api,

            // override
            #[dr(Something)] a_derive_api,
        },
    },
    #[proc_macro_derive(DeriveA)]
    mod_a::{
        #[at]
        mod_b::test_attr_0,
        mod_b::test_derive_0
    }
}

proc_macro_api!();
proc_macro_api! {
    #[proc_macro]
    /// bbb
    #[allow(unused)]
    mod_a::{
        #[fn] an_fn_api as milk
    },
    mod_a::{}
}

proc_macro_api!(a as _, a as _);