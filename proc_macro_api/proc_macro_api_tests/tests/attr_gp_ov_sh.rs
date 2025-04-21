use proc_macro_api_tests::{attr_gp_ov_sh_t_bg_proc_oth_1, attr_gp_ov_sh_t_oth_proc_0};

#[test]
fn output() {
    assert_eq!(0, attr_gp_ov_sh_t_oth_proc_0!());

    assert_eq!(0, attr_gp_ov_sh_t_bg_proc_oth_1!());
}
