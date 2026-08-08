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
use crate::com::github::javaparser::metamodel::ModuleOpensDirectiveMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ModuleOpensDirective {
	name: com::github::javaparser::ast::expr::name::Name,
	module_names: com::github::javaparser::ast::node_list::NodeList,
}

impl ModuleOpensDirective {
	pub fn new() -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		this(null, Name::new(), NodeList<>::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, module_names: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		this(null, name, module_names);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, module_names: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		super(token_range);
		self.set_name(name);
		self.set_module_names(module_names);
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
			while i < self.module_names.size() {
				{
					if self.module_names.get(i) == node {
						self.module_names.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
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

	pub fn get_module_names(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.module_names;
	}

	pub fn set_module_names(&mut self, module_names: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		com::github::javaparser::utils::utils::Utils::assert_not_null(module_names)?;
		if module_names == self.moduleNames {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODULE_NAMES, self.moduleNames, module_names);
		if self.moduleNames != null {
			self.moduleNames.set_parent_node(null);
		}
	
		self.moduleNames = module_names;
		self.set_as_parent_node_of(module_names);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		return self.accept(CloneVisitor::new(), null) as ModuleOpensDirective;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.module_names.size() {
				{
					if self.module_names.get(i) == node {
						self.module_names.set(i, replacement_node as Name)?;
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
		return super.replace(node, replacement_node);
	}

	pub fn is_module_opens_stmt(&self) -> bool {
		return true;
	}

	pub fn as_module_opens_stmt(&self) -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		return self;
	}

	pub fn if_module_opens_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_module_opens_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_module_opens_directive(&self) -> bool {
		return true;
	}

	pub fn as_module_opens_directive(&self) -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		return self;
	}

	pub fn to_module_opens_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_module_opens_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::module_opens_directive_meta_model::ModuleOpensDirectiveMetaModel {
		return JavaParserMetaModel::moduleOpensDirectiveMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ModuleOpensDirective {}

impl /* Java */ java::lang::Cloneable /**/ for ModuleOpensDirective {}

impl com::github::javaparser::has_parent_node::HasParentNode for ModuleOpensDirective {}

impl com::github::javaparser::ast::observer::observable::Observable for ModuleOpensDirective {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ModuleOpensDirective {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ModuleOpensDirective {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ModuleOpensDirective {}