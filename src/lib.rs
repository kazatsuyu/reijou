use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    Error, Result, Token, UseTree,
};

fn unexpected_ident<T>(ident: &Ident) -> Result<T> {
    Err(Error::new(
        ident.span(),
        format!("予期せぬ識別子ですわ～！: {}", ident),
    ))
}

fn expect_ident(input: ParseStream, name: &str) -> Result<Ident> {
    let ident: Ident = input.parse()?;
    if ident.to_string() == name {
        Ok(ident)
    } else {
        unexpected_ident(&ident)
    }
}

fn セバスチャン_parse(input: ParseStream) -> Result<TokenStream> {
    let ident: Ident = input.parse()?;
    if ident.to_string() == "わたくし" {
        let tree: UseTree = input.parse()?;
        expect_ident(input, "様を使わせていただきますわ")?;
        input.parse::<Token![.]>()?;
        Ok(quote! {
            use #tree;
        })
    } else {
        return unexpected_ident(&ident);
    }
}

fn セバスチャン_impl(tokens: TokenStream) -> TokenStream {
    セバスチャン_parse
        .parse2(tokens)
        .unwrap_or_else(Error::into_compile_error)
}

#[proc_macro]
pub fn セバスチャン(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    セバスチャン_impl(tokens.into()).into()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::*;

    #[test]
    fn test_セバスチャン() {
        assert_eq! {
            セバスチャン_impl(quote!{
                わたくし std::io::args 様を使わせていただきますわ.
            }).to_string(),
            quote!{
                use std::io::args;
            }.to_string()
        };
    }

    #[test]
    fn test_unexpected_ident() {
        assert_eq! {
            セバスチャン_impl(quote!{
                わたし
            }).to_string(),
            quote!{
                compile_error!{ "予期せぬ識別子ですわ～！: わたし" }
            }.to_string()
        };
    }
}
