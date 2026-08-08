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
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::WildcardTypeMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedWildcard;
use java::util::Optional;
use java::util::function::Consumer;

pub struct WildcardType {
	extended_type: com::github::javaparser::ast::type::reference_type::ReferenceType,
	super_type: com::github::javaparser::ast::type::reference_type::ReferenceType,
}

impl WildcardType {
	pub fn new() -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		this(null, null, null, NodeList<>::new());
	}

	pub fn new(extended_type: &com::github::javaparser::ast::type::reference_type::ReferenceType) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		this(null, extended_type, null, NodeList<>::new());
	}

	pub fn new(extended_type: &com::github::javaparser::ast::type::reference_type::ReferenceType, super_type: &com::github::javaparser::ast::type::reference_type::ReferenceType, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		this(null, extended_type, super_type, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, extended_type: &com::github::javaparser::ast::type::reference_type::ReferenceType, super_type: &com::github::javaparser::ast::type::reference_type::ReferenceType, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		super(token_range, annotations);
		self.set_extended_type(extended_type);
		self.set_super_type(super_type);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_extended_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.extended_type);
	}

	pub fn get_super_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.super_type);
	}

	pub fn set_extended_type(&mut self, extended_type: &com::github::javaparser::ast::type::reference_type::ReferenceType) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		if extended_type == self.extendedType {
			return self;
		}
		self.notify_property_change(ObservableProperty::EXTENDED_TYPE, self.extendedType, extended_type);
		if self.extendedType != null {
			self.extendedType.set_parent_node(null);
		}
	
		self.extendedType = extended_type;
		self.set_as_parent_node_of(extended_type);
		return self;
	}

	pub fn set_super_type(&mut self, super_type: &com::github::javaparser::ast::type::reference_type::ReferenceType) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		if super_type == self.superType {
			return self;
		}
		self.notify_property_change(ObservableProperty::SUPER_TYPE, self.superType, super_type);
		if self.superType != null {
			self.superType.set_parent_node(null);
		}
	
		self.superType = super_type;
		self.set_as_parent_node_of(super_type);
		return self;
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return super.set_annotations(annotations)? as WildcardType;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.extended_type != null {
			if node == self.extended_type {
				self.remove_extended_type();
				return true;
			}
		}
		if self.super_type != null {
			if node == self.super_type {
				self.remove_super_type();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		let str: StringBuilder = StringBuilder::new("?");
		self.get_extended_type().ifPresent(|t|str.append(" extends ").append(&t.as_string()));
		self.get_super_type().ifPresent(|t|str.append(" super ").append(&t.as_string()));
		return str.toString();
	}

	pub fn remove_extended_type(&self) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return self.set_extended_type(null as ReferenceType);
	}

	pub fn remove_super_type(&self) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return self.set_super_type(null as ReferenceType);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return self.accept(CloneVisitor::new(), null) as WildcardType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::wildcard_type_meta_model::WildcardTypeMetaModel {
		return JavaParserMetaModel::wildcardTypeMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.extended_type != null {
			if node == self.extended_type {
				self.set_extended_type(replacement_node as ReferenceType);
				return true;
			}
		}
		if self.super_type != null {
			if node == self.super_type {
				self.set_super_type(replacement_node as ReferenceType);
				return true;
			}
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, extended_type: &com::github::javaparser::ast::type::reference_type::ReferenceType, super_type: &com::github::javaparser::ast::type::reference_type::ReferenceType) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		super(token_range);
		self.set_extended_type(extended_type);
		self.set_super_type(super_type);
		self.custom_initialization();
	}

	pub fn is_wildcard_type(&self) -> bool {
		return true;
	}

	pub fn as_wildcard_type(&self) -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return self;
	}

	pub fn if_wildcard_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedWildcard.class);
	}

	pub fn to_wildcard_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.get_extended_type().isPresent() && !self.get_super_type().isPresent() {
			// removed (ReferenceTypeImpl)
			return ResolvedWildcard::extends_bound(&self.get_extended_type().get().convert_to_usage(context));
		}
		if !self.get_extended_type().isPresent() && self.get_super_type().isPresent() {
			// removed (ReferenceTypeImpl)
			return ResolvedWildcard::super_bound(&self.get_super_type().get().convert_to_usage(context));
		}
		if !self.get_extended_type().isPresent() && !self.get_super_type().isPresent() {
			return ResolvedWildcard::com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard::UNBOUNDED;
		}
		return Err(UnsupportedOperationException::new(&self.to_string()));
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for WildcardType {}

impl /* Java */ java::lang::Cloneable /**/ for WildcardType {}

impl com::github::javaparser::has_parent_node::HasParentNode for WildcardType {}

impl com::github::javaparser::ast::observer::observable::Observable for WildcardType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for WildcardType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for WildcardType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for WildcardType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for WildcardType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for WildcardType {}