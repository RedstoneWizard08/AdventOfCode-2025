use proc_macro::TokenStream;
use quote::quote;
use std::{env, fs, path::PathBuf};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn embed_input(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let input = lit.value();

    let source = match lit.span().unwrap().local_file() {
        Some(it) => it,
        None => PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("dummy"),
    };

    let path = if input.starts_with("/") {
        PathBuf::from(input)
    } else {
        source
            .parent()
            .expect("No parent for local file found: ")
            .join(input)
    };

    let mut width = 0;
    let mut height = 0usize;
    let mut parts = Vec::new();

    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let lines = content.lines();

    for line in lines {
        let chars = line.chars();

        width = width.max(line.chars().count());
        height += 1;

        parts.push(quote! {
            [#(#chars),*]
        });
    }

    quote! {
        const WIDTH: usize = #width;
        const HEIGHT: usize = #height;
        const INPUT: [[char; WIDTH]; HEIGHT] = [#(#parts),*];
    }
    .into()
}
