extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(RawSerialize)]
pub fn raw_serialize_gen(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_raw_serialize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_raw_serialize(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut data_fields: Vec<syn::Ident> = vec![];
    match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref data)) => {
            for x in data {
                use syn::Ty::*;
                match x.ty {
                    Rptr(_, _)  => panic!("Reference types are not supported ({:?} is a reference type)", x),
                    Ptr(_)      => panic!("Pointer types are not supported ({:?} is a reference type)", x),
                    BareFn(_)   => panic!("Functions cannot be serialized ({:?} is a bare function type)", x),
                    _ => {}
                };
                data_fields.push(x.ident.clone().unwrap());
            }
        },
        _ => {
            panic!("Tuple Structs not yet supported");
        }
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ret = quote! {
        impl #impl_generics RawSerialize #ty_generics for #name #where_clause {
            fn raw_serialize(&self, to: &mut std::io::Write) -> Result<(), std::io::Error> {
                #( check!(self.#data_fields.raw_serialize(to));  )*
                Ok(())
            }
        }
    };
    ret
}

#[proc_macro_derive(RawDeserialize)]
pub fn raw_deserialize_gen(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_raw_deserialize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_raw_deserialize(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut data_fields: Vec<syn::Ident> = vec![];
    let mut data_ty: Vec<syn::Ty> = vec![];
    match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref data)) => {
            for x in data {
                use syn::Ty::*;
                match x.ty {
                    Rptr(_, _)  => panic!("Reference types are not supported ({:?} is a reference type)", x),
                    Ptr(_)      => panic!("Pointer types are not supported ({:?} is a reference type)", x),
                    BareFn(_)   => panic!("Functions cannot be serialized ({:?} is a bare function type)", x),
                    _ => {}
                };
                data_fields.push(x.ident.clone().unwrap());
                data_ty.push(x.ty.clone());
            }
        },
        _ => {
            panic!("Tuple Structs not yet supported");
        }
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let dclone_1 = data_fields.clone();
    let dclone_2 = data_fields.clone();
    let dclone_3 = data_fields.clone();
    let ret = quote! {
        impl #impl_generics RawDeserialize #ty_generics for #name #where_clause {
            fn raw_deserialize(from: &mut std::io::Read) -> Result<#name, std::io::Error> {
                #(
                    let #dclone_1;
                    check!(#data_ty::raw_deserialize(from), #dclone_2);
                )*
                Ok(#name {
                    #(#dclone_3: #data_fields),*
                })
            }
        }
    };
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
