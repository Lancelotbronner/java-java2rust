use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ReceiverParameterMetaModel;

pub struct ReceiverParameter {
	type: com::github::javaparser::ast::type::type::Type,
	annotations: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::name::Name,
}

impl ReceiverParameter {
	pub fn new() -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		this(null, NodeList<>::new(), ClassOrInterfaceType::new(), Name::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		this(null, NodeList<>::new(), type, name);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		this(null, NodeList<>::new(), type, Name::new(name));
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		this(null, annotations, type, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		super(token_range);
		self.set_annotations(annotations);
		self.set_type(type);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		if self.type != null {
			self.type.set_parent_node(null);
		}
	
		self.type = type;
		self.set_as_parent_node_of(type);
		return self;
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		if annotations == self.annotations {
			return self;
		}
		self.notify_property_change(ObservableProperty::ANNOTATIONS, self.annotations, annotations);
		if self.annotations != null {
			self.annotations.set_parent_node(null);
		}
	
		self.annotations = annotations;
		self.set_as_parent_node_of(annotations);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		return self.accept(CloneVisitor::new(), null) as ReceiverParameter;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::receiver_parameter_meta_model::ReceiverParameterMetaModel {
		return JavaParserMetaModel::receiverParameterMetaModel;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.set(i, replacement_node as AnnotationExpr)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as Name)?;
			return true;
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for ReceiverParameter {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ReceiverParameter {}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ReceiverParameter {}

impl /* Java */ java::lang::Cloneable /**/ for ReceiverParameter {}

impl com::github::javaparser::has_parent_node::HasParentNode for ReceiverParameter {}

impl com::github::javaparser::ast::observer::observable::Observable for ReceiverParameter {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ReceiverParameter {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ReceiverParameter {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ReceiverParameter {}