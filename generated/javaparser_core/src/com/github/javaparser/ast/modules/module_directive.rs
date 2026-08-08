use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ModuleDirectiveMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ModuleDirective;

impl ModuleDirective {
	pub fn new() -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		super(token_range);
		self.custom_initialization();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		return self.accept(CloneVisitor::new(), null) as ModuleDirective;
	}

	pub fn is_module_exports_stmt(&self) -> bool {
		return false;
	}

	pub fn as_module_exports_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not an ModuleExportsDirective", self)));
	}

	pub fn is_module_opens_stmt(&self) -> bool {
		return false;
	}

	pub fn as_module_opens_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not an ModuleOpensDirective", self)));
	}

	pub fn is_module_provides_stmt(&self) -> bool {
		return false;
	}

	pub fn as_module_provides_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not an ModuleProvidesDirective", self)));
	}

	pub fn is_module_requires_stmt(&self) -> bool {
		return false;
	}

	pub fn as_module_requires_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not an ModuleRequiresDirective", self)));
	}

	pub fn is_module_uses_stmt(&self) -> bool {
		return false;
	}

	pub fn as_module_uses_stmt(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not an ModuleUsesDirective", self)));
	}

	pub fn if_module_exports_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_module_opens_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_module_provides_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_module_requires_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_module_uses_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn to_module_exports_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_module_opens_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_module_provides_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_module_requires_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_module_uses_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn is_module_exports_directive(&self) -> bool {
		return false;
	}

	pub fn as_module_exports_directive(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ModuleExportsDirective, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_module_exports_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_module_exports_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_module_opens_directive(&self) -> bool {
		return false;
	}

	pub fn as_module_opens_directive(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ModuleOpensDirective, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_module_opens_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_module_opens_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_module_provides_directive(&self) -> bool {
		return false;
	}

	pub fn as_module_provides_directive(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ModuleProvidesDirective, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_module_provides_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_module_provides_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_module_requires_directive(&self) -> bool {
		return false;
	}

	pub fn as_module_requires_directive(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ModuleRequiresDirective, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_module_requires_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_module_requires_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_module_uses_directive(&self) -> bool {
		return false;
	}

	pub fn as_module_uses_directive(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ModuleUsesDirective, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_module_uses_directive(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_module_uses_directive(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::module_directive_meta_model::ModuleDirectiveMetaModel {
		return JavaParserMetaModel::moduleDirectiveMetaModel;
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ModuleDirective {}

impl com::github::javaparser::has_parent_node::HasParentNode for ModuleDirective {}

impl com::github::javaparser::ast::observer::observable::Observable for ModuleDirective {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ModuleDirective {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ModuleDirective {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ModuleDirective {}