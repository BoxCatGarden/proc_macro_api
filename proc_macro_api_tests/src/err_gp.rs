#[cfg(not(any(feature = "deny_group_attr", feature = "with_default")))]
use api as error_gp;
#[cfg(any(feature = "deny_group_attr", feature = "with_default"))]
use error as error_gp;

error_gp! {
    err_gp_0: { #[fn] {b as _} },
    err_gp_1: { #[fn]#[doc=""] ::{b as _} },
    err_gp_2: { #[fn]#[doc=""]#[doc=""] a::{b as _} },
    err_gp_3: { #[fn] a::a::{b as _} },
    err_gp_4: { #[fn] a::a::a::{b as _} },
    err_gp_5: { #[fn] ::a::{b as _} },
    err_gp_6: { #[fn] ::a::a::{b as _} },
}
