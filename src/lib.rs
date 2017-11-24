#[macro_use]
extern crate proc_macro_hack;
extern crate syn;
#[macro_use]
extern crate quote;

proc_macro_expr_impl! {
    pub fn js_impl(input: &str) -> String {
        js_impl_impl(input)
    }
}

fn js_impl_impl(input: &str) -> String {
    let tokens = syn::parse_token_trees(input).unwrap();
    let fmt = syn::Token::Literal(syn::Lit::Str(js_format(&tokens), syn::StrStyle::Cooked));
    let mut subst = Vec::new();
    js_subst(&mut subst, &tokens);
    format!("::webby::run_script({})", quote!(&format!(#fmt #(,::webby::IntoJson::into_json(#subst))*)))
}

fn js_format(input: &[syn::TokenTree]) -> String {
    let mut i = 0;
    let mut result = String::new();
    while i < input.len() {
        if let &syn::TokenTree::Token(syn::Token::At) = &input[i] {
            if i + 1 < input.len() {
                result += "{}";
                i += 2;
                continue;
            }
        }
        if let &syn::TokenTree::Delimited(syn::Delimited { delim, ref tts }) = &input[i] {
            result += match delim {
                syn::DelimToken::Bracket => "[",
                syn::DelimToken::Brace => "{{",
                syn::DelimToken::Paren => "(",
            };
            result += &js_format(tts);
            result += match delim {
                syn::DelimToken::Bracket => "]",
                syn::DelimToken::Brace => "}}",
                syn::DelimToken::Paren => ")",
            };
        } else {
            let tt = &input[i];
            result += quote!(#tt).as_str();
        }
        i += 1;
    }
    result
}

fn js_subst(subst: &mut Vec<syn::TokenTree>, input: &[syn::TokenTree]) {
    let mut i = 0;
    while i < input.len() {
        if let &syn::TokenTree::Token(syn::Token::At) = &input[i] {
            if i + 1 < input.len() {
                subst.push(input[i + 1].clone());
                i += 2;
                continue;
            }
        }
        if let &syn::TokenTree::Delimited(syn::Delimited { delim: _, ref tts }) = &input[i] {
            js_subst(subst, tts);
        }
        i += 1;
    }
}