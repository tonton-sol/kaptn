use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

mod codegen;
mod parse;

pub use parse::parse_transfer_hook_input;

pub struct TransferHookInput {
    pub fn_name: syn::Ident,
    pub item_fn: ItemFn,
}

pub fn generate_transfer_hook_code(input: TransferHookInput) -> TokenStream {
    generate(&input)
}

fn generate(program: &TransferHookInput) -> proc_macro2::TokenStream {
    let entry = codegen::entry::generate(program);
    let processor = codegen::processor::generate(program);
    let user_defined_function = &program.item_fn;

    quote! {
        #user_defined_function
        #entry
        #processor
    }
}
