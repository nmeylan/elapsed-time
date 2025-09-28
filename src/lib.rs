use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ItemFn, parse_macro_input};
use syn::AttributeArgs;

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
    let new_function_def = quote! {
        #(#attrs)* #vis #sig {
            let start_for_elapsed_macro = std::time::Instant::now();
            let mut wrapped_func = || #function_body;
            let res = wrapped_func();
            let elapsed = start_for_elapsed_macro.elapsed().as_nanos();
            if elapsed < 1000 {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_ns, elapsed);
                #[cfg(feature="log")]
                log::debug!(#log_ns, elapsed);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_ns, elapsed);
            } else if elapsed < 1000 * 1000 {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_us, elapsed as f64 / 1000.0);
                #[cfg(feature="log")]
                log::debug!(#log_us, elapsed as f64 / 1000.0);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_us, elapsed as f64 / 1000.0);
            } else {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
                #[cfg(feature="log")]
                log::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
            }
            res
        }
    };
    TokenStream::from(new_function_def)
}

#[proc_macro_attribute]
pub fn elapsed_block(args: TokenStream, block_def: TokenStream) -> TokenStream {
    let mut block_name = "unnamed block".to_string();
    let attrs = parse_macro_input!(args as AttributeArgs);
    if attrs.len() > 0 {
        block_name = attrs.get(0).unwrap().to_token_stream().to_string();
    }
    let item = syn::parse::<Block>(block_def).unwrap();
    let log_ns = format!("{} took {{}}ns", block_name);
    let log_us = format!("{} took {{}}µs", block_name);
    let log_ms = format!("{} took {{}}ms", block_name);
    let new_block_def = quote! {
        {
            let start_for_elapsed_macro = std::time::Instant::now();
            #item
            let elapsed = start_for_elapsed_macro.elapsed().as_nanos();
            if elapsed < 1000 {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_ns, elapsed);
                #[cfg(feature="log")]
                log::debug!(#log_ns, elapsed);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_ns, elapsed);
            } else if elapsed < 1000 * 1000 {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_us, elapsed as f64 / 1000.0);
                #[cfg(feature="log")]
                log::debug!(#log_us, elapsed as f64 / 1000.0);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_us, elapsed as f64 / 1000.0);
            } else {
                #[cfg(feature="tracing")]
                tracing::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
                #[cfg(feature="log")]
                log::debug!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
                #[cfg(not(feature="tracing", feature="log"))]
                println!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
            }
        }
    };
    TokenStream::from(new_block_def)
}