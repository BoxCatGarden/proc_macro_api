extern crate proc_macro;

use proc_macro_api_test_base::*;
use quote::quote;

type PmFnB = fn(proc_macro::TokenStream) -> proc_macro::TokenStream;
type PmFnC = fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream;
type PmTwoFnB = fn(proc_macro2::TokenStream) -> proc_macro2::TokenStream;
type PmTwoFnC = fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream;

macro_rules! assert_signature {
    ($([ $($prv:tt),* $(,)? ]),* $(,)?) => {{
        $({
            let _: PmFnB = $($prv::)*b;
            let _: PmFnC = $($prv::)*c;
            let _: PmTwoFnB = pm2::$($prv::)*b;
            let _: PmTwoFnC = pm2::$($prv::)*c;
        })*
    }};
}

#[test]
fn signature() {
    assert_signature!([], [a], [a, a]);
}

macro_rules! assert_token_eq {
    ($pm:ty => $expected:tt $token_stream:expr) => {{
        let expected: $pm = (quote! $expected).into();
        let actual: $pm = $token_stream;
        assert_eq!(expected.to_string(), actual.to_string());
    }};
}

macro_rules! assert_depth {
    ($depth:expr , [ $($prv:tt),* $(,)? ]) => {{
        const NUM: usize = $depth;
        assert_token_eq! {
            proc_macro2::TokenStream =>
            { const D_NUM: usize = #NUM; }
            pm2::$($prv::)*b(quote! {a})
        }
        assert_token_eq! {
            proc_macro2::TokenStream =>
            { #NUM }
            pm2::$($prv::)*b(quote! {})
        }
        assert_token_eq! {
            proc_macro2::TokenStream =>
            { const NUM: usize = #NUM; }
            pm2::$($prv::)*c(quote! {}, quote! {})
        }
    }};
}

#[test]
fn output() {
    assert_depth!(0usize, []);
    assert_depth!(1usize, [a]);
    assert_depth!(2usize, [a, a]);
}
