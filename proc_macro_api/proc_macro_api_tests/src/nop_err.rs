macro_rules! error {
    ($($name:ident : { $($tt:tt)* }),* $(,)?) => {$(
        /// ```
        /// use ::proc_macro_api::proc_macro_api;
        /// proc_macro_api! {
        #[doc = concat!($(stringify!($tt), " ",)*)]
        /// }
        /// fn main() {}
        /// ```
        fn $name() {}
    )*};
}

error! {
    blk_al: {
        {} as _,
    },

    inner_cc_0: {
        ::{::a::b as _},
    },
    inner_cc_1: {
        a::{::b as _},
    },

    no_seg: {
        #[fn],
    },

    mul_seg: {
        a {},
        a::a {},
        a::a::a {},
        a::a::a::{} {},
        a::a::{} {},
        a::{} {},
        ::a {},
        ::a::a {},
        ::a::a::{} {},
        ::a::{} {},
        ::{} {},
    },

    no_proc_0: {
        b,
    },
    no_proc_1: {
        ::a::b as b,
    },
    no_proc_2: {
        a::a::b,
    },

    err_seg_0: {
        {;},
        {#[fn];},
        {a;},
        {::;},
        {::a;},
        {::a::a;},
        {::a::a::;},
        {::{};},
        {{};},
    },
    err_seg_1: {
        {#[fn] b as ;},
    },

    // no_path
    // #[fn] nonexistent_api,
}
