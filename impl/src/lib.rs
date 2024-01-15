use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

#[proc_macro_derive(FromPgFitRow)]
pub fn derive_from_fit_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("#[derive(FromPgFitRow)] is only for structs!"),
    };

    let struct_name = &input.ident;

    let fields_named = match fields {
        Fields::Named(fields_named) => fields_named,
        _ => panic!("#[derive(FromPgFitRow)] is only for structs with named fields"),
    };

    let fields_from_row = fields_named.named.iter().enumerate().map(|(i, f)| {
        let name = &f.ident;
        quote! {
            #name: row.get(#i)
        }
    });

    quote! {
        impl FromPgFitRow for #struct_name {
            fn from_fit_pg_row(row: Row) -> Self {
                Self {
                    #(#fields_from_row),*
                }
            }
        }
    }.into()
}
