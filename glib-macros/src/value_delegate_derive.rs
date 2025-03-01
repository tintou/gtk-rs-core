// Take a look at the license at the top of the repository in the LICENSE file.

use quote::quote;
use syn::{parse::Parse, Token};

use crate::utils::crate_ident_new;

#[derive(Default, Debug, Clone)]
enum DeriveMode {
    From,
    #[default]
    Private,
}

pub struct ValueDelegateInput {
    delegated_ty: syn::Path,
    ident: syn::Ident,
    mode: DeriveMode,
    nullable: bool,
}

enum Arg {
    FromPath(syn::Path),
    Nullable,
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let argname: syn::Ident = input.parse()?;
        if argname == "nullable" {
            Ok(Arg::Nullable)
        } else if argname == "from" {
            let _eq: Token![=] = input.parse()?;
            Ok(Arg::FromPath(input.parse()?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected `nullable` or `from`",
            ))
        }
    }
}

#[derive(Default)]
struct Args {
    nullable: bool,
    from_path: Option<syn::Path>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args = syn::punctuated::Punctuated::<Arg, Token![,]>::parse_terminated(input)?;
        let mut this = Args::default();
        for a in args {
            match a {
                Arg::FromPath(p) => this.from_path = Some(p),
                Arg::Nullable => this.nullable = true,
            }
        }
        Ok(this)
    }
}

impl Parse for ValueDelegateInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: syn::DeriveInput = input.parse()?;
        let args: Option<Args> = if let Some(attr) = derive_input
            .attrs
            .iter()
            .find(|x| x.path.is_ident("value_delegate"))
        {
            let args: Args = attr.parse_args()?;
            Some(args)
        } else {
            None
        };

        let (delegated_ty, mode) =
            if let Some(path) = args.as_ref().and_then(|a| a.from_path.as_ref()) {
                (Some(path.clone()), DeriveMode::From)
            } else {
                let path = match derive_input.data {
                    syn::Data::Struct(s) => match s.fields {
                        syn::Fields::Unnamed(fields) if fields.unnamed.iter().count() == 1 => {
                            fields.unnamed.into_iter().next().and_then(|x| match x.ty {
                                syn::Type::Path(p) => Some(p.path),
                                _ => None,
                            })
                        }
                        _ => None,
                    },
                    _ => None,
                };
                (path, DeriveMode::Private)
            };
        let delegated_ty = delegated_ty.ok_or_else(|| {
            syn::Error::new(
                derive_input.ident.span(),
                "Unless `derive(ValueDelegate)` is used over a newtype with 1 field, \
                the delegated type must be specified using \
                #[value_delegate(from = chosen_type)]",
            )
        })?;

        Ok(ValueDelegateInput {
            delegated_ty,
            ident: derive_input.ident,
            mode,
            nullable: args.map(|a| a.nullable).unwrap_or(false),
        })
    }
}

pub fn impl_value_delegate(input: ValueDelegateInput) -> syn::Result<proc_macro::TokenStream> {
    let ValueDelegateInput {
        delegated_ty,
        ident,
        mode,
        nullable,
        ..
    } = &input;
    let crate_ident = crate_ident_new();

    // this must be called in a context where `this` is defined.
    let delegate_value = match mode {
        DeriveMode::From => quote!(#delegated_ty::from(this)),
        DeriveMode::Private => quote!(this.0),
    };

    let to_value_optional = nullable.then(|| {
        quote! {
            impl #crate_ident::value::ToValueOptional for #ident {
                fn to_value_optional(s: Option<&Self>) -> #crate_ident::value::Value {
                    if let Some(this) = s {
                        Some(&#delegate_value).to_value()
                    } else {
                        #crate_ident::value::ToValueOptional::to_value_optional(None::<&#delegated_ty>)
                    }
                }
            }
        }
    });

    let from_value = match mode {
        DeriveMode::From => quote!(#ident::from(#delegated_ty::from_value(value))),
        DeriveMode::Private => quote!(#ident(#delegated_ty::from_value(value))),
    };

    let res = quote! {
        impl #crate_ident::types::StaticType for #ident {
            fn static_type() -> glib::types::Type {
                #delegated_ty::static_type()
            }
        }
        impl #crate_ident::value::ToValue for #ident {
            fn to_value(&self) -> #crate_ident::value::Value {
                let this = self;
                #delegate_value.to_value()
            }
            fn value_type(&self) -> #crate_ident::types::Type {
                let this = self;
                #delegate_value.value_type()
            }
        }

        #to_value_optional

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #ident {
            type Checker = <#delegated_ty as #crate_ident::value::FromValue<'a>>::Checker;

            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #from_value
            }
        }

        impl #crate_ident::HasParamSpec for #ident {
            type ParamSpec = <#delegated_ty as #crate_ident::HasParamSpec>::ParamSpec;
            type SetValue = Self;
            type BuilderFn = <#delegated_ty as #crate_ident::HasParamSpec>::BuilderFn;

            fn param_spec_builder() -> Self::BuilderFn {
                #delegated_ty::param_spec_builder()
            }
        }
    };
    Ok(res.into())
}
