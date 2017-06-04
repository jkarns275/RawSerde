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
    match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref data)) => {
            impl_raw_serialize_struct(data.clone(), ast)
        },
        syn::Body::Struct(syn::VariantData::Tuple(ref data)) => {
            impl_raw_serialize_tuple(data.clone(), ast)
        },
        _ => {
            panic!("Enums are not yet supported");
        }
    }
}

fn impl_raw_serialize_struct(data: Vec<syn::Field>, ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    for x in data.iter() {
        use syn::Ty::*;
        match x.ty {
            Rptr(_, _)  => panic!("Reference types are not supported ({:?} is a reference type)", x),
            Ptr(_)      => panic!("Pointer types are not supported ({:?} is a reference type)", x),
            BareFn(_)   => panic!("Functions cannot be serialized ({:?} is a bare function type)", x),
            _ => {}
        };
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let data_ty = data.into_iter().map(|x| x.ident.unwrap()).collect::<Vec<syn::Ident>>();
    let ret = quote! {
        impl #impl_generics RawSerialize #ty_generics for #name #where_clause {
            fn raw_serialize(&self, to: &mut std::io::Write) -> Result<u64, std::io::Error> {
                let mut size = 0;
                #( let size_x; check!(self.#data_ty.raw_serialize(to), size_x); size += size_x; )*
                Ok(size)
            }
        }
    };
    println!("sestr: {:?}", ret);
    ret
}

fn impl_raw_serialize_tuple(data: Vec<syn::Field>, ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    for x in data.iter() {
        use syn::Ty::*;
        match x.ty {
            Rptr(_, _)  => panic!("Reference types are not supported ({:?} is a reference type)", x),
            Ptr(_)      => panic!("Pointer types are not supported ({:?} is a reference type)", x),
            BareFn(_)   => panic!("Functions cannot be serialized ({:?} is a bare function type)", x),
            _ => {}
        };
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let x = (0..data.len()).map(|x| syn::Ident::new(x.to_string())).collect::<Vec<syn::Ident>>();

    let ret = quote! {
        impl #impl_generics RawSerialize #ty_generics for #name #where_clause {
            fn raw_serialize(&self, to: &mut std::io::Write) -> Result<u64, std::io::Error> {
                let mut size = 0;
                #( let size_x; check!(self.#x.raw_serialize(to), size_x); size += size_x; )*
                Ok(size)
            }
        }
    };
    println!("setup: {:?}", ret);
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
    match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref data)) => {
            impl_raw_deserialize_struct(data.clone(), ast)
        },
        syn::Body::Struct(syn::VariantData::Tuple(ref data)) => {
            impl_raw_deserialize_tuple(data.clone(), ast)
        },
        _ => {
            panic!("Enums are not yet supported");
        }
    }
}

fn impl_raw_deserialize_struct(data: Vec<syn::Field>, ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut data_fields: Vec<syn::Ident> = vec![];
    let mut data_ty: Vec<syn::Ty> = vec![];

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

fn impl_raw_deserialize_tuple(data: Vec<syn::Field>, ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut data_ty: Vec<syn::Ty> = vec![];

    for x in data.iter() {
        use syn::Ty::*;
        match x.ty {
            Rptr(_, _)  => panic!("Reference types are not supported ({:?} is a reference type)", x),
            Ptr(_)      => panic!("Pointer types are not supported ({:?} is a reference type)", x),
            BareFn(_)   => panic!("Functions cannot be serialized ({:?} is a bare function type)", x),
            _ => {}
        };
        data_ty.push(x.ty.clone());
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let data_fields = (0..data.len()).map(|x| syn::Ident::new("__field_".to_string() + x.to_string().as_str())).collect::<Vec<syn::Ident>>();
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
                Ok(#name(
                    #(#dclone_3),*
                ))
            }
        }
    };
    println!("detup: {:?}", ret);
    ret
}
