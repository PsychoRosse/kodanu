//! Derive macros for the Kodanu game engine.
//!
//! This crate provides [`Getters`], which generates read-only accessor
//! methods for struct fields.
//!
//! # Features
//!
//! - Generates public getter methods for named and tuple struct fields.
//! - Supports returning references, copying values, and returning `&str`.
//! - Supports custom getter names and `#[must_use]`.
//! - Preserves field documentation on generated methods.
//! - Supports generic structs and their `where` clauses.
//!
//! # Example
//!
//! ```
//! use kodanu_macros::Getters;
//!
//! #[derive(Getters)]
//! struct WindowAttributes {
//!     /// Window title.
//!     #[getter(str)]
//!     title: String,
//!
//!     /// Whether the window is maximized.
//!     #[getter(copy, must_use)]
//!     maximized: bool,
//!
//!     /// Internal identifier.
//!     #[getter(name = "raw_id")]
//!     id: usize,
//! }
//!
//! let attributes = WindowAttributes {
//!     title: "Kodanu".to_owned(),
//!     maximized: false,
//!     id: 42,
//! };
//!
//! assert_eq!(attributes.title(), "Kodanu");
//! assert!(!attributes.maximized());
//! assert_eq!(attributes.raw_id(), &42);
//! ```
//!
//! The derive macro is available as [`kodanu_macros::Getters`] and can also
//! be imported through `kodanu_macros::prelude`, if that module is enabled
//! by the crate.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

use syn::{Data, DeriveInput, Field, Fields, Ident, Index, LitStr, Result, parse_macro_input};

use {
    proc_macro::TokenStream as TokenStream1, proc_macro2::TokenStream as TokenStream2, quote::quote,
};

use std::collections::HashSet;

/// Derives read-only accessor methods for the fields of a struct.
///
/// A public method is generated for each field. By default, the method has
/// the same name as the field, takes `&self`, and returns a reference to the
/// field. The generated method is `const`.
///
/// For tuple structs, methods are named `field_0`, `field_1`, and so on.
/// Unit structs produce no getter methods.
///
/// Documentation attributes (`#[doc = "..."]`) on fields are copied to the
/// corresponding generated methods.
///
/// # Field attributes
///
/// The `#[getter(...)]` attribute configures the generated getter. Multiple
/// options can be combined in a single attribute.
///
/// ## `copy`
///
/// Returns the field by value instead of by reference.
///
/// The field type must implement [`Copy`]. This requirement is checked by
/// the Rust compiler when it type-checks the generated method.
///
/// The generated method remains `const`.
///
/// ```
/// use kodanu_macros::Getters;
///
/// #[derive(Getters)]
/// struct Entity {
///     #[getter(copy)]
///     id: u64,
/// }
///
/// let entity = Entity { id: 7 };
/// assert_eq!(entity.id(), 7);
/// ```
///
/// ## `str`
///
/// Returns the field as `&str` instead of returning a reference to its
/// declared type.
///
/// The field type must support coercion to `&str`. This works for `String`
/// and `&str`, for example. The generated method is not `const`.
///
/// This option cannot be combined with `copy`.
///
/// ```
/// use kodanu_macros::Getters;
///
/// #[derive(Getters)]
/// struct Window {
///     #[getter(str)]
///     title: String,
/// }
///
/// let window = Window {
///     title: "Kodanu".to_owned(),
/// };
///
/// assert_eq!(window.title(), "Kodanu");
/// ```
///
/// ## `must_use`
///
/// Adds `#[must_use]` to the generated getter, warning when its return value
/// is ignored.
///
/// ```
/// use kodanu_macros::Getters;
///
/// #[derive(Getters)]
/// struct Config {
///     #[getter(copy, must_use)]
///     enabled: bool,
/// }
///
/// let config = Config { enabled: true };
/// assert!(config.enabled());
/// ```
///
/// ## `name = "method"`
///
/// Changes the generated method name without changing the field name.
///
/// The value must be a valid Rust identifier.
///
/// ```
/// use kodanu_macros::Getters;
///
/// #[derive(Getters)]
/// struct Config {
///     #[getter(name = "raw_id")]
///     id: usize,
/// }
///
/// let config = Config { id: 42 };
/// assert_eq!(config.raw_id(), &42);
/// ```
///
/// # Generics
///
/// Generic parameters and `where` clauses are preserved in the generated
/// implementation.
///
/// ```
/// use kodanu_macros::Getters;
///
/// #[derive(Getters)]
/// struct Container<T>
/// where
///     T: Clone,
/// {
///     value: T,
/// }
///
/// let container = Container { value: 42_u32 };
/// assert_eq!(*container.value(), 42);
/// ```
///
/// # Errors
///
/// Compilation fails when:
///
/// - The derive is applied to an enum or union.
/// - An option is repeated on the same field.
/// - `copy` and `str` are used together.
/// - Two fields generate getters with the same method name.
/// - A generated getter is incompatible with its field type, such as using
///   `copy` on a type that does not implement `Copy`.
///
/// # Limitations
///
/// - Getters are always public.
/// - Getters borrow through `&self`; mutable getters are not generated.
/// - The macro does not check trait bounds such as `Copy` itself. The Rust
///   compiler checks the generated code.
/// - A custom method name can conflict with an existing inherent method.
///   Such conflicts are reported by the Rust compiler.
///
/// [`Copy`]: std::marker::Copy
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
