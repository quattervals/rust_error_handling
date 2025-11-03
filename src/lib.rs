use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Lit, Meta, Type, parse_macro_input};

/// A derive macro that generates `From` implementations for enum variants
/// marked with the `#[stringfrom]` attribute.
///
/// This macro allows you to automatically convert from other error types
/// to string representations within your error enum variants.
///
/// # Example
///
/// ```rust,no_run
/// use error_handling::StringFrom;
///
/// #[derive(Debug)]
/// enum InternalError{};
///
/// #[derive(Debug, StringFrom)]
/// pub enum ApiError {
///     #[stringfrom("use case error")]
///     InternalError(String, #[from] InternalError),
///     BadRequest(String),
/// }
/// ```
///
/// This will generate:
/// ```
/// //impl From<InternalError> for ApiError {
/// //    fn from(err: InternalError) -> Self {
/// //        ApiError::InternalError(format!("use case error: {}", err), err)
/// //    }
/// //}
/// ```
#[proc_macro_derive(StringFrom, attributes(stringfrom, from))]
pub fn string_from_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_string_from_impls(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_string_from_impls(input: &DeriveInput) -> Result<TokenStream2, syn::Error> {
    let enum_name = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "StringFrom can only be derived for enums",
            ));
        }
    };

    let mut impls = Vec::new();

    for variant in &data_enum.variants {
        if let Some(context_str) = get_stringfrom_context(&variant.attrs)? {
            let variant_name = &variant.ident;

            if let Fields::Unnamed(fields) = &variant.fields {
                let (from_field_type, from_field_index) = find_from_field(fields)?;

                let mut field_initializers = Vec::new();
                let format_string = format!("{}: {{}}", context_str);

                for (i, _) in fields.unnamed.iter().enumerate() {
                    if i == from_field_index {
                        field_initializers.push(quote! { err });
                    } else {
                        // Assuming other fields can be constructed from the context string
                        field_initializers.push(quote! { format!(#format_string, err) });
                    }
                }

                let impl_block = quote! {
                    impl From<#from_field_type> for #enum_name {
                        fn from(err: #from_field_type) -> Self {
                            #enum_name::#variant_name(#(#field_initializers),*)
                        }
                    }
                };
                impls.push(impl_block);
            } else {
                return Err(syn::Error::new_spanned(
                    variant,
                    "stringfrom with context can only be used with tuple variants",
                ));
            }
        }
    }

    Ok(quote! {
        #(#impls)*
    })
}

fn get_stringfrom_context(attrs: &[Attribute]) -> Result<Option<String>, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("stringfrom") {
            if let Meta::List(meta_list) = &attr.meta {
                let lit: Lit = meta_list.parse_args()?;
                if let Lit::Str(lit_str) = lit {
                    return Ok(Some(lit_str.value()));
                }
            }
        }
    }
    Ok(None)
}

fn find_from_field(fields: &syn::FieldsUnnamed) -> Result<(&Type, usize), syn::Error> {
    let mut from_fields = Vec::new();
    for (i, field) in fields.unnamed.iter().enumerate() {
        for attr in &field.attrs {
            if attr.path().is_ident("from") {
                from_fields.push((&field.ty, i));
            }
        }
    }

    if from_fields.len() == 1 {
        Ok(from_fields[0])
    } else if from_fields.is_empty() {
        Err(syn::Error::new_spanned(
            fields,
            "A field with the `#[from]` attribute is required.",
        ))
    } else {
        Err(syn::Error::new_spanned(
            fields,
            "Only one field can have the `#[from]` attribute.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::DeriveInput;

    #[test]
    fn test_stringfrom_macro_parsing() {
        // Test that our macro can parse the stringfrom attribute correctly
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            pub enum TestError {
                #[stringfrom("some error")]
                SomeVariant(String, #[from] SomeError),
                RegularVariant(String),
            }
        };

        let parsed: DeriveInput = syn::parse2(input).unwrap();
        let result = generate_string_from_impls(&parsed);

        assert!(result.is_ok());
        let generated = result.unwrap();
        let generated_str = generated.to_string();

        println!("Generated code: {}", generated_str);

        // Verify the generated code contains the expected From implementation
        assert!(generated_str.contains("impl From < SomeError > for TestError"));
        assert!(generated_str.contains("TestError :: SomeVariant"));
    }

    #[test]
    fn test_stringfrom_attribute_parsing() {
        // Test parsing of the stringfrom attribute by creating a full enum
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            pub enum TestError {
                #[stringfrom("my custom error")]
                TestVariant(String, #[from] MyCustomError),
            }
        };

        let parsed: DeriveInput = syn::parse2(input).unwrap();
        if let Data::Enum(data_enum) = &parsed.data {
            let variant = &data_enum.variants[0];
            let result = get_stringfrom_context(&variant.attrs);

            assert!(result.is_ok());
            let context_opt = result.unwrap();
            assert!(context_opt.is_some());

            if let Some(context) = context_opt {
                assert_eq!(context, "my custom error");
            }
        }
    }

    #[test]
    fn test_macro_error_handling() {
        // Test that the macro properly handles invalid input
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            struct NotAnEnum {
                field: String,
            }
        };

        let parsed: DeriveInput = syn::parse2(input).unwrap();
        let result = generate_string_from_impls(&parsed);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error
                .to_string()
                .contains("StringFrom can only be derived for enums")
        );
    }

    #[test]
    fn test_stringfrom_with_from_attribute() {
        // Test that our macro can parse the stringfrom attribute correctly
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            pub enum TestError {
                #[stringfrom("context for the error")]
                MyError(String, #[from] SourceError),
            }
        };

        let parsed: DeriveInput = syn::parse2(input).unwrap();
        let result = generate_string_from_impls(&parsed);

        assert!(result.is_ok());
        let generated = result.unwrap();
        let generated_str = generated.to_string();

        println!("Generated code: {}", generated_str);

        // Verify the generated code contains the expected From implementation
        assert!(generated_str.contains("impl From < SourceError > for TestError"));
        assert!(generated_str.contains(
            "TestError :: MyError (format ! (\"context for the error: {}\" , err) , err)"
        ));
    }
}
