use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{env, fs, path::PathBuf};
use syn::{
    LitBool, LitStr, Type,
    parse::{Parse, ParseStream},
    parse_macro_input, token,
};

fn resolve_file_path(lit: LitStr) -> PathBuf {
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

    path
}

#[proc_macro]
pub fn embed_input(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let path = resolve_file_path(lit);

    let mut width = 0;
    let mut height = 0usize;
    let mut parts = Vec::new();

    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let lines = content.trim_end_matches('\n').lines();

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

#[proc_macro]
pub fn embed_positions(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let path = resolve_file_path(lit);

    let mut min_x = usize::MAX;
    let mut max_x = 0_usize;

    let mut min_y = usize::MAX;
    let mut max_y = 0_usize;

    let mut parts = Vec::new();

    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let lines = content.trim_end_matches('\n').lines();

    for line in lines {
        let (a, b) = line.split_once(",").unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();

        min_x = min_x.min(a);
        max_x = max_x.max(a);

        min_y = min_y.min(b);
        max_y = max_y.max(b);

        parts.push(quote! { (#a, #b) });
    }

    let len = parts.len();

    quote! {
        const MIN_X: usize = #min_x;
        const MAX_X: usize = #max_x;

        const MIN_Y: usize = #min_y;
        const MAX_Y: usize = #max_y;

        const SIZE: usize = #len;

        const INPUT: [(usize, usize); SIZE] = [#(#parts),*];
    }
    .into()
}

#[proc_macro]
pub fn embed_lines(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let path = resolve_file_path(lit);

    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let lines = content.trim_end_matches('\n').lines().collect::<Vec<_>>();
    let height = lines.len();

    quote! {
        const HEIGHT: usize = #height;
        const INPUT: [&str; HEIGHT] = [#(#lines),*];
    }
    .into()
}

#[proc_macro]
pub fn embed_split_strip_colon_lines(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let path = resolve_file_path(lit);

    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let lines = content.trim_end_matches('\n').lines().collect::<Vec<_>>();
    let height = lines.len();
    let mut parts = Vec::new();

    for line in lines {
        let mut bits = line
            .split(" ")
            .map(|it| it.trim().trim_end_matches(':'))
            .collect::<Vec<_>>();

        let first = bits.remove(0);

        parts.push(quote! { (#first, &[#(#bits),*]) });
    }

    quote! {
        const HEIGHT: usize = #height;
        const INPUT: [(&str, &[&str]); HEIGHT] = [#(#parts),*];
    }
    .into()
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(ty);
    custom_keyword!(inclusive);
    custom_keyword!(sep);
}

struct EmbedRangeParams {
    path: LitStr,
    ty: Type,
    inclusive: LitBool,
    sep: LitStr,
}

impl Parse for EmbedRangeParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let mut inclusive = LitBool::new(false, Span::call_site());
        let mut sep = LitStr::new("\n", Span::call_site());
        let ty = Type::Verbatim(quote! { u64 });

        while !input.is_empty() {
            let look = input.lookahead1();

            if look.peek(token::Comma) {
                input.parse::<token::Comma>()?;

                let look = input.lookahead1();

                if look.peek(kw::sep) {
                    input.parse::<kw::sep>()?;
                    input.parse::<token::Eq>()?;

                    sep = input.parse()?;
                } else if look.peek(kw::inclusive) {
                    input.parse::<kw::inclusive>()?;
                    input.parse::<token::Eq>()?;

                    inclusive = input.parse()?;
                } else if look.peek(kw::ty) {
                    // TODO:

                    // input.parse::<kw::ty>()?;
                    // input.parse::<token::Eq>()?;

                    // ty = input.parse()?;

                    todo!("Custom types are implemented yet!");
                } else {
                    return Err(look.error());
                }
            } else {
                break;
            }
        }

        Ok(Self {
            ty,
            path,
            inclusive,
            sep,
        })
    }
}

#[proc_macro]
pub fn embed_ranges(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EmbedRangeParams);
    let path = resolve_file_path(input.path);
    let content = fs::read_to_string(path).expect("Cannot read file: ");
    let sep = input.sep.value();
    let lines = content.trim().split(&sep).collect::<Vec<_>>();
    let mut parts = Vec::new();

    let inclusive = if input.inclusive.value {
        quote! { ..= }
    } else {
        quote! { .. }
    };

    let base_ty = input.ty;

    let ty = if input.inclusive.value {
        quote! { std::ops::RangeInclusive<#base_ty> }
    } else {
        quote! { std::ops::Range<#base_ty> }
    };

    let len = lines.len();

    for line in lines {
        let (first, last) = line
            .split_once("-")
            .expect(&format!("Cannot split line ('{line}') once on '-': "));

        let first = first
            .parse::<u64>()
            .expect(&format!("Failed to parse lhs ('{first}'): "));

        let last = last
            .parse::<u64>()
            .expect(&format!("Failed to parse rhs ('{last}'): "));

        parts.push(quote! {
            #first #inclusive #last
        });
    }

    quote! {
        const INPUT: [#ty; #len] = [#(#parts),*];
    }
    .into()
}
