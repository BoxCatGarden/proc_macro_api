Tests for `proc-macro-api`.

# Run

Use `cargo test` to run the tests.

# Test-option features

`test_*` features are test-option features. Test-option features are used
for enabling optional tests. In order to enable an optional test, enable
the corresponding test-option features.

* `test_normal` is provided for exhaustiveness, and the tests in it are
  not optional.
* `test_comp_err_*` will cause a compiling error. It is used for testing the
  error detecting and the error messages, manually.
* `test_err_*` will be compiled successfully, but specific doc-tests in it will
  always fail to be compiled. It is used for testing the error detecting
  and the error messages, manually.

See *Cargo.toml* for the list of the test-option features.

# Combination of test-options

Compatible test-option features can be enabled together.
Incompatible test-option features will cause a compiling error.

A `test_comp_err_*` feature should not be enabled together with other
test-option features.

# Examples

```text
cargo test --package proc-macro-api-tests --features test_normal
```

# SemVer compatibility

The public APIs of the library crate are the test-option features.
`pub` items from the library crate should be treated as a private part of
the crate.
The features annotated as "private" should be treated as a private part of
the crate.
