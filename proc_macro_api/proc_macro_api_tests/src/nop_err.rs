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

error! {
    blk_al: { {} as _ },

    inner_cc_0: { ::{::a::b as _} },
    inner_cc_1: { a::{::b as _} },
    inner_cc_2: { ::{,::a::b as _} },
    inner_cc_3: { a::{,::b as _} },

    no_seg_0: { #[fn] },
    no_seg_1: {, #[fn] },

    mul_seg_0: { a {} },
    mul_seg_1: { a::a {} },
    mul_seg_2: { a::a::a {} },
    mul_seg_3: { a::a::a::{} {} },
    mul_seg_4: { a::a::{} {} },
    mul_seg_5: { a::{} {} },
    mul_seg_6: { ::a {} },
    mul_seg_7: { ::a::a {} },
    mul_seg_8: { ::a::a::{} {} },
    mul_seg_9: { ::a::{} {} },
    mul_seg_10: { ::{} {} },

    mul_seg_21: {, a {} },
    mul_seg_11: {, a::a {} },
    mul_seg_12: {, a::a::a {} },
    mul_seg_13: {, a::a::a::{} {} },
    mul_seg_14: {, a::a::{} {} },
    mul_seg_15: {, a::{} {} },
    mul_seg_16: {, ::a {} },
    mul_seg_17: {, ::a::a {} },
    mul_seg_18: {, ::a::a::{} {} },
    mul_seg_19: {, ::a::{} {} },
    mul_seg_20: {, ::{} {} },

    no_proc_0: { b },
    no_proc_1: { ::a::b as b },
    no_proc_2: { a::a::b },

    err_seg_0: { ; },
    err_seg_1: { #[fn]; },
    err_seg_2: { a; },
    err_seg_3: { ::; },
    err_seg_4: { ::a; },
    err_seg_5: { ::a::a; },
    err_seg_6: { ::a::a::; },
    err_seg_7: { ::{}; },
    err_seg_8: { {}; },
    err_seg_9: { {#[fn] b as ;} },

    err_seg_10: {, ; },
    err_seg_11: {, #[fn]; },
    err_seg_12: {, a; },
    err_seg_13: {, ::; },
    err_seg_14: {, ::a; },
    err_seg_15: {, ::a::a; },
    err_seg_16: {, ::a::a::; },
    err_seg_17: {, ::{}; },
    err_seg_18: {, {}; },
    err_seg_19: {, {#[fn] b as ;} },

    // no_path
    // #[fn] nonexistent_api,
}
