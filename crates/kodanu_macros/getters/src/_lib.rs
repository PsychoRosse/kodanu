
use syn::{Data, DeriveInput, Field, Fields, Ident, Index, LitStr, Result, parse_macro_input};

use {
    proc_macro::TokenStream as TokenStream1, proc_macro2::TokenStream as TokenStream2, quote::quote,
};

use std::collections::HashSet;

#[proc_macro_derive(Getters, attributes(getter))]
pub fn derive_getters(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_getters(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_getters(input: DeriveInput) -> Result<TokenStream2> {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Getters can only be derived for structs",
            ));
        }
    };

    let mut methods = Vec::new();
    let mut method_names = HashSet::new();

    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let field_name = field
                    .ident
                    .as_ref()
                    .ok_or_else(|| syn::Error::new_spanned(field, "expected a named field"))?;

                let (method_name, method) =
                    expand_getter(field, quote!(#field_name), field_name.clone())?;

                if !method_names.insert(method_name.to_string()) {
                    return Err(syn::Error::new_spanned(
                        method_name,
                        "duplicate getter method name",
                    ));
                }

                methods.push(method);
            }
        }

        Fields::Unnamed(fields) => {
            for (index, field) in fields.unnamed.iter().enumerate() {
                let field_index = Index::from(index);
                let default_name =
                    Ident::new(&format!("field_{index}"), proc_macro2::Span::call_site());

                let (method_name, method) =
                    expand_getter(field, quote!(#field_index), default_name)?;

                if !method_names.insert(method_name.to_string()) {
                    return Err(syn::Error::new_spanned(
                        method_name,
                        "duplicate getter method name",
                    ));
                }

                methods.push(method);
            }
        }

        Fields::Unit => {}
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#methods)*
        }
    })
}

fn expand_getter(
    field: &Field,
    access: TokenStream2,
    mut method_name: Ident,
) -> Result<(Ident, TokenStream2)> {
    let field_type = &field.ty;

    let mut returns_copy = false;
    let mut returns_str = false;
    let mut must_use = false;
    let mut name_set = false;
    let mut docs = Vec::new();

    for attr in &field.attrs {
        if attr.path().is_ident("doc") {
            docs.push(attr);
            continue;
        }

        if !attr.path().is_ident("getter") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("copy") {
                if returns_copy {
                    return Err(meta.error("duplicate `copy` option"));
                }

                returns_copy = true;
                Ok(())
            } else if meta.path.is_ident("str") {
                if returns_str {
                    return Err(meta.error("duplicate `str` option"));
                }

                returns_str = true;
                Ok(())
            } else if meta.path.is_ident("must_use") {
                if must_use {
                    return Err(meta.error("duplicate `must_use` option"));
                }

                must_use = true;
                Ok(())
            } else if meta.path.is_ident("name") {
                if name_set {
                    return Err(meta.error("duplicate `name` option"));
                }

                let value: LitStr = meta.value()?.parse()?;
                method_name = value.parse()?;
                name_set = true;
                Ok(())
            } else {
                Err(meta.error("expected `copy`, `str`, `must_use`, or `name = \"method\"`"))
            }
        })?;
    }

    if returns_copy && returns_str {
        return Err(syn::Error::new_spanned(
            field,
            "`copy` and `str` cannot be used together",
        ));
    }

    let must_use_attr = if must_use {
        quote! { #[must_use] }
    } else {
        quote! {}
    };

    let method = if returns_str {
        quote! {
            #(#docs)*
            #[inline]
            #must_use_attr
            pub fn #method_name(&self) -> &str {
                &self.#access
            }
        }
    } else if returns_copy {
        quote! {
            #(#docs)*
            #[inline]
            #must_use_attr
            pub const fn #method_name(&self) -> #field_type {
                self.#access
            }
        }
    } else {
        quote! {
            #(#docs)*
            #[inline]
            #must_use_attr
            pub const fn #method_name(&self) -> &#field_type {
                &self.#access
            }
        }
    };

    Ok((method_name, method))
}
