extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Token,
};

struct StaticClosure {
    ident: syn::Ident,
    closure: syn::ExprClosure,
}

impl Parse for StaticClosure {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: syn::Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let closure: syn::ExprClosure = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(StaticClosure { ident, closure })
    }
}

struct MultipleStaticClosures {
    static_closures: Vec<StaticClosure>,
}

impl Parse for MultipleStaticClosures {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut static_closures = vec![StaticClosure::parse(input)?];
        while let Ok(sc) = StaticClosure::parse(input) {
            static_closures.push(sc);
        }
        Ok(MultipleStaticClosures { static_closures })
    }
}

#[proc_macro]
pub fn static_closures(input: TokenStream) -> TokenStream {
    let MultipleStaticClosures { static_closures } =
        parse_macro_input!(input as MultipleStaticClosures);
    let all_tokens = static_closures.iter().map(|input| {
        let StaticClosure{ident, closure} = input;
        let closure_inputs = closure.inputs.clone();
        let closure_types: Vec<syn::Type> = closure_inputs
            .into_iter()
            .map(|p| match p {
                syn::Pat::Type(pat_type) => pat_type.ty.as_ref().to_owned(),
                _ => panic!("Expected types in closure input, got {}", stringify!(p)),
            })
            .collect();
        let tokens = quote! {
            thread_local! {
                static #ident: wasm_bindgen::prelude::Closure<dyn FnMut(#(#closure_types),*)> = wasm_bindgen::prelude::Closure::wrap(Box::new(#closure) as Box<dyn FnMut(#(#closure_types),*)>);
            }
        };
        tokens
    });

    TokenStream::from(quote!(#(#all_tokens)*))
}
