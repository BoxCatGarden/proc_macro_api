use proc_macro_api_tests::{
    attr_gp_ap_ov_t_global_local_0, attr_gp_ap_ov_t_global_local_1, attr_gp_ap_ov_t_global_local_2,
    attr_gp_ap_ov_t_global_local_3,
};

#[test]
fn output() {
    assert_eq!(0, attr_gp_ap_ov_t_global_local_0!());
    {
        #[attr_gp_ap_ov_t_global_local_1]
        struct A;
        assert_eq!(1, NUM);
    }
    {
        #[attr_gp_ap_ov_t_global_local_2]
        struct A;
        assert_eq!(0, NUM);
    }
    assert_eq!(1, attr_gp_ap_ov_t_global_local_3!());
}
