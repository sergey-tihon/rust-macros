use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{Expr, ItemFn, ReturnType, Stmt};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).expect("Failed to parse function");
    ast.vis = syn::Visibility::Public(Default::default());

    // Change the return type to Result
    ast.sig.output = signature_output_as_result(&ast);

    // Change return value to Ok
    let last_statement = ast.block.stmts.pop();
    ast.block
        .stmts
        .push(last_statement_as_result(last_statement));

    // Change panic to Err
    ast.block.stmts = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => s,
        })
        .collect();

    ast.to_token_stream().into()
}

fn handle_expression(expression: syn::Expr, token: Option<syn::token::Semi>) -> Stmt {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_context(expr_macro);
                        if output.map(|v| v.is_empty()).unwrap_or(false) {
                            emit_error!(
                                expr_macro,
                                "panic needs a mesage!".to_string();
                                help = "try to add a message: panic!(\"Example\".to_string());";
                                note = "we will add message to Result's Err"
                            );
                            s
                        } else {
                            extract_panic_context(expr_macro)
                                .map(|t| quote! { return Err(#t.to_string()); })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s)
                        }
                    }
                    _ => s,
                })
                .collect();

            ex_if.then_branch.stmts = new_statements;
            Stmt::Expr(Expr::If(ex_if), token)
        }
        _ => Stmt::Expr(expression, token),
    }
}

fn extract_panic_context(expr_macro: &syn::StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));

    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
        Ok(#last_unwrapped)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn signature_output_as_result(ast: &ItemFn) -> ReturnType {
    let output = match ast.sig.output {
        ReturnType::Default => {
            quote! { -> Result<(), String> }
        }
        ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                emit_error!(ty,
                    format!("this macro can only be applied to a function that does not return a Result. Signature: {}", quote!(#ty))
                );
                ast.sig.output.to_token_stream()
            } else {
                quote! { -> Result<#ty, String> }
            }
        }
    };
    syn::parse2(output).unwrap()
}
