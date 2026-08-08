use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::joining;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::IntersectionTypeMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedIntersectionType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;
use java::util::function::Consumer;

pub struct IntersectionType {
	elements: com::github::javaparser::ast::node_list::NodeList,
}

impl IntersectionType {
	pub fn new(elements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		this(null, elements);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, elements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		super(token_range);
		self.set_elements(elements);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_elements(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.elements;
	}

	pub fn set_elements(&mut self, elements: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		com::github::javaparser::utils::utils::Utils::assert_not_null(elements)?;
		if elements == self.elements {
			return self;
		}
		self.notify_property_change(ObservableProperty::ELEMENTS, self.elements, elements);
		if self.elements != null {
			self.elements.set_parent_node(null);
		}
	
		self.elements = elements;
		self.set_as_parent_node_of(elements);
		return self;
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		return super.set_annotations(annotations) as IntersectionType;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.elements.size() {
				{
					if self.elements.get(i) == node {
						self.elements.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.elements.stream().map(Type::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining("&"));
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		return self.accept(CloneVisitor::new(), null) as IntersectionType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::intersection_type_meta_model::IntersectionTypeMetaModel {
		return JavaParserMetaModel::intersectionTypeMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.elements.size() {
				{
					if self.elements.get(i) == node {
						self.elements.set(i, replacement_node as ReferenceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_intersection_type(&self) -> bool {
		return true;
	}

	pub fn as_intersection_type(&self) -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		return self;
	}

	pub fn if_intersection_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_intersection_type::ResolvedIntersectionType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedIntersectionType.class);
	}

	pub fn to_intersection_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return Err(UnsupportedOperationException::new(&self.getClass().getCanonicalName()));
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for IntersectionType {}

impl /* Java */ java::lang::Cloneable /**/ for IntersectionType {}

impl com::github::javaparser::has_parent_node::HasParentNode for IntersectionType {}

impl com::github::javaparser::ast::observer::observable::Observable for IntersectionType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for IntersectionType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for IntersectionType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for IntersectionType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for IntersectionType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for IntersectionType {}