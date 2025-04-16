use proc_macro_api_tests::{bg_proc_doc_oth_0, bg_proc_doc_oth_1};

#[test]
fn output() {
    assert_eq!(0, bg_proc_doc_oth_0!());
    assert_eq!(0, bg_proc_doc_oth_1!());
}
