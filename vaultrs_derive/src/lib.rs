#[macro_use]
extern crate synstructure;

extern crate proc_macro;

use std::ops::Deref;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{self, spanned::Spanned};

const MACRO_NAME: &str = "VaultEndpoint";
const ATTR_NAME: &str = "endpoint";
const DATA_ATTR_NAME: &str = "data";

#[derive(Debug)]
struct Error(proc_macro2::TokenStream);

impl Error {
    fn new(span: Span, message: &str) -> Error {
        Error(quote_spanned! { span =>
            compile_error!(#message);
        })
    }

    fn into_tokens(self) -> proc_macro2::TokenStream {
        self.0
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Error {
        Error(e.to_compile_error())
    }
}

#[derive(Default, Debug)]
struct Parameters {
    path: Option<syn::LitStr>,
    method: Option<syn::Expr>,
    result: Option<syn::Type>,
}

fn parse_attr(meta: &syn::Meta) -> Result<Parameters, Error> {
    let mut params = Parameters::default();
    if let syn::Meta::List(l) = meta {
        // Verify the attribute list isn't empty
        if l.nested.is_empty() {
            return Err(Error::new(
                meta.span(),
                format!(
                    "The `{}` attribute must be a list of name/value pairs",
                    ATTR_NAME
                )
                .as_str(),
            ));
        }

        // Collect name/value arguments
        let mut args: Vec<&syn::MetaNameValue> = Vec::new();
        for nm in l.nested.iter() {
            if let syn::NestedMeta::Meta(m) = nm {
                if let syn::Meta::NameValue(nv) = m {
                    args.push(nv);
                } else {
                    return Err(Error::new(
                        m.span(),
                        format!(
                            "The `{}` attribute must only contain name/value pairs",
                            ATTR_NAME
                        )
                        .as_str(),
                    ));
                }
            } else {
                return Err(Error::new(
                    nm.span(),
                    "The `action` attribute must not contain any literals",
                ));
            }
        }

        // Extract arguments
        for arg in args {
            if let syn::Lit::Str(val) = &arg.lit {
                match arg.path.get_ident().unwrap().to_string().as_str() {
                    "path" => {
                        params.path = Some(val.deref().clone());
                    }
                    "method" => {
                        params.method = Some(val.deref().clone().parse().map_err(|_| {
                            Error::new(arg.lit.span(), "Unable to parse value into expression")
                        })?);
                    }
                    "result" => {
                        params.result = Some(val.deref().clone().parse().map_err(|_| {
                            Error::new(arg.lit.span(), "Unable to parse value into expression")
                        })?);
                    }
                    _ => {
                        return Err(Error::new(arg.span(), "Unsupported argument"));
                    }
                }
            } else {
                return Err(Error::new(arg.span(), "Invalid value for argument"));
            }
        }
    } else {
        return Err(Error::new(
            meta.span(),
            format!(
                "The `{}` attribute must be a list of key/value pairs",
                ATTR_NAME
            )
            .as_str(),
        ));
    }
    Ok(params)
}

fn gen_action(path: &syn::LitStr) -> Result<proc_macro2::TokenStream, Error> {
    let re = Regex::new(r"\{(.*?)\}").unwrap();
    let mut fmt_args: Vec<syn::Expr> = Vec::new();
    for cap in re.captures_iter(path.value().as_str()) {
        let expr = syn::parse_str(&cap[1]);
        match expr {
            Ok(ex) => fmt_args.push(ex),
            Err(_) => {
                return Err(Error::new(
                    path.span(),
                    format!("Failed parsing format argument as expression: {}", &cap[1]).as_str(),
                ));
            }
        }
    }
    let path = syn::LitStr::new(
        re.replace_all(path.value().as_str(), "{}")
            .to_string()
            .as_str(),
        Span::call_site(),
    );

    if !fmt_args.is_empty() {
        Ok(quote! {
            format!(#path, #(#fmt_args),*)
        })
    } else {
        Ok(quote! {
            String::from(#path)
        })
    }
}

fn endpoint_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let mut found_attr = false;
    let mut params = Parameters::default();
    for attr in &s.ast().attrs {
        match attr.parse_meta() {
            Ok(meta) => {
                if meta.path().is_ident(ATTR_NAME) {
                    found_attr = true;
                    match parse_attr(&meta) {
                        Ok(p) => {
                            params = p;
                        }
                        Err(e) => return e.into_tokens(),
                    }
                }
            }
            Err(e) => return e.to_compile_error(),
        }
    }

    if !found_attr {
        return Error::new(
            Span::call_site(),
            format!(
                "Must supply the `{}` attribute when deriving `{}`",
                ATTR_NAME, MACRO_NAME
            )
            .as_str(),
        )
        .into_tokens();
    }

    // Find data attribute
    let mut field_name: Option<proc_macro2::TokenStream> = None;
    let mut ty: Option<proc_macro2::TokenStream> = None;
    if let syn::Data::Struct(data) = &s.ast().data {
        for field in data.fields.iter() {
            if &field.ident.clone().unwrap().to_string() == DATA_ATTR_NAME {
                field_name = Some(field.ident.to_token_stream());
                ty = Some(field.ty.to_token_stream());
            } else {
                for attr in field.attrs.iter() {
                    if attr.path.is_ident(DATA_ATTR_NAME) {
                        field_name = Some(field.ident.to_token_stream());
                        ty = Some(field.ty.to_token_stream());
                    }
                }
            }
        }
    }

    let mut data_empty = false;
    let data_type = match ty {
        Some(t) => t,
        None => {
            data_empty = true;
            quote! {EmptyEndpointData}
        }
    };

    let data_fn = match data_empty {
        true => quote! {None},
        false => quote! {Some(&self.#field_name)},
    };

    // Parse arguments
    let path = match params.path {
        Some(p) => p,
        None => {
            return Error::new(Span::call_site(), "Missing required `path` argument").into_tokens()
        }
    };
    let method = match params.method {
        Some(m) => m,
        None => match data_empty {
            true => syn::parse_str("RequestType::GET").unwrap(),
            false => syn::parse_str("RequestType::POST").unwrap(),
        },
    };
    let result = match params.result {
        Some(r) => r,
        None => syn::parse_str("EmptyEndpointResult").unwrap(),
    };

    // Hacky variable substitution
    let action = match gen_action(&path) {
        Ok(a) => a,
        Err(e) => return e.into_tokens(),
    };

    // Generate Endpoint implementation
    s.gen_impl(quote! {
        gen impl Endpoint for @Self {
            type RequestData = #data_type;
            type Response = #result;

            fn action(&self) -> String {
                #action
            }

            fn method(&self) -> RequestType {
                #method
            }

            fn data(&self) -> Option<&Self::RequestData> {
                #data_fn
            }
        }
    })
}

synstructure::decl_derive!([VaultEndpoint, attributes(endpoint, data)] => endpoint_derive);
