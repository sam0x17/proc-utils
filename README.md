# proc-utils

This crate provides a series of traits, macros, functions, and utilities that make writing and
debugging proc macros easier

This includes pretty-printing facilities for anything implementing `syn::ToTokens`, as well as
some useful macros including `#[overwrite_with(..)]` and `#[suppress_item]`, which allow for
suppressing and/or replacing the underlying item that the attribute is attached to, which can
be a useful debugging trick.

See the documentation for specific usage examples.
