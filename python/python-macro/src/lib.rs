// For macro spans
#![feature(proc_macro_span)]

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn python(input: TokenStream) -> TokenStream {
    print(input);
    todo!()
}

fn print(input: TokenStream) {
    for t in input {
        if let TokenTree::Group(g) = t {
            println!("{:?}: open {:?}", g.span_open().start(), g.delimiter());
            print(g.stream());
            println!("{:?}: close {:?}", g.span_close().start(), g.delimiter());
        } else {
            println!("{:?}: {}", t.span().start(), t.to_string());
        }
    }
}
