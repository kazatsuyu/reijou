use proc_macro2::TokenStream;

fn セバスチャン_impl(tokens: TokenStream) -> TokenStream {
    tokens
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
}
