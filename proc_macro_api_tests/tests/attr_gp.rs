#![cfg(not(feature = "deny_group_attr"))]

use proc_macro_api_tests::{
    attr_gp_t_bg_proc_doc_oth_0, attr_gp_t_bg_proc_doc_oth_1, attr_gp_t_call_at_bg_1,
};

use proc_macro_api_tests::{attr_gp_t_global_local_gp_0, attr_gp_t_global_local_gp_1};

#[test]
fn output() {
    assert_eq!(0, attr_gp_t_global_local_gp_0!());
    assert_eq!(1, attr_gp_t_global_local_gp_1!());

    assert_eq!(0, attr_gp_t_bg_proc_doc_oth_0!());
    assert_eq!(0, attr_gp_t_bg_proc_doc_oth_1!());

    assert_eq!(0, attr_gp_t_call_at_bg_1!());
}
