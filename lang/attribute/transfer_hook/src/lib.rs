use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn transfer_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        kaptn_lang::solana_program::entrypoint!(__process_instruction);

        pub fn __process_instruction<'info>(
            program_id: &kaptn_lang::solana_program::Pubkey,
            accounts: &[kaptn_lang::solana_program::AccountInfo<'info>],
            instruction_data: &[u8],
        ) -> kaptn_lang::solana_program::ProgramResult {
            process_instruction::<MyExtraMetas<'info>>(program_id, accounts, instruction_data, #fn_name)
        }

        pub fn process_instruction<'info, E: kaptn_lang::context::ExtraMetas<'info>>(
            program_id: &kaptn_lang::solana_program::Pubkey,
            accounts: &[kaptn_lang::solana_program::AccountInfo<'info>],
            instruction_data: &[u8],
            process_transfer: fn(kaptn_lang::context::TransferContext<E>) -> kaptn_lang::solana_program::ProgramResult,
        ) -> kaptn_lang::solana_program::ProgramResult {
            let instruction = kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::unpack(instruction_data)?;

            match instruction {
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::Execute { amount } => {
                    kaptn_lang::solana_program::msg!("Instruction: Execute");
                    kaptn_lang::execute::process_execute(program_id, accounts, amount, process_transfer)
                }
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::InitializeExtraAccountMetaList {
                    extra_account_metas: _,
                } => {
                    kaptn_lang::solana_program::msg!("Instruction: InitializeExtraAccountMetaList");
                    let user_extra_metas = E::to_extra_account_metas();
                    kaptn_lang::initialize::process_initialize_extra_account_meta_list(program_id, accounts, &user_extra_metas)
                }
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::UpdateExtraAccountMetaList {
                    extra_account_metas: _,
                } => {
                    kaptn_lang::solana_program::msg!("Instruction: UpdateExtraAccountMetaList");
                    let user_extra_metas = E::to_extra_account_metas();
                    kaptn_lang::update::process_update_extra_account_meta_list(program_id, accounts, &user_extra_metas)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
