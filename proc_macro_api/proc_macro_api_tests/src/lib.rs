#![no_implicit_prelude]

mod test_invalid_combination;

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
    ,,,,,,,,,
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
    ,,,,,,,,,
    #[fn] ::base::b as cc_0,
    ::base::{#[fn] b as cc_1},
    ::base::a::{#[fn] b as cc_2},
    ::{#[fn] base::b as cc_3},
}

mod no_as {
    macro_rules! no_as {
        ($name:ident) => {
            #[inline(always)]
            pub fn $name(_input: TokenStream) -> TokenStream {
                let name = stringify!($name);

                quote! {
                    #name
                }
                .into()
            }
        };
    }

    macro_rules! no_as_mod {
        ($name_0:ident, $name_1:ident $(; $($tt:tt)*)?) => {
            use ::core::convert::Into as _;
            use ::proc_macro::TokenStream;
            use ::quote::quote;
            no_as!($name_0);
            no_as!($name_1);

            $(
            pub mod a {
                no_as_mod!($($tt)*);
            }
            )?
        };

        () => {};
    }

    no_as_mod! {
        no_as_0, no_as_1;
        no_as_2, no_as_3;
        no_as_4, no_as_5;
        no_as_6, no_as_7;
    }
}

proc_macro_api! {
    {
        #[fn] no_as::no_as_0,
        #[fn] no_as::no_as_1,
    },
    {
        #[fn] no_as::a::no_as_2,
        #[fn] no_as::a::no_as_3,
    },
    {
        #[fn] no_as::a::a::no_as_4,
        #[fn] no_as::a::a::no_as_5,
    },
    no_as::a::a::a::{
        #[fn] no_as_6,
        #[fn] no_as_7,
    },
}

proc_macro_api! {
    {
        #[fn]#[doc=""] b as seg_matcher_2_0,
        #[fn]#[doc=""] b as seg_matcher_2_1,
    },
    {
        #[fn] a::b as seg_matcher_3_0,
        #[fn] a::b as seg_matcher_3_1,
    },
    {
        #[fn] a::a::b as seg_matcher_4_0,
        #[fn] a::a::b as seg_matcher_4_1,
    },
    {
        #[fn] a::a::a::b as seg_matcher_5_0,
        #[fn] a::a::a::b as seg_matcher_5_1,
    },
    {
        a::{#[fn] a::a::a::b as seg_matcher_6_0},
        a::{#[fn] a::a::a::b as seg_matcher_6_1},
    },
    {
        a::a::{#[fn] a::a::a::b as seg_matcher_7_0},
        a::a::{#[fn] a::a::a::b as seg_matcher_7_1},
    },
    {
        a::a::a::{#[fn] a::a::a::b as seg_matcher_8_0},
        a::a::a::{#[fn] a::a::a::b as seg_matcher_8_1},
    },
    {
        a::a::a::a::{#[fn] a::a::a::b as seg_matcher_9_0},
        a::a::a::a::{#[fn] a::a::a::b as seg_matcher_9_1},
    },
    {
        #[fn]#[doc=""] ::b as _,
        #[fn]#[doc=""] ::b as _,
    },
    {
        #[fn] ::base::b as seg_matcher_10_0,
        #[fn] ::base::b as seg_matcher_10_1,
    },
    {
        #[fn] ::base::a::b as seg_matcher_11_0,
        #[fn] ::base::a::b as seg_matcher_11_1,
    },
    {
        ::base::{#[fn] a::a::b as seg_matcher_12_0},
        ::base::{#[fn] a::a::b as seg_matcher_12_1},
    },
    {
        ::base::a::{#[fn] a::a::b as seg_matcher_13_0},
        ::base::a::{#[fn] a::a::b as seg_matcher_13_1},
    },
    {
        ::base::a::a::{#[fn] a::a::b as seg_matcher_14_0},
        ::base::a::a::{#[fn] a::a::b as seg_matcher_14_1},
    },
    {
        ::{#[fn] base::a::a::a::a::a::b as seg_matcher_15_0},
        ::{#[fn] base::a::a::a::a::a::b as seg_matcher_15_1},
    },
}

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

        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_override")]
        /// ```
        /// ```
        a::c as global_local_1,
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

        #[cfg(feature = "allow_override")]
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        a::c as global_local_1,
    },

    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[cfg(not(feature = "allow_group_attr"))]
    #[at]
    {
        #[cfg(feature = "allow_override")]
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        c as global_local_2,

        #[cfg(feature = "allow_override")]
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        #[fn] a::b as global_local_3,
    },

    #[cfg(feature = "allow_group_attr")]
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[at]
    {
        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_override")]
        /// ```
        /// ```
        c as global_local_2,

        /// ```
        #[cfg(feature = "allow_override")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_override")]
        /// ```
        /// ```
        #[fn] a::b as global_local_3,
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
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[cfg(not(feature = "allow_group_attr"))]
    #[fn]
    {
        /// ```
        #[cfg(feature = "allow_group_attr")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_group_attr")]
        /// ```
        /// ```
        b as global_local_gp_0,

        /// ```
        #[cfg(feature = "allow_group_attr")]
        /// let MisMatchCausedByOverride: () = 0;
        #[cfg(feature = "allow_group_attr")]
        /// ```
        /// ```
        a::b as global_local_gp_1,
    },

    #[cfg(feature = "allow_group_attr")]
    /// ```
    #[cfg(feature = "allow_group_attr")]
    /// let ok: i32;
    #[fn]
    {
        #[cfg(feature = "allow_group_attr")]
        /// ```
        #[cfg(feature = "allow_group_attr")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        b as global_local_gp_0,

        #[cfg(feature = "allow_group_attr")]
        /// ```
        #[cfg(feature = "allow_group_attr")]
        /// let MisMatchCausedByOverride: () = 0;
        /// ```
        /// ```
        a::b as global_local_gp_1,
    },

    /// a
    {
        #[fn] b as bg_proc_doc_oth_0,

        /// a
        #[fn] {b as bg_proc_doc_oth_1},
    },

    #[fn] {{b as call_at_bg_1}},
}

#[allow(unused)]
macro_rules! error {
    ($($name:ident : { $($tt:tt)* }),* $(,)?) => {$(
        /// ```
        /// use ::proc_macro_api::proc_macro_api;
        #[doc = concat!("proc_macro_api!\n{ ",
            $(stringify!($tt), " ",)*
        "}")]
        /// fn main() {}
        /// ```
        #[allow(dead_code)]
        fn $name() {}
    )*};
}

#[allow(unused)]
macro_rules! api {
    ($($name:ident : { $($tt:tt)* }),* $(,)?) => {$(
        ::proc_macro_api::proc_macro_api! {
            $($tt)*
        }
    )*};
}

#[cfg(feature = "non_optional_err")]
mod nop_err;

#[cfg(feature = "err_nonexistent_fn")]
proc_macro_api! {
    #[fn] nonexistent_fn as err_nonexistent_fn_0,
}

#[cfg(feature = "attr_tests")]
mod err_gp;

#[cfg(feature = "attr_tests")]
mod err_ov;

#[cfg(feature = "attr_tests")]
mod err_sh;
