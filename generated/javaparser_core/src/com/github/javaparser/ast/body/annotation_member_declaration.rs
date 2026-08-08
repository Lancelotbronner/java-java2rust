use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithJavadoc;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAbstractModifier;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithPublicModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::AnnotationMemberDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedAnnotationMemberDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct AnnotationMemberDeclaration {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	type: com::github::javaparser::ast::type::type::Type,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	default_value: com::github::javaparser::ast::expr::expression::Expression,
}

impl AnnotationMemberDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), ClassOrInterfaceType::new(), SimpleName::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/, default_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		this(null, modifiers, NodeList<>::new(), type, SimpleName::new(name), default_value);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, default_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		this(null, modifiers, annotations, type, name, default_value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, default_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		super(token_range, annotations);
		self.set_modifiers(modifiers);
		self.set_type(type);
		self.set_name(name);
		self.set_default_value(default_value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_default_value(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.default_value);
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn remove_default_value(&self) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		return self.set_default_value(null as Expression);
	}

	pub fn set_default_value(&mut self, default_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		if default_value == self.defaultValue {
			return self;
		}
		self.notify_property_change(ObservableProperty::DEFAULT_VALUE, self.defaultValue, default_value);
		if self.defaultValue != null {
			self.defaultValue.set_parent_node(null);
		}
	
		self.defaultValue = default_value;
		self.set_as_parent_node_of(default_value);
		return self;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
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

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
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

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.default_value != null {
			if node == self.default_value {
				self.remove_default_value();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		return self.accept(CloneVisitor::new(), null) as AnnotationMemberDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::annotation_member_declaration_meta_model::AnnotationMemberDeclarationMetaModel {
		return JavaParserMetaModel::annotationMemberDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.default_value != null {
			if node == self.default_value {
				self.set_default_value(replacement_node as Expression);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.set(i, replacement_node as Modifier)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_annotation_member_declaration(&self) -> bool {
		return true;
	}

	pub fn as_annotation_member_declaration(&self) -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		return self;
	}

	pub fn if_annotation_member_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_annotation_member_declaration::ResolvedAnnotationMemberDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedAnnotationMemberDeclaration.class);
	}

	pub fn to_annotation_member_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationMemberDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for AnnotationMemberDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for AnnotationMemberDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for AnnotationMemberDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for AnnotationMemberDeclaration {}