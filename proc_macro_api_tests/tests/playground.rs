use proc_macro_api_tests::{
    play_t_0, play_t_1, play_t_2, play_t_3, play_t_4, play_t_5, play_t_6, play_t_7, play_t_8,
};

#[test]
fn output() {
    assert_eq!(0, play_t_0!());
    assert_eq!(3, play_t_1!());
    assert_eq!(7, play_t_2!());
    assert_eq!(7, play_t_3!());
    assert_eq!(7, play_t_4!());
    assert_eq!(7, play_t_5!());
    assert_eq!(7, play_t_6!());
    assert_eq!(7, play_t_7!());
    assert_eq!(7, play_t_8!());
}
