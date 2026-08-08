use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ModuleProvidesDirectiveMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ModuleProvidesDirective {
	name: com::github::javaparser::ast::expr::name::Name,
	with: com::github::javaparser::ast::node_list::NodeList,
}

impl ModuleProvidesDirective {
	pub fn new() -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		this(null, Name::new(), NodeList<>::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, with: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		this(null, name, with);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, with: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		super(token_range);
		self.set_name(name);
		self.set_with(with);
		self.custom_initialization();
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
			while i < self.with.size() {
				{
					if self.with.get(i) == node {
						self.with.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		return self.accept(CloneVisitor::new(), null) as ModuleProvidesDirective;
	}

	pub fn is_module_provides_stmt(&self) -> bool {
		return true;
	}

	pub fn as_module_provides_stmt(&self) -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		return self;
	}

	pub fn if_module_provides_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_module_provides_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
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

	pub fn set_with(&mut self, with: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		com::github::javaparser::utils::utils::Utils::assert_not_null(with)?;
		if with == self.with {
			return self;
		}
		self.notify_property_change(ObservableProperty::WITH, self.with, with);
		if self.with != null {
			self.with.set_parent_node(null);
		}
	
		self.with = with;
		self.set_as_parent_node_of(with);
		return self;
	}

	pub fn get_with(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.with;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as Name)?;
			return true;
		}
		 {
			let i: i32 = 0;
			while i < self.with.size() {
				{
					if self.with.get(i) == node {
						self.with.set(i, replacement_node as Name)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_module_provides_directive(&self) -> bool {
		return true;
	}

	pub fn as_module_provides_directive(&self) -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		return self;
	}

	pub fn to_module_provides_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_module_provides_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::module_provides_directive_meta_model::ModuleProvidesDirectiveMetaModel {
		return JavaParserMetaModel::moduleProvidesDirectiveMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ModuleProvidesDirective {}

impl /* Java */ java::lang::Cloneable /**/ for ModuleProvidesDirective {}

impl com::github::javaparser::has_parent_node::HasParentNode for ModuleProvidesDirective {}

impl com::github::javaparser::ast::observer::observable::Observable for ModuleProvidesDirective {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ModuleProvidesDirective {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ModuleProvidesDirective {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ModuleProvidesDirective {}