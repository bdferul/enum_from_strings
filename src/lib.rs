use std::{fmt::Display, fs};

use proc_macro::TokenStream;

#[tokio::main]
#[proc_macro]
pub async fn roles_enum(enum_name: TokenStream) -> TokenStream {
    let strings = match fs::read_to_string("strings.txt") {
        Err(e) => {
            return error("\"strings.txt\" could not be found >> ".to_string() + &e.to_string())
        }
        Ok(x) => x
            .lines()
            .map(|x| x.replace(" ", "_"))
            .collect::<Vec<_>>()
            .join(","),
    };

    format!(
        "#[derive(::std::fmt::Debug)]enum {} {{ {} }}",
        enum_name, strings
    )
    .parse()
    .unwrap()
}

fn error<T: Display>(e: T) -> TokenStream {
    let error = e.to_string();
    quote::quote!(compile_error!(#error)).into()
}
