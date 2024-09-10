use crate::TransferHookInput;
use proc_macro2::TokenStream;
use syn::{parse2, ItemFn};

pub fn parse_transfer_hook_input(item: TokenStream) -> TransferHookInput {
    let item_fn = parse2::<ItemFn>(item).expect("Failed to parse function");
    let fn_name = item_fn.sig.ident.clone();
    TransferHookInput { fn_name, item_fn }
}
