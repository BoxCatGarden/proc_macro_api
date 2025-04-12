extern crate proc_macro;

#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

#[macro_use]
mod test_base;
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
