use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LocalClassDeclarationStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LocalClassDeclarationStmt {
	class_declaration: com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration,
}

impl LocalClassDeclarationStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		this(null, ClassOrInterfaceDeclaration::new());
	}

	pub fn new(class_declaration: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration) -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		this(null, class_declaration);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, class_declaration: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration) -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		super(token_range);
		self.set_class_declaration(class_declaration);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_class_declaration(&self) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return self.class_declaration;
	}

	pub fn set_class_declaration(&mut self, class_declaration: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(class_declaration)?;
		if class_declaration == self.classDeclaration {
			return self;
		}
		self.notify_property_change(ObservableProperty::CLASS_DECLARATION, self.classDeclaration, class_declaration);
		if self.classDeclaration != null {
			self.classDeclaration.set_parent_node(null);
		}
	
		self.classDeclaration = class_declaration;
		self.set_as_parent_node_of(class_declaration);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		return self.accept(CloneVisitor::new(), null) as LocalClassDeclarationStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::local_class_declaration_stmt_meta_model::LocalClassDeclarationStmtMetaModel {
		return JavaParserMetaModel::localClassDeclarationStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.class_declaration {
			self.set_class_declaration(replacement_node as ClassOrInterfaceDeclaration)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_local_class_declaration_stmt(&self) -> bool {
		return true;
	}

	pub fn as_local_class_declaration_stmt(&self) -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		return self;
	}

	pub fn if_local_class_declaration_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_local_class_declaration_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LocalClassDeclarationStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for LocalClassDeclarationStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for LocalClassDeclarationStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LocalClassDeclarationStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LocalClassDeclarationStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LocalClassDeclarationStmt {}