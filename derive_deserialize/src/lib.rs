extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AccountsDeserialize)]
pub fn accounts_deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let field_names = fields_named.named.iter().map(|f| &f.ident);
                let field_indices = (0..fields_named.named.len()).map(syn::Index::from);

                quote! {
                    impl<'a> AccountsDeserialize<'a> for #name<'a> {
                        fn deserialize(
                            instruction_view: &'a InstructionView
                        ) -> Option<Self>
                        where
                            Self: Sized,
                        {
                            let instr_accounts: Vec<&Vec<u8>> = instruction_view
                                .accounts()
                                .iter()
                                .map(|i| i.0)
                                .collect();

                            Some(Self {
                                #(#field_names: Address(instr_accounts.get(#field_indices)?)),*
                            })
                        }
                    }
                }
            }
            _ => panic!("AccountsDeserialize can only be derived for structs with named fields"),
        },
        _ => panic!("AccountsDeserialize can only be derived for structs"),
    };

    TokenStream::from(expanded)
}
