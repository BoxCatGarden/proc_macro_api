use proc_macro_api_tests::{
    AliasDr0, AliasDr1, alias_at_0, alias_at_1, alias_fn_0, alias_fn_1, cc_0, cc_1, cc_2, cc_3,
    empty_rest_0, full_seg_0, full_seg_1, full_seg_2, no_trailing_comma_0, one_prv_0, one_rest_0,
    one_rest_1, one_rest_2, one_rest_out_0, seg_matcher_0, seg_matcher_1,
};

use proc_macro_api_tests::{
    no_as_0, no_as_1, no_as_2, no_as_3, no_as_4, no_as_5, no_as_6, no_as_7,
};

use proc_macro_api_tests::{
    seg_matcher_2_0, seg_matcher_2_1, seg_matcher_3_0, seg_matcher_3_1, seg_matcher_4_0,
    seg_matcher_4_1, seg_matcher_5_0, seg_matcher_5_1, seg_matcher_6_0, seg_matcher_6_1,
    seg_matcher_7_0, seg_matcher_7_1, seg_matcher_8_0, seg_matcher_8_1, seg_matcher_9_0,
    seg_matcher_9_1, seg_matcher_10_0, seg_matcher_10_1, seg_matcher_11_0, seg_matcher_11_1,
    seg_matcher_12_0, seg_matcher_12_1, seg_matcher_13_0, seg_matcher_13_1, seg_matcher_14_0,
    seg_matcher_14_1, seg_matcher_15_0, seg_matcher_15_1,
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

    assert_eq!("no_as_0", no_as_0!());
    assert_eq!("no_as_1", no_as_1!());
    assert_eq!("no_as_2", no_as_2!());
    assert_eq!("no_as_3", no_as_3!());
    assert_eq!("no_as_4", no_as_4!());
    assert_eq!("no_as_5", no_as_5!());
    assert_eq!("no_as_6", no_as_6!());
    assert_eq!("no_as_7", no_as_7!());

    assert_eq!(0, seg_matcher_2_0!());
    assert_eq!(0, seg_matcher_2_1!());
    assert_eq!(1, seg_matcher_3_0!());
    assert_eq!(1, seg_matcher_3_1!());
    assert_eq!(2, seg_matcher_4_0!());
    assert_eq!(2, seg_matcher_4_1!());
    assert_eq!(3, seg_matcher_5_0!());
    assert_eq!(3, seg_matcher_5_1!());
    assert_eq!(4, seg_matcher_6_0!());
    assert_eq!(4, seg_matcher_6_1!());
    assert_eq!(5, seg_matcher_7_0!());
    assert_eq!(5, seg_matcher_7_1!());
    assert_eq!(6, seg_matcher_8_0!());
    assert_eq!(6, seg_matcher_8_1!());
    assert_eq!(7, seg_matcher_9_0!());
    assert_eq!(7, seg_matcher_9_1!());

    assert_eq!(0, seg_matcher_10_0!());
    assert_eq!(0, seg_matcher_10_1!());
    assert_eq!(1, seg_matcher_11_0!());
    assert_eq!(1, seg_matcher_11_1!());
    assert_eq!(2, seg_matcher_12_0!());
    assert_eq!(2, seg_matcher_12_1!());
    assert_eq!(3, seg_matcher_13_0!());
    assert_eq!(3, seg_matcher_13_1!());
    assert_eq!(4, seg_matcher_14_0!());
    assert_eq!(4, seg_matcher_14_1!());
    assert_eq!(5, seg_matcher_15_0!());
    assert_eq!(5, seg_matcher_15_1!());
}
