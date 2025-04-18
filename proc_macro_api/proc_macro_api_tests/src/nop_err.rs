proc_macro_api! {
    // blk_al
    {} as _,

    // inner_cc
    ::{::b as _},
    a::{::b as _},

    // no_seg
    #[fn],

    // mul_seg
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

    // no_proc
    b,
    ::a::b as b,
    a::a::b,

    // err_seg
    {;},
    {#[fn];},
    {a;},
    {::;},
    {::a;},
    {::a::a;},
    {::a::a::;},
    {::{};},
    {{};},
    {#[fn] b as ;},

    // no_path
    #[fn] nonexistent_api,
}
