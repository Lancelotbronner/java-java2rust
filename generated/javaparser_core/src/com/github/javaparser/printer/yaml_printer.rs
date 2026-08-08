use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::metamodel::NodeMetaModel;
use crate::com::github::javaparser::metamodel::PropertyMetaModel;
use java::util::List;

pub struct YamlPrinter {
	output_node_type: bool,
}

impl YamlPrinter {
	static NUM_SPACES_FOR_INDENT: i32 = 4;

	pub fn new(output_node_type: bool) -> com::github::javaparser::printer::yaml_printer::YamlPrinter {
		self.outputNodeType = output_node_type;
	}

	pub fn output(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		let output: StringBuilder = StringBuilder::new();
		output.append("---");
		self.output(node, "root", 0, output);
		output.append(System::lineSeparator() + "...");
		return output.toString();
	}

	pub fn output(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/, level: i32, builder: &/* Java */ java::lang::StringBuilder /**/) /* thrown(java.lang.NoSuchFieldError | java.lang.AssertionError | java.lang.RuntimeException) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(node)?;
		let meta_model: NodeMetaModel = node.get_meta_model();
		let all_property_meta_models: List<PropertyMetaModel> = meta_model.get_all_property_meta_models();
		let attributes: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isAttribute).filter(PropertyMetaModel::isSingular).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let sub_nodes: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isNode).filter(PropertyMetaModel::isSingular).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let sub_lists: List<PropertyMetaModel> = all_property_meta_models.stream().filter(PropertyMetaModel::isNodeList).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		if self.output_node_type {
			builder.append(System::lineSeparator() + self.indent(level) + name + "(Type=" + meta_model.get_type_name() + "): ");
		}
		else {builder.append(System::lineSeparator() + self.indent(level) + name + ": ");
		}
	
		level += 1;
		for a in attributes {
			builder.append(System::lineSeparator() + self.indent(level) + a.get_name() + ": " + self.escape_value(&a.get_value(node)?.toString()));
		}
		for sn in sub_nodes {
			let nd: Node = sn.get_value(node)? as Node;
			if nd != null {
				self.output(nd, &sn.get_name(), level, builder)?;
			}
	
		}
		for sl in sub_lists {
			let nl: NodeList<? extends Node> = sl.get_value(node)? as NodeList<? extends Node>;
			if nl != null && nl.is_non_empty() {
				builder.append(System::lineSeparator() + self.indent(level) + sl.get_name() + ": ");
				let sl_name: String = sl.get_name();
				sl_name =  if sl_name.endsWith("s") { sl_name.substring(0, sl.get_name().length() - 1) } else { sl_name };
				for nd in nl {
					self.output(nd, "- " + sl_name, level + 1, builder)?;
				}
			}
		}
	}

	fn indent(&self, level: i32) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = 0;
			while i < level {
				{
					 {
						let j: i32 = 0;
						while j < self.NUM_SPACES_FOR_INDENT {
							{
								sb.append(" ");
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	fn escape_value(&self, value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return "\"" + value.replace("\\", "\\\\").replaceAll("\"", "\\\\\"").replace("\n", "\\n").replace("\r", "\\r").replace("\f", "\\f").replace("\b", "\\b").replace("\t", "\\t") + "\"";
	}

	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) {
		System::out.println(&YamlPrinter::new(true).output(node));
	}
}