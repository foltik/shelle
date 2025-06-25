use proc_macro_error2::proc_macro_error;
use quote::quote;

mod lexer;
mod parser;

#[proc_macro]
#[proc_macro_error]
pub fn exec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let script = lexer::Lexer::new(input.into()).scan().parse();
    quote!(#script.exec()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn eval(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let script = lexer::Lexer::new(input.into()).scan().parse();
    quote!(#script.eval()).into()
}
