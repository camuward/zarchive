use proc_macro2::TokenStream;

pub fn derive_valid(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let syn::DeriveInput { attrs, vis, ident, generics, data } = input;
    let syn::Generics { where_clause, .. } = generics;
    
    todo!()
}
