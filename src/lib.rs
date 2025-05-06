mod visitor;

use proc_macro::TokenStream;
use quote::quote;
use rstml::{visitor::visit_nodes, Parser, ParserConfig};
use visitor::WalkNodes;

/// Elements which should not contain any children and should not have an end
/// tag.
///
/// See: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
const VOID_ELEMENTS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// Defines a template using JSX-like syntax.
#[proc_macro]
pub fn template(tokens: TokenStream) -> TokenStream {
    let config = ParserConfig::new()
        .always_self_closed_elements(VOID_ELEMENTS.into())
        .raw_text_elements(["script", "style"].into());

    let (nodes, errors) = Parser::new(config).parse_recoverable(tokens).split();

    let WalkNodes {
        string,
        values,
        diagnostics,
    } = visit_nodes(&mut nodes.unwrap(), WalkNodes::default());

    let errors = errors
        .into_iter()
        .map(|e| e.emit_as_expr_tokens())
        .chain(diagnostics);

    quote! {
        #(#errors;)*
        format!(#string, #(#values),*)
    }
    .into()
}
