use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, PathArguments, Type};

const LEAF_TYPES: &[&str] = &[
    "bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    "usize", "isize", "String",
];

fn emit_docs(docs: &[String]) -> Vec<TokenStream2> {
    docs.iter()
        .map(|doc| {
            if doc.is_empty() {
                quote! { out.push_str("#\n"); }
            } else {
                quote! { out.push_str(&format!("#{}\n", #doc)); }
            }
        })
        .collect()
}

#[proc_macro_derive(TomlComment, attributes(toml_comment))]
pub fn derive_toml_comment(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return TokenStream::new();
    };
    let Fields::Named(named) = &data.fields else {
        panic!("TomlComment only supports structs with named fields");
    };

    let struct_docs = extract_docs(&input.attrs);
    let mut render_body: Vec<TokenStream2> = Vec::new();

    let struct_doc_tokens = emit_docs(&struct_docs);
    if !struct_doc_tokens.is_empty() {
        render_body.extend(struct_doc_tokens);
    }

    let mut first_section = true;

    for field in &named.named {
        let field_name = field.ident.as_ref().expect("named field");
        let field_name_str = field_name.to_string();
        let field_docs = extract_docs(&field.attrs);
        let force_inline = has_toml_comment_attr(&field.attrs, "inline");

        if !force_inline && is_map_type(&field.ty) {
            let doc_tokens = emit_docs(&field_docs);
            render_body.push(quote! {
                let map_val = toml::Value::try_from(&self.#field_name).unwrap();
                if let toml::Value::Table(table) = map_val {
                    if !table.is_empty() {
                        #(#doc_tokens)*
                        for (k, v) in &table {
                            out.push_str(&format!("{} = {}\n", k, toml_comment::fmt_value(v)));
                        }
                    }
                }
            });
        } else if !force_inline && is_section_type(&field.ty) {
            let emit_blank = !first_section || !struct_docs.is_empty();
            first_section = false;

            render_body.push(quote! {
                let section = if prefix.is_empty() {
                    #field_name_str.to_string()
                } else {
                    format!("{}.{}", prefix, #field_name_str)
                };
            });

            if emit_blank {
                render_body.push(quote! { out.push('\n'); });
            }

            let doc_tokens = emit_docs(&field_docs);
            render_body.extend(doc_tokens);

            render_body.push(quote! {
                out.push_str(&format!("[{}]\n", section));
                self.#field_name._render(out, &section);
            });
        } else if is_option_type(&field.ty) {
            let doc_tokens = emit_docs(&field_docs);
            render_body.push(quote! {
                if self.#field_name.is_some() {
                    #(#doc_tokens)*
                    let val = toml::Value::try_from(&self.#field_name).unwrap();
                    out.push_str(&format!("{} = {}\n", #field_name_str, toml_comment::fmt_value(&val)));
                }
            });
        } else {
            render_body.extend(emit_docs(&field_docs));
            render_body.push(quote! {
                let val = toml::Value::try_from(&self.#field_name).unwrap();
                out.push_str(&format!("{} = {}\n", #field_name_str, toml_comment::fmt_value(&val)));
            });
        }
    }

    quote! {
        impl toml_comment::TomlComment for #name {
            fn to_commented_toml(&self) -> String {
                let mut out = String::new();
                self._render(&mut out, "");
                out
            }

            fn _render(&self, out: &mut String, prefix: &str) {
                #(#render_body)*
            }
        }
    }
    .into()
}

fn extract_docs(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            let syn::Meta::NameValue(nv) = &attr.meta else {
                return None;
            };
            let syn::Expr::Lit(expr_lit) = &nv.value else {
                return None;
            };
            let syn::Lit::Str(lit) = &expr_lit.lit else {
                return None;
            };
            Some(lit.value())
        })
        .collect()
}

fn has_toml_comment_attr(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("toml_comment")
            && matches!(&attr.meta, syn::Meta::List(list) if list.tokens.to_string().trim() == name)
    })
}

fn is_section_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(seg) = type_path.path.segments.last() else {
        return false;
    };

    if matches!(seg.arguments, PathArguments::AngleBracketed(_)) {
        return false;
    }

    !LEAF_TYPES.contains(&seg.ident.to_string().as_str())
}

fn is_option_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(seg) = type_path.path.segments.last() else {
        return false;
    };
    seg.ident == "Option"
}

fn is_map_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(seg) = type_path.path.segments.last() else {
        return false;
    };
    seg.ident == "HashMap" || seg.ident == "BTreeMap"
}
