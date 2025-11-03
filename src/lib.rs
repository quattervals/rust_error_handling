use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Type, TypePath, parse_macro_input};

/// A derive macro that generates `From` implementations for enum variants
/// marked with the `#[stringfrom]` attribute.
///
/// This macro allows you to automatically convert from other error types
/// to string representations within your error enum variants.
///
/// # Example
///
/// ```rust
/// use error_handling::StringFrom;
///
/// #[derive(Debug, StringFrom)]
/// pub enum ApiError {
///     #[stringfrom(UseCaseError)]
///     UseCaseError(String),
///     BadRequest(String),
/// }
/// ```
///
/// This will generate:
/// ```rust
/// impl From<UseCaseError> for ApiError {
///     fn from(err: UseCaseError) -> Self {
///         ApiError::UseCaseError(err.to_string())
///     }
/// }
/// ```
#[proc_macro_derive(StringFrom, attributes(stringfrom))]
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
        // Look for the #[stringfrom] attribute and extract the type
        if let Some(source_type) = get_stringfrom_type(&variant.attrs)? {
            let variant_name = &variant.ident;

            // Verify this is a single String field
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field = &fields.unnamed[0];
                    match &field.ty {
                        Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
                            // Generate the From implementation
                            let impl_block = quote! {
                                impl From<#source_type> for #enum_name {
                                    fn from(err: #source_type) -> Self {
                                        #enum_name::#variant_name(err.to_string())
                                    }
                                }
                            };
                            impls.push(impl_block);
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                field,
                                "stringfrom can only be used with String fields",
                            ));
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "stringfrom can only be used with single-field tuple variants",
                    ));
                }
            }
        }
    }

    Ok(quote! {
        #(#impls)*
    })
}

fn get_stringfrom_type(attrs: &[Attribute]) -> Result<Option<Type>, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("stringfrom") {
            // Parse the attribute as a meta list: #[stringfrom(Type)]
            if let syn::Meta::List(meta_list) = &attr.meta {
                // Parse the content as a type
                let source_type: Type = meta_list.parse_args()?;
                return Ok(Some(source_type));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stringfrom_macro_parsing() {
        // Test that our macro can parse the stringfrom attribute correctly
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            pub enum TestError {
                #[stringfrom(SomeError)]
                SomeVariant(String),
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
        
        println!("✅ Macro parsing test passed");
    }

    #[test] 
    fn test_stringfrom_attribute_parsing() {
        // Test parsing of the stringfrom attribute by creating a full enum
        let input = quote::quote! {
            #[derive(Debug, StringFrom)]
            pub enum TestError {
                #[stringfrom(MyCustomError)]
                TestVariant(String),
            }
        };
        
        let parsed: DeriveInput = syn::parse2(input).unwrap();
        if let Data::Enum(data_enum) = &parsed.data {
            let variant = &data_enum.variants[0];
            let result = get_stringfrom_type(&variant.attrs);
            
            assert!(result.is_ok());
            let type_opt = result.unwrap();
            assert!(type_opt.is_some());
            
            if let Some(ty) = type_opt {
                let type_str = quote::quote!(#ty).to_string();
                assert_eq!(type_str, "MyCustomError");
            }
        }
        
        println!("✅ Attribute parsing test passed");
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
        assert!(error.to_string().contains("StringFrom can only be derived for enums"));
        
        println!("✅ Error handling test passed");
    }
}
