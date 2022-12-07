use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{Result, Expr};
use syn::parse::{Parse, ParseStream};

struct Num {
    num: usize,
}

impl Parse for Num {
    fn parse(input: ParseStream) -> Result<Self> {
        let num = input.parse::<Expr>()?;
        let num = match num {
            Expr::Lit(lit) => lit.lit,
            _ => panic!("Expected a literal"),
        };
        let num = match num {
            syn::Lit::Int(lit) => lit.base10_parse::<usize>().unwrap(),
            _ => panic!("Expected an integer"),
        };
        Ok(Num { num })
    }
}

#[proc_macro]
pub fn num_to_prob(input: TokenStream) -> TokenStream {
    let n = syn::parse_macro_input!(input as Num);
    let num = n.num;
    let prob = format_ident!("P{:03}", num);
    let expanded = quote! {
        #prob::new()
    };
    TokenStream::from(expanded)
}
