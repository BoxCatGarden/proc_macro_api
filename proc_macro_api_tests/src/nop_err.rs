mod blk_al {
    error! {
        blk_al_0: { {} as _ },
        blk_al_1: { ::{} as _ },
    }
}

mod inner_cc {
    error! {
        inner_cc_0: { ::{::a::b as _} },
        inner_cc_1: { a::{::b as _} },
        inner_cc_2: { ::{,::a::b as _} },
        inner_cc_3: { a::{,::b as _} },
    }
}

mod no_seg {
    error! {
        no_seg_0_0: { #[fn] },
        no_seg_1_0: { #[fn]#[doc=""] },
        no_seg_2_0: { #[fn]#[doc=""]#[doc=""] },

        no_seg_0_1: {, #[fn] },
        no_seg_1_1: {, #[fn]#[doc=""] },
        no_seg_2_1: {, #[fn]#[doc=""]#[doc=""] },
    }
}

mod mul_seg {
    error! {
        mul_seg_0_0: { a {} },
        mul_seg_1_0: { a::a {} },
        mul_seg_2_0: { a::a::a {} },
        mul_seg_3_0: { a::a::a::{} {} },
        mul_seg_4_0: { a::a::{} {} },
        mul_seg_5_0: { a::{} {} },
        mul_seg_6_0: { ::a {} },
        mul_seg_7_0: { ::a::a {} },
        mul_seg_8_0: { ::a::a::{} {} },
        mul_seg_9_0: { ::a::{} {} },
        mul_seg_10_0: { ::{} {} },

        mul_seg_0_1: {, a {} },
        mul_seg_1_1: {, a::a {} },
        mul_seg_2_1: {, a::a::a {} },
        mul_seg_3_1: {, a::a::a::{} {} },
        mul_seg_4_1: {, a::a::{} {} },
        mul_seg_5_1: {, a::{} {} },
        mul_seg_6_1: {, ::a {} },
        mul_seg_7_1: {, ::a::a {} },
        mul_seg_8_1: {, ::a::a::{} {} },
        mul_seg_9_1: {, ::a::{} {} },
        mul_seg_10_1: {, ::{} {} },
    }
}

mod no_proc {
    error! {
        no_proc_0: { b },
        no_proc_1: { ::a::b as b },
        no_proc_2: { a::a::b },
    }
}

mod err_seg {
    error! {
        err_seg_0_0: { ; },
        err_seg_1_0: { #[fn]; },
        err_seg_2_0: { a; },
        err_seg_3_0: { ::; },
        err_seg_4_0: { ::a; },
        err_seg_5_0: { ::a::a; },
        err_seg_6_0: { ::a::a::; },
        err_seg_7_0: { ::{}; },
        err_seg_8_0: { {}; },
        err_seg_9_0: { {#[fn] b as ;} },
        err_seg_10_0: { #,#[fn] ::base::b },
        err_seg_11_0: { #[fn]#, ::base::b },
        err_seg_12_0: { #[fn]#,#[doc=""] ::base::b },
        err_seg_13_0: { #[fn]#[doc=""]#, ::base::b },
        err_seg_14_0: { #[fn]#[doc=""]#,#[doc=""] ::base::b },
        err_seg_15_0: { #;#[fn] ::base::b },
        err_seg_16_0: { #[fn]#; ::base::b },
        err_seg_17_0: { #[fn]#;#[doc=""] ::base::b },
        err_seg_18_0: { #[fn]#[doc=""]#; ::base::b },
        err_seg_19_0: { #[fn]#[doc=""]#;#[doc=""] ::base::b },

        err_seg_0_1: {, ; },
        err_seg_1_1: {, #[fn]; },
        err_seg_2_1: {, a; },
        err_seg_3_1: {, ::; },
        err_seg_4_1: {, ::a; },
        err_seg_5_1: {, ::a::a; },
        err_seg_6_1: {, ::a::a::; },
        err_seg_7_1: {, ::{}; },
        err_seg_8_1: {, {}; },
        err_seg_9_1: {, {, #[fn] b as ;} },
        err_seg_10_1: {, #,#[fn] ::base::b },
        err_seg_11_1: {, #[fn]#, ::base::b },
        err_seg_12_1: {, #[fn]#,#[doc=""] ::base::b },
        err_seg_13_1: {, #[fn]#[doc=""]#, ::base::b },
        err_seg_14_1: {, #[fn]#[doc=""]#,#[doc=""] ::base::b },
        err_seg_15_1: {, #;#[fn] ::base::b },
        err_seg_16_1: {, #[fn]#; ::base::b },
        err_seg_17_1: {, #[fn]#;#[doc=""] ::base::b },
        err_seg_18_1: {, #[fn]#[doc=""]#; ::base::b },
        err_seg_19_1: {, #[fn]#[doc=""]#;#[doc=""] ::base::b },

        // no_path
        // #[fn] nonexistent_api,
    }
}
