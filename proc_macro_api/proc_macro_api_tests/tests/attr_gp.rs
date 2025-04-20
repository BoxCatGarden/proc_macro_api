use proc_macro_api_tests::{bg_proc_doc_oth_0, bg_proc_doc_oth_1, call_at_bg_1};

use proc_macro_api_tests::{global_local_gp_0, global_local_gp_1};

#[test]
fn output() {
    assert_eq!(0, global_local_gp_0!());
    assert_eq!(1, global_local_gp_1!());

    assert_eq!(0, bg_proc_doc_oth_0!());
    assert_eq!(0, bg_proc_doc_oth_1!());

    assert_eq!(0, call_at_bg_1!());
}
