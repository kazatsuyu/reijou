use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    Block, Error, FnArg, Result, Token, Type, UseTree,
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
    match ident.to_string().as_str() {
        "わたくし" => {
            let tree: UseTree = input.parse()?;
            expect_ident(input, "様を使わせていただきますわ")?;
            input.parse::<Token![.]>()?;
            Ok(quote! {
                use #tree;
            })
        }
        "こちらの" => {
            let name: Ident = input.parse()?;
            expect_ident(input, "様は")?;
            input.parse::<Token![,]>()?;
            let mut args = Punctuated::<FnArg, Token![,]>::new();
            loop {
                let arg: FnArg = input.parse()?;
                let ident: Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "と" => {
                        args.push(arg);
                    }
                    "をお受け取りになって" => {
                        args.push(arg);
                        break;
                    }
                    _ => return unexpected_ident(&ident),
                }
            }
            input.parse::<Token![,]>()?;
            let output: Type = input.parse()?;
            expect_ident(input, "をお返しになり")?;
            input.parse::<Token![,]>()?;
            expect_ident(input, "以下のことをなさいますのよ")?;
            input.parse::<Token![.]>()?;
            let block: Block = input.parse()?;
            Ok(quote! {
                fn #name (#args) -> #output #block
            })
        }
        _ => return unexpected_ident(&ident),
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
    fn test_use() {
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

    #[test]
    fn test_fn() {
        assert_eq! {
            セバスチャン_impl(quote!{
                こちらの f 様は,
                a: i32 と b: &str をお受け取りになって,
                std::io::Result<()> をお返しになり,
                以下のことをなさいますのよ. {
                    writeln!(std::io::stdout(), "a: {}", a)?;
                    writeln!(std::io::stdout(), "b: {}", b)?;
                    Ok(())
                }
            }).to_string(),
            quote!{
                fn f(a: i32, b: &str) -> std::io::Result<()> {
                    writeln!(std::io::stdout(), "a: {}", a)?;
                    writeln!(std::io::stdout(), "b: {}", b)?;
                    Ok(())
                }
            }.to_string()
        };
    }
}
