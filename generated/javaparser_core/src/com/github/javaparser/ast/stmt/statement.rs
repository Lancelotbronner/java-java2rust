use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::StatementMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct Statement;

impl Statement {
	pub fn new() -> com::github::javaparser::ast::stmt::statement::Statement {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::stmt::statement::Statement {
		super(token_range);
		self.custom_initialization();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.accept(CloneVisitor::new(), null) as Statement;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::statement_meta_model::StatementMetaModel {
		return JavaParserMetaModel::statementMetaModel;
	}

	pub fn is_assert_stmt(&self) -> bool {
		return false;
	}

	pub fn as_assert_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not AssertStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_block_stmt(&self) -> bool {
		return false;
	}

	pub fn as_block_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not BlockStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_break_stmt(&self) -> bool {
		return false;
	}

	pub fn as_break_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::break_stmt::BreakStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not BreakStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_continue_stmt(&self) -> bool {
		return false;
	}

	pub fn as_continue_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ContinueStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_do_stmt(&self) -> bool {
		return false;
	}

	pub fn as_do_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::do_stmt::DoStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not DoStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_empty_stmt(&self) -> bool {
		return false;
	}

	pub fn as_empty_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not EmptyStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_explicit_constructor_invocation_stmt(&self) -> bool {
		return false;
	}

	pub fn as_explicit_constructor_invocation_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ExplicitConstructorInvocationStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_expression_stmt(&self) -> bool {
		return false;
	}

	pub fn as_expression_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ExpressionStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_for_stmt(&self) -> bool {
		return false;
	}

	pub fn as_for_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ForStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_if_stmt(&self) -> bool {
		return false;
	}

	pub fn as_if_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not IfStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_labeled_stmt(&self) -> bool {
		return false;
	}

	pub fn as_labeled_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LabeledStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_local_class_declaration_stmt(&self) -> bool {
		return false;
	}

	pub fn is_local_record_declaration_stmt(&self) -> bool {
		return false;
	}

	pub fn as_local_class_declaration_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LocalClassDeclarationStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn as_local_record_declaration_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LocalRecordDeclarationStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_return_stmt(&self) -> bool {
		return false;
	}

	pub fn as_return_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ReturnStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_switch_stmt(&self) -> bool {
		return false;
	}

	pub fn as_switch_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not SwitchStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_synchronized_stmt(&self) -> bool {
		return false;
	}

	pub fn as_synchronized_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not SynchronizedStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_throw_stmt(&self) -> bool {
		return false;
	}

	pub fn as_throw_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ThrowStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_try_stmt(&self) -> bool {
		return false;
	}

	pub fn as_try_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TryStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_unparsable_stmt(&self) -> bool {
		return false;
	}

	pub fn as_unparsable_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not UnparsableStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_while_stmt(&self) -> bool {
		return false;
	}

	pub fn as_while_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::while_stmt::WhileStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not WhileStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn if_assert_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_block_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_break_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_continue_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_do_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_empty_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_explicit_constructor_invocation_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_expression_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_for_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_if_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_labeled_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_local_class_declaration_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_local_record_declaration_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_return_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_switch_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_synchronized_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_throw_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_try_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_unparsable_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_while_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn to_assert_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_block_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_break_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_continue_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_do_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_empty_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_explicit_constructor_invocation_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_expression_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_for_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_if_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_labeled_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_local_class_declaration_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_local_record_declaration_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_return_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_switch_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_synchronized_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_throw_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_try_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_unparsable_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_while_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn is_for_each_stmt(&self) -> bool {
		return false;
	}

	pub fn as_for_each_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ForEachStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_for_each_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_for_each_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_yield_stmt(&self) -> bool {
		return false;
	}

	pub fn as_yield_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not YieldStmt, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_yield_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_yield_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}
}

impl /* Java */ java::lang::Cloneable /**/ for Statement {}

impl com::github::javaparser::has_parent_node::HasParentNode for Statement {}

impl com::github::javaparser::ast::observer::observable::Observable for Statement {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Statement {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Statement {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Statement {}