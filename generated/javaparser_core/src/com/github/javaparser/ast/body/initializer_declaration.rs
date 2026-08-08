use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithBlockStmt;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithJavadoc;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::InitializerDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct InitializerDeclaration {
	is_static: bool,
	body: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
}

impl InitializerDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		this(null, false, BlockStmt::new());
	}

	pub fn new(is_static: bool, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		this(null, is_static, body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, is_static: bool, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		super(token_range);
		self.set_static(is_static);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.body;
	}

	pub fn is_static(&self) -> bool {
		return self.is_static;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(body)?;
		if body == self.body {
			return self;
		}
		self.notify_property_change(ObservableProperty::BODY, self.body, body);
		if self.body != null {
			self.body.set_parent_node(null);
		}
	
		self.body = body;
		self.set_as_parent_node_of(body);
		return self;
	}

	pub fn set_static(&mut self, is_static: bool) -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		if is_static == self.isStatic {
			return self;
		}
		self.notify_property_change(ObservableProperty::STATIC, self.isStatic, is_static);
		self.isStatic = is_static;
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		return self.accept(CloneVisitor::new(), null) as InitializerDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::initializer_declaration_meta_model::InitializerDeclarationMetaModel {
		return JavaParserMetaModel::initializerDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as BlockStmt)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_initializer_declaration(&self) -> bool {
		return true;
	}

	pub fn as_initializer_declaration(&self) -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		return self;
	}

	pub fn if_initializer_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_initializer_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for InitializerDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_block_stmt::NodeWithBlockStmt for InitializerDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for InitializerDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for InitializerDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for InitializerDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for InitializerDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for InitializerDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for InitializerDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for InitializerDeclaration {}