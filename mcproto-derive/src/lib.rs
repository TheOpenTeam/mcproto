/*
 *
 *  * Created: 2026-3-7 2:58:22
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use proc_macro::TokenStream;
use mcproto_utils::ServerboundPacket;
#[proc_macro_derive(ServerboundPacket, attributes(packet))]
pub fn serverbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // 解析 packet id
    let id = input.attrs
        .iter()
        .find(|attr| attr.path().is_ident("packet"))
        .and_then(|attr| {
            let mut id = None;
            let _: syn::Result<()> = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value = meta.value()?;
                    let lit: syn::LitInt = value.parse()?;
                    id = Some(lit.base10_parse::<i32>()?);
                }
                Ok(())
            });
            id
        });
    if let syn::Data::Struct(data) = &input.data {
        let struct_name = &input.ident;
        let names: Vec<_> = data.fields.iter().map(|f| &f.ident).collect();
        let types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();

        let expanded = quote::quote! {
            impl ServerboundPacket for #struct_name {
                fn packet_id(&self) -> i32 {
                    #id
                }
                fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), mcproto_utils::CodecError> {
                    #(
                        <#types as mcproto_utils::PacketCodec>::encode(&self.#names, buf)?;
                    )*
                    Ok(())
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        syn::Error::new_spanned(&input.ident, "ServerboundPacket can only be derived for structs").to_compile_error().into()
    }
}
#[proc_macro_derive(ClientboundPacket, attributes(packet))]
pub fn clientbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // 解析 packet id
    let id = input.attrs
        .iter()
        .find(|attr| attr.path().is_ident("packet"))
        .and_then(|attr| {
            let mut id = None;
            let _: syn::Result<()> = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value = meta.value()?;
                    let lit: syn::LitInt = value.parse()?;
                    id = Some(lit.base10_parse::<i32>()?);
                }
                Ok(())
            });
            id
        });
    if let syn::Data::Struct(data) = &input.data {
        let struct_name = &input.ident;
        let names: Vec<_> = data.fields.iter().map(|f| &f.ident).collect();
        let types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();

        let expanded = quote::quote! {
            impl ServerboundPacket for #struct_name {
                fn packet_id(&self) -> i32 {
                    #id
                }
                fn decode(buf: &mut impl std::io::Read) -> Result<Self, mcproto_utils::CodecError> {
                    Ok(Self {
                        #(
                            #names: <#types as mcproto_utils::PacketCodec>::decode(buf)?,
                        )*
                    })
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        syn::Error::new_spanned(&input.ident, "ServerboundPacket can only be derived for structs").to_compile_error().into()
    }
}