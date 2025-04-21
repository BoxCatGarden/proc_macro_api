#![allow(unused)]
#![allow(non_camel_case_types)]

#[cfg(any(
    feature = "test_attr_gp",
    feature = "test_attr_gp_ov",
    feature = "test_attr_gp_ov_sh"
))]
struct attr_OR_ONE_err_attr_OR_err_default;

#[cfg(feature = "test_err_attr_sh")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_ov")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_gp")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_sh_ov")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_sh_gp")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_ov_gp")]
struct attr_OR_ONE_err_attr_OR_err_default;
#[cfg(feature = "test_err_attr_sh_ov_gp")]
struct attr_OR_ONE_err_attr_OR_err_default;

#[cfg(feature = "test_err_default")]
struct attr_OR_ONE_err_attr_OR_err_default;
