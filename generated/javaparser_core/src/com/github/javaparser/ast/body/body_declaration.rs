use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::BodyDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BodyDeclaration<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> {
	annotations: com::github::javaparser::ast::node_list::NodeList,
}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> BodyDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		this(null, NodeList<>::new());
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		this(null, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		super(token_range);
		self.set_annotations(annotations);
		self.custom_initialization();
	}

	fn new(range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		this(range, NodeList<>::new());
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		if annotations == self.annotations {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::ANNOTATIONS, self.annotations, annotations);
		if self.annotations != null {
			self.annotations.set_parent_node(null);
		}
	
		self.annotations = annotations;
		self.set_as_parent_node_of(annotations);
		return self as T;
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
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		return self.accept(CloneVisitor::new(), null) as BodyDeclaration<?>;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::body_declaration_meta_model::BodyDeclarationMetaModel {
		return JavaParserMetaModel::bodyDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
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
	
		return super.replace(node, replacement_node);
	}

	pub fn is_annotation_declaration(&self) -> bool {
		return false;
	}

	pub fn as_annotation_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not AnnotationDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_annotation_member_declaration(&self) -> bool {
		return false;
	}

	pub fn as_annotation_member_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not AnnotationMemberDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_callable_declaration(&self) -> bool {
		return false;
	}

	pub fn as_callable_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not CallableDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_class_or_interface_declaration(&self) -> bool {
		return false;
	}

	pub fn as_class_or_interface_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ClassOrInterfaceDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_constructor_declaration(&self) -> bool {
		return false;
	}

	pub fn as_constructor_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ConstructorDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_compact_constructor_declaration(&self) -> bool {
		return false;
	}

	pub fn as_compact_constructor_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not CompactConstructorDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_enum_constant_declaration(&self) -> bool {
		return false;
	}

	pub fn as_enum_constant_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not EnumConstantDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_enum_declaration(&self) -> bool {
		return false;
	}

	pub fn as_enum_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not EnumDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_field_declaration(&self) -> bool {
		return false;
	}

	pub fn as_field_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not FieldDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_initializer_declaration(&self) -> bool {
		return false;
	}

	pub fn as_initializer_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not InitializerDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_method_declaration(&self) -> bool {
		return false;
	}

	pub fn as_method_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MethodDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_type_declaration(&self) -> bool {
		return false;
	}

	pub fn as_type_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TypeDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn if_annotation_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_annotation_member_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_callable_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_class_or_interface_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_constructor_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_enum_constant_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_enum_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_field_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_initializer_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_method_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_type_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_record_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_compact_constructor_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn to_annotation_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_annotation_member_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_callable_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_class_or_interface_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_constructor_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_enum_constant_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_enum_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_field_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_initializer_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_method_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_type_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn is_record_declaration(&self) -> bool {
		return false;
	}

	pub fn as_record_declaration(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not RecordDeclaration, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_record_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_compact_constructor_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}
}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> /* Java */ java::lang::Cloneable /**/ for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::has_parent_node::HasParentNode for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::ast::observer::observable::Observable for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::ast::visitor::visitable::Visitable for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BodyDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration> com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BodyDeclaration<T> {}