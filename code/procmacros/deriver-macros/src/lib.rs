extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_id = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, ..}) => {
            // The `quote!` macro allows you to write Rust instead of
            // trying to write AST entries yourself. It's a lot easier!
            //
            // We'll build an implementation block first.
            let mut implementation = quote!{
                println!("I am a struct of type [{}]", stringify!(#struct_id));
            };

            // Now we'll iterate over the fields and add a print statement
            // for each one.
            for field in fields {
                let field_name = field.ident.unwrap();
                implementation.extend(quote!{
                    println!("Field: [{}] = [{}]", 
                        stringify!(#field_name),
                        self.#field_name,
                    );
                });
            }

            // Now we'll embed the implementation block into the final
            // output.

            quote! {
                impl #struct_id {
                    fn hello_macro(&self) {
                        #implementation
                    }
                }
            }.into() // The .into() is necessary to convert the output back into a token stream.
        }
        _ => unimplemented!(),
    }
}