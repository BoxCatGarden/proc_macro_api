A crate helping with structuring the library crate of a proc-macro package.

The major of this crate is macro `proc_macro_api!`,
which can export functions in submodules of a proc-macro crate as
the Application Programing Interfaces (APIs) of that proc-macro crate.

For example, assuming `sub` is a submodule of the root of
a proc-macro crate, in order to export
`pub fn proc_fn(input: TokenStream) -> TokenStream` in the `sub`
as a function-like macro, `proc_macro_api!` can be used in the
root of that crate like:

```rust
proc_macro_api! {
    sub::{
        #[proc_macro]
        proc_fn,
    },
}
```

Then, a function-like macro, `proc_fn!()`, is available from that
proc-macro crate.

> Note: See the package documentation for more details.
