use proc_macro_api_tests::{bg_proc_oth_1, oth_proc_0};

#[test]
fn output() {
    assert_eq!(0, oth_proc_0!());

    assert_eq!(0, bg_proc_oth_1!());
}
