use proc_macro_api_tests::{
    AliasDr0, AliasDr1, alias_at_0, alias_at_1, alias_fn_0, alias_fn_1, cc_0, cc_1, cc_2, cc_3,
    empty_rest_0, full_seg_0, full_seg_1, full_seg_2, no_as_0, no_trailing_comma_0, one_prv_0,
    one_rest_0, one_rest_1, one_rest_2, one_rest_out_0, seg_matcher_0, seg_matcher_1,
};

#[test]
fn output() {
    assert_eq!(10, full_seg_0!());
    assert_eq!(10, full_seg_1!());
    assert_eq!(10, full_seg_2!());

    assert_eq!(0, empty_rest_0!());

    assert_eq!(1, one_rest_0!());
    assert_eq!(1, one_rest_1!());
    assert_eq!(1, one_rest_2!());

    assert_eq!(1, one_rest_out_0!());

    assert_eq!(2, one_prv_0!());

    assert_eq!(0, alias_fn_0!());
    assert_eq!(0, alias_fn_1!());
    {
        #[alias_at_0]
        struct A;
        assert_eq!(0, NUM);
    }
    {
        #[alias_at_1]
        struct A;
        assert_eq!(0, NUM);
    }
    {
        #[derive(AliasDr0)]
        struct A;
        assert_eq!(0, A::NUM);
    }
    {
        #[derive(AliasDr1)]
        struct A;
        assert_eq!(0, A::NUM);
    }

    assert_eq!(3, seg_matcher_0!());
    assert_eq!(0, seg_matcher_1!());

    assert_eq!(2, no_trailing_comma_0!());

    assert_eq!(0, cc_0!());
    assert_eq!(0, cc_1!());
    assert_eq!(1, cc_2!());
    assert_eq!(0, cc_3!());

    assert_eq!("no_as", no_as_0!());
}
