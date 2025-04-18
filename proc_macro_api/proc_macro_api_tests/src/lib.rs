#![no_implicit_prelude]

#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

use ::base::dummy_api;

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
    {#[fn] b as seg_matcher_1},
    ::a as _,
    ::a::a as _,
    ::a::a::{},
    ::{},
    {},,,,,,,,,

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
    use ::base::dummy_api;

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
#[cfg(all(feature = "allow_group_attr", feature = "allow_override"))]
proc_macro_api! {
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[cfg(not(feature = "allow_group_attr"))]
    #[at]
    {
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_override")]
        /// ```
        /// ```
        #[fn] b as global_local_0,
    },

    #[cfg(feature = "allow_group_attr")]
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[at]
    {
        #[cfg(feature = "allow_override")]
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        #[fn] b as global_local_0,
    },

    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[cfg(not(feature = "allow_group_attr"))]
    #[at]
    {
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_override")]
        /// ```
        /// ```
        c as global_local_0,
    },

    #[cfg(feature = "allow_group_attr")]
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[at]
    {
        #[cfg(feature = "allow_override")]
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        c as global_local_1,
    },

    #[at] {#[fn] b as override_0},
    #[at] {#[dr(Override1)] b as override_1},
    #[fn] {#[at] c as override_2},

    #[at] {#[fn] {b as bg_proc_oth_0}},

    #[at] {#[dr] {#[fn] {b as call_at_bg_0}}},

    #[at] {#[dr] {#[fn] b as prs_fn_bg_0}},
}

#[cfg(not(any(feature = "deny_group_attr", feature = "deny_override")))]
#[cfg(all(feature = "allow_group_attr", feature = "allow_override"))]
#[cfg(all(not(feature = "deny_shadow"), feature = "allow_shadow"))]
proc_macro_api! {
    #[at]#[at]#[at]#[at] {#[fn] b as oth_proc_0},

    #[at]#[at] {#[fn] b as bg_proc_oth_1},
}

#[cfg(all(not(feature = "deny_group_attr"), feature = "allow_group_attr"))]
proc_macro_api! {
    /// a
    {
        #[fn] b as bg_proc_doc_oth_0,

        /// a
        #[fn] {b as bg_proc_doc_oth_1},
    },

    #[fn] {{b as call_at_bg_1}},
}

#[cfg(feature = "non_optional_err")]
mod nop_err;

// error
