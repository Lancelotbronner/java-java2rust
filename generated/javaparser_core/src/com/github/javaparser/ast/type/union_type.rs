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
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::UnionTypeMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedUnionType;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::stream::Collectors;

pub struct UnionType {
	elements: com::github::javaparser::ast::node_list::NodeList,
}

impl UnionType {
	pub fn new() -> com::github::javaparser::ast::type::union_type::UnionType {
		this(null, NodeList<>::new());
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, elements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::union_type::UnionType {
		super(token_range);
		self.set_elements(elements);
		self.custom_initialization();
	}

	pub fn new(elements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::union_type::UnionType {
		this(null, elements);
	}

	pub fn get_elements(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.elements;
	}

	pub fn set_elements(&mut self, elements: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::union_type::UnionType {
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

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::union_type::UnionType {
		return super.set_annotations(annotations)? as UnionType;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
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
		return self.elements.stream().map(Type::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining("|"));
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::union_type::UnionType {
		return self.accept(CloneVisitor::new(), null) as UnionType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::union_type_meta_model::UnionTypeMetaModel {
		return JavaParserMetaModel::unionTypeMetaModel;
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
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_union_type(&self) -> bool {
		return true;
	}

	pub fn as_union_type(&self) -> com::github::javaparser::ast::type::union_type::UnionType {
		return self;
	}

	pub fn if_union_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_union_type::ResolvedUnionType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedUnionType.class);
	}

	pub fn to_union_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let resolved_elements: List<ResolvedType> = self.get_elements().stream().map(|el|el.convert_to_usage(context)).collect(&Collectors::toList());
		return ResolvedUnionType::new(resolved_elements);
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for UnionType {}

impl /* Java */ java::lang::Cloneable /**/ for UnionType {}

impl com::github::javaparser::has_parent_node::HasParentNode for UnionType {}

impl com::github::javaparser::ast::observer::observable::Observable for UnionType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for UnionType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for UnionType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for UnionType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for UnionType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for UnionType {}