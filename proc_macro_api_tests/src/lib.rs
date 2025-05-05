#![doc = include_str!("../README.md")]
#![no_implicit_prelude]
#![no_std]

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
    a::a::a::a::a::a::a::a::a::a::b as norm_t_full_seg_0,

    /// a
    #[allow(unused)]
    #[proc_macro]
    /// b
    a::a::a::a::a::a::a::a::a::a::b as norm_t_full_seg_1,

    /// a
    #[proc_macro]
    /// b
    #[allow(unused)]
    a::a::a::a::a::a::a::a::a::a::b as norm_t_full_seg_2,

    /// b
    #[allow(unused)]
    #[proc_macro]
    b as norm_t_empty_rest_0,

    /// b
    #[allow(unused)]
    #[proc_macro]
    a::b as norm_t_one_rest_0,

    #[allow(unused)]
    #[proc_macro]
    /// b
    a::b as norm_t_one_rest_1,

    #[proc_macro]
    /// b
    #[allow(unused)]
    a::b as norm_t_one_rest_2,

    a::{#[fn] b as norm_t_one_rest_out_0},

    a::a::{#[fn] b as norm_t_one_prv_0},

    #[fn] b as norm_t_alias_fn_0,
    #[proc_macro] b as norm_t_alias_fn_1,
    #[at] c as norm_t_alias_at_0,
    #[proc_macro_attribute] c as norm_t_alias_at_1,
    #[dr(norm_t_AliasDr0)] b as norm_t_alias_dr_0,
    #[proc_macro_derive(norm_t_AliasDr1)] b as norm_t_alias_dr_1,

    a::a::a::{#[fn] b as norm_t_seg_matcher_0},
    {#[fn] b as norm_t_seg_matcher_1},
    ::a as _,
    ::a::a as _,
    ::a::a::{},
    ::{},
    {},,,,,,,,,

    a::{a::{#[fn] b as norm_t_no_trailing_comma_0}},
}

proc_macro_api! {
    ,,,,,,,,,
    #[fn] ::base::b as norm_t_cc_0,
    ::base::{#[fn] b as norm_t_cc_1},
    ::base::a::{#[fn] b as norm_t_cc_2},
    ::{#[fn] base::b as norm_t_cc_3},
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
        norm_t_no_as_0, norm_t_no_as_1;
        norm_t_no_as_2, norm_t_no_as_3;
        norm_t_no_as_4, norm_t_no_as_5;
        norm_t_no_as_6, norm_t_no_as_7;
    }
}

proc_macro_api! {
    {
        #[fn] no_as::norm_t_no_as_0,
        #[fn] no_as::norm_t_no_as_1,
    },
    {
        #[fn] no_as::a::norm_t_no_as_2,
        #[fn] no_as::a::norm_t_no_as_3,
    },
    {
        #[fn] no_as::a::a::norm_t_no_as_4,
        #[fn] no_as::a::a::norm_t_no_as_5,
    },
    no_as::a::a::a::{
        #[fn] norm_t_no_as_6,
        #[fn] norm_t_no_as_7,
    },
}

proc_macro_api! {
    {
        #[fn]#[doc=""] b as norm_t_seg_matcher_2_0,
        #[fn]#[doc=""] b as norm_t_seg_matcher_2_1,
    },
    {
        #[fn] a::b as norm_t_seg_matcher_3_0,
        #[fn] a::b as norm_t_seg_matcher_3_1,
    },
    {
        #[fn] a::a::b as norm_t_seg_matcher_4_0,
        #[fn] a::a::b as norm_t_seg_matcher_4_1,
    },
    {
        #[fn] a::a::a::b as norm_t_seg_matcher_5_0,
        #[fn] a::a::a::b as norm_t_seg_matcher_5_1,
    },
    {
        a::{#[fn] a::a::a::b as norm_t_seg_matcher_6_0},
        a::{#[fn] a::a::a::b as norm_t_seg_matcher_6_1},
    },
    {
        a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_7_0},
        a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_7_1},
    },
    {
        a::a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_8_0},
        a::a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_8_1},
    },
    {
        a::a::a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_9_0},
        a::a::a::a::{#[fn] a::a::a::b as norm_t_seg_matcher_9_1},
    },
    {
        #[fn]#[doc=""] ::b as _,
        #[fn]#[doc=""] ::b as _,
    },
    {
        #[fn] ::base::b as norm_t_seg_matcher_10_0,
        #[fn] ::base::b as norm_t_seg_matcher_10_1,
    },
    {
        #[fn] ::base::a::b as norm_t_seg_matcher_11_0,
        #[fn] ::base::a::b as norm_t_seg_matcher_11_1,
    },
    {
        ::base::{#[fn] a::a::b as norm_t_seg_matcher_12_0},
        ::base::{#[fn] a::a::b as norm_t_seg_matcher_12_1},
    },
    {
        ::base::a::{#[fn] a::a::b as norm_t_seg_matcher_13_0},
        ::base::a::{#[fn] a::a::b as norm_t_seg_matcher_13_1},
    },
    {
        ::base::a::a::{#[fn] a::a::b as norm_t_seg_matcher_14_0},
        ::base::a::a::{#[fn] a::a::b as norm_t_seg_matcher_14_1},
    },
    {
        ::{#[fn] base::a::a::a::a::a::b as norm_t_seg_matcher_15_0},
        ::{#[fn] base::a::a::a::a::a::b as norm_t_seg_matcher_15_1},
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
    #[fn] pm2::b as trans_t_trans_0,
    #[at] pm2::c as trans_t_trans_1,
    #[fn] PM2_B as trans_t_trans_2,
    #[at] PM2_C as trans_t_trans_3,
    #[fn] ::base::pm2::b as trans_t_cc_trans_0,
    #[at] ::base::pm2::c as trans_t_cc_trans_1,
}

#[cfg(feature = "attr_tests")]
#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_append"))]
#[cfg(not(feature = "deny_override"))]
#[cfg(not(feature = "with_default"))]
proc_macro_api! {
    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// let ok: i32;
    #[cfg(not(feature = "attr_tests"))]
    #[at]
    {
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        #[cfg(feature = "attr_tests")]
        /// ```no_run
        /// ```
        #[fn] b as attr_gp_ap_ov_t_global_local_0,

        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        #[cfg(feature = "attr_tests")]
        /// ```no_run
        /// ```
        a::c as attr_gp_ap_ov_t_global_local_1,
    },

    #[cfg(feature = "attr_tests")]
    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// let ok: i32;
    #[at]
    {
        #[cfg(feature = "attr_tests")]
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        /// ```no_run
        /// ```
        #[fn] b as attr_gp_ap_ov_t_global_local_0,

        #[cfg(feature = "attr_tests")]
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        /// ```no_run
        /// ```
        a::c as attr_gp_ap_ov_t_global_local_1,
    },

    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// #![no_implicit_prelude]
    /// fn main() {
    /// let err: MissingTheInnerDoc;
    /// }
    #[cfg(feature = "attr_tests")]
    #[at]
    {
        #[cfg(feature = "attr_tests")]
        /// struct MissingTheInnerDoc;
        #[cfg(not(feature = "attr_tests"))]
        /// ```
        c as attr_gp_ap_ov_t_global_local_2,

        #[cfg(feature = "attr_tests")]
        /// struct MissingTheInnerDoc;
        #[cfg(feature = "attr_tests")]
        /// ```
        #[fn] a::b as attr_gp_ap_ov_t_global_local_3,
    },

    #[cfg(feature = "attr_tests")]
    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// #![no_implicit_prelude]
    /// fn main() {
    /// let err: MissingTheInnerDoc;
    /// }
    #[at]
    {
        /// struct MissingTheInnerDoc;
        #[cfg(feature = "attr_tests")]
        /// ```
        #[cfg(feature = "attr_tests")]
        c as attr_gp_ap_ov_t_global_local_2,

        /// struct MissingTheInnerDoc;
        #[cfg(not(feature = "attr_tests"))]
        /// ```
        #[cfg(feature = "attr_tests")]
        #[fn] a::b as attr_gp_ap_ov_t_global_local_3,
    },
}

#[cfg(feature = "attr_tests")]
#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_override"))]
#[cfg(not(feature = "with_default"))]
proc_macro_api! {
    #[at] {#[fn] b as attr_gp_ov_t_override_0},
    #[at] {#[dr(attr_gp_ov_t_Override1)] b as attr_gp_ov_t_override_1},
    #[fn] {#[at] c as attr_gp_ov_t_override_2},

    #[at] {#[fn] {b as attr_gp_ov_t_bg_proc_oth_0}},

    #[at] {#[dr] {#[fn] {b as attr_gp_ov_t_call_at_bg_0}}},

    #[at] {#[dr] {#[fn] b as attr_gp_ov_t_prs_fn_bg_0}},
}

#[cfg(feature = "attr_tests")]
#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_override"))]
#[cfg(not(feature = "deny_shadow"))]
#[cfg(not(feature = "with_default"))]
proc_macro_api! {
    #[at]#[at]#[at]#[at] {#[fn] b as attr_gp_ov_sh_t_oth_proc_0},

    #[at]#[at] {#[fn] b as attr_gp_ov_sh_t_bg_proc_oth_1},
}

#[cfg(feature = "attr_tests")]
#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_append"))]
#[cfg(not(feature = "with_default"))]
proc_macro_api! {
    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// let ok: i32;
    #[cfg(not(feature = "attr_tests"))]
    #[fn]
    {
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        #[cfg(feature = "attr_tests")]
        /// ```no_run
        /// ```
        b as attr_gp_ap_t_global_local_0,

        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        #[cfg(feature = "attr_tests")]
        /// ```no_run
        /// ```
        a::b as attr_gp_ap_t_global_local_1,
    },

    #[cfg(feature = "attr_tests")]
    /// ```no_run
    #[cfg(feature = "attr_tests")]
    /// let ok: i32;
    #[fn]
    {
        #[cfg(feature = "attr_tests")]
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        /// ```no_run
        /// ```
        b as attr_gp_ap_t_global_local_0,

        #[cfg(feature = "attr_tests")]
        /// ```
        #[cfg(feature = "attr_tests")]
        /// let MissingTheOuterDoc: () = 0;
        /// ```no_run
        /// ```
        a::b as attr_gp_ap_t_global_local_1,
    },

    /// a
    {
        #[fn] b as attr_gp_ap_t_bg_proc_doc_oth_0,

        /// a
        #[fn] {b as attr_gp_ap_t_bg_proc_doc_oth_1},
    },
}

#[cfg(feature = "attr_tests")]
#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "with_default"))]
proc_macro_api! {
    #[fn] {{b as attr_gp_t_call_at_bg_1}},

    #[doc=""] {#[fn] a::b as attr_gp_t_err_ap_0},
    #[fn] {#[doc=""] a::a::b as attr_gp_t_err_ov_0},
}

#[allow(unused)]
macro_rules! error {
    ($($name:ident : { $($tt:tt)* }),* $(,)?) => {$(
        /// ```no_run
        /// mod _a {
        /// use ::proc_macro_api::proc_macro_api;
        #[doc = concat!("proc_macro_api!\n{ ",
            $(stringify!($tt), " ",)*
        "}")]
        /// }
        /// fn main() {}
        /// ```
        #[allow(dead_code)]
        fn $name() {}
    )*};
}

#[cfg(feature = "non_optional_err")]
mod nop_err;

#[cfg(not(feature = "comp_err"))]
mod comp_err;

#[cfg(feature = "comp_err")]
include!("./comp_err.rs");

#[cfg(feature = "err_attr_tests")]
mod err_attr;

#[cfg(feature = "playground")]
#[macro_use]
mod playground;

#[cfg(feature = "playground")]
playground!();
