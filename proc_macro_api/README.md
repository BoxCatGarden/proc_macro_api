A crate helping with structuring the library crate of a proc_macro package.

The major function of this crate is macro [`proc_macro_api!`],
which can export functions in submodules of a proc_macro crate as
the Application Programing Interfaces (APIs) of that proc_macro crate.

For example, assuming there is a submodule `sub` of the root of
a proc_macro crate, in order to export
`pub fn proc_fn(input: TokenStream) -> TokenStream` in the `sub`
as a function-like macro, [`proc_macro_api!`] can be used in the
root of that crate like:
```ignore
proc_macro_api! {
    sub::{
        #[proc_macro]
        proc_fn,
    },
}
```
Then, a function-like macro, `proc_fn!()`, is available from that
proc_macro crate.

The macro should be used at somewhere that the proc_macro annotation
(i.e., `#[proc_macro]`, `#[proc_macro_attribute]`, etc.) is allowed.

# Input syntax

[`proc_macro_api!`] requires paths with proc_macro annotations as
its input.

## Path and function

The syntax of the input paths is like the path syntax of _`use` declaration_,
but always requires curly braces (i.e., `{}`) and denys the leading `::`.

All the paths should be a path of a function. The function should have
a signature required by the proc_macro annotation.

## Annotation and forwarding

A proc_macro annotation can be added to a path segment,
and the annotation will be forwarded to all the sub-segments of that segment
unless the annotation is overridden.

For a path segment `Seg`, the proc_macro annotation forwarded from a
super-segment can be overridden by adding an annotation to segment `Seg`,
and the new annotation will be forwarded then.

## Annotation aliases

* `#[proc_macro]` => `#[fn]`
* `#[proc_macro_attribute]` => `#[at]`
* `#[proc_macro_derive]` => `#[dr]`

# Expansion

Each function path will be expanded to a `pub fn` annotated with the
specific proc_macro annotation.

The expanded `pub fn` will have a name provided with by the path.

* If there isn't an `as`: the same name as the name of the function
  supplied by the path.
* If there is an `as`: the name after the `as`.

For the function supplied by a path, the expanded `pub fn` will
and only will directly call the supplied
function with the arguments that input to it. It is recommended
to annotate the supplied function as `#[inline(always)]`.

# Examples

```ignore
// doctest is ignored because the crate is not in a proc_macro context.

// in 'lib.rs'
#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

mod mod_a {
    use proc_macro::TokenStream;

    pub fn an_fn_api(input: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    pub mod mod_b {
        use proc_macro::TokenStream;

        pub fn an_attr_api(args: TokenStream, item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn a_derive_api(item: TokenStream) -> TokenStream {
            TokenStream::new()
        }
    }
}

proc_macro_api! {
    mod_a::{
        // alias and document
        /// A function-like macro.
        #[fn] an_fn_api,

        // forwarding
        #[proc_macro_attribute]
        mod_b::{
            // use the forwarded annotation
            an_attr_api as the_attr_api,

            // override
            #[dr(Something)] a_derive_api,
        },
    },
}
// It will expand to three `pub fn` in 'lib.rs' naming
// `an_fn_api`, `the_attr_api`, and `a_derive_api`, respectively.
```

# Extern requirement

The macro requires some external _names_ available in the scope
where it is used:

* The name `proc_macro`. Optional if compiled successfully.
  Most of the time, it is needed because of `proc_macro::TokenStream`.
* The name `std`. Optional if compiled successfully.
  Mainly for reporting `compile_err!`.
