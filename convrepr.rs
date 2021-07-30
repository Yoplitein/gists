/**cargo.toml:
[dependencies]
syn = "1.0.74"
quote = "1.0.9"
proc-macro2 = "1.0.27"
*/
#![allow(non_snake_case)]

use proc_macro2::{TokenStream, TokenTree};
use syn::{parse_macro_input, AttrStyle, DeriveInput};
use quote::quote;

#[proc_macro_derive(ConvRepr)]
pub fn conv_repr_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ref name = input.ident;
    let reprType = {
        let mut res = None;
        for attr in input.attrs {
            use std::mem::{discriminant as disc};
            if disc(&attr.style) != disc(&AttrStyle::Outer) || !attr.path.is_ident("repr") { continue; }
            
            let group = match attr.tokens.into_iter().next() {
                Some(TokenTree::Group(v)) => v,
                _ => panic!("repr attr does not contain group of tokens"),
            };
            let stream = group.stream();
            res = stream.into_iter().next();
            break;
        }
        
        res.expect("repr attribute does not specify a type")
    };
    let enumData = match input.data {
        syn::Data::Enum(ref v) => v,
        _ => panic!("derive(ConvRepr) only works on enums")
    };
    
    let mut fromArms = TokenStream::new();
    let mut toArms = TokenStream::new();
    let mut repr: isize = 0;
    for variant in enumData.variants.iter() {
        match variant.fields {
            syn::Fields::Unit => {},
            _ => panic!("derive(ConvRepr) enums must not use fields"),
        }
        if let Some((_, ref expr)) = variant.discriminant {
            if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(val), .. }) = expr {
                let val = val.base10_parse::<isize>().expect("failed to parse discriminator literal");
                repr = val;
            } else {
                panic!("expected enum discriminant to be a literal integer");
            }
        }
        let ref id = variant.ident;
        
        let unsuffixed = proc_macro2::Literal::isize_unsuffixed(repr);
        fromArms.extend(quote !{
            #unsuffixed => Ok(#name::#id),
        });
        toArms.extend(quote !{
            #name::#id => #unsuffixed,
        });
        repr += 1;
    }
    
    let impls = quote! {
        impl std::convert::TryFrom<#reprType> for #name {
            type Error = String;
            fn try_from(v: #reprType) -> Result<Self, Self::Error> {
                match v {
                    #fromArms
                    _ => Err(format!("enum {} has no variant with a discriminant of {}", stringify!(#name), v))
                }
            }
        }
        
        impl From<#name> for #reprType {
            fn from(v: #name) -> Self {
                match v {
                    #toArms
                }
            }
        }
    };
    impls.into()
}