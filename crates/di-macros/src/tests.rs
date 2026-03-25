//! Snapshot tests for derive macro output.
use crate::generate;
use crate::parse::parse_struct;
use quote::quote;
use syn::DeriveInput;

/// Format a token stream as a Rust source file for snapshot comparison.
fn format_tokens(tokens: proc_macro2::TokenStream) -> String {
    let file = syn::parse2::<syn::File>(tokens).expect("tokens should be valid syntax");
    prettyplease::unparse(&file)
}

#[test]
fn sync_single_field() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Database {
            config: Arc<Config>,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_sync(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn sync_multiple_fields() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct AsyncHandler {
            config: Arc<Config>,
            db: Arc<AsyncDatabase>,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_sync(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn async_multiple_fields() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct AsyncHandler {
            config: Arc<Config>,
            db: Arc<AsyncDatabase>,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_async(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn error_on_enum() {
    let input: DeriveInput = syn::parse2(quote! {
        pub enum NotAStruct {
            A,
            B,
        }
    })
    .expect("input should parse");
    let err = parse_struct(&input).expect_err("enum should fail");
    assert_eq!(err.to_string(), "FromServices derive only supports structs");
}

#[test]
fn error_on_tuple_struct() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct TupleStruct(Arc<Config>);
    })
    .expect("input should parse");
    let err = parse_struct(&input).expect_err("tuple struct should fail");
    assert_eq!(
        err.to_string(),
        "FromServices derive only supports structs with named fields"
    );
}

#[test]
fn error_on_generic_struct() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Generic<T> {
            value: Arc<T>,
        }
    })
    .expect("input should parse");
    let err = parse_struct(&input).expect_err("generic struct should fail");
    assert_eq!(
        err.to_string(),
        "FromServices derive does not support generic structs"
    );
}

#[test]
fn sync_unit_struct() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct UnitService;
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_sync(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn async_unit_struct() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct UnitService;
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_async(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn sync_mixed_fields() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Handler {
            db: Arc<Database>,
            #[di(default)]
            retries: u16,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_sync(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn async_mixed_fields() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Handler {
            db: Arc<Database>,
            #[di(default)]
            retries: u16,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_async(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn sync_all_default_fields() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Defaults {
            #[di(default)]
            retries: u16,
            #[di(default)]
            label: String,
        }
    })
    .expect("input should parse");
    let parsed = parse_struct(&input).expect("struct should parse");
    let output = generate::generate_sync(&parsed);
    insta::assert_snapshot!(format_tokens(output));
}

#[test]
fn error_on_unknown_di_attribute() {
    let input: DeriveInput = syn::parse2(quote! {
        pub struct Bad {
            #[di(foo)]
            value: u16,
        }
    })
    .expect("input should parse");
    let err = parse_struct(&input).expect_err("unknown attribute should fail");
    assert_eq!(err.to_string(), "unknown di attribute");
}
