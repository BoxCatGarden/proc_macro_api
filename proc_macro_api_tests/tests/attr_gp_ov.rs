use proc_macro_api_tests::{
    attr_gp_ov_t_Override1, attr_gp_ov_t_bg_proc_oth_0, attr_gp_ov_t_call_at_bg_0,
    attr_gp_ov_t_override_0, attr_gp_ov_t_override_2, attr_gp_ov_t_prs_fn_bg_0,
};

use proc_macro_api_tests::{
    attr_gp_ov_t_global_local_0, attr_gp_ov_t_global_local_1, attr_gp_ov_t_global_local_2,
    attr_gp_ov_t_global_local_3,
};

#[test]
fn output() {
    assert_eq!(0, attr_gp_ov_t_global_local_0!());
    {
        #[attr_gp_ov_t_global_local_1]
        struct A;
        assert_eq!(1, NUM);
    }
    {
        #[attr_gp_ov_t_global_local_2]
        struct A;
        assert_eq!(0, NUM);
    }
    assert_eq!(1, attr_gp_ov_t_global_local_3!());

    assert_eq!(0, attr_gp_ov_t_override_0!());
    {
        #[derive(attr_gp_ov_t_Override1)]
        struct A;
        assert_eq!(0, A::NUM);
    }
    {
        #[attr_gp_ov_t_override_2]
        struct A;
        assert_eq!(0, NUM);
    }

    assert_eq!(0, attr_gp_ov_t_bg_proc_oth_0!());

    assert_eq!(0, attr_gp_ov_t_call_at_bg_0!());

    assert_eq!(0, attr_gp_ov_t_prs_fn_bg_0!());
}
