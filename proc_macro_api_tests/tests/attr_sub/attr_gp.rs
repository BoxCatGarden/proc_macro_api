use proc_macro_api_tests::{attr_gp_t_call_at_bg_1, attr_gp_t_err_ap_0, attr_gp_t_err_ov_0};

#[test]
fn output() {
    assert_eq!(0, attr_gp_t_call_at_bg_1!());

    assert_eq!(1, attr_gp_t_err_ap_0!());
    assert_eq!(2, attr_gp_t_err_ov_0!());
}
