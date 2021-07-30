/**cargo.toml:
[dependencies]
syn = "1.0.74"
quote = "1.0.9"
proc-macro2 = "1.0.27"
*/

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
    let mut repr: usize = 0;
    for variant in enumData.variants.iter() {
        match variant.fields {
            syn::Fields::Unit => {},
            _ => panic!("derive(ConvRepr) enums must not use fields"),
        }
        if variant.discriminant.is_some() {
            // TODO: allow if ranges do not overlap
            panic!("derive(ConvRepr) enums must not specify a discriminant");
        }
        
        let ref id = variant.ident;
        fromArms.extend(quote !{
            #repr => #name::#id,
        });
        toArms.extend(quote !{
            #name::#id => #repr,
        });
        repr += 1;
    }
    
    let impls = quote! {
        impl From<#reprType> for #name {
            fn from(v: #reprType) -> Self {
                let v = v as usize;
                match v {
                    #fromArms
                    _ => panic!("cannot convert {} to a {}", stringify!(#name), stringify!(#reprType)),
                }
            }
        }
        
        impl From<#name> for #reprType {
            fn from(v: #name) -> Self {
                let res = match v {
                    #toArms
                    _ => panic!("cannot convert {} to a {}", stringify!(#reprType), stringify!(#name)),
                };
                res as Self
            }
        }
    };
    impls.into()
}