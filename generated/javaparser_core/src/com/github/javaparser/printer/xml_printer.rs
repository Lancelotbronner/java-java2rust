use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::metamodel::NodeMetaModel;
use crate::com::github::javaparser::metamodel::PropertyMetaModel;
use java::io::StringWriter;
use java::io::Writer;
use java::util::List;
use java::util::function::Predicate;
use javax::xml::stream::XMLOutputFactory;
use javax::xml::stream::XMLStreamException;
use javax::xml::stream::XMLStreamWriter;

pub struct XmlPrinter {
	output_node_type: bool,
}

impl XmlPrinter {
	static TYPE_CLASS: /* Java */ java::lang::Class /**/ = Type.class;

	pub fn new(output_node_type: bool) -> com::github::javaparser::printer::xml_printer::XmlPrinter {
		self.outputNodeType = output_node_type;
	}

	pub fn output(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(com.github.javaparser.printer.RuntimeXMLStreamException) */ -> /* Java */ java::lang::String /**/ {
		return self.string_writer_output(node, "root")?.toString();
	}

	pub fn output(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/, level: i32, builder: &/* Java */ java::lang::StringBuilder /**/) /* thrown(com.github.javaparser.printer.RuntimeXMLStreamException) */ {
		builder.append(&self.string_writer_output(node, name)?.toString());
	}

	pub fn string_writer_output(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.printer.RuntimeXMLStreamException) */ -> /* Java */ java::io::StringWriter /**/ {
		let string_writer: StringWriter = StringWriter::new();
		let r0 = 'try0: {
			self.output_document(node, name, string_writer);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ XMLStreamException) => {
				break 'try0 Err(RuntimeXMLStreamException::new(ex));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return string_writer;
	}

	pub fn output_document(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/, writer: &/* Java */ java::io::Writer /**/) /* thrown(javax.xml.stream.XMLStreamException) */ {
		let output_factory: XMLOutputFactory = XMLOutputFactory::newInstance();
		let xml_writer: XMLStreamWriter = output_factory.createXMLStreamWriter(writer);
		let r0 = 'try0: {
			self.output_document(node, name, xml_writer);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		xml_writer.close();
	
	}

	pub fn output_document(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/, xml_writer: &/* Java */ javax::xml::stream::XMLStreamWriter /**/) /* thrown(javax.xml.stream.XMLStreamException) */ {
		xml_writer.writeStartDocument();
		self.output_node(node, name, xml_writer)?;
		xml_writer.writeEndDocument();
	}

	pub fn output_node(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/, xml_writer: &/* Java */ javax::xml::stream::XMLStreamWriter /**/) /* thrown(java.lang.AssertionError | javax.xml.stream.XMLStreamException) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(node)?;
		com::github::javaparser::utils::utils::Utils::assert_non_empty(name)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(xml_writer)?;
		let meta_model: NodeMetaModel = node.get_meta_model();
		let all_property_meta_models: List<PropertyMetaModel> = meta_model.get_all_property_meta_models();
		let non_null_node: Predicate<PropertyMetaModel> = |property_meta_model|property_meta_model.get_value(node)? != null;
		let non_empty_list: Predicate<PropertyMetaModel> = |property_meta_model|(property_meta_model.get_value(node)? as NodeList).is_non_empty();
		let type_list: Predicate<PropertyMetaModel> = |property_meta_model|self.TYPE_CLASS == property_meta_model.get_type();
		xml_writer.writeStartElement(name);
		// Output node type attribute
		if self.output_node_type {
			xml_writer.writeAttribute("nodeType", &meta_model.get_type_name());
		}
		let r0 = 'try0: {
			// Output attributes
			all_property_meta_models.stream().filter(PropertyMetaModel::isAttribute).filter(PropertyMetaModel::isSingular).forEach(|attribute_meta_model|{
				let r1 = 'try1: {
					/* final */ let attribute_name: String = attribute_meta_model.get_name();
					/* final */ let attribute_value: String = match attribute_meta_model.get_value(node) {
						Err(e) => break 'try1 Err(e),
						Ok(s) => s,
					}.toString();
					xml_writer.writeAttribute(attribute_name, attribute_value);
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ XMLStreamException) => {
						break 'try1 Err(RuntimeXMLStreamException::new(ex));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			});
			// Output singular subNodes
			all_property_meta_models.stream().filter(PropertyMetaModel::isNode).filter(PropertyMetaModel::isSingular).filter(non_null_node).forEach(|sub_node_meta_model|{
				let r1 = 'try1: {
					/* final */ let sub_node: Node = match sub_node_meta_model.get_value(node) {
						Err(e) => break 'try1 Err(e),
						Ok(s) => s,
					} as Node;
					/* final */ let sub_node_name: String = sub_node_meta_model.get_name();
					if let Err(e) = self.output_node(sub_node, sub_node_name, xml_writer) {
						return Err(e);
					};
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ XMLStreamException) => {
						break 'try1 Err(RuntimeXMLStreamException::new(ex));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			});
			// Output list subNodes
			all_property_meta_models.stream().filter(PropertyMetaModel::isNodeList).filter(non_null_node).filter(&non_empty_list.or(type_list)).forEach(|list_meta_model|{
				let r1 = 'try1: {
					let list_name: String = list_meta_model.get_name();
					let singular: String = list_name.substring(0, list_name.length() - 1);
					let node_list: NodeList<? extends Node> = match list_meta_model.get_value(node) {
						Err(e) => break 'try1 Err(e),
						Ok(s) => s,
					} as NodeList;
					xml_writer.writeStartElement(list_name);
					for sub_node in node_list {
						if let Err(e) = self.output_node(sub_node, singular, xml_writer) {
							return Err(e);
						};
					}
					xml_writer.writeEndElement();
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ XMLStreamException) => {
						break 'try1 Err(RuntimeXMLStreamException::new(ex));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			});
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeXMLStreamException) => {
				break 'try0 Err(ex.getxml_stream_cause());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		xml_writer.writeEndElement();
	}

	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(com.github.javaparser.printer.RuntimeXMLStreamException) */ {
		System::out.println(&XmlPrinter::new(true).output(node)?);
	}
}

struct RuntimeXMLStreamException;

impl RuntimeXMLStreamException {
	pub fn new(cause: &/* Java */ javax::xml::stream::XMLStreamException /**/) -> com::github::javaparser::printer::xml_printer::RuntimeXMLStreamException {
		super(cause);
	}

	pub fn getxml_stream_cause(&self) -> /* Java */ javax::xml::stream::XMLStreamException /**/ {
		return super.getCause() as XMLStreamException;
	}
}

impl /* Java */ java::io::Serializable /**/ for RuntimeXMLStreamException {}