extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(FromLine, attributes(from_line))]
pub fn from_line_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_assignments = fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let attrs = field.attrs;
        let field_type = field.ty;

        let from_line_attr = attrs.iter().find(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|segment| segment.ident.to_string().eq("from_line"))
        });

        let (start, end) = match from_line_attr {
            Some(attr) => {
                let tokens = attr
                    .meta
                    .require_list()
                    .expect("Missing range, example: #[from_line(0..6)]")
                    .tokens
                    .to_string();
                let range: Vec<&str> = tokens.split("..").collect();

                if range.len() != 2 {
                    panic!("Please provide a correct #[from_line(X..Y)] attribute")
                }

                let start = range[0]
                    .trim()
                    .parse::<usize>()
                    .expect("Invalid range start");

                let end = range[1].trim().parse::<usize>().expect("Invalid range end");
                (start, end)
            }
            None => panic!("Missing #[from_line(X..Y)] attribute"),
        };

        let error_message = format!(
            "Cannot convert String to {}, at {} field. Implement FromString trait",
            field_type.to_token_stream(),
            field_name
        );

        quote! {
            #field_name: {
                let end = if #end > line.len() { line.len() } else { #end };
                <#field_type as from_line::traits::from_string::FromString>::from_string(
                    &line[#start..end].trim()
                )
                .expect(#error_message)
            }
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn from_line(line: &str) -> Self {
                Self {
                    #(#field_assignments),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
