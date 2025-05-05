use proc_macro_api_tests::{
    attr_gp_ap_t_bg_proc_doc_oth_0, attr_gp_ap_t_bg_proc_doc_oth_1, attr_gp_ap_t_global_local_0,
    attr_gp_ap_t_global_local_1,
};

#[test]
fn output() {
    assert_eq!(0, attr_gp_ap_t_global_local_0!());
    assert_eq!(1, attr_gp_ap_t_global_local_1!());

    assert_eq!(0, attr_gp_ap_t_bg_proc_doc_oth_0!());
    assert_eq!(0, attr_gp_ap_t_bg_proc_doc_oth_1!());
}
