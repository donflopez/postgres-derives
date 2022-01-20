use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};

#[proc_macro_derive(FromRow)]
pub fn from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type we want to implement the trait for
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().enumerate().map(|(i, _)| i);
  
    // println!("{:?}", input.);

    let expanded = quote! {
        impl #name {
            pub fn from_row(row: Row) -> #name {
                #name {
                    #(
                        #field_name: row.get(#field_type),
                    )*
                }
            }
        }
    };
  
    TokenStream::from(expanded)
}
