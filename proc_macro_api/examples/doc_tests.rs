extern crate proc_macro;

// in the crate root
#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

mod mod_a {
    use proc_macro::TokenStream;

    pub fn an_fn_api(_input: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    pub mod mod_b {
        use proc_macro::TokenStream;

        pub fn an_attr_api(_args: TokenStream, _item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn a_derive_api(_item: TokenStream) -> TokenStream {
            TokenStream::new()
        }
    }
}

proc_macro_api! {
    // a path group
    mod_a::{
        // use an alias of the proc-macro attributes
        #[fn] an_fn_api,

        // a path subgroup;
        // apply local attributes to this path group
        #[allow(unused)]
        #[proc_macro_attribute]
        mod_b::{
            // apply the local attributes from the outside:
            // #[allow(unused)]
            // #[proc_macro_attribute]
            /// Documents won't override local attributes.
            /// This API is renamed `the_attr_api`.
            an_attr_api as the_attr_api,

            // override the local attributes from the outside:
            // only `#[dr(Something)]` is applied to this path
            #[dr(Something)] a_derive_api,

            // syntactically valid
            nonexistent_api as _,
        },
    },

    // syntactically valid
    ::nonexistent_mod::nonexistent_api as _,
}
// It will expand to three `pub` functions in the crate root, named
// `an_fn_api`, `the_attr_api`, and `a_derive_api`, respectively.
