extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse, parse::Parser, punctuated::Punctuated, Expr, ItemFn};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let exprs = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let (event, day, level) = match &*exprs {
        [event, day, level] => (event, day, level),
        _ => panic!("aoc: Invalid number of parameters"),
    };
    let fun = parse::<ItemFn>(item)
        .ok()
        .expect("aoc can only be applied to functions");
    let ident = &fun.sig.ident;

    let out = quote! {
        fn #ident() {
            #fun
            ::aoc::cli::run(&#event.to_string(), #day, #level, #ident);
        }
    };

    out.into()
}
