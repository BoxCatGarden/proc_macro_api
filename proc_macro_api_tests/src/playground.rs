macro_rules! playground {
    () => {
        // Local items should be defined in this module or
        // with a name starting with `play_t_` in this tanscriber scope.
        #[allow(dead_code)]
        mod pg {}

        // APIs exported here should have a name starting with `play_t_`
        // and ending with `_\d+`.
        proc_macro_api! {
            #[fn] b as play_t_0,

            a::{
                a::a::{
                    a::a::a::{
                        a::{
                            #[fn] b as play_t_2,
                            #[fn] b as play_t_3,
                            #[fn] b as play_t_4,
                            #[fn] b as play_t_5,
                            #[fn] b as play_t_6,
                            #[fn] b as play_t_7,
                            #[fn] b as play_t_8,
                        },
                    },

                    #[fn] b as play_t_1,
                },
            },
        }
    };
}
