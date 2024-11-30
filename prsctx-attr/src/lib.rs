// Copyright (C) 2024 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};


#[proc_macro_attribute]
pub fn mark(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.sig.ident;
    let new_stmt = quote! {
        prsctx::mark!(stringify!(#ident));
    };
    let new_stmt: TokenStream = new_stmt.into();
    let new_stmt = parse_macro_input!(new_stmt as Stmt);

    ast.block.stmts.insert(0, new_stmt);

    let gen = quote! {
        #ast
    };

    gen.into()
}
