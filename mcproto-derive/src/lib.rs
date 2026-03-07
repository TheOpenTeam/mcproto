/*
 *
 *  * Created: 2026-3-7 2:58:22
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */

use proc_macro::{Ident, TokenStream};

// serverbound packet derive
#[proc_macro_derive(ServerboundPacket, attributes(packet))]
pub fn serverbound_packet_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput); // 转
    // 第一步，先把包id拿到（普通属性）
    let id = input.attrs.iter()
        .find(|attr| attr.path().is_ident("packet"))
        .and_then(|attr| {
            let mut id = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    id = Some(meta.value()?.parse::<syn::LitInt>()?.base10_parse::<i32>().unwrap());
                }
                Ok(())
            });
            id
        });

    if let syn::Data::Struct(data) = &input.data { // 包只能是结构体
        let struct_name = &input.ident;
        let names: Vec<_> = data.fields.iter().map(|f| &f.ident).collect();
        let types: Vec<_> = data.fields.iter().map(|f| &f.ty).collect();
        let expanded = quote::quote! {
                impl ServerboundPacket for #struct_name {
                    fn packet_id() -> i32 {
                        #id
                    }
                    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::Error> {
                        #(
                            self.#names.encode(buf)?;
                        )*
                        Ok(())
                    }
                    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::Error> {
                        #(
                            self.#names.decode(buf)?;
                        )*
                        Ok(())
                    }
                }
            };
        TokenStream::from(expanded)
    } else {
        panic!("Packet must be a struct"); // 实际上宏都是我自己调用的，因为他不是features而是硬编码的依赖，unwrap和panic应该是安全的吧（qwq
    }

}