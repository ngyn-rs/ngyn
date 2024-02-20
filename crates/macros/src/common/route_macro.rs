use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Receiver, Signature};

pub fn route_macro(_args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        block,
        vis,
        attrs,
    } = match syn::parse::<ItemFn>(raw_input) {
        Ok(input) => input,
        Err(err) => {
            let error_message = err.to_string();
            panic!("failed to parse method: {}", error_message)
        }
    };

    let Signature {
        ident,
        inputs,
        output,
        fn_token,
        ..
    } = sig;

    let transducers: Vec<_> = inputs
		.iter()
		.map(|input| {
			if let syn::FnArg::Typed(pat) = input {
				let ty = &pat.ty;
				let pat = &pat.pat;
				if let syn::Type::Path(path) = *ty.clone() {
					let path = &path.path;
					quote! {
						let mut #pat: #path = ngyn::prelude::Transducer::reduce::<#path>(request, response);
					}
				} else {
					panic!("Expected a valid struct");
				}
			} else {
				quote! {}
			}
		})
		.collect();

    // initial self varn obtained from the first input
    let self_var = match inputs.iter().next() {
        Some(syn::FnArg::Receiver(receiver)) => {
            let Receiver {
                reference,
                mutability,
                self_token,
                ..
            } = receiver;
            if reference.is_some() {
                if mutability.is_some() {
                    quote! { &mut #self_token }
                } else {
                    quote! { &#self_token }
                }
            } else {
                quote! { #self_token }
            }
        }
        _ => quote! {},
    };

    let return_val = match output {
        syn::ReturnType::Type(_, _) => quote! {},
        _ => quote! { return ngyn::prelude::NgynBody::None; },
    };

    let output = match output {
        syn::ReturnType::Type(_, ty) => quote! { -> #ty },
        _ => quote! { -> ngyn::prelude::NgynBody },
    };

    let expanded = quote! {
        #(#attrs)*
        #vis async #fn_token #ident(#self_var, request: &mut ngyn::prelude::NgynRequest, response: &mut ngyn::prelude::NgynResponse) #output {
            #(#transducers)*
            #block
            #return_val
        }
    };

    expanded.into()
}
