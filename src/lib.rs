pub use proc_macro2::TokenStream as TokenStream2;
pub use proc_util_macros::*;
use quote::ToTokens;

/// Bring this trait into scope for a blanket `to_pretty()` implementation for all
/// `syn`-compatible types as well as `TokenStream2`.
///
/// Calling [`to_pretty`](`ToPretty::to_pretty`) on any such objects will result in a
/// pretty-formatted string representation of the (parsed) source code of that item.
pub trait ToPretty {
    /// Produces a pretty-formatted [`String`] (formatted by [`prettyplease`]) representation
    /// of the underlying token stream.
    ///
    /// Can be used on anything that implements [`ToTokens`] if [`ToPretty`] is brought into
    /// scope.
    ///
    /// ## Example
    ///
    /// Using `to_pretty` directly on a [`TokenStream2`]:
    /// ```
    /// use proc_utils::ToPretty;
    ///
    /// let struct_tokens = quote::quote!(struct MyStruct;);
    /// assert_eq!(struct_tokens.to_pretty(), "struct MyStruct;\n");
    /// ```
    ///
    /// Using `to_pretty` on a [`syn::ItemStruct`]:
    /// ```
    /// use proc_utils::ToPretty;
    ///
    /// let struct_tokens = quote::quote!(struct MyStruct { field1: usize, field2: u32, field3: bool });
    /// let item_struct = syn::parse2::<syn::ItemStruct>(struct_tokens).unwrap();
    /// assert_eq!(item_struct.to_pretty(),
    ///     "struct MyStruct {\n    \
    ///          field1: usize,\n    \
    ///          field2: u32,\n    \
    ///          field3: bool,\n\
    ///     }\n"
    /// );
    /// ```
    fn to_pretty(&self) -> String;
}

impl<T: ToTokens> ToPretty for T {
    fn to_pretty(&self) -> String {
        let tokens: TokenStream2 = self.to_token_stream();
        let file = syn::parse_file(&tokens.to_string()).unwrap();
        prettyplease::unparse(&file)
    }
}
