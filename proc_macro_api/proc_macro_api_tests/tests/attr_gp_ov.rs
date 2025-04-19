use proc_macro_api_tests::{
    Override1, bg_proc_oth_0, call_at_bg_0, global_local_0, override_0, override_2, prs_fn_bg_0,
};

#[test]
fn output() {
    assert_eq!(0, global_local_0!());

    assert_eq!(0, override_0!());
    {
        #[derive(Override1)]
        struct A;
        assert_eq!(0, A::NUM);
    }
    {
        #[override_2]
        struct A;
        assert_eq!(0, NUM);
    }

    assert_eq!(0, bg_proc_oth_0!());

    assert_eq!(0, call_at_bg_0!());

    assert_eq!(0, prs_fn_bg_0!());
}
