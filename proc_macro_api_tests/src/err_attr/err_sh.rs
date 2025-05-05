#[cfg(not(feature = "deny_shadow"))]
use super::api as error_sh;
#[cfg(feature = "deny_shadow")]
use error as error_sh;

error_sh! {
    err_sh_0: { #[fn]#[fn] b as _ },
    err_sh_1: { #[fn]#[proc_macro] b as _ },
    err_sh_2: { #[fn]#[at] c as _ },
    err_sh_3: { #[fn]#[proc_macro_attribute] c as _ },
    err_sh_4: { #[fn]#[dr(ErrSh4)] b as _ },
    err_sh_5: { #[fn]#[proc_macro_derive(ErrSh5)] b as _ },

    err_sh_6: { #[fn]#[fn] a::b as _ },
    err_sh_7: { #[fn]#[doc=""]#[fn] a::b as _ },
    err_sh_8: { #[fn]#[doc=""]#[doc=""]#[fn] a::a::b as _ },
    err_sh_9: { #[fn]#[doc=""]#[fn] a::a::b as _ },
    err_sh_10: { #[fn]#[doc=""]#[doc=""]#[fn] a::a::a::b as _ },
    err_sh_11: { #[fn]#[fn] a::a::b as _ },
    err_sh_12: { #[fn]#[fn] a::a::a::b as _ },
    err_sh_13: { #[fn]#[fn] a::a::a::a::b as _ },
}
