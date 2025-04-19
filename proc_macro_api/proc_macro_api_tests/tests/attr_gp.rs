use proc_macro_api_tests::{bg_proc_doc_oth_0, bg_proc_doc_oth_1, call_at_bg_1, global_local_1};

#[test]
fn output() {
    {
        #[global_local_1]
        struct A;
        assert_eq!(0, NUM);
    }

    assert_eq!(0, bg_proc_doc_oth_0!());
    assert_eq!(0, bg_proc_doc_oth_1!());

    assert_eq!(0, call_at_bg_1!());
}
