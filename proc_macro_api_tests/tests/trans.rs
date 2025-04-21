use proc_macro_api_tests::{
    trans_t_cc_trans_0, trans_t_cc_trans_1, trans_t_trans_0, trans_t_trans_1, trans_t_trans_2,
    trans_t_trans_3,
};

#[test]
fn output() {
    assert_eq!(0, trans_t_cc_trans_0!());
    {
        #[trans_t_cc_trans_1]
        struct A;
        assert_eq!(0, NUM);
    }

    assert_eq!(0, trans_t_trans_0!());
    assert_eq!(0, trans_t_trans_2!());
    {
        #[trans_t_trans_1]
        struct A;
        assert_eq!(0, NUM);
    }
    {
        #[trans_t_trans_3]
        struct A;
        assert_eq!(0, NUM);
    }
}
