extern crate proc_macro;
use proc_macro::{Literal, TokenStream, TokenTree};
use std::env;
use syn::{parse_macro_input, Lit, LitStr, Token, LitBool, LitByte, LitInt};
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

//! # envime
//!
//! This crate provides a procedural macro that can retrieve an environment variable at compile time or runtime.
//!
//! Specify an environment variable at compile time and the runtime variable will never be evaluated.
//! Don't specify the environment variable at compile-time and it will be evaluated at runtime.
//!
//! ## Examples
//!
//! ```
//! use std::env;
//! use envtime::*;
//!
//! // You can use unwrap_or_else on the envtime macro
//! let var = envtime!("TEST_NON_ENV").unwrap_or_else(|| String::from("hello"));
//! assert_eq!(var, String::from("012"));
//! // This resolves to "hello" assuming it is not defined at compile-time or runtime
//!
//! // Lets set a runtime variable to "123"
//! env::set_var("TEST_RUN_ENV", "123");
//! let var = envtime!("TEST_RUN_ENV");
//! assert_eq!(var, Some(String::from("123")));
//! // And you can see it gets the value
//!
//! // Assume we have a compile-time variable set to "456"
//! env::set_var("TEST_COMP_ENV", "123"); // We set the runtime variable to "123"
//! let var = envtime!("TEST_COMP_ENV");
//! assert_eq!(var, Some(String::from("456")));
//! // And the runtime variable is ignored, as the macro resolves to 'String::from("456")' at compile time
//!
//! // Assume we don't have the runtime variable set at first, you can see the default value being used
//! assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "test");
//! env::set_var("TEST_STR_RUN_ENV", "not");
//! assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "not");
//! // And once it is set it resolves to our runtime value
//!
//! // Assume we have "TEST_BOOL_COMP_ENV" at compile-time to "true"
//! env::set_var("TEST_BOOL_COMP_ENV", "false"); // Sets the runtime-variable, which is ignored
//! let enable_test = envtime_def!("TEST_BOOL_COMP_ENV", false);  // Resolves to the literal "true"
//! assert_eq!(enable_test, true);
//! // With the default value being false, and the runtime value being false, it still evaluates to true
//!
//! // Example with u8
//! assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 77u8);
//! env::set_var("TEST_U8_RUN_ENV", "53");
//! assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 53u8);
//! ```

/// Gets a environment variable as a String either at compile or runtime
/// # Example
/// ```
/// use envtime::*;
///
/// // Assuming the variable isn't set
/// let var = envtime!("DOMAIN");
/// assert_eq!(var, None);
/// let var = var.unwrap_or_else(|| String::from("example.com"));
/// assert_eq!(var, String::from("example.com"));
/// ```
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

/// Gets a environment variable as the type specified by the default value, either at compile or runtime
/// # Example
/// ```
/// use envtime::*;
///
/// // Assuming we set the value of "PORT" to 5678 at compile or runtime
/// let var = envtime_def!("PORT", 1234u16);
/// assert_eq!(var, 5678u16);
/// ```
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