use proc_macro::TokenStream;
use syn::{parse::Nothing, parse_macro_input};

/// Completely suppresses the attached item from being emitted into the surrounding source
/// file. If you put this attribute last, the previous attributes will still see the item.
///
/// This is extremely useful for debugging in general, as it allows you to suppress mundane
/// errors resulting from the generated code of an item so you can focus on compile errors that
/// originated from proc macros related to that item.
///
/// ## Example
///
/// The following will compile only if `#[suppress_item]` is attached:
///
/// ```ignore
/// #[suppress_item]
/// fn invalid_rust() {
///     return value_that_doesnt_exist;
/// }
/// ```
///
/// This compiles because the item expands to nothing, so rustc doesn't have a chance to detect
/// any problems with this syntax.
#[proc_macro_attribute]
pub fn suppress_item(attr: TokenStream, _tokens: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    TokenStream::new()
}

/// Similar to [`macro@suppress_item`], but will instead entirely replace the item this
/// attribute is attached to with whatever item is specified in the attribute arguments.
///
/// ## Example
///
/// The following test will pass because `invalid_rust()` is not emitted and is instead
/// replaced with `hello_world()`, which can then be referred to on subsequent lines:
///
/// ```ignore
/// #[overwrite_with {
///     fn hello_world() -> usize {
///         return 3;
///     }
/// }]
/// fn invalid_rust() {
///     return nonexistent_value;
/// }
/// assert_eq!(hello_world(), 3);
/// ```
#[proc_macro_attribute]
pub fn overwrite_with(attr: TokenStream, _tokens: TokenStream) -> TokenStream {
    attr
}
