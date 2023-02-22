use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn binary_op(input: TokenStream) -> TokenStream {
    let op = input.to_string();
    let op = op.trim_matches('"');
    let op = op.trim_matches('\'');
    let op = op.trim_matches('`');
    let op = op.trim_matches(' ');

    let op = match op {
        "+" => quote! { + },
        "-" => quote! { - },
        "*" => quote! { * },
        "/" => quote! { / },
        _ => panic!("Invalid operator"),
    };

    let output = quote! {
        let b = self.pop();
        let a = self.pop();
        self.push(a #op b);
    };

    output.into()
}
