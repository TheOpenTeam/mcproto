/*
 *
 *  * Created: 2026-3-7 2:58:22
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use proc_macro::TokenStream;

fn codec_root() -> proc_macro2::TokenStream {
    match proc_macro_crate::crate_name("mcproto") {
        Ok(proc_macro_crate::FoundCrate::Itself) => quote::quote!(crate::utils),
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote::quote!(::#ident::utils)
        }
        Err(_) => match proc_macro_crate::crate_name("mcproto-utils") {
            Ok(proc_macro_crate::FoundCrate::Itself) => quote::quote!(crate),
            Ok(proc_macro_crate::FoundCrate::Name(name)) => {
                let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                quote::quote!(::#ident)
            }
            Err(_) => quote::quote!(::mcproto_utils),
        },
    }
}

#[proc_macro_derive(ServerboundPacket, attributes(packet))]
pub fn serverbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let codec_root = codec_root();
    let packet_trait = quote::quote!(#codec_root::ServerboundPacketTrait);
    let codec_error = quote::quote!(#codec_root::CodecError);
    let packet_codec = quote::quote!(#codec_root::PacketCodec);

    // 解析 packet id
    let id = input
        .attrs
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
            impl #packet_trait for #struct_name {
                fn packet_id(&self) -> i32 {
                    #id
                }
                fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), #codec_error> {
                    #(
                        <#types as #packet_codec>::encode(&self.#names, buf)?;
                    )*
                    Ok(())
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        syn::Error::new_spanned(
            &input.ident,
            "ServerboundPacket can only be derived for structs",
        )
        .to_compile_error()
        .into()
    }
}
#[proc_macro_derive(ClientboundPacket, attributes(packet))]
pub fn clientbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let codec_root = codec_root();
    let packet_trait = quote::quote!(#codec_root::ClientboundPacketTrait);
    let codec_error = quote::quote!(#codec_root::CodecError);
    let packet_codec = quote::quote!(#codec_root::PacketCodec);

    // 解析 packet id
    let id = input
        .attrs
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
            impl #packet_trait for #struct_name {
                fn packet_id(&self) -> i32 {
                    #id
                }
                fn decode(buf: &mut impl std::io::Read) -> Result<Self, #codec_error> {
                    Ok(Self {
                        #(
                            #names: <#types as #packet_codec>::decode(buf)?,
                        )*
                    })
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        syn::Error::new_spanned(
            &input.ident,
            "ServerboundPacket can only be derived for structs",
        )
        .to_compile_error()
        .into()
    }
}
