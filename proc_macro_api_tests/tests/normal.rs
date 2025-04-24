use proc_macro_api_tests::{
    norm_t_AliasDr0, norm_t_AliasDr1, norm_t_alias_at_0, norm_t_alias_at_1, norm_t_alias_fn_0,
    norm_t_alias_fn_1, norm_t_cc_0, norm_t_cc_1, norm_t_cc_2, norm_t_cc_3, norm_t_empty_rest_0,
    norm_t_full_seg_0, norm_t_full_seg_1, norm_t_full_seg_2, norm_t_no_trailing_comma_0,
    norm_t_one_prv_0, norm_t_one_rest_0, norm_t_one_rest_1, norm_t_one_rest_2,
    norm_t_one_rest_out_0, norm_t_seg_matcher_0, norm_t_seg_matcher_1,
};

use proc_macro_api_tests::{
    norm_t_no_as_0, norm_t_no_as_1, norm_t_no_as_2, norm_t_no_as_3, norm_t_no_as_4, norm_t_no_as_5,
    norm_t_no_as_6, norm_t_no_as_7,
};

use proc_macro_api_tests::{
    norm_t_seg_matcher_2_0, norm_t_seg_matcher_2_1, norm_t_seg_matcher_3_0, norm_t_seg_matcher_3_1,
    norm_t_seg_matcher_4_0, norm_t_seg_matcher_4_1, norm_t_seg_matcher_5_0, norm_t_seg_matcher_5_1,
    norm_t_seg_matcher_6_0, norm_t_seg_matcher_6_1, norm_t_seg_matcher_7_0, norm_t_seg_matcher_7_1,
    norm_t_seg_matcher_8_0, norm_t_seg_matcher_8_1, norm_t_seg_matcher_9_0, norm_t_seg_matcher_9_1,
    norm_t_seg_matcher_10_0, norm_t_seg_matcher_10_1, norm_t_seg_matcher_11_0,
    norm_t_seg_matcher_11_1, norm_t_seg_matcher_12_0, norm_t_seg_matcher_12_1,
    norm_t_seg_matcher_13_0, norm_t_seg_matcher_13_1, norm_t_seg_matcher_14_0,
    norm_t_seg_matcher_14_1, norm_t_seg_matcher_15_0, norm_t_seg_matcher_15_1,
};

#[test]
fn output() {
    assert_eq!(10, norm_t_full_seg_0!());
    assert_eq!(10, norm_t_full_seg_1!());
    assert_eq!(10, norm_t_full_seg_2!());

    assert_eq!(0, norm_t_empty_rest_0!());

    assert_eq!(1, norm_t_one_rest_0!());
    assert_eq!(1, norm_t_one_rest_1!());
    assert_eq!(1, norm_t_one_rest_2!());

    assert_eq!(1, norm_t_one_rest_out_0!());

    assert_eq!(2, norm_t_one_prv_0!());

    assert_eq!(0, norm_t_alias_fn_0!());
    assert_eq!(0, norm_t_alias_fn_1!());
    {
        #[norm_t_alias_at_0]
        struct A;
        assert_eq!(0, NUM);
    }
    {
        #[norm_t_alias_at_1]
        struct A;
        assert_eq!(0, NUM);
    }
    {
        #[allow(unused)]
        #[derive(norm_t_AliasDr0)]
        struct A;
        assert_eq!(0, D_NUM);
    }
    {
        #[allow(unused)]
        #[derive(norm_t_AliasDr1)]
        struct A;
        assert_eq!(0, D_NUM);
    }

    assert_eq!(3, norm_t_seg_matcher_0!());
    assert_eq!(0, norm_t_seg_matcher_1!());

    assert_eq!(2, norm_t_no_trailing_comma_0!());

    assert_eq!(0, norm_t_cc_0!());
    assert_eq!(0, norm_t_cc_1!());
    assert_eq!(1, norm_t_cc_2!());
    assert_eq!(0, norm_t_cc_3!());

    assert_eq!("norm_t_no_as_0", norm_t_no_as_0!());
    assert_eq!("norm_t_no_as_1", norm_t_no_as_1!());
    assert_eq!("norm_t_no_as_2", norm_t_no_as_2!());
    assert_eq!("norm_t_no_as_3", norm_t_no_as_3!());
    assert_eq!("norm_t_no_as_4", norm_t_no_as_4!());
    assert_eq!("norm_t_no_as_5", norm_t_no_as_5!());
    assert_eq!("norm_t_no_as_6", norm_t_no_as_6!());
    assert_eq!("norm_t_no_as_7", norm_t_no_as_7!());

    assert_eq!(0, norm_t_seg_matcher_2_0!());
    assert_eq!(0, norm_t_seg_matcher_2_1!());
    assert_eq!(1, norm_t_seg_matcher_3_0!());
    assert_eq!(1, norm_t_seg_matcher_3_1!());
    assert_eq!(2, norm_t_seg_matcher_4_0!());
    assert_eq!(2, norm_t_seg_matcher_4_1!());
    assert_eq!(3, norm_t_seg_matcher_5_0!());
    assert_eq!(3, norm_t_seg_matcher_5_1!());
    assert_eq!(4, norm_t_seg_matcher_6_0!());
    assert_eq!(4, norm_t_seg_matcher_6_1!());
    assert_eq!(5, norm_t_seg_matcher_7_0!());
    assert_eq!(5, norm_t_seg_matcher_7_1!());
    assert_eq!(6, norm_t_seg_matcher_8_0!());
    assert_eq!(6, norm_t_seg_matcher_8_1!());
    assert_eq!(7, norm_t_seg_matcher_9_0!());
    assert_eq!(7, norm_t_seg_matcher_9_1!());

    assert_eq!(0, norm_t_seg_matcher_10_0!());
    assert_eq!(0, norm_t_seg_matcher_10_1!());
    assert_eq!(1, norm_t_seg_matcher_11_0!());
    assert_eq!(1, norm_t_seg_matcher_11_1!());
    assert_eq!(2, norm_t_seg_matcher_12_0!());
    assert_eq!(2, norm_t_seg_matcher_12_1!());
    assert_eq!(3, norm_t_seg_matcher_13_0!());
    assert_eq!(3, norm_t_seg_matcher_13_1!());
    assert_eq!(4, norm_t_seg_matcher_14_0!());
    assert_eq!(4, norm_t_seg_matcher_14_1!());
    assert_eq!(5, norm_t_seg_matcher_15_0!());
    assert_eq!(5, norm_t_seg_matcher_15_1!());
}
