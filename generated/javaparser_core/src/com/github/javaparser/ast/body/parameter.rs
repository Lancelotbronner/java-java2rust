use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ParameterMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedParameterDeclaration;

pub struct Parameter {
	type: com::github::javaparser::ast::type::type::Type,
	is_var_args: bool,
	var_args_annotations: com::github::javaparser::ast::node_list::NodeList,
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	annotations: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl Parameter {
	pub fn new() -> com::github::javaparser::ast::body::parameter::Parameter {
		this(null, NodeList<>::new(), NodeList<>::new(), ClassOrInterfaceType::new(), false, NodeList<>::new(), SimpleName::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::body::parameter::Parameter {
		this(null, NodeList<>::new(), NodeList<>::new(), type, false, NodeList<>::new(), name);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::parameter::Parameter {
		this(null, NodeList<>::new(), NodeList<>::new(), type, false, NodeList<>::new(), SimpleName::new(name));
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::body::parameter::Parameter {
		this(null, modifiers, NodeList<>::new(), type, false, NodeList<>::new(), name);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, is_var_args: bool, var_args_annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::body::parameter::Parameter {
		this(null, modifiers, annotations, type, is_var_args, var_args_annotations, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, is_var_args: bool, var_args_annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::body::parameter::Parameter {
		super(token_range);
		self.set_modifiers(modifiers);
		self.set_annotations(annotations);
		self.set_type(type);
		self.set_var_args(is_var_args);
		self.set_var_args_annotations(var_args_annotations);
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

	pub fn is_var_args(&self) -> bool {
		return self.is_var_args;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
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

	pub fn set_var_args(&mut self, is_var_args: bool) -> com::github::javaparser::ast::body::parameter::Parameter {
		if is_var_args == self.isVarArgs {
			return self;
		}
		self.notify_property_change(ObservableProperty::VAR_ARGS, self.isVarArgs, is_var_args);
		self.isVarArgs = is_var_args;
		return self;
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
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

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
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

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
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
	
		 {
			let i: i32 = 0;
			while i < self.var_args_annotations.size() {
				{
					if self.var_args_annotations.get(i) == node {
						self.var_args_annotations.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn get_var_args_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.var_args_annotations;
	}

	pub fn set_var_args_annotations(&mut self, var_args_annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
		com::github::javaparser::utils::utils::Utils::assert_not_null(var_args_annotations)?;
		if var_args_annotations == self.varArgsAnnotations {
			return self;
		}
		self.notify_property_change(ObservableProperty::VAR_ARGS_ANNOTATIONS, self.varArgsAnnotations, var_args_annotations);
		if self.varArgsAnnotations != null {
			self.varArgsAnnotations.set_parent_node(null);
		}
	
		self.varArgsAnnotations = var_args_annotations;
		self.set_as_parent_node_of(var_args_annotations);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::parameter::Parameter {
		return self.accept(CloneVisitor::new(), null) as Parameter;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::parameter_meta_model::ParameterMetaModel {
		return JavaParserMetaModel::parameterMetaModel;
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
		 {
			let i: i32 = 0;
			while i < self.var_args_annotations.size() {
				{
					if self.var_args_annotations.get(i) == node {
						self.var_args_annotations.set(i, replacement_node as AnnotationExpr)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_parameter_declaration::ResolvedParameterDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedParameterDeclaration.class);
	}

	pub fn is_final(&self) -> bool {
		// RecordDeclaration-specific code
		if self.get_parent_node().isPresent() {
			let parent_node: Node = self.get_parent_node().get();
			if parent_node instanceof RecordDeclaration {
				return true;
			}
		}
		// Otherwise use the default implementation.
		return NodeWithFinalModifier.super.is_final();
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for Parameter {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for Parameter {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for Parameter {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for Parameter {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for Parameter {}

impl com::github::javaparser::resolution::resolvable::Resolvable for Parameter {}

impl /* Java */ java::lang::Cloneable /**/ for Parameter {}

impl com::github::javaparser::has_parent_node::HasParentNode for Parameter {}

impl com::github::javaparser::ast::observer::observable::Observable for Parameter {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Parameter {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Parameter {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Parameter {}