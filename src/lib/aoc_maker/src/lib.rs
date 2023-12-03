extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn lines(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let return_type = &function.sig.output;
    let block = &function.block;

    let output = quote! {
        #visibility fn #fn_name(input: &str) #return_type {
            let lines: Vec<String> = input.split('\n').map(String::from).collect();
            #details_name(lines)
        }

        fn #details_name(#inputs) #return_type #block
    };

    output.into()
}

#[proc_macro_attribute]
pub fn regex(attr: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let re_pattern = parse_macro_input!(attr as LitStr);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let return_type = &function.sig.output;
    let block = &function.block;

    let output = quote! {
        #visibility fn #fn_name(input: &str) #return_type {
            let lines: Vec<String> = input.split('\n').map(String::from).collect();
            let re = regex::Regex::new(#re_pattern).expect(format!("{} is an invalid regex pattern!", #re_pattern));
            #details_name(lines)
        }

        fn #details_name(#inputs) #return_type #block
    };

    output.into()
}