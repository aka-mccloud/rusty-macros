extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ ItemFn, Visibility, ReturnType, Type, parse, spanned::Spanned };

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error
            ::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = match parse(input) {
        Ok(data) => data,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    if !validate_main_signature(&f) {
        return parse::Error
            ::new(f.span(), "function must have signature `fn() -> !`")
            .to_compile_error()
            .into();
    }

    let ident = &f.sig.ident;
    quote!(
        #[no_mangle]
        pub unsafe extern "C" fn __main() {
            #ident()
        }

        #f
    ).into()
}

fn validate_main_signature(f: &ItemFn) -> bool {
    f.vis == Visibility::Inherited &&
        f.sig.constness.is_none() &&
        f.sig.abi.is_none() &&
        f.sig.inputs.is_empty() &&
        f.sig.generics.params.is_empty() &&
        f.sig.generics.where_clause.is_none() &&
        f.sig.variadic.is_none() &&
        (match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) =>
                match **ty {
                    Type::Never(_) => true,
                    _ => false,
                }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
