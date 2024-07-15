extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Attribute, Data, DeriveInput, Meta};

use quote::quote;

#[proc_macro_derive(Model, attributes(belongs_to, has_many))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs = handle_model_attrs(&input, &input.attrs);

    match input.data {
        Data::Struct(ref data) => {
            let ident = input.ident.clone();
            let from_row_fields = data.fields.iter().map(|field| {
                let ident = field.ident.clone();
                quote! {
                    #ident: row.get(stringify!(#ident)),
                }
            });
            let has_id = data.fields.iter().any(|field| field.ident.clone().unwrap() == "id");

            let id = if has_id {
                quote! {
                    fn id(&self) -> i64 {
                        self.id
                    }
                }
            } else {
                quote!{}
            };

            let column_names = data.fields.iter().map(|field| {
                let ident = field.ident.clone();

                quote! {
                    String::from(stringify!(#ident)),
                }
            });

            let values = data.fields
                .iter()
                .filter(|field| field.ident.clone().unwrap() != "id")
                .map(|field| {
                let ident = field.ident.clone();

                quote! {
                    self.#ident.to_value(),
                }

            });

            let singular = snake_case(&ident.to_string());
            let foreign_key = format!("{}_id", singular);

            let table_name = pluralizer::pluralize(singular.as_str(), 2, false);

            quote! {
                #[automatically_derived]
                impl rum::model::FromRow for #ident {
                    fn from_row(row: rum::tokio_postgres::Row) -> Self {
                        Self {
                            #(#from_row_fields)*
                        }
                    }
                }

                #[automatically_derived]
                impl rum::model::Model for #ident {
                    fn table_name() -> String {
                        #table_name.to_string()
                    }

                    fn foreign_key() -> String {
                        #foreign_key.to_string()
                    }

                    fn column_names() -> Vec<String> {
                        vec![
                            #(#column_names)*
                        ]
                    }

                    fn values(&self) -> Vec<rum::model::Value> {
                        use rum::model::ToValue;
                        vec![
                            #(#values)*
                        ]
                    }

                    #id
                }

                #attrs
            }
            .into()
        }

        _ => panic!("macro can only be used on structs"),
    }
}

fn handle_model_attrs(input: &DeriveInput, attributes: &[Attribute]) -> proc_macro2::TokenStream {
    let ident = match &input.data {
        Data::Struct(_data) => input.ident.clone(),

        _ => panic!("macro can only be used on structs"),
    };

    let rels = attributes
        .iter()
        .filter(|attr| {
            ["belongs_to", "has_many"].contains(
                &attr
                    .meta
                    .path()
                    .segments
                    .first()
                    .expect("segment")
                    .ident
                    .to_string()
                    .as_str(),
            )
        })
        .map(|attr| match &attr.meta {
            Meta::List(list) => {
                let path = list.path.segments.first().expect("segment");

                let association = if path.ident == "belongs_to" {
                    quote! {
                        rum::model::AssociationType::BelongsTo
                    }
                } else if path.ident == "has_many" {
                    quote! {
                        rum::model::AssociationType::HasMany
                    }
                } else {
                    panic!("unsupported association: {}", path.ident);
                };

                let associations = list.tokens.clone().into_iter().map(|token| {
                    quote! {
                        #[automatically_derived]
                        impl rum::model::Association<#token> for #ident {
                            fn association_type() -> rum::model::AssociationType {
                                #association
                            }
                        }
                    }
                });

                quote! {
                    #(#associations)*
                }
            }

            _ => panic!("macro can only be used on structs"),
        });

    quote! {
        #(#rels)*
    }
}

#[proc_macro_derive(FromRow)]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(ref data) => {
            let ident = input.ident;

            let from_row_fields = data.fields.iter().map(|field| {
                let ident = &field.ident;
                quote! {
                    #ident: row.get(stringify!(#ident)),
                }
            });

            quote! {
                #[automatically_derived]
                impl rum::model::FromRow for #ident {
                    fn from_row(row: rum::tokio_postgres::Row) -> Self {
                        Self {
                            #(#from_row_fields)*
                        }
                    }
                }
            }
            .into()
        }

        _ => panic!("macro can only be used on structs"),
    }
}

fn snake_case(string: &str) -> String {
    let mut result = "".to_string();

    for (i, c) in string.chars().enumerate() {
        if c.is_ascii_uppercase() && i != 0 {
            result.push('_');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}