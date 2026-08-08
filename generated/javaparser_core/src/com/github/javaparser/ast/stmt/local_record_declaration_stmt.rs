use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::RecordDeclaration;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LocalRecordDeclarationStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LocalRecordDeclarationStmt {
	record_declaration: com::github::javaparser::ast::body::record_declaration::RecordDeclaration,
}

impl LocalRecordDeclarationStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		this(null, RecordDeclaration::new());
	}

	pub fn new(record_declaration: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration) -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		this(null, record_declaration);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, record_declaration: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration) -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		super(token_range);
		self.set_record_declaration(record_declaration);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_record_declaration(&self) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		return self.record_declaration;
	}

	pub fn set_record_declaration(&mut self, record_declaration: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(record_declaration)?;
		if record_declaration == self.recordDeclaration {
			return self;
		}
		self.notify_property_change(ObservableProperty::RECORD_DECLARATION, self.recordDeclaration, record_declaration);
		if self.recordDeclaration != null {
			self.recordDeclaration.set_parent_node(null);
		}
	
		self.recordDeclaration = record_declaration;
		self.set_as_parent_node_of(record_declaration);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		return self.accept(CloneVisitor::new(), null) as LocalRecordDeclarationStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::local_record_declaration_stmt_meta_model::LocalRecordDeclarationStmtMetaModel {
		return JavaParserMetaModel::localRecordDeclarationStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.record_declaration {
			self.set_record_declaration(replacement_node as RecordDeclaration)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_local_record_declaration_stmt(&self) -> bool {
		return true;
	}

	pub fn as_local_record_declaration_stmt(&self) -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		return self;
	}

	pub fn if_local_record_declaration_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_local_record_declaration_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LocalRecordDeclarationStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for LocalRecordDeclarationStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for LocalRecordDeclarationStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LocalRecordDeclarationStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LocalRecordDeclarationStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LocalRecordDeclarationStmt {}