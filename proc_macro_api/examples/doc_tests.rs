#![recursion_limit = "19"]

// in the crate root
extern crate proc_macro;

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

        // a path subgroup
        #[allow(unused)] // global attribute
        #[proc_macro_attribute] // local attribute
        mod_b::{
            // #[allow(unused)]
            /// `#[doc]` is global.
            /// This API is renamed `the_attr_api`.
            // #[proc_macro_attribute]
            an_attr_api as the_attr_api,

            // #[allow(unused)]
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

// ===============================================

proc_macro_api! {
    sub::{
        #[proc_macro]
        proc_fn,
    },
}

mod sub {
    use proc_macro::TokenStream;

    pub fn proc_fn(_input: TokenStream) -> TokenStream {
        TokenStream::new()
    }
}

// ===============================================

proc_macro_api! {
    group_a::{
        subgroup_of_a::{
            path_of_fn as _,
        },
    },
}

// ===============================================

proc_macro_api!(a as _, a as _);
