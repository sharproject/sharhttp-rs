extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Item;

trait Handler {
    fn callback(&mut self);
}

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let body: Item = syn::parse(item).unwrap();

    match body {
        Item::Fn(a) => {
            let name = &a.sig.ident;
            let attrs = a.attrs;
            let sig = &a.sig;
            let body = a.block;
            dbg!(sig.to_token_stream().to_string());
            let code = quote! {
            #[derive(Debug,Default)]
            struct #name;
            impl #name {
                    #(#attrs)*
                    #sig {
                        #body
                    }
                }
            };
            TokenStream::from(code)
        }
        _ => {
            panic!("error when try to parse code")
        }
    }
}
