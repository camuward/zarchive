use proc_macro::TokenStream;

mod validate;

#[proc_macro_derive(Valid, attributes(valid))]
pub fn derive_valid(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match crate::validate::derive_valid(input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
