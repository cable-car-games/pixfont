// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::char;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{Token, ext::IdentExt, parse::Parse, parse_macro_input};

#[proc_macro]
pub fn layer(input: TokenStream) -> TokenStream {
    let layer = parse_macro_input!(input as Layer);
    layer.into_token_stream().into()
}

#[proc_macro]
pub fn layers(input: TokenStream) -> TokenStream {
    let layers = parse_macro_input!(input as Layers);
    layers.into_token_stream().into()
}

#[proc_macro]
pub fn icon(input: TokenStream) -> TokenStream {
    let layers = parse_macro_input!(input as Layers);

    quote! {
        ::pixicons::icon::Icon::new(#layers)
    }
    .into()
}

#[derive(Clone)]
struct Layer(String);

impl syn::parse::Parse for Layer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Layer(
            syn::Ident::parse_any(input)?
                .to_string()
                .split('_')
                .map(|segment| {
                    let mut chars = segment.chars();
                    let mut str = String::new();

                    if let Some(char) = chars.next() {
                        str.push(char.to_ascii_uppercase());
                    }

                    str.extend(chars.flat_map(char::to_lowercase));
                    str
                })
                .fold(String::new(), |mut acc, segment| {
                    acc.push_str(&segment);
                    acc
                }),
        ))
    }
}

impl ToTokens for Layer {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Layer(name) = self;
        let name = quote::format_ident!("{}", name);

        tokens.extend(quote! {
            ::pixicons::Layer::#name
        })
    }
}

struct Layers(Vec<Layer>);

impl Parse for Layers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let layers = input.parse_terminated(Layer::parse, Token![.])?;
        Ok(Layers(layers.iter().cloned().collect()))
    }
}

impl ToTokens for Layers {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Layers(layers) = self;
        tokens.extend(quote! {
            ::pixicons::Layers::from(&[#(#layers ,)*])
        });
    }
}
