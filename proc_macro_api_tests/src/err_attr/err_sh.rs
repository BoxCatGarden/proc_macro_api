error! {
    err_sh_0: { #[proc_macro_derive(ErrSh0)]#[fn] b as _ },
    err_sh_1: { #[fn]#[proc_macro] b as _ },
    err_sh_2: { #[proc_macro]#[at] c as _ },
    err_sh_3: { #[at]#[proc_macro_attribute] c as _ },
    err_sh_4: { #[proc_macro_attribute]#[dr(ErrSh4)] b as _ },
    err_sh_5: { #[dr(ErrSh5_0)]#[proc_macro_derive(ErrSh5_1)] b as _ },

    err_sh_6: { #[fn]#[fn] a::b as _ },
    err_sh_7: { #[fn]#[doc=""]#[fn] a::b as _ },
    err_sh_8: { #[fn]#[doc=""]#[doc=""]#[fn] a::a::b as _ },
    err_sh_9: { #[fn]#[doc=""]#[fn] a::a::b as _ },
    err_sh_10: { #[fn]#[doc=""]#[doc=""]#[fn] a::a::a::b as _ },
    err_sh_11: { #[fn]#[fn] a::a::b as _ },
    err_sh_12: { #[fn]#[fn] a::a::a::b as _ },
    err_sh_13: { #[fn]#[fn] a::a::a::a::b as _ },
}
