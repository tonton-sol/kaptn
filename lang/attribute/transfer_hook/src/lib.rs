use kaptn_syn::{generate_transfer_hook_code, parse_transfer_hook_input};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn transfer_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_transfer_hook_input(item.into());
    TokenStream::from(generate_transfer_hook_code(input_fn))
}
