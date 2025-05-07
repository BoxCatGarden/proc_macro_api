// This file will be included by "lib.rs" when
// compiling with feature `comp_err`.

// APIs exported here should have a name starting with `comp_err_`
// and ending with `_\d+`.

#[cfg(feature = "comp_err_nonexistent_fn")]
proc_macro_api! {
    #[fn] nonexistent_fn as comp_err_nonexistent_fn_0,
}

#[cfg(feature = "comp_err_mul_pm_attr")]
proc_macro_api! {
    #[fn]#[fn] b as comp_err_mul_pm_attr_0,
}

#[cfg(feature = "comp_err_mul_pm_attr")]
proc_macro_api! {
    #[fn]#[at] c as comp_err_mul_pm_attr_1,
}
