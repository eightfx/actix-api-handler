use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ReturnType, Pat};

#[proc_macro_attribute]
pub fn post_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as Item);

    let fn_item = if let Item::Fn(item) = ast {
        item
    } else {
        panic!("The attribute should be applied to functions.");
    };

    let fn_name = &fn_item.sig.ident;
    let api_fn_name = Ident::new(&format!("{}_api", fn_name), fn_name.span());

    let args = &fn_item.sig.inputs;

    let return_type = match &fn_item.sig.output {
        ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(path) => &path.path.segments[0].ident,
            _ => panic!("The function must return a Result."),
        },
        _ => panic!("The function must return a Result."),
    };

    let mut request_type = None;
    let mut new_args = Vec::new();
    let mut call_args = Vec::new();

    for arg in args.iter() {
        if let syn::FnArg::Typed(pat_type) = arg {
            let arg_name = pat_type.pat.clone();
            let arg_type = &*pat_type.ty;
            if let Pat::Ident(ref pat_ident) = *arg_name {
                if pat_ident.ident == "data" {
                    request_type = Some(arg_type);
                    new_args.push(quote! {form: web::Json<#request_type>});
                    call_args.push(quote! {form.into_inner()});
                } else {
                    new_args.push(quote! {#arg_name: #arg_type});
                    call_args.push(quote! {#arg_name});
				}
            }
        }
    }

    let output = quote! {
        #fn_item

        pub async fn #api_fn_name(#(#new_args),*) -> impl actix_web::Responder {
            match #fn_name(#(#call_args),*).await {
                Ok(d) => {
                    let res = ApiResponse { message: "".to_string(), data: d };
                    actix_web::HttpResponse::Ok().json(res)
                }
                Err(e) => { e.error_response() }
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn get_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as Item);

    let fn_item = if let Item::Fn(item) = ast {
        item
    } else {
        panic!("The attribute should be applied to functions.");
    };

    let fn_name = &fn_item.sig.ident;
    let api_fn_name = Ident::new(&format!("{}_api", fn_name), fn_name.span());

    let args = &fn_item.sig.inputs;

    let return_type = match &fn_item.sig.output {
        ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(path) => &path.path.segments[0].ident,
            _ => panic!("The function must return a Result."),
        },
        _ => panic!("The function must return a Result."),
    };

    let mut request_type = None;
    let mut new_args = Vec::new();
    let mut call_args = Vec::new();

    for arg in args.iter() {
        if let syn::FnArg::Typed(pat_type) = arg {
            let arg_name = pat_type.pat.clone();
            let arg_type = &*pat_type.ty;
            if let Pat::Ident(ref pat_ident) = *arg_name {
                if pat_ident.ident == "data" {
                    request_type = Some(arg_type);
                    new_args.push(quote! {web::Query(info): web::Query<#request_type>});
                    call_args.push(quote! {info});
                } else {
                    new_args.push(quote! {#arg_name: #arg_type});
                    call_args.push(quote! {#arg_name});
				}
            }
        }
    }

    let output = quote! {
        #fn_item

        pub async fn #api_fn_name(#(#new_args),*) -> impl actix_web::Responder {
            match #fn_name(#(#call_args),*).await {
                Ok(d) => {
                    let res = ApiResponse { message: "".to_string(), data: d };
                    actix_web::HttpResponse::Ok().json(res)
                }
                Err(e) => { e.error_response() }
            }
        }
    };

    output.into()
}
