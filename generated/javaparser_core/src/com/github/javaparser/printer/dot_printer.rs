use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::metamodel::NodeMetaModel;
use crate::com::github::javaparser::metamodel::PropertyMetaModel;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::List;

pub struct DotPrinter {
	node_count: i32,
	output_node_type: bool,
}

impl DotPrinter {
	pub fn new(output_node_type: bool) -> com::github::javaparser::printer::dot_printer::DotPrinter {
		self.outputNodeType = output_node_type;
	}

	pub fn output(&mut self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		self.node_count = 0;
		let output: StringBuilder = StringBuilder::new();
		output.append("digraph {");
		self.output(node, null, "root", output);
		output.append(LineSeparator::SYSTEM + "}");
		return output.toString();
	}

	pub fn output(&self, node: &com::github::javaparser::ast::node::Node, parent_node_name: &/* Java */ java::lang::String /**/, name: &/* Java */ java::lang::String /**/, builder: &/* Java */ java::lang::StringBuilder /**/) /* thrown(java.lang.AssertionError | java.lang.NoSuchFieldError | java.lang.RuntimeException) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(node)?;
		let meta_model: NodeMetaModel = node.get_meta_model();
		let all_property_meta_models: List<PropertyMetaModel> = meta_model.get_all_property_meta_models();
		let attributes: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isAttribute).filter(PropertyMetaModel::isSingular).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let sub_nodes: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isNode).filter(PropertyMetaModel::isSingular).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let sub_lists: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isNodeList).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let nd_name: String = self.next_node_name();
		if self.output_node_type {
			builder.append(LineSeparator::SYSTEM + nd_name + " [label=\"" + com::github::javaparser::printer::dot_printer::DotPrinter::escape(name) + " (" + meta_model.get_type_name() + ")\"];");
		}
		else {builder.append(LineSeparator::SYSTEM + nd_name + " [label=\"" + com::github::javaparser::printer::dot_printer::DotPrinter::escape(name) + "\"];");
		}
	
		if parent_node_name != null {
			builder.append(LineSeparator::SYSTEM + parent_node_name + " -> " + nd_name + ";");
		}
	
		for a in attributes {
			let attr_name: String = self.next_node_name();
			builder.append(LineSeparator::SYSTEM + attr_name + " [label=\"" + com::github::javaparser::printer::dot_printer::DotPrinter::escape(&a.get_name()) + "='" + com::github::javaparser::printer::dot_printer::DotPrinter::escape(&a.get_value(node)?.toString()) + "'\"];");
			builder.append(LineSeparator::SYSTEM + nd_name + " -> " + attr_name + ";");
		}
		for sn in sub_nodes {
			let nd: Node = sn.get_value(node)? as Node;
			if nd != null {
				self.output(nd, nd_name, &sn.get_name(), builder)?;
			}
	
		}
		for sl in sub_lists {
			let nl: NodeList<? extends Node> = sl.get_value(node)? as NodeList<? extends Node>;
			if nl != null && nl.is_non_empty() {
				let nd_lst_name: String = self.next_node_name();
				builder.append(LineSeparator::SYSTEM + nd_lst_name + " [label=\"" + com::github::javaparser::printer::dot_printer::DotPrinter::escape(&sl.get_name()) + "\"];");
				builder.append(LineSeparator::SYSTEM + nd_name + " -> " + nd_lst_name + ";");
				let sl_name: String = sl.get_name().substring(0, sl.get_name().length() - 1);
				for nd in nl {
					self.output(nd, nd_lst_name, sl_name, builder)?;
				}
			}
		}
	}

	fn next_node_name(&self) -> /* Java */ java::lang::String /**/ {
		return "n" + (self.node_count += 1 !!!check!!! post increment);
	}

	fn escape(&self, value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return value.replace("\"", "\\\"");
	}
}