use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::Printer;

pub struct DefaultLexicalPreservingPrinter;

impl DefaultLexicalPreservingPrinter {
	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		let visitor: LexicalPreservingVisitor = LexicalPreservingVisitor::new();
		/* final */ let node_text: NodeText = LexicalPreservingPrinter::get_or_create_node_text(node);
		node_text.get_elements().forEach(|element|element.accept(visitor));
		return visitor.to_string();
	}
}

impl com::github::javaparser::printer::printer::Printer for DefaultLexicalPreservingPrinter {}