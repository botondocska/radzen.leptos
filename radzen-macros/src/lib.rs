use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, Pat, Token, parse_macro_input, punctuated::Punctuated};

const BASE_PROP_NAMES: &[&str] = &[
    "style",
    "visible",
    "id",
    "attrs",
    "locale",
    "on_mouse_enter",
    "on_mouse_leave",
    "on_context_menu",
];

#[proc_macro_attribute]
pub fn radzen_component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);

    for attr in &func.attrs {
        if attr.path().is_ident("component") {
            return syn::Error::new_spanned(
                attr,
                "#[radzen_component] already emits #[component] — remove the extra #[component]",
            )
            .to_compile_error()
            .into();
        }
    }

    for arg in func.sig.inputs.iter() {
        if let FnArg::Typed(pt) = arg {
            if let Pat::Ident(pi) = pt.pat.as_ref() {
                let name = pi.ident.to_string();
                if BASE_PROP_NAMES.contains(&name.as_str()) {
                    return syn::Error::new_spanned(
                        &pi.ident,
                        format!(
                            "'{name}' is a reserved base prop injected by #[radzen_component]. \
                             Remove it from the function signature."
                        ),
                    )
                    .to_compile_error()
                    .into();
                }
                if name == "base" {
                    return syn::Error::new_spanned(
                        &pi.ident,
                        "'base: ComponentProps' is no longer needed with #[radzen_component].",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
    }

    let base_params = build_base_params();

    let original_inputs: Vec<FnArg> = func.sig.inputs.into_iter().collect();
    let mut new_inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
    for p in base_params {
        new_inputs.push(p);
    }
    for p in original_inputs {
        new_inputs.push(p);
    }
    func.sig.inputs = new_inputs;

    let prelude = build_prelude();
    let original_stmts = func.block.stmts.clone();
    let new_block: syn::Block = syn::parse2(quote! {
        {
            #prelude
            #(#original_stmts)*
        }
    })
    .expect("radzen_component: failed to build new block");
    func.block = Box::new(new_block);

    let output = quote! {
        #[::leptos::prelude::component]
        #func
    };

    output.into()
}

fn build_base_params() -> Vec<FnArg> {
    let params: Vec<TokenStream> = vec![
        quote! {
            #[prop(default = None, into)]
            style: Option<String>
        },
        quote! {
            #[prop(default = true)]
            visible: bool
        },
        quote! {
            #[prop(default = None, into)]
            id: Option<String>
        },
        quote! {
            #[prop(default = None)]
            attrs: Option<::std::collections::HashMap<String, String>>
        },
        quote! {
            #[prop(default = None, into)]
            locale: Option<String>
        },
        quote! {
            #[prop(default = None)]
            on_mouse_enter: Option<::std::sync::Arc<dyn Fn(::web_sys::MouseEvent) + Send + Sync>>
        },
        quote! {
            #[prop(default = None)]
            on_mouse_leave: Option<::std::sync::Arc<dyn Fn(::web_sys::MouseEvent) + Send + Sync>>
        },
        quote! {
            #[prop(default = None)]
            on_context_menu: Option<::std::sync::Arc<dyn Fn(::web_sys::MouseEvent) + Send + Sync>>
        },
    ];

    params
        .into_iter()
        .map(|ts| syn::parse2::<FnArg>(ts).expect("failed to parse base param"))
        .collect()
}

fn build_prelude() -> TokenStream {
    quote! {
        let __base = crate::components::base_component::ComponentProps {
            style,
            visible: Some(visible),
            id,
            attrs,
            locale,
            on_mouse_enter,
            on_mouse_leave,
            on_context_menu,
        };
        let handle = crate::components::base_component::use_radzen_base(&__base, "");
    }
}
