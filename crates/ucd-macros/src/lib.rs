// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    range::RangeInclusive,
};

use proc_macro::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt as _, quote};
use syn::parse::Parse;

#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let file = span.local_file().expect("not a local file");

    let BlockSource(source) = syn::parse_macro_input!(input as BlockSource);
    let source = file.parent().unwrap().join(source);

    let mut file = File::open(source).expect("and i oop");
    let mut reader = BufReader::new(&mut file);

    let mut blocks = LinkedList::new();

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        };

        if line.starts_with('#') {
            continue;
        }

        let Some((range, name)) = line.split_once(';') else {
            continue;
        };
        let name = name.trim();

        let Some((start, end)) = range.split_once("..") else {
            continue;
        };

        let start = u32::from_str_radix(start, 16).unwrap();
        let end = u32::from_str_radix(end, 16).unwrap();

        blocks.push_back(Block {
            range: (start..=end).into(),
            name: name.to_owned(),
        });
    }

    let all = blocks.iter().cloned().map(BlockList);

    quote! {
        pub const ALL: &[Block] = &[
            #(#all ,)*
        ];
    }
    .into()
}

struct BlockSource(PathBuf);

#[derive(Clone)]
struct Block {
    range: RangeInclusive<u32>,
    name: String,
}

impl Parse for BlockSource {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: syn::LitStr = input.parse()?;
        Ok(BlockSource(PathBuf::from(lit.value())))
    }
}

struct BlockList(Block);

impl ToTokens for BlockList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let BlockList(Block { range, name }) = self;
        let RangeInclusive { start, last } = range;

        tokens.append_all(quote! {
            Block {
                range: RangeInclusive {
                    start: #start,
                    last: #last
                },
                name: #name,
            }
        });
    }
}
