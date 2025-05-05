use proc_macro_api_tests::attr_gp_t_call_at_bg_1;

#[test]
fn output() {
    assert_eq!(0, attr_gp_t_call_at_bg_1!());
}
