use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithOptionalScope;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::ExpressionMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;

pub struct Expression;

impl Expression {
	pub static IS_NOT_ENCLOSED_EXPR: /* Java */ java::util::function::Predicate /**/ = |n|!(n instanceof EnclosedExpr);

	pub static EXCLUDE_ENCLOSED_EXPR: /* Java */ java::util::function::Function /**/ = |expr|{
		while expr.is_enclosed_expr() {
			expr = expr.as_enclosed_expr()?.get_inner();
		}
		return expr;
	};

	pub fn new() -> com::github::javaparser::ast::expr::expression::Expression {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::expr::expression::Expression {
		super(token_range);
		self.custom_initialization();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.accept(CloneVisitor::new(), null) as Expression;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::expression_meta_model::ExpressionMetaModel {
		return JavaParserMetaModel::expressionMetaModel;
	}

	pub fn is_annotation_expr(&self) -> bool {
		return false;
	}

	pub fn as_annotation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not AnnotationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_array_access_expr(&self) -> bool {
		return false;
	}

	pub fn as_array_access_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ArrayAccessExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_array_creation_expr(&self) -> bool {
		return false;
	}

	pub fn as_array_creation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ArrayCreationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_array_initializer_expr(&self) -> bool {
		return false;
	}

	pub fn as_array_initializer_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ArrayInitializerExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_assign_expr(&self) -> bool {
		return false;
	}

	pub fn as_assign_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not AssignExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_binary_expr(&self) -> bool {
		return false;
	}

	pub fn as_binary_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not BinaryExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_boolean_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_boolean_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not BooleanLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_cast_expr(&self) -> bool {
		return false;
	}

	pub fn as_cast_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not CastExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_char_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_char_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not CharLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_class_expr(&self) -> bool {
		return false;
	}

	pub fn as_class_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::class_expr::ClassExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ClassExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_conditional_expr(&self) -> bool {
		return false;
	}

	pub fn as_conditional_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ConditionalExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_double_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_double_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not DoubleLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_enclosed_expr(&self) -> bool {
		return false;
	}

	pub fn as_enclosed_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not EnclosedExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_field_access_expr(&self) -> bool {
		return false;
	}

	pub fn as_field_access_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not FieldAccessExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_instance_of_expr(&self) -> bool {
		return false;
	}

	pub fn as_instance_of_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not InstanceOfExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_integer_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_integer_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not IntegerLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_lambda_expr(&self) -> bool {
		return false;
	}

	pub fn as_lambda_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::lambda_expr::LambdaExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LambdaExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::literal_expr::LiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_literal_string_value_expr(&self) -> bool {
		return false;
	}

	pub fn as_literal_string_value_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LiteralStringValueExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_long_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_long_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LongLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_marker_annotation_expr(&self) -> bool {
		return false;
	}

	pub fn as_marker_annotation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MarkerAnnotationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_method_call_expr(&self) -> bool {
		return false;
	}

	pub fn as_method_call_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MethodCallExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_method_reference_expr(&self) -> bool {
		return false;
	}

	pub fn as_method_reference_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MethodReferenceExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_name_expr(&self) -> bool {
		return false;
	}

	pub fn as_name_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::name_expr::NameExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not NameExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_normal_annotation_expr(&self) -> bool {
		return false;
	}

	pub fn as_normal_annotation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not NormalAnnotationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_null_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_null_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not NullLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_object_creation_expr(&self) -> bool {
		return false;
	}

	pub fn as_object_creation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ObjectCreationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_single_member_annotation_expr(&self) -> bool {
		return false;
	}

	pub fn as_single_member_annotation_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not SingleMemberAnnotationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_string_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_string_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not StringLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_super_expr(&self) -> bool {
		return false;
	}

	pub fn as_super_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::super_expr::SuperExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not SuperExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_this_expr(&self) -> bool {
		return false;
	}

	pub fn as_this_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::this_expr::ThisExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ThisExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_type_expr(&self) -> bool {
		return false;
	}

	pub fn as_type_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::type_expr::TypeExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TypeExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_unary_expr(&self) -> bool {
		return false;
	}

	pub fn as_unary_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not UnaryExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_variable_declaration_expr(&self) -> bool {
		return false;
	}

	pub fn as_variable_declaration_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not VariableDeclarationExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn if_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_array_access_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_array_creation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_array_initializer_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_assign_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_binary_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_boolean_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_cast_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_char_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_class_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_conditional_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_double_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_enclosed_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_field_access_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_instance_of_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_integer_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_lambda_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_literal_string_value_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_long_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_marker_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_method_call_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_method_reference_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_name_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_normal_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_null_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_object_creation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_single_member_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_string_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_super_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_this_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_type_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_unary_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_variable_declaration_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn calculate_resolved_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_symbol_resolver().calculate_type(self);
	}

	pub fn to_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_array_access_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_array_creation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_array_initializer_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_assign_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_binary_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_boolean_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_cast_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_char_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_class_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_conditional_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_double_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_enclosed_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_field_access_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_instance_of_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_integer_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_lambda_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_literal_string_value_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_long_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_marker_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_method_call_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_method_reference_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_name_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_normal_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_null_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_object_creation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_single_member_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_string_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_super_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_this_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_type_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_unary_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_variable_declaration_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn is_switch_expr(&self) -> bool {
		return false;
	}

	pub fn as_switch_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::switch_expr::SwitchExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not SwitchExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_switch_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_switch_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_text_block_literal_expr(&self) -> bool {
		return false;
	}

	pub fn as_text_block_literal_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TextBlockLiteralExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_text_block_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_text_block_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_standalone_expression(&self) -> bool {
		return !self.is_poly_expression();
	}

	pub fn is_poly_expression(&self) -> bool {
		return false;
	}

	pub fn is_qualified(&self) -> bool {
		return self.has_scope();
	}

	pub fn appears_in_assignment_context(&self) -> bool {
		if self.get_parent_node().isPresent() && self.get_parent_node().get() instanceof Expression {
			return (self.get_parent_node().get() as Expression).is_assignment_context();
		}
		return false;
	}

	fn is_assignment_context(&self) -> bool {
		return false;
	}

	pub fn appears_in_invocation_context(&self) -> bool {
		if self.get_parent_node().isPresent() && self.get_parent_node().get() instanceof Expression {
			return (self.get_parent_node().get() as Expression).is_invocation_context();
		}
		return false;
	}

	fn is_invocation_context(&self) -> bool {
		return false;
	}

	pub fn elides_type_arguments(&self) -> bool {
		if !(self.has_scope() && self instanceof NodeWithTypeArguments) {
			return true;
		}
		let scope: Expression = (self as NodeWithOptionalScope).get_scope().get() as Expression;
		let nwta: NodeWithTypeArguments = self as NodeWithTypeArguments;
		return scope.elides_type_arguments() && (!nwta.get_type_arguments().isPresent() || nwta.is_using_diamond_operator());
	}

	pub fn is_type_pattern_expr(&self) -> bool {
		return false;
	}

	pub fn as_type_pattern_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TypePatternExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_type_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_type_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_component_pattern_expr(&self) -> bool {
		return false;
	}

	pub fn as_component_pattern_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::component_pattern_expr::ComponentPatternExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not ComponentPatternExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_component_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_component_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_record_pattern_expr(&self) -> bool {
		return false;
	}

	pub fn as_record_pattern_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not RecordPatternExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_record_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_record_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_match_all_pattern_expr(&self) -> bool {
		return false;
	}

	pub fn as_match_all_pattern_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MatchAllPatternExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_match_all_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_match_all_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_pattern_expr(&self) -> bool {
		return false;
	}

	pub fn as_pattern_expr(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::expr::pattern_expr::PatternExpr {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not PatternExpr, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}
}

impl /* Java */ java::lang::Cloneable /**/ for Expression {}

impl com::github::javaparser::has_parent_node::HasParentNode for Expression {}

impl com::github::javaparser::ast::observer::observable::Observable for Expression {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Expression {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Expression {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Expression {}