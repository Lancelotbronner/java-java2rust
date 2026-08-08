use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ModuleUsesDirectiveMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ModuleUsesDirective {
	name: com::github::javaparser::ast::expr::name::Name,
}

impl ModuleUsesDirective {
	pub fn new() -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		this(null, Name::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		this(null, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		super(token_range);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn set_type(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self as ModuleUsesDirective;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		return self.accept(CloneVisitor::new(), null) as ModuleUsesDirective;
	}

	pub fn is_module_uses_stmt(&self) -> bool {
		return true;
	}

	pub fn as_module_uses_stmt(&self) -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		return self;
	}

	pub fn if_module_uses_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_module_uses_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
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

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as Name)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_module_uses_directive(&self) -> bool {
		return true;
	}

	pub fn as_module_uses_directive(&self) -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		return self;
	}

	pub fn to_module_uses_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_module_uses_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::module_uses_directive_meta_model::ModuleUsesDirectiveMetaModel {
		return JavaParserMetaModel::moduleUsesDirectiveMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ModuleUsesDirective {}

impl /* Java */ java::lang::Cloneable /**/ for ModuleUsesDirective {}

impl com::github::javaparser::has_parent_node::HasParentNode for ModuleUsesDirective {}

impl com::github::javaparser::ast::observer::observable::Observable for ModuleUsesDirective {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ModuleUsesDirective {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ModuleUsesDirective {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ModuleUsesDirective {}