use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ImportDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;

pub struct ImportDeclaration {
	name: com::github::javaparser::ast::expr::name::Name,
	is_static: bool,
	is_asterisk: bool,
	is_module: bool,
}

impl ImportDeclaration {
	fn new() -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		this(null, Name::new(), false, false, false);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, is_static: bool, is_asterisk: bool, is_module: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		// create an import declaration of the form ‘x.*’.
		this(null, &com::github::javaparser::ast::import_declaration::ImportDeclaration::get_name_from_string(name), is_static,  if is_asterisk { is_asterisk } else { com::github::javaparser::ast::import_declaration::ImportDeclaration::has_asterisk(name) }, is_module);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, is_static: bool, is_asterisk: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		// create an import declaration of the form ‘x.*’.
		this(null, &com::github::javaparser::ast::import_declaration::ImportDeclaration::get_name_from_string(name), is_static,  if is_asterisk { is_asterisk } else { com::github::javaparser::ast::import_declaration::ImportDeclaration::has_asterisk(name) });
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, is_static: bool, is_asterisk: bool, is_module: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		this(null, name, is_static, is_asterisk, is_module);
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, is_static: bool, is_asterisk: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		this(null, name, is_static, is_asterisk);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, is_static: bool, is_asterisk: bool, is_module: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		super(token_range);
		self.set_name(name);
		self.set_static(is_static);
		self.set_asterisk(is_asterisk);
		self.set_module(is_module);
		self.custom_initialization();
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, is_static: bool, is_asterisk: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		super(token_range);
		self.set_name(name);
		self.set_static(is_static);
		self.set_asterisk(is_asterisk);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	fn is_qualified(&self, name: &/* Java */ java::lang::String /**/) -> bool {
		return name != null & name.indexOf(".") >= 0;
	}

	fn has_asterisk(&self, name: &/* Java */ java::lang::String /**/) -> bool {
		return name != null & name.endsWith("*");
	}

	fn get_name_from_string(&self, mut name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::name::Name {
		if !com::github::javaparser::ast::import_declaration::ImportDeclaration::is_qualified(name) {
			return Name::new(name);
		}
		if com::github::javaparser::ast::import_declaration::ImportDeclaration::has_asterisk(name) {
			name = name.substring(0, name.length() - 2);
		}
		let last_separator: i32 = name.lastIndexOf(".");
		return Name::new(&com::github::javaparser::ast::import_declaration::ImportDeclaration::get_name_from_string(&name.substring(0, last_separator)), &name.substring(last_separator + 1));
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn is_asterisk(&self) -> bool {
		return self.is_asterisk;
	}

	pub fn is_module(&self) -> bool {
		return self.is_module;
	}

	pub fn is_static(&self) -> bool {
		return self.is_static;
	}

	pub fn set_asterisk(&mut self, is_asterisk: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		if is_asterisk == self.isAsterisk {
			return self;
		}
		self.notify_property_change(ObservableProperty::ASTERISK, self.isAsterisk, is_asterisk);
		self.isAsterisk = is_asterisk;
		return self;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
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

	pub fn set_static(&mut self, is_static: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		if is_static == self.isStatic {
			return self;
		}
		self.notify_property_change(ObservableProperty::STATIC, self.isStatic, is_static);
		self.isStatic = is_static;
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		return self.accept(CloneVisitor::new(), null) as ImportDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::import_declaration_meta_model::ImportDeclarationMetaModel {
		return JavaParserMetaModel::importDeclarationMetaModel;
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

	pub fn set_module(&mut self, is_module: bool) -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		if is_module == self.isModule {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODULE, self.isModule, is_module);
		self.isModule = is_module;
		return self;
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ImportDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for ImportDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for ImportDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for ImportDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ImportDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ImportDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ImportDeclaration {}