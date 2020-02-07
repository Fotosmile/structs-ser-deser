extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(StructsSer)]
pub fn structs_ser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let gen = match input.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            syn::Fields::Named(ref named_fields) => {
                let names_iter = named_fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                let names_iter_clone = names_iter.clone();

                gen_ser(&struct_name, names_iter, names_iter_clone)
            }
            syn::Fields::Unnamed(ref unnamed_fields) => {
                let fields_iter = 0..unnamed_fields.unnamed.len();
                let fields_iter_clone = fields_iter.clone();

                gen_ser(&struct_name, fields_iter, fields_iter_clone)
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    gen.into()
}

#[proc_macro_derive(StructsDeser)]
pub fn structs_deser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let gen = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref named_fields) => {
                let names_iter = named_fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                let types_iter = named_fields.named.iter().map(|field| &field.ty);

                gen_deser_named(&struct_name, names_iter, types_iter)
            }
            Fields::Unnamed(ref unnamed_fields) => {
                let deser_fields = unnamed_fields.unnamed.iter().map(|field| &field.ty);

                gen_deser_unnamed(&struct_name, deser_fields)
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    gen.into()
}

fn gen_ser<I1, I2>(
    struct_name: &impl ToTokens,
    names_iter: I1,
    names_iter2: I2,
) -> proc_macro2::TokenStream
where
    I1: Iterator,
    I2: Iterator,
    I1::Item: ToTokens,
    I2::Item: ToTokens,
{
    quote! {
        impl Ser for #struct_name {
            fn ser(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
                #(
                    self.#names_iter.ser(&mut buf)?;
                )*

                Ok(())
            }
            fn ser_len(&self) -> usize {
                let mut len = 0;
                #(
                    len += self.#names_iter2.ser_len();
                )*

                len
            }
        }
    }
}

fn gen_deser_named<N, T>(
    struct_name: &impl ToTokens,
    names_iter: N,
    types_iter: T,
) -> proc_macro2::TokenStream
where
    N: Iterator,
    N::Item: ToTokens,
    T: Iterator,
    T::Item: ToTokens,
{
    quote! {
        impl Deser for #struct_name {
            fn deser(mut buf: impl std::io::Read) -> std::io::Result<Self>
            where
                Self: std::marker::Sized
            {
                Ok(Self {
                    #(
                        #names_iter : #types_iter::deser(&mut buf)?,
                    )*
                })
            }
        }
    }
}

fn gen_deser_unnamed<T>(struct_name: &impl ToTokens, types_iter: T) -> proc_macro2::TokenStream
where
    T: Iterator,
    T::Item: ToTokens,
{
    quote! {
        impl Deser for #struct_name {
            fn deser(mut buf: impl std::io::Read) -> std::io::Result<Self> {
                Ok(Self(
                    #(
                        #types_iter::deser(&mut buf)?,
                    )*
                ))
            }
        }
    }
}
