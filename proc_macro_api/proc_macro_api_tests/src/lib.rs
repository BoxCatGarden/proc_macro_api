#![no_implicit_prelude]

#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

extern crate proc_macro_api_test_base as base;
use base::dummy_api;

dummy_api!(mod ,,,,,,,,,,,);

proc_macro_api!();
proc_macro_api!({});
proc_macro_api!(::{});
proc_macro_api!(a::{});
proc_macro_api!(::a::{});
proc_macro_api!(a::a::{});
proc_macro_api!(::a::a::{});

proc_macro_api! {
    /// a
    /// b
    #[allow(unused)]
    #[proc_macro]
    a::a::a::a::a::a::a::a::a::a::b as full_seg_0,

    /// a
    #[allow(unused)]
    #[proc_macro]
    /// b
    a::a::a::a::a::a::a::a::a::a::b as full_seg_1,

    /// a
    #[proc_macro]
    /// b
    #[allow(unused)]
    a::a::a::a::a::a::a::a::a::a::b as full_seg_2,

    /// b
    #[allow(unused)]
    #[proc_macro]
    b as empty_rest_0,

    /// b
    #[allow(unused)]
    #[proc_macro]
    a::b as one_rest_0,

    #[allow(unused)]
    #[proc_macro]
    /// b
    a::b as one_rest_1,

    #[proc_macro]
    /// b
    #[allow(unused)]
    a::b as one_rest_2,

    a::{#[fn] b as one_rest_out_0},

    a::a::{#[fn] b as one_prv_0},

    #[fn] b as alias_fn_0,
    #[proc_macro] b as alias_fn_1,
    #[at] c as alias_at_0,
    #[proc_macro_attribute] c as alias_at_1,
    #[dr(AliasDr0)] b as alias_dr_0,
    #[proc_macro_derive(AliasDr1)] b as alias_dr_1,

    a::a::a::{#[fn] b as seg_matcher_0},
    ::a as _,
    ::a::a as _,
    ::a::a::{},
    ::{},
    {},

    a::{a::{#[fn] b as no_trailing_comma_0}},
}

proc_macro_api! {
    #[fn] ::base::b as cc_0,
    ::base::{#[fn] b as cc_1},
    ::base::a::{#[fn] b as cc_2},
    ::{#[fn] base::b as cc_3},
}

mod no_as {
    use ::core::convert::Into as _;
    use ::proc_macro::TokenStream;
    use ::quote::quote;

    #[inline(always)]
    pub fn no_as_0(_input: TokenStream) -> TokenStream {
        quote! {
            "no_as"
        }
        .into()
    }
}

proc_macro_api!(#[fn] no_as::no_as_0);

#[cfg(feature = "auto_transform")]
mod pm2 {
    use ::proc_macro_api_test_base::dummy_api;

    dummy_api!(pub pm2 mod ,,,,,,,,,,,);
}

#[cfg(feature = "auto_transform")]
const PM2_B: fn(::proc_macro2::TokenStream) -> ::proc_macro2::TokenStream = pm2::b;

#[cfg(feature = "auto_transform")]
const PM2_C: fn(
    ::proc_macro2::TokenStream,
    ::proc_macro2::TokenStream,
) -> ::proc_macro2::TokenStream = pm2::c;

#[cfg(feature = "auto_transform")]
proc_macro_api! {
    #[fn] pm2::b as trans_0,
    #[at] pm2::c as trans_1,
    #[fn] PM2_B as trans_2,
    #[at] PM2_C as trans_3,
    #[fn] ::base::pm2::b as cc_trans_0,
    #[at] ::base::pm2::c as cc_trans_1,
}

#[cfg(not(any(feature = "deny_group_attr", feature = "deny_override")))]
proc_macro_api! {}

// call_attr
// fn
// {{}}
// attr
// error
