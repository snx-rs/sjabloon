use quote::ToTokens;
use rstml::{
    node::{self, CustomNode, NodeAttribute},
    visitor::{visit_attributes, visit_nodes, Visitor},
};
use syn::{spanned::Spanned, visit_mut::VisitMut};

use crate::VOID_ELEMENTS;

/// Represents the internal visitor state.
#[derive(Default)]
pub struct WalkNodes {
    pub string: String,
    pub values: Vec<proc_macro2::TokenStream>,
    pub diagnostics: Vec<proc_macro2::TokenStream>,
}

impl WalkNodes {
    /// Extends this WalkNodes with data from the given WalkNodes.
    fn extend(&mut self, other: WalkNodes) {
        self.string.push_str(&other.string);
        self.values.extend(other.values);
    }
}

impl VisitMut for WalkNodes {}

impl<C> Visitor<C> for WalkNodes
where
    C: node::CustomNode + 'static,
{
    fn visit_block(&mut self, block: &mut node::NodeBlock) -> bool {
        self.string.push_str("{}");
        self.values.push(block.to_token_stream());

        false
    }

    fn visit_doctype(&mut self, doctype: &mut node::NodeDoctype) -> bool {
        self.string.push_str(&format!(
            "<!doctype {}>",
            doctype.value.to_token_stream_string()
        ));

        false
    }

    fn visit_raw_node<AnyC: CustomNode>(&mut self, node: &mut node::RawText<AnyC>) -> bool {
        self.string.push_str(&node.to_string_best());

        false
    }

    fn visit_text_node(&mut self, text: &mut node::NodeText) -> bool {
        self.string.push_str(&text.value_string());

        false
    }

    fn visit_element(&mut self, element: &mut node::NodeElement<C>) -> bool {
        let name = element.name().to_string();

        self.string.push_str(&format!("<{}", name));
        let visitor = WalkNodes::default();
        let attributes = visit_attributes(element.attributes_mut(), visitor);
        self.extend(attributes);

        if VOID_ELEMENTS.contains(&name.as_str()) {
            self.string.push_str("/>");

            if !element.children.is_empty() {
                self.diagnostics
                    .push(proc_macro2_diagnostics::Diagnostic::spanned(
                        element.span(),
                        proc_macro2_diagnostics::Level::Warning,
                        "This is a void element and cannot have any children",
                    ).emit_as_expr_tokens());
            }

            return false;
        } else {
            self.string.push('>');
        }

        let visitor = WalkNodes::default();
        let children = visit_nodes(&mut element.children, visitor);
        self.extend(children);
        self.string.push_str(&format!("</{}>", name));

        false
    }

    fn visit_fragment(&mut self, fragment: &mut node::NodeFragment<C>) -> bool {
        let visitor = WalkNodes::default();
        let children = visit_nodes(&mut fragment.children, visitor);
        self.extend(children);

        false
    }

    fn visit_attribute(&mut self, attribute: &mut node::NodeAttribute) -> bool {
        match attribute {
            NodeAttribute::Block(block) => {
                self.string.push_str(" {}");
                self.values.push(block.to_token_stream());
            }
            NodeAttribute::Attribute(attribute) => {
                self.string.push_str(&format!(" {}", attribute.key));
                if let Some(value) = attribute.value() {
                    self.string.push_str(r#"="{}""#);
                    self.values.push(value.to_token_stream());
                }
            }
        }

        false
    }
}
