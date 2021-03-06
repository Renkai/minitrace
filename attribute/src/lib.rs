#![feature(proc_macro_diagnostic)]
#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn trace(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let tag = syn::parse_macro_input!(args as syn::Expr);

    let syn::ItemFn {
        attrs,
        vis,
        block,
        sig,
    } = input;

    let syn::Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    let body = if asyncness.is_some() {
        let async_kwd = syn::token::Async { span: block.span() };
        let await_kwd = syn::Ident::new("await", block.span());
        quote::quote_spanned! {block.span()=>
            minitrace::future::Instrumented {
                inner: #async_kwd move { #block },
                span: __tracer_span,
            }
                .#await_kwd
        }
    } else {
        quote::quote_spanned!(block.span()=>
            let __tracing_attr_guard = __tracer_span.enter();
            #block
        )
    };

    quote::quote!(
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            let __tracer_span = minitrace::new_span(#tag as u32);
            #body
        }
    )
    .into()
}
