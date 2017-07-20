#![recursion_limit="256"]

extern crate proc_macro;
extern crate url;
extern crate syn;
#[macro_use]
extern crate quote;

mod extractors;
mod extenders;
mod helpers;

use helpers::ty_params;

#[proc_macro_derive(BasePathExtractor)]
pub fn base_path_extractor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let gen = extractors::base_path(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(PathExtractor)]
pub fn path_extractor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let brp = extractors::base_path(&ast);
    let brsre = extenders::bad_request_static_response_extender(&ast);

    let gen = quote! { #brp #brsre};
    gen.parse().unwrap()
}

#[proc_macro_derive(BaseQueryStringExtractor)]
pub fn base_query_string_extractor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let gen = extractors::base_query_string(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(QueryStringExtractor)]
pub fn query_string_extractor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let bqs = extractors::base_query_string(&ast);
    let brsre = extenders::bad_request_static_response_extender(&ast);

    let gen = quote! { #bqs #brsre };
    gen.parse().unwrap()
}

#[proc_macro_derive(StateData)]
pub fn state_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let (name, borrowed, where_clause) = ty_params(&ast);
    let gen = quote! {
        impl #borrowed gotham::state::StateData for #name #borrowed #where_clause {}
    };

    gen.parse().unwrap()
}
