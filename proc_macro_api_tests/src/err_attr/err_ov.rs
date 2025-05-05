error! {
    err_ov_0: { #[fn] {#[fn] b as _} },
    err_ov_1: { #[at] {#[fn] a::{}} },
    err_ov_2: { #[fn] {#[fn]#[doc=""] a::b as _} },
    err_ov_3: { #[fn] {#[fn]#[doc=""]#[doc=""] a::a::b as _} },
    err_ov_4: { #[fn] {#[fn]#[doc=""] a::a::{}} },
    err_ov_5: { #[fn] {#[fn]#[doc=""]#[doc=""] a::a::a::{}} },
    err_ov_6: { #[fn] {#[fn] a::a::b as _} },
    err_ov_7: { #[fn] {#[fn] a::a::a::b as _} },
    err_ov_8: { #[fn] {#[fn] {}} },
}
