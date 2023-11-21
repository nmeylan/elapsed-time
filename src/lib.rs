use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Block, ItemFn, Meta};

type AttributeArgs = Punctuated<Meta, syn::token::Comma>;

#[proc_macro_attribute]
pub fn elapsed(_args: TokenStream, function_def: TokenStream) -> TokenStream {
    let mut item = syn::parse(function_def).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn")
    };
    let ItemFn { attrs, vis, sig, block } = fn_item;
    let function_body = block.clone();
    let fn_name = sig.ident.clone();
    let log_ns = format!("{} took {{}}ns", fn_name);
    let log_us = format!("{} took {{}}µs", fn_name);
    let log_ms = format!("{} took {{}}ms", fn_name);
    #[cfg(feature = "tracing")]
    let log_ns_stmt = quote! { tracing::debug!(#log_ns, elapsed); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_ns_stmt = quote! { log::debug!(#log_ns, elapsed); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_ns_stmt = quote! { println!(#log_ns, elapsed); };

    #[cfg(feature = "tracing")]
    let log_us_stmt = quote! { tracing::debug!(#log_us, elapsed as f64 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_us_stmt = quote! { log::debug!(#log_us, elapsed as f64 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_us_stmt = quote! { println!(#log_us, elapsed as f64 / 1000.0); };

    #[cfg(feature = "tracing")]
    let log_ms_stmt = quote! { tracing::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_ms_stmt = quote! { log::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_ms_stmt = quote! { println!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    let new_function_def = quote! {
        #(#attrs)* #vis #sig {
            let start_for_elapsed_macro = std::time::Instant::now();
            let mut wrapped_func = || #function_body;
            let res = wrapped_func();
            let elapsed = start_for_elapsed_macro.elapsed().as_nanos();
            if elapsed < 1000 {
                #log_ns_stmt
            } else if elapsed < 1000 * 1000 {
                #log_us_stmt
            } else {
                #log_ms_stmt
            }
            res
        }
    };
    TokenStream::from(new_function_def)
}

#[proc_macro_attribute]
pub fn elapsed_block(args: TokenStream, block_def: TokenStream) -> TokenStream {
    let mut block_name = "unnamed block".to_string();
    let attrs = parse_macro_input!(args with AttributeArgs::parse_terminated);
    if let Some(first_attr) = attrs.first() {
        block_name = first_attr.to_token_stream().to_string();
    }
    let item = syn::parse::<Block>(block_def).unwrap();
    let log_ns = format!("{} took {{}}ns", block_name);
    let log_us = format!("{} took {{}}µs", block_name);
    let log_ms = format!("{} took {{}}ms", block_name);
    #[cfg(feature = "tracing")]
    let log_ns_stmt = quote! { tracing::debug!(#log_ns, elapsed); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_ns_stmt = quote! { log::debug!(#log_ns, elapsed); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_ns_stmt = quote! { println!(#log_ns, elapsed); };

    #[cfg(feature = "tracing")]
    let log_us_stmt = quote! { tracing::debug!(#log_us, elapsed as f64 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_us_stmt = quote! { log::debug!(#log_us, elapsed as f64 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_us_stmt = quote! { println!(#log_us, elapsed as f64 / 1000.0); };

    #[cfg(feature = "tracing")]
    let log_ms_stmt = quote! { tracing::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), feature = "log"))]
    let log_ms_stmt = quote! { log::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    #[cfg(all(not(feature = "tracing"), not(feature = "log")))]
    let log_ms_stmt = quote! { println!(#log_ms, elapsed as f64 / 1000.0 / 1000.0); };
    let new_block_def = quote! {
        {
            let start_for_elapsed_macro = std::time::Instant::now();
            #item
            let elapsed = start_for_elapsed_macro.elapsed().as_nanos();
            if elapsed < 1000 {
                #log_ns_stmt
            } else if elapsed < 1000 * 1000 {
                #log_us_stmt
            } else {
                #log_ms_stmt
            }
        }
    };
    TokenStream::from(new_block_def)
}
