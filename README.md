# proc-utils

[![Crates.io](https://img.shields.io/crates/v/proc-utils)](https://crates.io/crates/proc-utils)
[![docs.rs](https://img.shields.io/docsrs/proc-utils?label=docs)](https://docs.rs/proc-utils/latest/proc-utils/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/proc-utils/ci.yaml)](https://github.com/sam0x17/proc-utils/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/proc-utils)](https://github.com/sam0x17/proc-utils/blob/main/LICENSE)

This crate provides a series of traits, macros, functions, and utilities that make writing and
debugging proc macros easier

This includes pretty-printing facilities for anything implementing `syn::ToTokens`, as well as
some useful macros including `#[overwrite_with(..)]` and `#[suppress_item]`, which allow for
suppressing and/or replacing the underlying item that the attribute is attached to, which can
be a useful debugging trick.

See the documentation for specific usage examples.
