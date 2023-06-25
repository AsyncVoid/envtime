extern crate proc_macro;
use proc_macro::{Literal, TokenStream, TokenTree};
use std::env;
use syn::{parse_macro_input, Lit, LitStr, Token, LitBool, LitByte, LitInt};
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn envtime(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as LitStr);
    let comp_env = env::var(lit_str.value());
    if let Ok(comp_env_val) = comp_env {
        let literal = LitStr::new(comp_env_val.as_str(), lit_str.span());
        return quote! {
            Some(String::from(#literal))
        }.into()
    }
    return  quote! {
        env::var(#lit_str).ok()
    }.into()
}

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn envtime_def(input: TokenStream) -> TokenStream {
    let input : Punctuated<Lit,Token![,]> = parse_macro_input!(input with Punctuated<Lit,Token![,]>::parse_terminated);
    if input.len() != 2 {
        panic!("A env variable name and a default value is required. 2 arguments expected!");
    }
    let env_var = match input.first().unwrap() {
        Lit::Str(lit) => lit,
        _ => panic!("First parameter has to be a string literal")
    };

    let def_val = input.last().unwrap();

    let comp_env = env::var(env_var.value());
    if let Ok(comp_env_val) = comp_env {
        return match def_val {
            Lit::Str(_) => {
                let lit = LitStr::new(comp_env_val.as_str(), input.span());
                (quote! { String::from(#lit) }).into()
            },
            Lit::Bool(_) => {
                let lit = LitBool::new(match comp_env_val.as_str() {
                    "y" | "Y" | "Yes" | "yes" | "true" => true,
                    _ => false
                }, input.span());
                (quote! { #lit }).into()
            },
            Lit::Byte(_) => {
                let lit = LitByte::new(
                    comp_env_val
                        .parse()
                        .expect("Cannot parse compilation env var (byte)"),
                    input.span());
                (quote! { #lit }).into()
            },
            Lit::Char(_) => {
                TokenStream::from(TokenTree::Literal(Literal::character(
                    comp_env_val.parse()
                        .expect("Cannot parse compilation env var (char)"))))
            },
            Lit::Int(lit_int) => {
                let s = lit_int.to_string();
                let type_index = find_int_type_index(&s);

                if let None = type_index {
                    let lit = LitInt::new(&comp_env_val, input.span());
                    return quote! {
                        #lit
                    }.into()
                }

                let type_index = type_index.unwrap();
                let type_str = &s[type_index .. s.len()];

                TokenStream::from(TokenTree::Literal(match type_str {
                    "u8" => Literal::u8_suffixed(
                        comp_env_val.parse::<u8>().expect("Invalid u8")
                    ),
                    "i8" => Literal::i8_suffixed(
                        comp_env_val.parse::<i8>().expect("Invalid i8")
                    ),
                    "u16" => Literal::u16_suffixed(
                        comp_env_val.parse::<u16>().expect("Invalid u16")
                    ),
                    "i16" => Literal::i16_suffixed(
                        comp_env_val.parse::<i16>().expect("Invalid i16")
                    ),
                    "u32" => Literal::u32_suffixed(
                        comp_env_val.parse::<u32>().expect("Invalid u32")
                    ),
                    "i32" => Literal::i32_suffixed(
                        comp_env_val.parse::<i32>().expect("Invalid i32")
                    ),
                    "u64" => Literal::u64_suffixed(
                        comp_env_val.parse::<u64>().expect("Invalid u64")
                    ),
                    "i64" => Literal::i64_suffixed(
                        comp_env_val.parse::<i64>().expect("Invalid i64")
                    ),
                    "u128" => Literal::u128_suffixed(
                        comp_env_val.parse::<u128>().expect("Invalid u128")
                    ),
                    "i128" => Literal::i128_suffixed(
                        comp_env_val.parse::<i128>().expect("Invalid i128")
                    ),
                    "usize" => Literal::usize_suffixed(
                        comp_env_val.parse::<usize>().expect("Invalid usize")
                    ),
                    "isize" => Literal::isize_suffixed(
                        comp_env_val.parse::<isize>().expect("Invalid isize")
                    ),
                    _ => panic!("Unknown type: {:?}", type_str)
                }))
            }
            _ => panic!("Unknown type of default value")
        };
    }

    return match def_val {
        Lit::Str(_) => {
            (quote! {
                env::var(#env_var).unwrap_or(String::from(#def_val))
            }).into()
        },
        Lit::Bool(_) => {
            (quote! {
                env::var(#env_var).map_or(#def_val, |s| match s.as_str() {
                    "y" | "Y" | "Yes" | "yes" | "true" => true,
                    _ => false
                })
            }).into()
        },
        Lit::Byte(_) => {
            (quote! {
                env::var(#env_var).ok().and_then(|s| s.parse::<u8>().ok()).unwrap_or(#def_val)
            }).into()
        },
        Lit::Int(_) => {
            (quote! {
                env::var(#env_var).ok().and_then(|s| s.parse().ok()).unwrap_or(#def_val)
            }).into()
        }
        _ => panic!("Unknown default value type")
    }
}

fn find_int_type_index(s: &str) -> Option<usize> {
    s.find(|c|
        match c {
            'u' | 'i' => true,
            _ => false
        })
}