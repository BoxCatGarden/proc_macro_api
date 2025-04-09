A crate helping with structuring the library crate of a proc-macro package.

The major of this crate is macro [`proc_macro_api!`],
which can export functions in submodules of a proc-macro crate as
the Application Programing Interfaces (APIs) of that proc-macro crate.

For example, assuming `sub` is a submodule of the root of
a proc-macro crate, in order to export
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
proc-macro crate.

The macro should be used at somewhere that the proc-macro attributes
(i.e., `#[proc_macro]`, `#[proc_macro_attribute]`, etc.) are allowed.

# Input

>
> **<small>Syntax</small>**  
> Input:  
> &nbsp;&nbsp;&nbsp;PathList
>
> PathList:  
> &nbsp;&nbsp;
> PathWithAttr<sup>?</sup> ( `,` PathWithAttr<sup>?</sup> )<sup>\*</sup>
>
> PathWithAttr:  
> &nbsp;&nbsp;&nbsp;[OuterAttribute]<sup>\*</sup>  
> &nbsp;&nbsp;&nbsp;PathTree
>
> PathTree:  
> &nbsp;&nbsp;&nbsp;&nbsp;
> ( [SimplePath]<sup>?</sup> `::` )<sup>?</sup> `{` PathList `}`  
> &nbsp;&nbsp;&nbsp;|
> [SimplePath] ( `as` ( [IDENTIFIER] | `_` ) )<sup>?</sup>
>

[`proc_macro_api!`] requires paths with proc-macro attributes as
its input. The syntax of the input paths is like the path syntax
of [`use` declarations][use], but allows attribute and disallows
asterisk (i.e., `*`).

[SimplePath]: https://doc.rust-lang.org/reference/paths.html#simple-paths

[IDENTIFIER]: https://doc.rust-lang.org/reference/identifiers.html

[OuterAttribute]: https://doc.rust-lang.org/reference/attributes.html

[use]: https://doc.rust-lang.org/reference/items/use-declarations.html

## Paths

All the input paths should be either a path of a function or
a path renamed with the underscore alias.

When a path is of a function and isn't renamed with the underscore alias,
a proc-macro attribute should be applied to the path, and
the function should have a signature required by the proc-macro attribute.

When a path is renamed with the underscore alias,
it is not required to be a path of function, and
not required to be annotated with proc-macro attributes.
The path is only required to be syntactically valid.

## Brace syntax and path groups

Braces can be used for grouping paths with a common prefix.

When some paths are grouped by a pair of curly braces, they compose
a path group.

Braces can be nested, and nested braces create subgroups of paths.

## Attributes

Attributes can be applied to single paths and path groups.

Input attributes are classed into two types:

* **Global**. When a global attribute is applied to a path group,
  it is applied to all the paths in that group. That is, global
  attributes inside curly braces will _be appended after_ the global
  attributes from the outside.
* **Local**. For each path in a path group, inside the curly braces
  of that group,
    * if some local attributes are applied to the path, the local
      attributes applied to the group won't be applied to the path;
    * if no local attribute is applied to the path, the local
      attributes applied to the group will be applied to the path.
    * That is, local attributes inside curly braces will _override_
      the local attributes from the outside.

After being parsed by the macro, for the input attributes applied to
an input path, all the global attributes will always be placed _before_
all the local attributes.

Only `#[doc]` is global, and all other attributes are local.
Proc-macro attributes are local.

## Attribute aliases

Proc-macro attributes have aliases in the input:

* `#[proc_macro]` => `#[fn]`
* `#[proc_macro_attribute]` => `#[at]`
* `#[proc_macro_derive]` => `#[dr]`

If there is an imported attribute proc-macro having the same name as one
of the aliases, in order to use it in the input, it will need
to be renamed (e.g., rename it by using `use` declaration).

# Expansion

When a path is not renamed with the underscore alias, it will be
expanded to a `pub` function annotated with all the input attributes
that are applied to that path.

* If the path doesn't have an alias, the expanded function will have
  the same name as the name of the function in the path.
