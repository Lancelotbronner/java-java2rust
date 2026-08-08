use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TypeMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;
use java::util::function::Consumer;

pub struct Type {
	annotations: com::github::javaparser::ast::node_list::NodeList,
}

impl Type {
	fn new(range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::type::type::Type {
		this(range, NodeList<>::new());
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::type::Type {
		this(null, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::type::Type {
		super(token_range);
		self.set_annotations(annotations);
		self.custom_initialization();
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn get_annotation(&self, i: i32) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		return self.get_annotations().get(i);
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type::Type {
		com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		if annotations == self.annotations {
			return self;
		}
		self.notify_property_change(ObservableProperty::ANNOTATIONS, self.annotations, annotations);
		if self.annotations != null {
			self.annotations.set_parent_node(null);
		}
	
		self.annotations = annotations;
		self.set_as_parent_node_of(annotations);
		return self;
	}

	pub fn get_element_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self;
	}

	pub fn get_array_level(&self) -> i32 {
		return 0;
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return "";
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

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ ;

	pub fn clone(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.accept(CloneVisitor::new(), null) as Type;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::type_meta_model::TypeMetaModel {
		return JavaParserMetaModel::typeMetaModel;
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
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_array_type(&self) -> bool {
		return false;
	}

	pub fn as_array_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::array_type::ArrayType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ArrayType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_class_or_interface_type(&self) -> bool {
		return false;
	}

	pub fn as_class_or_interface_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ClassOrInterfaceType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_intersection_type(&self) -> bool {
		return false;
	}

	pub fn as_intersection_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::intersection_type::IntersectionType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not IntersectionType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_primitive_type(&self) -> bool {
		return false;
	}

	pub fn as_primitive_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not PrimitiveType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_reference_type(&self) -> bool {
		return false;
	}

	pub fn as_reference_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ReferenceType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_type_parameter(&self) -> bool {
		return false;
	}

	pub fn as_type_parameter(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TypeParameter, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_union_type(&self) -> bool {
		return false;
	}

	pub fn as_union_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::union_type::UnionType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not UnionType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_unknown_type(&self) -> bool {
		return false;
	}

	pub fn as_unknown_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not UnknownType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_void_type(&self) -> bool {
		return false;
	}

	pub fn as_void_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::void_type::VoidType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not VoidType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_wildcard_type(&self) -> bool {
		return false;
	}

	pub fn as_wildcard_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::wildcard_type::WildcardType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not WildcardType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn if_array_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_class_or_interface_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_intersection_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_primitive_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_reference_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_type_parameter(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_union_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_unknown_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_void_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_wildcard_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType ;

	pub fn to_array_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_class_or_interface_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_intersection_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_primitive_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_reference_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_type_parameter(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_union_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_unknown_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_void_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_wildcard_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn is_var_type(&self) -> bool {
		return false;
	}

	pub fn as_var_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::var_type::VarType {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not VarType, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_var_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_var_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}
}

impl com::github::javaparser::resolution::resolvable::Resolvable for Type {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for Type {}

impl /* Java */ java::lang::Cloneable /**/ for Type {}

impl com::github::javaparser::has_parent_node::HasParentNode for Type {}

impl com::github::javaparser::ast::observer::observable::Observable for Type {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Type {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Type {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Type {}