* If the path has an alias, the expanded function will have the same
  name as the alias.

The expanded function will and only will directly call the function
in that path, with the arguments input to the expanded function.

It is recommended to annotate the functions in input paths with
`#[inline(always)]`.

When a path is renamed with the underscore alias, it will be expanded
to empty.

# Examples

```ignore
// Doctest is ignored because the crate is not in a proc-macro context.
// This example can be found in the examples of the package.

// in the crate root
extern crate proc_macro;

#[no_link]
extern crate proc_macro_api;
use proc_macro_api::proc_macro_api;

mod mod_a {
    use proc_macro::TokenStream;

    pub fn an_fn_api(_input: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    pub mod mod_b {
        use proc_macro::TokenStream;

        pub fn an_attr_api(_args: TokenStream, _item: TokenStream) -> TokenStream {
            TokenStream::new()
        }

        pub fn a_derive_api(_item: TokenStream) -> TokenStream {
            TokenStream::new()
        }
    }
}

proc_macro_api! {
    // a path group
    mod_a::{
        // use an alias of the proc-macro attributes
        #[fn] an_fn_api,

        // a path subgroup;
        // apply local attributes to this path group
        #[allow(unused)]
        #[proc_macro_attribute]
        mod_b::{
            // apply the local attributes from the outside:
            // #[allow(unused)]
            // #[proc_macro_attribute]
            /// `#[doc]` won't override local attributes.
            /// This API is renamed `the_attr_api`.
            an_attr_api as the_attr_api,

            // override the local attributes from the outside:
            // only `#[dr(Something)]` is applied to this path
            #[dr(Something)] a_derive_api,

            // syntactically valid
            nonexistent_api as _,
        },
    },

    // syntactically valid
    ::nonexistent_mod::nonexistent_api as _,
}
// It will expand to three `pub` functions in the crate root, named
// `an_fn_api`, `the_attr_api`, and `a_derive_api`, respectively.
```

# Extern requirement

The macro requires some external _names_ to be available in the scope
where it is used:

* The name `proc_macro`, binding [`proc_macro`] or something else equivalent.
  It is required only for [`proc_macro::TokenStream`].
* The name `core`, binding [`core`] or something else equivalent.
  `core` is required only for error reporting. If there is an error
  inside `proc_macro_api!` caused by incorrect binding of `core`,
  it will indicate an error of the input.

# Depth of recursion

For an input path, the number of the recursive steps for parsing it
depends on the number of the segments in the path, the number of the
groups it belongs to, the number of the attributes applied to the path,
and the number of the attributes applied to the groups it belongs to.

Let _N_ be the number of the segments of the path.  
Let _G_ be the number of the groups the path belongs to.  
Let _A_ be the number of the attributes applied to the groups
the path belongs to.  
Let _B_ be the number of the attributes that are applied directly
to the path but not to the groups the path belongs to.  
Let _d_ be the recursion depth of the path.

When there isn't an error in the input:  
_d_ <= max { _A_ + _B_, _N_ } + 2 _G_ + 6 .

<!-- max { _A_ + _B_, _N_ } + (_G_ + 1) + (_G_ + 1) + 1 + (1 + 2) -->

When calling `proc_macro_api!`, the recursion depth of the macro call
depends on the maximum recursion depth of all the input paths.

Let _d<sub>p</sub>_ be the recursion depth of input path _p_.  
Let _D_ be the recursion depth of the macro call.

_D_ = max ( { _d<sub>p</sub>_ | _p_ is in the input } &cup; { 1 } ) .

> Note: In the input, the distinction between paths is not according to
> their segments, but to their appearances. That is, for example, in the
> input of `proc_macro_api!(a as _, a as _)`, there are two paths
> (i.e., the first `a` and the second `a`), instead of one.

# Optional features

* **`deny_shadow`** (enabled by default) - A feature for debugging.
  Deny the shadowing of proc-macro attributes. The feature will check
  whether a single path or a path group is annotated with multiple
  proc-macro attributes, and will generate a compile error if it does.
* **`deny_override`** - Deny the overriding of [local attributes](#attributes).
