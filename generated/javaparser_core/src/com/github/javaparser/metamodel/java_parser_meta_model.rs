use crate::com::github::javaparser::ast::Generated;
use java::util::ArrayList;
use java::util::List;
use java::util::Optional;

pub struct JavaParserMetaModel;

impl JavaParserMetaModel {
	static nodeMetaModels: /* Java */ java::util::List /**/ = ArrayList<>::new();

	pub static nodeMetaModel: com::github::javaparser::metamodel::node_meta_model::NodeMetaModel = NodeMetaModel::new(&Optional::empty());

	pub static bodyDeclarationMetaModel: com::github::javaparser::metamodel::body_declaration_meta_model::BodyDeclarationMetaModel = BodyDeclarationMetaModel::new(&Optional::of(node_meta_model));

	pub static callableDeclarationMetaModel: com::github::javaparser::metamodel::callable_declaration_meta_model::CallableDeclarationMetaModel = CallableDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static expressionMetaModel: com::github::javaparser::metamodel::expression_meta_model::ExpressionMetaModel = ExpressionMetaModel::new(&Optional::of(node_meta_model));

	pub static statementMetaModel: com::github::javaparser::metamodel::statement_meta_model::StatementMetaModel = StatementMetaModel::new(&Optional::of(node_meta_model));

	pub static typeMetaModel: com::github::javaparser::metamodel::type_meta_model::TypeMetaModel = TypeMetaModel::new(&Optional::of(node_meta_model));

	pub static annotationExprMetaModel: com::github::javaparser::metamodel::annotation_expr_meta_model::AnnotationExprMetaModel = AnnotationExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static referenceTypeMetaModel: com::github::javaparser::metamodel::reference_type_meta_model::ReferenceTypeMetaModel = ReferenceTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static typeDeclarationMetaModel: com::github::javaparser::metamodel::type_declaration_meta_model::TypeDeclarationMetaModel = TypeDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static literalExprMetaModel: com::github::javaparser::metamodel::literal_expr_meta_model::LiteralExprMetaModel = LiteralExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static literalStringValueExprMetaModel: com::github::javaparser::metamodel::literal_string_value_expr_meta_model::LiteralStringValueExprMetaModel = LiteralStringValueExprMetaModel::new(&Optional::of(literal_expr_meta_model));

	pub static stringLiteralExprMetaModel: com::github::javaparser::metamodel::string_literal_expr_meta_model::StringLiteralExprMetaModel = StringLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static moduleDeclarationMetaModel: com::github::javaparser::metamodel::module_declaration_meta_model::ModuleDeclarationMetaModel = ModuleDeclarationMetaModel::new(&Optional::of(node_meta_model));

	pub static moduleDirectiveMetaModel: com::github::javaparser::metamodel::module_directive_meta_model::ModuleDirectiveMetaModel = ModuleDirectiveMetaModel::new(&Optional::of(node_meta_model));

	pub static arrayCreationLevelMetaModel: com::github::javaparser::metamodel::array_creation_level_meta_model::ArrayCreationLevelMetaModel = ArrayCreationLevelMetaModel::new(&Optional::of(node_meta_model));

	pub static compilationUnitMetaModel: com::github::javaparser::metamodel::compilation_unit_meta_model::CompilationUnitMetaModel = CompilationUnitMetaModel::new(&Optional::of(node_meta_model));

	pub static importDeclarationMetaModel: com::github::javaparser::metamodel::import_declaration_meta_model::ImportDeclarationMetaModel = ImportDeclarationMetaModel::new(&Optional::of(node_meta_model));

	pub static modifierMetaModel: com::github::javaparser::metamodel::modifier_meta_model::ModifierMetaModel = ModifierMetaModel::new(&Optional::of(node_meta_model));

	pub static packageDeclarationMetaModel: com::github::javaparser::metamodel::package_declaration_meta_model::PackageDeclarationMetaModel = PackageDeclarationMetaModel::new(&Optional::of(node_meta_model));

	pub static annotationDeclarationMetaModel: com::github::javaparser::metamodel::annotation_declaration_meta_model::AnnotationDeclarationMetaModel = AnnotationDeclarationMetaModel::new(&Optional::of(type_declaration_meta_model));

	pub static annotationMemberDeclarationMetaModel: com::github::javaparser::metamodel::annotation_member_declaration_meta_model::AnnotationMemberDeclarationMetaModel = AnnotationMemberDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static classOrInterfaceDeclarationMetaModel: com::github::javaparser::metamodel::class_or_interface_declaration_meta_model::ClassOrInterfaceDeclarationMetaModel = ClassOrInterfaceDeclarationMetaModel::new(&Optional::of(type_declaration_meta_model));

	pub static constructorDeclarationMetaModel: com::github::javaparser::metamodel::constructor_declaration_meta_model::ConstructorDeclarationMetaModel = ConstructorDeclarationMetaModel::new(&Optional::of(callable_declaration_meta_model));

	pub static enumConstantDeclarationMetaModel: com::github::javaparser::metamodel::enum_constant_declaration_meta_model::EnumConstantDeclarationMetaModel = EnumConstantDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static enumDeclarationMetaModel: com::github::javaparser::metamodel::enum_declaration_meta_model::EnumDeclarationMetaModel = EnumDeclarationMetaModel::new(&Optional::of(type_declaration_meta_model));

	pub static fieldDeclarationMetaModel: com::github::javaparser::metamodel::field_declaration_meta_model::FieldDeclarationMetaModel = FieldDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static initializerDeclarationMetaModel: com::github::javaparser::metamodel::initializer_declaration_meta_model::InitializerDeclarationMetaModel = InitializerDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static methodDeclarationMetaModel: com::github::javaparser::metamodel::method_declaration_meta_model::MethodDeclarationMetaModel = MethodDeclarationMetaModel::new(&Optional::of(callable_declaration_meta_model));

	pub static parameterMetaModel: com::github::javaparser::metamodel::parameter_meta_model::ParameterMetaModel = ParameterMetaModel::new(&Optional::of(node_meta_model));

	pub static receiverParameterMetaModel: com::github::javaparser::metamodel::receiver_parameter_meta_model::ReceiverParameterMetaModel = ReceiverParameterMetaModel::new(&Optional::of(node_meta_model));

	pub static recordDeclarationMetaModel: com::github::javaparser::metamodel::record_declaration_meta_model::RecordDeclarationMetaModel = RecordDeclarationMetaModel::new(&Optional::of(type_declaration_meta_model));

	pub static compactConstructorDeclarationMetaModel: com::github::javaparser::metamodel::compact_constructor_declaration_meta_model::CompactConstructorDeclarationMetaModel = CompactConstructorDeclarationMetaModel::new(&Optional::of(body_declaration_meta_model));

	pub static variableDeclaratorMetaModel: com::github::javaparser::metamodel::variable_declarator_meta_model::VariableDeclaratorMetaModel = VariableDeclaratorMetaModel::new(&Optional::of(node_meta_model));

	pub static commentMetaModel: com::github::javaparser::metamodel::comment_meta_model::CommentMetaModel = CommentMetaModel::new(&Optional::of(node_meta_model));

	pub static javadocCommentMetaModel: com::github::javaparser::metamodel::javadoc_comment_meta_model::JavadocCommentMetaModel = JavadocCommentMetaModel::new(&Optional::of(comment_meta_model));

	pub static blockCommentMetaModel: com::github::javaparser::metamodel::block_comment_meta_model::BlockCommentMetaModel = BlockCommentMetaModel::new(&Optional::of(comment_meta_model));

	pub static traditionalJavadocCommentMetaModel: com::github::javaparser::metamodel::traditional_javadoc_comment_meta_model::TraditionalJavadocCommentMetaModel = TraditionalJavadocCommentMetaModel::new(&Optional::of(javadoc_comment_meta_model));

	pub static lineCommentMetaModel: com::github::javaparser::metamodel::line_comment_meta_model::LineCommentMetaModel = LineCommentMetaModel::new(&Optional::of(comment_meta_model));

	pub static markdownCommentMetaModel: com::github::javaparser::metamodel::markdown_comment_meta_model::MarkdownCommentMetaModel = MarkdownCommentMetaModel::new(&Optional::of(javadoc_comment_meta_model));

	pub static arrayAccessExprMetaModel: com::github::javaparser::metamodel::array_access_expr_meta_model::ArrayAccessExprMetaModel = ArrayAccessExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static arrayCreationExprMetaModel: com::github::javaparser::metamodel::array_creation_expr_meta_model::ArrayCreationExprMetaModel = ArrayCreationExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static arrayInitializerExprMetaModel: com::github::javaparser::metamodel::array_initializer_expr_meta_model::ArrayInitializerExprMetaModel = ArrayInitializerExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static assignExprMetaModel: com::github::javaparser::metamodel::assign_expr_meta_model::AssignExprMetaModel = AssignExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static binaryExprMetaModel: com::github::javaparser::metamodel::binary_expr_meta_model::BinaryExprMetaModel = BinaryExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static booleanLiteralExprMetaModel: com::github::javaparser::metamodel::boolean_literal_expr_meta_model::BooleanLiteralExprMetaModel = BooleanLiteralExprMetaModel::new(&Optional::of(literal_expr_meta_model));

	pub static castExprMetaModel: com::github::javaparser::metamodel::cast_expr_meta_model::CastExprMetaModel = CastExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static charLiteralExprMetaModel: com::github::javaparser::metamodel::char_literal_expr_meta_model::CharLiteralExprMetaModel = CharLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static classExprMetaModel: com::github::javaparser::metamodel::class_expr_meta_model::ClassExprMetaModel = ClassExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static conditionalExprMetaModel: com::github::javaparser::metamodel::conditional_expr_meta_model::ConditionalExprMetaModel = ConditionalExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static doubleLiteralExprMetaModel: com::github::javaparser::metamodel::double_literal_expr_meta_model::DoubleLiteralExprMetaModel = DoubleLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static enclosedExprMetaModel: com::github::javaparser::metamodel::enclosed_expr_meta_model::EnclosedExprMetaModel = EnclosedExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static fieldAccessExprMetaModel: com::github::javaparser::metamodel::field_access_expr_meta_model::FieldAccessExprMetaModel = FieldAccessExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static instanceOfExprMetaModel: com::github::javaparser::metamodel::instance_of_expr_meta_model::InstanceOfExprMetaModel = InstanceOfExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static integerLiteralExprMetaModel: com::github::javaparser::metamodel::integer_literal_expr_meta_model::IntegerLiteralExprMetaModel = IntegerLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static lambdaExprMetaModel: com::github::javaparser::metamodel::lambda_expr_meta_model::LambdaExprMetaModel = LambdaExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static longLiteralExprMetaModel: com::github::javaparser::metamodel::long_literal_expr_meta_model::LongLiteralExprMetaModel = LongLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static markerAnnotationExprMetaModel: com::github::javaparser::metamodel::marker_annotation_expr_meta_model::MarkerAnnotationExprMetaModel = MarkerAnnotationExprMetaModel::new(&Optional::of(annotation_expr_meta_model));

	pub static memberValuePairMetaModel: com::github::javaparser::metamodel::member_value_pair_meta_model::MemberValuePairMetaModel = MemberValuePairMetaModel::new(&Optional::of(node_meta_model));

	pub static methodCallExprMetaModel: com::github::javaparser::metamodel::method_call_expr_meta_model::MethodCallExprMetaModel = MethodCallExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static methodReferenceExprMetaModel: com::github::javaparser::metamodel::method_reference_expr_meta_model::MethodReferenceExprMetaModel = MethodReferenceExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static nameExprMetaModel: com::github::javaparser::metamodel::name_expr_meta_model::NameExprMetaModel = NameExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static nameMetaModel: com::github::javaparser::metamodel::name_meta_model::NameMetaModel = NameMetaModel::new(&Optional::of(node_meta_model));

	pub static normalAnnotationExprMetaModel: com::github::javaparser::metamodel::normal_annotation_expr_meta_model::NormalAnnotationExprMetaModel = NormalAnnotationExprMetaModel::new(&Optional::of(annotation_expr_meta_model));

	pub static nullLiteralExprMetaModel: com::github::javaparser::metamodel::null_literal_expr_meta_model::NullLiteralExprMetaModel = NullLiteralExprMetaModel::new(&Optional::of(literal_expr_meta_model));

	pub static objectCreationExprMetaModel: com::github::javaparser::metamodel::object_creation_expr_meta_model::ObjectCreationExprMetaModel = ObjectCreationExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static componentPatternExprMetaModel: com::github::javaparser::metamodel::component_pattern_expr_meta_model::ComponentPatternExprMetaModel = ComponentPatternExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static patternExprMetaModel: com::github::javaparser::metamodel::pattern_expr_meta_model::PatternExprMetaModel = PatternExprMetaModel::new(&Optional::of(component_pattern_expr_meta_model));

	pub static recordPatternExprMetaModel: com::github::javaparser::metamodel::record_pattern_expr_meta_model::RecordPatternExprMetaModel = RecordPatternExprMetaModel::new(&Optional::of(pattern_expr_meta_model));

	pub static singleMemberAnnotationExprMetaModel: com::github::javaparser::metamodel::single_member_annotation_expr_meta_model::SingleMemberAnnotationExprMetaModel = SingleMemberAnnotationExprMetaModel::new(&Optional::of(annotation_expr_meta_model));

	pub static simpleNameMetaModel: com::github::javaparser::metamodel::simple_name_meta_model::SimpleNameMetaModel = SimpleNameMetaModel::new(&Optional::of(node_meta_model));

	pub static superExprMetaModel: com::github::javaparser::metamodel::super_expr_meta_model::SuperExprMetaModel = SuperExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static switchExprMetaModel: com::github::javaparser::metamodel::switch_expr_meta_model::SwitchExprMetaModel = SwitchExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static textBlockLiteralExprMetaModel: com::github::javaparser::metamodel::text_block_literal_expr_meta_model::TextBlockLiteralExprMetaModel = TextBlockLiteralExprMetaModel::new(&Optional::of(literal_string_value_expr_meta_model));

	pub static thisExprMetaModel: com::github::javaparser::metamodel::this_expr_meta_model::ThisExprMetaModel = ThisExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static typeExprMetaModel: com::github::javaparser::metamodel::type_expr_meta_model::TypeExprMetaModel = TypeExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static typePatternExprMetaModel: com::github::javaparser::metamodel::type_pattern_expr_meta_model::TypePatternExprMetaModel = TypePatternExprMetaModel::new(&Optional::of(pattern_expr_meta_model));

	pub static unaryExprMetaModel: com::github::javaparser::metamodel::unary_expr_meta_model::UnaryExprMetaModel = UnaryExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static matchAllPatternExprMetaModel: com::github::javaparser::metamodel::match_all_pattern_expr_meta_model::MatchAllPatternExprMetaModel = MatchAllPatternExprMetaModel::new(&Optional::of(component_pattern_expr_meta_model));

	pub static variableDeclarationExprMetaModel: com::github::javaparser::metamodel::variable_declaration_expr_meta_model::VariableDeclarationExprMetaModel = VariableDeclarationExprMetaModel::new(&Optional::of(expression_meta_model));

	pub static assertStmtMetaModel: com::github::javaparser::metamodel::assert_stmt_meta_model::AssertStmtMetaModel = AssertStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static blockStmtMetaModel: com::github::javaparser::metamodel::block_stmt_meta_model::BlockStmtMetaModel = BlockStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static breakStmtMetaModel: com::github::javaparser::metamodel::break_stmt_meta_model::BreakStmtMetaModel = BreakStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static catchClauseMetaModel: com::github::javaparser::metamodel::catch_clause_meta_model::CatchClauseMetaModel = CatchClauseMetaModel::new(&Optional::of(node_meta_model));

	pub static continueStmtMetaModel: com::github::javaparser::metamodel::continue_stmt_meta_model::ContinueStmtMetaModel = ContinueStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static doStmtMetaModel: com::github::javaparser::metamodel::do_stmt_meta_model::DoStmtMetaModel = DoStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static emptyStmtMetaModel: com::github::javaparser::metamodel::empty_stmt_meta_model::EmptyStmtMetaModel = EmptyStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static explicitConstructorInvocationStmtMetaModel: com::github::javaparser::metamodel::explicit_constructor_invocation_stmt_meta_model::ExplicitConstructorInvocationStmtMetaModel = ExplicitConstructorInvocationStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static expressionStmtMetaModel: com::github::javaparser::metamodel::expression_stmt_meta_model::ExpressionStmtMetaModel = ExpressionStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static forEachStmtMetaModel: com::github::javaparser::metamodel::for_each_stmt_meta_model::ForEachStmtMetaModel = ForEachStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static forStmtMetaModel: com::github::javaparser::metamodel::for_stmt_meta_model::ForStmtMetaModel = ForStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static ifStmtMetaModel: com::github::javaparser::metamodel::if_stmt_meta_model::IfStmtMetaModel = IfStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static labeledStmtMetaModel: com::github::javaparser::metamodel::labeled_stmt_meta_model::LabeledStmtMetaModel = LabeledStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static localClassDeclarationStmtMetaModel: com::github::javaparser::metamodel::local_class_declaration_stmt_meta_model::LocalClassDeclarationStmtMetaModel = LocalClassDeclarationStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static localRecordDeclarationStmtMetaModel: com::github::javaparser::metamodel::local_record_declaration_stmt_meta_model::LocalRecordDeclarationStmtMetaModel = LocalRecordDeclarationStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static returnStmtMetaModel: com::github::javaparser::metamodel::return_stmt_meta_model::ReturnStmtMetaModel = ReturnStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static switchEntryMetaModel: com::github::javaparser::metamodel::switch_entry_meta_model::SwitchEntryMetaModel = SwitchEntryMetaModel::new(&Optional::of(node_meta_model));

	pub static switchStmtMetaModel: com::github::javaparser::metamodel::switch_stmt_meta_model::SwitchStmtMetaModel = SwitchStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static synchronizedStmtMetaModel: com::github::javaparser::metamodel::synchronized_stmt_meta_model::SynchronizedStmtMetaModel = SynchronizedStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static throwStmtMetaModel: com::github::javaparser::metamodel::throw_stmt_meta_model::ThrowStmtMetaModel = ThrowStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static tryStmtMetaModel: com::github::javaparser::metamodel::try_stmt_meta_model::TryStmtMetaModel = TryStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static unparsableStmtMetaModel: com::github::javaparser::metamodel::unparsable_stmt_meta_model::UnparsableStmtMetaModel = UnparsableStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static whileStmtMetaModel: com::github::javaparser::metamodel::while_stmt_meta_model::WhileStmtMetaModel = WhileStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static yieldStmtMetaModel: com::github::javaparser::metamodel::yield_stmt_meta_model::YieldStmtMetaModel = YieldStmtMetaModel::new(&Optional::of(statement_meta_model));

	pub static arrayTypeMetaModel: com::github::javaparser::metamodel::array_type_meta_model::ArrayTypeMetaModel = ArrayTypeMetaModel::new(&Optional::of(reference_type_meta_model));

	pub static classOrInterfaceTypeMetaModel: com::github::javaparser::metamodel::class_or_interface_type_meta_model::ClassOrInterfaceTypeMetaModel = ClassOrInterfaceTypeMetaModel::new(&Optional::of(reference_type_meta_model));

	pub static intersectionTypeMetaModel: com::github::javaparser::metamodel::intersection_type_meta_model::IntersectionTypeMetaModel = IntersectionTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static primitiveTypeMetaModel: com::github::javaparser::metamodel::primitive_type_meta_model::PrimitiveTypeMetaModel = PrimitiveTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static typeParameterMetaModel: com::github::javaparser::metamodel::type_parameter_meta_model::TypeParameterMetaModel = TypeParameterMetaModel::new(&Optional::of(reference_type_meta_model));

	pub static unionTypeMetaModel: com::github::javaparser::metamodel::union_type_meta_model::UnionTypeMetaModel = UnionTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static unknownTypeMetaModel: com::github::javaparser::metamodel::unknown_type_meta_model::UnknownTypeMetaModel = UnknownTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static varTypeMetaModel: com::github::javaparser::metamodel::var_type_meta_model::VarTypeMetaModel = VarTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static voidTypeMetaModel: com::github::javaparser::metamodel::void_type_meta_model::VoidTypeMetaModel = VoidTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static wildcardTypeMetaModel: com::github::javaparser::metamodel::wildcard_type_meta_model::WildcardTypeMetaModel = WildcardTypeMetaModel::new(&Optional::of(type_meta_model));

	pub static moduleExportsDirectiveMetaModel: com::github::javaparser::metamodel::module_exports_directive_meta_model::ModuleExportsDirectiveMetaModel = ModuleExportsDirectiveMetaModel::new(&Optional::of(module_directive_meta_model));

	pub static moduleOpensDirectiveMetaModel: com::github::javaparser::metamodel::module_opens_directive_meta_model::ModuleOpensDirectiveMetaModel = ModuleOpensDirectiveMetaModel::new(&Optional::of(module_directive_meta_model));

	pub static moduleProvidesDirectiveMetaModel: com::github::javaparser::metamodel::module_provides_directive_meta_model::ModuleProvidesDirectiveMetaModel = ModuleProvidesDirectiveMetaModel::new(&Optional::of(module_directive_meta_model));

	pub static moduleRequiresDirectiveMetaModel: com::github::javaparser::metamodel::module_requires_directive_meta_model::ModuleRequiresDirectiveMetaModel = ModuleRequiresDirectiveMetaModel::new(&Optional::of(module_directive_meta_model));

	pub static moduleUsesDirectiveMetaModel: com::github::javaparser::metamodel::module_uses_directive_meta_model::ModuleUsesDirectiveMetaModel = ModuleUsesDirectiveMetaModel::new(&Optional::of(module_directive_meta_model));

	fn new() -> com::github::javaparser::metamodel::java_parser_meta_model::JavaParserMetaModel {
	}

	fn initialize_constructor_parameters(&self) {
		self.body_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.modifiersPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.typeParametersPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.namePropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.parametersPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.callable_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.annotation_expr_meta_model.get_constructor_parameters().add(self.annotation_expr_meta_model.namePropertyMetaModel);
		self.reference_type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.type_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.type_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.type_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.type_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.literal_string_value_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.string_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.module_declaration_meta_model.get_constructor_parameters().add(self.module_declaration_meta_model.annotationsPropertyMetaModel);
		self.module_declaration_meta_model.get_constructor_parameters().add(self.module_declaration_meta_model.namePropertyMetaModel);
		self.module_declaration_meta_model.get_constructor_parameters().add(self.module_declaration_meta_model.isOpenPropertyMetaModel);
		self.module_declaration_meta_model.get_constructor_parameters().add(self.module_declaration_meta_model.directivesPropertyMetaModel);
		self.array_creation_level_meta_model.get_constructor_parameters().add(self.array_creation_level_meta_model.dimensionPropertyMetaModel);
		self.array_creation_level_meta_model.get_constructor_parameters().add(self.array_creation_level_meta_model.annotationsPropertyMetaModel);
		self.compilation_unit_meta_model.get_constructor_parameters().add(self.compilation_unit_meta_model.packageDeclarationPropertyMetaModel);
		self.compilation_unit_meta_model.get_constructor_parameters().add(self.compilation_unit_meta_model.importsPropertyMetaModel);
		self.compilation_unit_meta_model.get_constructor_parameters().add(self.compilation_unit_meta_model.typesPropertyMetaModel);
		self.compilation_unit_meta_model.get_constructor_parameters().add(self.compilation_unit_meta_model.modulePropertyMetaModel);
		self.import_declaration_meta_model.get_constructor_parameters().add(self.import_declaration_meta_model.namePropertyMetaModel);
		self.import_declaration_meta_model.get_constructor_parameters().add(self.import_declaration_meta_model.isStaticPropertyMetaModel);
		self.import_declaration_meta_model.get_constructor_parameters().add(self.import_declaration_meta_model.isAsteriskPropertyMetaModel);
		self.import_declaration_meta_model.get_constructor_parameters().add(self.import_declaration_meta_model.isModulePropertyMetaModel);
		self.modifier_meta_model.get_constructor_parameters().add(self.modifier_meta_model.keywordPropertyMetaModel);
		self.package_declaration_meta_model.get_constructor_parameters().add(self.package_declaration_meta_model.annotationsPropertyMetaModel);
		self.package_declaration_meta_model.get_constructor_parameters().add(self.package_declaration_meta_model.namePropertyMetaModel);
		self.annotation_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.annotation_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.annotation_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.annotation_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.annotation_member_declaration_meta_model.get_constructor_parameters().add(self.annotation_member_declaration_meta_model.modifiersPropertyMetaModel);
		self.annotation_member_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.annotation_member_declaration_meta_model.get_constructor_parameters().add(self.annotation_member_declaration_meta_model.typePropertyMetaModel);
		self.annotation_member_declaration_meta_model.get_constructor_parameters().add(self.annotation_member_declaration_meta_model.namePropertyMetaModel);
		self.annotation_member_declaration_meta_model.get_constructor_parameters().add(self.annotation_member_declaration_meta_model.defaultValuePropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.isInterfacePropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.typeParametersPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.extendedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.permittedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.get_constructor_parameters().add(self.class_or_interface_declaration_meta_model.isCompactPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.modifiersPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.typeParametersPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.namePropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.parametersPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.constructor_declaration_meta_model.bodyPropertyMetaModel);
		self.constructor_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.enum_constant_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.enum_constant_declaration_meta_model.get_constructor_parameters().add(self.enum_constant_declaration_meta_model.namePropertyMetaModel);
		self.enum_constant_declaration_meta_model.get_constructor_parameters().add(self.enum_constant_declaration_meta_model.argumentsPropertyMetaModel);
		self.enum_constant_declaration_meta_model.get_constructor_parameters().add(self.enum_constant_declaration_meta_model.classBodyPropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.enum_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.enum_declaration_meta_model.entriesPropertyMetaModel);
		self.enum_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.field_declaration_meta_model.get_constructor_parameters().add(self.field_declaration_meta_model.modifiersPropertyMetaModel);
		self.field_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.field_declaration_meta_model.get_constructor_parameters().add(self.field_declaration_meta_model.variablesPropertyMetaModel);
		self.initializer_declaration_meta_model.get_constructor_parameters().add(self.initializer_declaration_meta_model.isStaticPropertyMetaModel);
		self.initializer_declaration_meta_model.get_constructor_parameters().add(self.initializer_declaration_meta_model.bodyPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.modifiersPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.typeParametersPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.method_declaration_meta_model.typePropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.namePropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.parametersPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.method_declaration_meta_model.bodyPropertyMetaModel);
		self.method_declaration_meta_model.get_constructor_parameters().add(self.callable_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.modifiersPropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.annotationsPropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.typePropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.isVarArgsPropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.varArgsAnnotationsPropertyMetaModel);
		self.parameter_meta_model.get_constructor_parameters().add(self.parameter_meta_model.namePropertyMetaModel);
		self.receiver_parameter_meta_model.get_constructor_parameters().add(self.receiver_parameter_meta_model.annotationsPropertyMetaModel);
		self.receiver_parameter_meta_model.get_constructor_parameters().add(self.receiver_parameter_meta_model.typePropertyMetaModel);
		self.receiver_parameter_meta_model.get_constructor_parameters().add(self.receiver_parameter_meta_model.namePropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.record_declaration_meta_model.parametersPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.record_declaration_meta_model.typeParametersPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.record_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.record_declaration_meta_model.get_constructor_parameters().add(self.record_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.compact_constructor_declaration_meta_model.modifiersPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.compact_constructor_declaration_meta_model.typeParametersPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.compact_constructor_declaration_meta_model.namePropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.compact_constructor_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.get_constructor_parameters().add(self.compact_constructor_declaration_meta_model.bodyPropertyMetaModel);
		self.variable_declarator_meta_model.get_constructor_parameters().add(self.variable_declarator_meta_model.typePropertyMetaModel);
		self.variable_declarator_meta_model.get_constructor_parameters().add(self.variable_declarator_meta_model.namePropertyMetaModel);
		self.variable_declarator_meta_model.get_constructor_parameters().add(self.variable_declarator_meta_model.initializerPropertyMetaModel);
		self.comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.javadoc_comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.block_comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.traditional_javadoc_comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.line_comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.markdown_comment_meta_model.get_constructor_parameters().add(self.comment_meta_model.contentPropertyMetaModel);
		self.array_access_expr_meta_model.get_constructor_parameters().add(self.array_access_expr_meta_model.namePropertyMetaModel);
		self.array_access_expr_meta_model.get_constructor_parameters().add(self.array_access_expr_meta_model.indexPropertyMetaModel);
		self.array_creation_expr_meta_model.get_constructor_parameters().add(self.array_creation_expr_meta_model.elementTypePropertyMetaModel);
		self.array_creation_expr_meta_model.get_constructor_parameters().add(self.array_creation_expr_meta_model.levelsPropertyMetaModel);
		self.array_creation_expr_meta_model.get_constructor_parameters().add(self.array_creation_expr_meta_model.initializerPropertyMetaModel);
		self.array_initializer_expr_meta_model.get_constructor_parameters().add(self.array_initializer_expr_meta_model.valuesPropertyMetaModel);
		self.assign_expr_meta_model.get_constructor_parameters().add(self.assign_expr_meta_model.targetPropertyMetaModel);
		self.assign_expr_meta_model.get_constructor_parameters().add(self.assign_expr_meta_model.valuePropertyMetaModel);
		self.assign_expr_meta_model.get_constructor_parameters().add(self.assign_expr_meta_model.operatorPropertyMetaModel);
		self.binary_expr_meta_model.get_constructor_parameters().add(self.binary_expr_meta_model.leftPropertyMetaModel);
		self.binary_expr_meta_model.get_constructor_parameters().add(self.binary_expr_meta_model.rightPropertyMetaModel);
		self.binary_expr_meta_model.get_constructor_parameters().add(self.binary_expr_meta_model.operatorPropertyMetaModel);
		self.boolean_literal_expr_meta_model.get_constructor_parameters().add(self.boolean_literal_expr_meta_model.valuePropertyMetaModel);
		self.cast_expr_meta_model.get_constructor_parameters().add(self.cast_expr_meta_model.typePropertyMetaModel);
		self.cast_expr_meta_model.get_constructor_parameters().add(self.cast_expr_meta_model.expressionPropertyMetaModel);
		self.char_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.class_expr_meta_model.get_constructor_parameters().add(self.class_expr_meta_model.typePropertyMetaModel);
		self.conditional_expr_meta_model.get_constructor_parameters().add(self.conditional_expr_meta_model.conditionPropertyMetaModel);
		self.conditional_expr_meta_model.get_constructor_parameters().add(self.conditional_expr_meta_model.thenExprPropertyMetaModel);
		self.conditional_expr_meta_model.get_constructor_parameters().add(self.conditional_expr_meta_model.elseExprPropertyMetaModel);
		self.double_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.enclosed_expr_meta_model.get_constructor_parameters().add(self.enclosed_expr_meta_model.innerPropertyMetaModel);
		self.field_access_expr_meta_model.get_constructor_parameters().add(self.field_access_expr_meta_model.scopePropertyMetaModel);
		self.field_access_expr_meta_model.get_constructor_parameters().add(self.field_access_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.field_access_expr_meta_model.get_constructor_parameters().add(self.field_access_expr_meta_model.namePropertyMetaModel);
		self.instance_of_expr_meta_model.get_constructor_parameters().add(self.instance_of_expr_meta_model.expressionPropertyMetaModel);
		self.instance_of_expr_meta_model.get_constructor_parameters().add(self.instance_of_expr_meta_model.typePropertyMetaModel);
		self.instance_of_expr_meta_model.get_constructor_parameters().add(self.instance_of_expr_meta_model.patternPropertyMetaModel);
		self.integer_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.lambda_expr_meta_model.get_constructor_parameters().add(self.lambda_expr_meta_model.parametersPropertyMetaModel);
		self.lambda_expr_meta_model.get_constructor_parameters().add(self.lambda_expr_meta_model.bodyPropertyMetaModel);
		self.lambda_expr_meta_model.get_constructor_parameters().add(self.lambda_expr_meta_model.isEnclosingParametersPropertyMetaModel);
		self.long_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.marker_annotation_expr_meta_model.get_constructor_parameters().add(self.annotation_expr_meta_model.namePropertyMetaModel);
		self.member_value_pair_meta_model.get_constructor_parameters().add(self.member_value_pair_meta_model.namePropertyMetaModel);
		self.member_value_pair_meta_model.get_constructor_parameters().add(self.member_value_pair_meta_model.valuePropertyMetaModel);
		self.method_call_expr_meta_model.get_constructor_parameters().add(self.method_call_expr_meta_model.scopePropertyMetaModel);
		self.method_call_expr_meta_model.get_constructor_parameters().add(self.method_call_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.method_call_expr_meta_model.get_constructor_parameters().add(self.method_call_expr_meta_model.namePropertyMetaModel);
		self.method_call_expr_meta_model.get_constructor_parameters().add(self.method_call_expr_meta_model.argumentsPropertyMetaModel);
		self.method_reference_expr_meta_model.get_constructor_parameters().add(self.method_reference_expr_meta_model.scopePropertyMetaModel);
		self.method_reference_expr_meta_model.get_constructor_parameters().add(self.method_reference_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.method_reference_expr_meta_model.get_constructor_parameters().add(self.method_reference_expr_meta_model.identifierPropertyMetaModel);
		self.name_expr_meta_model.get_constructor_parameters().add(self.name_expr_meta_model.namePropertyMetaModel);
		self.name_meta_model.get_constructor_parameters().add(self.name_meta_model.qualifierPropertyMetaModel);
		self.name_meta_model.get_constructor_parameters().add(self.name_meta_model.identifierPropertyMetaModel);
		self.normal_annotation_expr_meta_model.get_constructor_parameters().add(self.annotation_expr_meta_model.namePropertyMetaModel);
		self.normal_annotation_expr_meta_model.get_constructor_parameters().add(self.normal_annotation_expr_meta_model.pairsPropertyMetaModel);
		self.object_creation_expr_meta_model.get_constructor_parameters().add(self.object_creation_expr_meta_model.scopePropertyMetaModel);
		self.object_creation_expr_meta_model.get_constructor_parameters().add(self.object_creation_expr_meta_model.typePropertyMetaModel);
		self.object_creation_expr_meta_model.get_constructor_parameters().add(self.object_creation_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.object_creation_expr_meta_model.get_constructor_parameters().add(self.object_creation_expr_meta_model.argumentsPropertyMetaModel);
		self.object_creation_expr_meta_model.get_constructor_parameters().add(self.object_creation_expr_meta_model.anonymousClassBodyPropertyMetaModel);
		self.pattern_expr_meta_model.get_constructor_parameters().add(self.pattern_expr_meta_model.typePropertyMetaModel);
		self.record_pattern_expr_meta_model.get_constructor_parameters().add(self.record_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.record_pattern_expr_meta_model.get_constructor_parameters().add(self.pattern_expr_meta_model.typePropertyMetaModel);
		self.record_pattern_expr_meta_model.get_constructor_parameters().add(self.record_pattern_expr_meta_model.patternListPropertyMetaModel);
		self.single_member_annotation_expr_meta_model.get_constructor_parameters().add(self.annotation_expr_meta_model.namePropertyMetaModel);
		self.single_member_annotation_expr_meta_model.get_constructor_parameters().add(self.single_member_annotation_expr_meta_model.memberValuePropertyMetaModel);
		self.simple_name_meta_model.get_constructor_parameters().add(self.simple_name_meta_model.identifierPropertyMetaModel);
		self.super_expr_meta_model.get_constructor_parameters().add(self.super_expr_meta_model.typeNamePropertyMetaModel);
		self.switch_expr_meta_model.get_constructor_parameters().add(self.switch_expr_meta_model.selectorPropertyMetaModel);
		self.switch_expr_meta_model.get_constructor_parameters().add(self.switch_expr_meta_model.entriesPropertyMetaModel);
		self.text_block_literal_expr_meta_model.get_constructor_parameters().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.this_expr_meta_model.get_constructor_parameters().add(self.this_expr_meta_model.typeNamePropertyMetaModel);
		self.type_expr_meta_model.get_constructor_parameters().add(self.type_expr_meta_model.typePropertyMetaModel);
		self.type_pattern_expr_meta_model.get_constructor_parameters().add(self.type_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.type_pattern_expr_meta_model.get_constructor_parameters().add(self.pattern_expr_meta_model.typePropertyMetaModel);
		self.type_pattern_expr_meta_model.get_constructor_parameters().add(self.type_pattern_expr_meta_model.namePropertyMetaModel);
		self.unary_expr_meta_model.get_constructor_parameters().add(self.unary_expr_meta_model.expressionPropertyMetaModel);
		self.unary_expr_meta_model.get_constructor_parameters().add(self.unary_expr_meta_model.operatorPropertyMetaModel);
		self.match_all_pattern_expr_meta_model.get_constructor_parameters().add(self.match_all_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.variable_declaration_expr_meta_model.get_constructor_parameters().add(self.variable_declaration_expr_meta_model.modifiersPropertyMetaModel);
		self.variable_declaration_expr_meta_model.get_constructor_parameters().add(self.variable_declaration_expr_meta_model.annotationsPropertyMetaModel);
		self.variable_declaration_expr_meta_model.get_constructor_parameters().add(self.variable_declaration_expr_meta_model.variablesPropertyMetaModel);
		self.assert_stmt_meta_model.get_constructor_parameters().add(self.assert_stmt_meta_model.checkPropertyMetaModel);
		self.assert_stmt_meta_model.get_constructor_parameters().add(self.assert_stmt_meta_model.messagePropertyMetaModel);
		self.block_stmt_meta_model.get_constructor_parameters().add(self.block_stmt_meta_model.statementsPropertyMetaModel);
		self.break_stmt_meta_model.get_constructor_parameters().add(self.break_stmt_meta_model.labelPropertyMetaModel);
		self.catch_clause_meta_model.get_constructor_parameters().add(self.catch_clause_meta_model.parameterPropertyMetaModel);
		self.catch_clause_meta_model.get_constructor_parameters().add(self.catch_clause_meta_model.bodyPropertyMetaModel);
		self.continue_stmt_meta_model.get_constructor_parameters().add(self.continue_stmt_meta_model.labelPropertyMetaModel);
		self.do_stmt_meta_model.get_constructor_parameters().add(self.do_stmt_meta_model.bodyPropertyMetaModel);
		self.do_stmt_meta_model.get_constructor_parameters().add(self.do_stmt_meta_model.conditionPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.get_constructor_parameters().add(self.explicit_constructor_invocation_stmt_meta_model.typeArgumentsPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.get_constructor_parameters().add(self.explicit_constructor_invocation_stmt_meta_model.isThisPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.get_constructor_parameters().add(self.explicit_constructor_invocation_stmt_meta_model.expressionPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.get_constructor_parameters().add(self.explicit_constructor_invocation_stmt_meta_model.argumentsPropertyMetaModel);
		self.expression_stmt_meta_model.get_constructor_parameters().add(self.expression_stmt_meta_model.expressionPropertyMetaModel);
		self.for_each_stmt_meta_model.get_constructor_parameters().add(self.for_each_stmt_meta_model.variablePropertyMetaModel);
		self.for_each_stmt_meta_model.get_constructor_parameters().add(self.for_each_stmt_meta_model.iterablePropertyMetaModel);
		self.for_each_stmt_meta_model.get_constructor_parameters().add(self.for_each_stmt_meta_model.bodyPropertyMetaModel);
		self.for_stmt_meta_model.get_constructor_parameters().add(self.for_stmt_meta_model.initializationPropertyMetaModel);
		self.for_stmt_meta_model.get_constructor_parameters().add(self.for_stmt_meta_model.comparePropertyMetaModel);
		self.for_stmt_meta_model.get_constructor_parameters().add(self.for_stmt_meta_model.updatePropertyMetaModel);
		self.for_stmt_meta_model.get_constructor_parameters().add(self.for_stmt_meta_model.bodyPropertyMetaModel);
		self.if_stmt_meta_model.get_constructor_parameters().add(self.if_stmt_meta_model.conditionPropertyMetaModel);
		self.if_stmt_meta_model.get_constructor_parameters().add(self.if_stmt_meta_model.thenStmtPropertyMetaModel);
		self.if_stmt_meta_model.get_constructor_parameters().add(self.if_stmt_meta_model.elseStmtPropertyMetaModel);
		self.labeled_stmt_meta_model.get_constructor_parameters().add(self.labeled_stmt_meta_model.labelPropertyMetaModel);
		self.labeled_stmt_meta_model.get_constructor_parameters().add(self.labeled_stmt_meta_model.statementPropertyMetaModel);
		self.local_class_declaration_stmt_meta_model.get_constructor_parameters().add(self.local_class_declaration_stmt_meta_model.classDeclarationPropertyMetaModel);
		self.local_record_declaration_stmt_meta_model.get_constructor_parameters().add(self.local_record_declaration_stmt_meta_model.recordDeclarationPropertyMetaModel);
		self.return_stmt_meta_model.get_constructor_parameters().add(self.return_stmt_meta_model.expressionPropertyMetaModel);
		self.switch_entry_meta_model.get_constructor_parameters().add(self.switch_entry_meta_model.labelsPropertyMetaModel);
		self.switch_entry_meta_model.get_constructor_parameters().add(self.switch_entry_meta_model.typePropertyMetaModel);
		self.switch_entry_meta_model.get_constructor_parameters().add(self.switch_entry_meta_model.statementsPropertyMetaModel);
		self.switch_entry_meta_model.get_constructor_parameters().add(self.switch_entry_meta_model.isDefaultPropertyMetaModel);
		self.switch_entry_meta_model.get_constructor_parameters().add(self.switch_entry_meta_model.guardPropertyMetaModel);
		self.switch_stmt_meta_model.get_constructor_parameters().add(self.switch_stmt_meta_model.selectorPropertyMetaModel);
		self.switch_stmt_meta_model.get_constructor_parameters().add(self.switch_stmt_meta_model.entriesPropertyMetaModel);
		self.synchronized_stmt_meta_model.get_constructor_parameters().add(self.synchronized_stmt_meta_model.expressionPropertyMetaModel);
		self.synchronized_stmt_meta_model.get_constructor_parameters().add(self.synchronized_stmt_meta_model.bodyPropertyMetaModel);
		self.throw_stmt_meta_model.get_constructor_parameters().add(self.throw_stmt_meta_model.expressionPropertyMetaModel);
		self.try_stmt_meta_model.get_constructor_parameters().add(self.try_stmt_meta_model.resourcesPropertyMetaModel);
		self.try_stmt_meta_model.get_constructor_parameters().add(self.try_stmt_meta_model.tryBlockPropertyMetaModel);
		self.try_stmt_meta_model.get_constructor_parameters().add(self.try_stmt_meta_model.catchClausesPropertyMetaModel);
		self.try_stmt_meta_model.get_constructor_parameters().add(self.try_stmt_meta_model.finallyBlockPropertyMetaModel);
		self.while_stmt_meta_model.get_constructor_parameters().add(self.while_stmt_meta_model.conditionPropertyMetaModel);
		self.while_stmt_meta_model.get_constructor_parameters().add(self.while_stmt_meta_model.bodyPropertyMetaModel);
		self.yield_stmt_meta_model.get_constructor_parameters().add(self.yield_stmt_meta_model.expressionPropertyMetaModel);
		self.array_type_meta_model.get_constructor_parameters().add(self.array_type_meta_model.componentTypePropertyMetaModel);
		self.array_type_meta_model.get_constructor_parameters().add(self.array_type_meta_model.originPropertyMetaModel);
		self.array_type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.class_or_interface_type_meta_model.get_constructor_parameters().add(self.class_or_interface_type_meta_model.scopePropertyMetaModel);
		self.class_or_interface_type_meta_model.get_constructor_parameters().add(self.class_or_interface_type_meta_model.namePropertyMetaModel);
		self.class_or_interface_type_meta_model.get_constructor_parameters().add(self.class_or_interface_type_meta_model.typeArgumentsPropertyMetaModel);
		self.class_or_interface_type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.intersection_type_meta_model.get_constructor_parameters().add(self.intersection_type_meta_model.elementsPropertyMetaModel);
		self.primitive_type_meta_model.get_constructor_parameters().add(self.primitive_type_meta_model.typePropertyMetaModel);
		self.primitive_type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.type_parameter_meta_model.get_constructor_parameters().add(self.type_parameter_meta_model.namePropertyMetaModel);
		self.type_parameter_meta_model.get_constructor_parameters().add(self.type_parameter_meta_model.typeBoundPropertyMetaModel);
		self.type_parameter_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.union_type_meta_model.get_constructor_parameters().add(self.union_type_meta_model.elementsPropertyMetaModel);
		self.wildcard_type_meta_model.get_constructor_parameters().add(self.wildcard_type_meta_model.extendedTypePropertyMetaModel);
		self.wildcard_type_meta_model.get_constructor_parameters().add(self.wildcard_type_meta_model.superTypePropertyMetaModel);
		self.wildcard_type_meta_model.get_constructor_parameters().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.module_exports_directive_meta_model.get_constructor_parameters().add(self.module_exports_directive_meta_model.namePropertyMetaModel);
		self.module_exports_directive_meta_model.get_constructor_parameters().add(self.module_exports_directive_meta_model.moduleNamesPropertyMetaModel);
		self.module_opens_directive_meta_model.get_constructor_parameters().add(self.module_opens_directive_meta_model.namePropertyMetaModel);
		self.module_opens_directive_meta_model.get_constructor_parameters().add(self.module_opens_directive_meta_model.moduleNamesPropertyMetaModel);
		self.module_provides_directive_meta_model.get_constructor_parameters().add(self.module_provides_directive_meta_model.namePropertyMetaModel);
		self.module_provides_directive_meta_model.get_constructor_parameters().add(self.module_provides_directive_meta_model.withPropertyMetaModel);
		self.module_requires_directive_meta_model.get_constructor_parameters().add(self.module_requires_directive_meta_model.modifiersPropertyMetaModel);
		self.module_requires_directive_meta_model.get_constructor_parameters().add(self.module_requires_directive_meta_model.namePropertyMetaModel);
		self.module_uses_directive_meta_model.get_constructor_parameters().add(self.module_uses_directive_meta_model.namePropertyMetaModel);
	}

	pub fn get_node_meta_models(&self) -> /* Java */ java::util::List /**/ {
		return self.node_meta_models;
	}

	fn initialize_node_meta_models(&self) {
		self.node_meta_models.add(self.annotation_declaration_meta_model);
		self.node_meta_models.add(self.annotation_expr_meta_model);
		self.node_meta_models.add(self.annotation_member_declaration_meta_model);
		self.node_meta_models.add(self.array_access_expr_meta_model);
		self.node_meta_models.add(self.array_creation_expr_meta_model);
		self.node_meta_models.add(self.array_creation_level_meta_model);
		self.node_meta_models.add(self.array_initializer_expr_meta_model);
		self.node_meta_models.add(self.array_type_meta_model);
		self.node_meta_models.add(self.assert_stmt_meta_model);
		self.node_meta_models.add(self.assign_expr_meta_model);
		self.node_meta_models.add(self.binary_expr_meta_model);
		self.node_meta_models.add(self.block_comment_meta_model);
		self.node_meta_models.add(self.block_stmt_meta_model);
		self.node_meta_models.add(self.body_declaration_meta_model);
		self.node_meta_models.add(self.boolean_literal_expr_meta_model);
		self.node_meta_models.add(self.break_stmt_meta_model);
		self.node_meta_models.add(self.callable_declaration_meta_model);
		self.node_meta_models.add(self.cast_expr_meta_model);
		self.node_meta_models.add(self.catch_clause_meta_model);
		self.node_meta_models.add(self.char_literal_expr_meta_model);
		self.node_meta_models.add(self.class_expr_meta_model);
		self.node_meta_models.add(self.class_or_interface_declaration_meta_model);
		self.node_meta_models.add(self.class_or_interface_type_meta_model);
		self.node_meta_models.add(self.comment_meta_model);
		self.node_meta_models.add(self.compact_constructor_declaration_meta_model);
		self.node_meta_models.add(self.compilation_unit_meta_model);
		self.node_meta_models.add(self.component_pattern_expr_meta_model);
		self.node_meta_models.add(self.conditional_expr_meta_model);
		self.node_meta_models.add(self.constructor_declaration_meta_model);
		self.node_meta_models.add(self.continue_stmt_meta_model);
		self.node_meta_models.add(self.do_stmt_meta_model);
		self.node_meta_models.add(self.double_literal_expr_meta_model);
		self.node_meta_models.add(self.empty_stmt_meta_model);
		self.node_meta_models.add(self.enclosed_expr_meta_model);
		self.node_meta_models.add(self.enum_constant_declaration_meta_model);
		self.node_meta_models.add(self.enum_declaration_meta_model);
		self.node_meta_models.add(self.explicit_constructor_invocation_stmt_meta_model);
		self.node_meta_models.add(self.expression_meta_model);
		self.node_meta_models.add(self.expression_stmt_meta_model);
		self.node_meta_models.add(self.field_access_expr_meta_model);
		self.node_meta_models.add(self.field_declaration_meta_model);
		self.node_meta_models.add(self.for_each_stmt_meta_model);
		self.node_meta_models.add(self.for_stmt_meta_model);
		self.node_meta_models.add(self.if_stmt_meta_model);
		self.node_meta_models.add(self.import_declaration_meta_model);
		self.node_meta_models.add(self.initializer_declaration_meta_model);
		self.node_meta_models.add(self.instance_of_expr_meta_model);
		self.node_meta_models.add(self.integer_literal_expr_meta_model);
		self.node_meta_models.add(self.intersection_type_meta_model);
		self.node_meta_models.add(self.javadoc_comment_meta_model);
		self.node_meta_models.add(self.labeled_stmt_meta_model);
		self.node_meta_models.add(self.lambda_expr_meta_model);
		self.node_meta_models.add(self.line_comment_meta_model);
		self.node_meta_models.add(self.literal_expr_meta_model);
		self.node_meta_models.add(self.literal_string_value_expr_meta_model);
		self.node_meta_models.add(self.local_class_declaration_stmt_meta_model);
		self.node_meta_models.add(self.local_record_declaration_stmt_meta_model);
		self.node_meta_models.add(self.long_literal_expr_meta_model);
		self.node_meta_models.add(self.markdown_comment_meta_model);
		self.node_meta_models.add(self.marker_annotation_expr_meta_model);
		self.node_meta_models.add(self.match_all_pattern_expr_meta_model);
		self.node_meta_models.add(self.member_value_pair_meta_model);
		self.node_meta_models.add(self.method_call_expr_meta_model);
		self.node_meta_models.add(self.method_declaration_meta_model);
		self.node_meta_models.add(self.method_reference_expr_meta_model);
		self.node_meta_models.add(self.modifier_meta_model);
		self.node_meta_models.add(self.module_declaration_meta_model);
		self.node_meta_models.add(self.module_directive_meta_model);
		self.node_meta_models.add(self.module_exports_directive_meta_model);
		self.node_meta_models.add(self.module_opens_directive_meta_model);
		self.node_meta_models.add(self.module_provides_directive_meta_model);
		self.node_meta_models.add(self.module_requires_directive_meta_model);
		self.node_meta_models.add(self.module_uses_directive_meta_model);
		self.node_meta_models.add(self.name_expr_meta_model);
		self.node_meta_models.add(self.name_meta_model);
		self.node_meta_models.add(self.node_meta_model);
		self.node_meta_models.add(self.normal_annotation_expr_meta_model);
		self.node_meta_models.add(self.null_literal_expr_meta_model);
		self.node_meta_models.add(self.object_creation_expr_meta_model);
		self.node_meta_models.add(self.package_declaration_meta_model);
		self.node_meta_models.add(self.parameter_meta_model);
		self.node_meta_models.add(self.pattern_expr_meta_model);
		self.node_meta_models.add(self.primitive_type_meta_model);
		self.node_meta_models.add(self.receiver_parameter_meta_model);
		self.node_meta_models.add(self.record_declaration_meta_model);
		self.node_meta_models.add(self.record_pattern_expr_meta_model);
		self.node_meta_models.add(self.reference_type_meta_model);
		self.node_meta_models.add(self.return_stmt_meta_model);
		self.node_meta_models.add(self.simple_name_meta_model);
		self.node_meta_models.add(self.single_member_annotation_expr_meta_model);
		self.node_meta_models.add(self.statement_meta_model);
		self.node_meta_models.add(self.string_literal_expr_meta_model);
		self.node_meta_models.add(self.super_expr_meta_model);
		self.node_meta_models.add(self.switch_entry_meta_model);
		self.node_meta_models.add(self.switch_expr_meta_model);
		self.node_meta_models.add(self.switch_stmt_meta_model);
		self.node_meta_models.add(self.synchronized_stmt_meta_model);
		self.node_meta_models.add(self.text_block_literal_expr_meta_model);
		self.node_meta_models.add(self.this_expr_meta_model);
		self.node_meta_models.add(self.throw_stmt_meta_model);
		self.node_meta_models.add(self.traditional_javadoc_comment_meta_model);
		self.node_meta_models.add(self.try_stmt_meta_model);
		self.node_meta_models.add(self.type_declaration_meta_model);
		self.node_meta_models.add(self.type_expr_meta_model);
		self.node_meta_models.add(self.type_meta_model);
		self.node_meta_models.add(self.type_parameter_meta_model);
		self.node_meta_models.add(self.type_pattern_expr_meta_model);
		self.node_meta_models.add(self.unary_expr_meta_model);
		self.node_meta_models.add(self.union_type_meta_model);
		self.node_meta_models.add(self.unknown_type_meta_model);
		self.node_meta_models.add(self.unparsable_stmt_meta_model);
		self.node_meta_models.add(self.var_type_meta_model);
		self.node_meta_models.add(self.variable_declaration_expr_meta_model);
		self.node_meta_models.add(self.variable_declarator_meta_model);
		self.node_meta_models.add(self.void_type_meta_model);
		self.node_meta_models.add(self.while_stmt_meta_model);
		self.node_meta_models.add(self.wildcard_type_meta_model);
		self.node_meta_models.add(self.yield_stmt_meta_model);
	}

	fn initialize_property_meta_models(&mut self) {
		self.node_meta_model.commentPropertyMetaModel = PropertyMetaModel::new(self.node_meta_model, "comment", com.github.javaparser.ast.comments.Comment.class, &Optional::of(self.comment_meta_model), true, false, false, false);
		self.node_meta_model.get_declared_property_meta_models().add(self.node_meta_model.commentPropertyMetaModel);
		self.body_declaration_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.body_declaration_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.body_declaration_meta_model.get_declared_property_meta_models().add(self.body_declaration_meta_model.annotationsPropertyMetaModel);
		self.callable_declaration_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.modifiersPropertyMetaModel);
		self.callable_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.namePropertyMetaModel);
		self.callable_declaration_meta_model.parametersPropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "parameters", com.github.javaparser.ast.body.Parameter.class, &Optional::of(self.parameter_meta_model), false, false, true, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.parametersPropertyMetaModel);
		self.callable_declaration_meta_model.receiverParameterPropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "receiverParameter", com.github.javaparser.ast.body.ReceiverParameter.class, &Optional::of(self.receiver_parameter_meta_model), true, false, false, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.callable_declaration_meta_model.thrownExceptionsPropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "thrownExceptions", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), false, false, true, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.callable_declaration_meta_model.typeParametersPropertyMetaModel = PropertyMetaModel::new(self.callable_declaration_meta_model, "typeParameters", com.github.javaparser.ast.type.TypeParameter.class, &Optional::of(self.type_parameter_meta_model), false, false, true, false);
		self.callable_declaration_meta_model.get_declared_property_meta_models().add(self.callable_declaration_meta_model.typeParametersPropertyMetaModel);
		self.type_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.type_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.type_meta_model.get_declared_property_meta_models().add(self.type_meta_model.annotationsPropertyMetaModel);
		self.annotation_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.annotation_expr_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.annotation_expr_meta_model.get_declared_property_meta_models().add(self.annotation_expr_meta_model.namePropertyMetaModel);
		self.type_declaration_meta_model.membersPropertyMetaModel = PropertyMetaModel::new(self.type_declaration_meta_model, "members", com.github.javaparser.ast.body.BodyDeclaration.class, &Optional::of(self.body_declaration_meta_model), false, false, true, true);
		self.type_declaration_meta_model.get_declared_property_meta_models().add(self.type_declaration_meta_model.membersPropertyMetaModel);
		self.type_declaration_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.type_declaration_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.type_declaration_meta_model.get_declared_property_meta_models().add(self.type_declaration_meta_model.modifiersPropertyMetaModel);
		self.type_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.type_declaration_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.type_declaration_meta_model.get_declared_property_meta_models().add(self.type_declaration_meta_model.namePropertyMetaModel);
		self.literal_string_value_expr_meta_model.valuePropertyMetaModel = PropertyMetaModel::new(self.literal_string_value_expr_meta_model, "value", java.lang.String.class, &Optional::empty(), false, false, false, false);
		self.literal_string_value_expr_meta_model.get_declared_property_meta_models().add(self.literal_string_value_expr_meta_model.valuePropertyMetaModel);
		self.module_declaration_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.module_declaration_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.module_declaration_meta_model.get_declared_property_meta_models().add(self.module_declaration_meta_model.annotationsPropertyMetaModel);
		self.module_declaration_meta_model.directivesPropertyMetaModel = PropertyMetaModel::new(self.module_declaration_meta_model, "directives", com.github.javaparser.ast.modules.ModuleDirective.class, &Optional::of(self.module_directive_meta_model), false, false, true, false);
		self.module_declaration_meta_model.get_declared_property_meta_models().add(self.module_declaration_meta_model.directivesPropertyMetaModel);
		self.module_declaration_meta_model.isOpenPropertyMetaModel = PropertyMetaModel::new(self.module_declaration_meta_model, "isOpen", bool.class, &Optional::empty(), false, false, false, false);
		self.module_declaration_meta_model.get_declared_property_meta_models().add(self.module_declaration_meta_model.isOpenPropertyMetaModel);
		self.module_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_declaration_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_declaration_meta_model.get_declared_property_meta_models().add(self.module_declaration_meta_model.namePropertyMetaModel);
		self.array_creation_level_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.array_creation_level_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.array_creation_level_meta_model.get_declared_property_meta_models().add(self.array_creation_level_meta_model.annotationsPropertyMetaModel);
		self.array_creation_level_meta_model.dimensionPropertyMetaModel = PropertyMetaModel::new(self.array_creation_level_meta_model, "dimension", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.array_creation_level_meta_model.get_declared_property_meta_models().add(self.array_creation_level_meta_model.dimensionPropertyMetaModel);
		self.compilation_unit_meta_model.importsPropertyMetaModel = PropertyMetaModel::new(self.compilation_unit_meta_model, "imports", com.github.javaparser.ast.ImportDeclaration.class, &Optional::of(self.import_declaration_meta_model), false, false, true, false);
		self.compilation_unit_meta_model.get_declared_property_meta_models().add(self.compilation_unit_meta_model.importsPropertyMetaModel);
		self.compilation_unit_meta_model.modulePropertyMetaModel = PropertyMetaModel::new(self.compilation_unit_meta_model, "module", com.github.javaparser.ast.modules.ModuleDeclaration.class, &Optional::of(self.module_declaration_meta_model), true, false, false, false);
		self.compilation_unit_meta_model.get_declared_property_meta_models().add(self.compilation_unit_meta_model.modulePropertyMetaModel);
		self.compilation_unit_meta_model.packageDeclarationPropertyMetaModel = PropertyMetaModel::new(self.compilation_unit_meta_model, "packageDeclaration", com.github.javaparser.ast.PackageDeclaration.class, &Optional::of(self.package_declaration_meta_model), true, false, false, false);
		self.compilation_unit_meta_model.get_declared_property_meta_models().add(self.compilation_unit_meta_model.packageDeclarationPropertyMetaModel);
		self.compilation_unit_meta_model.typesPropertyMetaModel = PropertyMetaModel::new(self.compilation_unit_meta_model, "types", com.github.javaparser.ast.body.TypeDeclaration.class, &Optional::of(self.type_declaration_meta_model), false, false, true, true);
		self.compilation_unit_meta_model.get_declared_property_meta_models().add(self.compilation_unit_meta_model.typesPropertyMetaModel);
		self.import_declaration_meta_model.isAsteriskPropertyMetaModel = PropertyMetaModel::new(self.import_declaration_meta_model, "isAsterisk", bool.class, &Optional::empty(), false, false, false, false);
		self.import_declaration_meta_model.get_declared_property_meta_models().add(self.import_declaration_meta_model.isAsteriskPropertyMetaModel);
		self.import_declaration_meta_model.isModulePropertyMetaModel = PropertyMetaModel::new(self.import_declaration_meta_model, "isModule", bool.class, &Optional::empty(), false, false, false, false);
		self.import_declaration_meta_model.get_declared_property_meta_models().add(self.import_declaration_meta_model.isModulePropertyMetaModel);
		self.import_declaration_meta_model.isStaticPropertyMetaModel = PropertyMetaModel::new(self.import_declaration_meta_model, "isStatic", bool.class, &Optional::empty(), false, false, false, false);
		self.import_declaration_meta_model.get_declared_property_meta_models().add(self.import_declaration_meta_model.isStaticPropertyMetaModel);
		self.import_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.import_declaration_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.import_declaration_meta_model.get_declared_property_meta_models().add(self.import_declaration_meta_model.namePropertyMetaModel);
		self.modifier_meta_model.keywordPropertyMetaModel = PropertyMetaModel::new(self.modifier_meta_model, "keyword", com.github.javaparser.ast.Modifier.Keyword.class, &Optional::empty(), false, false, false, false);
		self.modifier_meta_model.get_declared_property_meta_models().add(self.modifier_meta_model.keywordPropertyMetaModel);
		self.package_declaration_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.package_declaration_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.package_declaration_meta_model.get_declared_property_meta_models().add(self.package_declaration_meta_model.annotationsPropertyMetaModel);
		self.package_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.package_declaration_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.package_declaration_meta_model.get_declared_property_meta_models().add(self.package_declaration_meta_model.namePropertyMetaModel);
		self.annotation_member_declaration_meta_model.defaultValuePropertyMetaModel = PropertyMetaModel::new(self.annotation_member_declaration_meta_model, "defaultValue", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.annotation_member_declaration_meta_model.get_declared_property_meta_models().add(self.annotation_member_declaration_meta_model.defaultValuePropertyMetaModel);
		self.annotation_member_declaration_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.annotation_member_declaration_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.annotation_member_declaration_meta_model.get_declared_property_meta_models().add(self.annotation_member_declaration_meta_model.modifiersPropertyMetaModel);
		self.annotation_member_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.annotation_member_declaration_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.annotation_member_declaration_meta_model.get_declared_property_meta_models().add(self.annotation_member_declaration_meta_model.namePropertyMetaModel);
		self.annotation_member_declaration_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.annotation_member_declaration_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.annotation_member_declaration_meta_model.get_declared_property_meta_models().add(self.annotation_member_declaration_meta_model.typePropertyMetaModel);
		self.class_or_interface_declaration_meta_model.extendedTypesPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "extendedTypes", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.extendedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.implementedTypesPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "implementedTypes", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.isCompactPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "isCompact", bool.class, &Optional::empty(), false, false, false, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.isCompactPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.isInterfacePropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "isInterface", bool.class, &Optional::empty(), false, false, false, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.isInterfacePropertyMetaModel);
		self.class_or_interface_declaration_meta_model.permittedTypesPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "permittedTypes", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.permittedTypesPropertyMetaModel);
		self.class_or_interface_declaration_meta_model.typeParametersPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_declaration_meta_model, "typeParameters", com.github.javaparser.ast.type.TypeParameter.class, &Optional::of(self.type_parameter_meta_model), false, false, true, false);
		self.class_or_interface_declaration_meta_model.get_declared_property_meta_models().add(self.class_or_interface_declaration_meta_model.typeParametersPropertyMetaModel);
		self.constructor_declaration_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.constructor_declaration_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.constructor_declaration_meta_model.get_declared_property_meta_models().add(self.constructor_declaration_meta_model.bodyPropertyMetaModel);
		self.enum_constant_declaration_meta_model.argumentsPropertyMetaModel = PropertyMetaModel::new(self.enum_constant_declaration_meta_model, "arguments", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.enum_constant_declaration_meta_model.get_declared_property_meta_models().add(self.enum_constant_declaration_meta_model.argumentsPropertyMetaModel);
		self.enum_constant_declaration_meta_model.classBodyPropertyMetaModel = PropertyMetaModel::new(self.enum_constant_declaration_meta_model, "classBody", com.github.javaparser.ast.body.BodyDeclaration.class, &Optional::of(self.body_declaration_meta_model), false, false, true, true);
		self.enum_constant_declaration_meta_model.get_declared_property_meta_models().add(self.enum_constant_declaration_meta_model.classBodyPropertyMetaModel);
		self.enum_constant_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.enum_constant_declaration_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.enum_constant_declaration_meta_model.get_declared_property_meta_models().add(self.enum_constant_declaration_meta_model.namePropertyMetaModel);
		self.enum_declaration_meta_model.entriesPropertyMetaModel = PropertyMetaModel::new(self.enum_declaration_meta_model, "entries", com.github.javaparser.ast.body.EnumConstantDeclaration.class, &Optional::of(self.enum_constant_declaration_meta_model), false, false, true, false);
		self.enum_declaration_meta_model.get_declared_property_meta_models().add(self.enum_declaration_meta_model.entriesPropertyMetaModel);
		self.enum_declaration_meta_model.implementedTypesPropertyMetaModel = PropertyMetaModel::new(self.enum_declaration_meta_model, "implementedTypes", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.enum_declaration_meta_model.get_declared_property_meta_models().add(self.enum_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.field_declaration_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.field_declaration_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.field_declaration_meta_model.get_declared_property_meta_models().add(self.field_declaration_meta_model.modifiersPropertyMetaModel);
		self.field_declaration_meta_model.variablesPropertyMetaModel = PropertyMetaModel::new(self.field_declaration_meta_model, "variables", com.github.javaparser.ast.body.VariableDeclarator.class, &Optional::of(self.variable_declarator_meta_model), false, true, true, false);
		self.field_declaration_meta_model.get_declared_property_meta_models().add(self.field_declaration_meta_model.variablesPropertyMetaModel);
		self.field_declaration_meta_model.maximumCommonTypePropertyMetaModel = PropertyMetaModel::new(self.field_declaration_meta_model, "maximumCommonType", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, false, false);
		self.field_declaration_meta_model.get_derived_property_meta_models().add(self.field_declaration_meta_model.maximumCommonTypePropertyMetaModel);
		self.initializer_declaration_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.initializer_declaration_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.initializer_declaration_meta_model.get_declared_property_meta_models().add(self.initializer_declaration_meta_model.bodyPropertyMetaModel);
		self.initializer_declaration_meta_model.isStaticPropertyMetaModel = PropertyMetaModel::new(self.initializer_declaration_meta_model, "isStatic", bool.class, &Optional::empty(), false, false, false, false);
		self.initializer_declaration_meta_model.get_declared_property_meta_models().add(self.initializer_declaration_meta_model.isStaticPropertyMetaModel);
		self.method_declaration_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.method_declaration_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), true, false, false, false);
		self.method_declaration_meta_model.get_declared_property_meta_models().add(self.method_declaration_meta_model.bodyPropertyMetaModel);
		self.method_declaration_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.method_declaration_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.method_declaration_meta_model.get_declared_property_meta_models().add(self.method_declaration_meta_model.typePropertyMetaModel);
		self.parameter_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.annotationsPropertyMetaModel);
		self.parameter_meta_model.isVarArgsPropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "isVarArgs", bool.class, &Optional::empty(), false, false, false, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.isVarArgsPropertyMetaModel);
		self.parameter_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.modifiersPropertyMetaModel);
		self.parameter_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.namePropertyMetaModel);
		self.parameter_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.typePropertyMetaModel);
		self.parameter_meta_model.varArgsAnnotationsPropertyMetaModel = PropertyMetaModel::new(self.parameter_meta_model, "varArgsAnnotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.parameter_meta_model.get_declared_property_meta_models().add(self.parameter_meta_model.varArgsAnnotationsPropertyMetaModel);
		self.receiver_parameter_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.receiver_parameter_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.receiver_parameter_meta_model.get_declared_property_meta_models().add(self.receiver_parameter_meta_model.annotationsPropertyMetaModel);
		self.receiver_parameter_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.receiver_parameter_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.receiver_parameter_meta_model.get_declared_property_meta_models().add(self.receiver_parameter_meta_model.namePropertyMetaModel);
		self.receiver_parameter_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.receiver_parameter_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.receiver_parameter_meta_model.get_declared_property_meta_models().add(self.receiver_parameter_meta_model.typePropertyMetaModel);
		self.record_declaration_meta_model.implementedTypesPropertyMetaModel = PropertyMetaModel::new(self.record_declaration_meta_model, "implementedTypes", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.record_declaration_meta_model.get_declared_property_meta_models().add(self.record_declaration_meta_model.implementedTypesPropertyMetaModel);
		self.record_declaration_meta_model.parametersPropertyMetaModel = PropertyMetaModel::new(self.record_declaration_meta_model, "parameters", com.github.javaparser.ast.body.Parameter.class, &Optional::of(self.parameter_meta_model), false, false, true, false);
		self.record_declaration_meta_model.get_declared_property_meta_models().add(self.record_declaration_meta_model.parametersPropertyMetaModel);
		self.record_declaration_meta_model.receiverParameterPropertyMetaModel = PropertyMetaModel::new(self.record_declaration_meta_model, "receiverParameter", com.github.javaparser.ast.body.ReceiverParameter.class, &Optional::of(self.receiver_parameter_meta_model), true, false, false, false);
		self.record_declaration_meta_model.get_declared_property_meta_models().add(self.record_declaration_meta_model.receiverParameterPropertyMetaModel);
		self.record_declaration_meta_model.typeParametersPropertyMetaModel = PropertyMetaModel::new(self.record_declaration_meta_model, "typeParameters", com.github.javaparser.ast.type.TypeParameter.class, &Optional::of(self.type_parameter_meta_model), false, false, true, false);
		self.record_declaration_meta_model.get_declared_property_meta_models().add(self.record_declaration_meta_model.typeParametersPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.compact_constructor_declaration_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.compact_constructor_declaration_meta_model.get_declared_property_meta_models().add(self.compact_constructor_declaration_meta_model.bodyPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.compact_constructor_declaration_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.compact_constructor_declaration_meta_model.get_declared_property_meta_models().add(self.compact_constructor_declaration_meta_model.modifiersPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.compact_constructor_declaration_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.compact_constructor_declaration_meta_model.get_declared_property_meta_models().add(self.compact_constructor_declaration_meta_model.namePropertyMetaModel);
		self.compact_constructor_declaration_meta_model.thrownExceptionsPropertyMetaModel = PropertyMetaModel::new(self.compact_constructor_declaration_meta_model, "thrownExceptions", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), false, false, true, false);
		self.compact_constructor_declaration_meta_model.get_declared_property_meta_models().add(self.compact_constructor_declaration_meta_model.thrownExceptionsPropertyMetaModel);
		self.compact_constructor_declaration_meta_model.typeParametersPropertyMetaModel = PropertyMetaModel::new(self.compact_constructor_declaration_meta_model, "typeParameters", com.github.javaparser.ast.type.TypeParameter.class, &Optional::of(self.type_parameter_meta_model), false, false, true, false);
		self.compact_constructor_declaration_meta_model.get_declared_property_meta_models().add(self.compact_constructor_declaration_meta_model.typeParametersPropertyMetaModel);
		self.variable_declarator_meta_model.initializerPropertyMetaModel = PropertyMetaModel::new(self.variable_declarator_meta_model, "initializer", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, true, false, false);
		self.variable_declarator_meta_model.get_declared_property_meta_models().add(self.variable_declarator_meta_model.initializerPropertyMetaModel);
		self.variable_declarator_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.variable_declarator_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.variable_declarator_meta_model.get_declared_property_meta_models().add(self.variable_declarator_meta_model.namePropertyMetaModel);
		self.variable_declarator_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.variable_declarator_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.variable_declarator_meta_model.get_declared_property_meta_models().add(self.variable_declarator_meta_model.typePropertyMetaModel);
		self.comment_meta_model.contentPropertyMetaModel = PropertyMetaModel::new(self.comment_meta_model, "content", java.lang.String.class, &Optional::empty(), false, false, false, false);
		self.comment_meta_model.get_declared_property_meta_models().add(self.comment_meta_model.contentPropertyMetaModel);
		self.array_access_expr_meta_model.indexPropertyMetaModel = PropertyMetaModel::new(self.array_access_expr_meta_model, "index", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.array_access_expr_meta_model.get_declared_property_meta_models().add(self.array_access_expr_meta_model.indexPropertyMetaModel);
		self.array_access_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.array_access_expr_meta_model, "name", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.array_access_expr_meta_model.get_declared_property_meta_models().add(self.array_access_expr_meta_model.namePropertyMetaModel);
		self.array_creation_expr_meta_model.elementTypePropertyMetaModel = PropertyMetaModel::new(self.array_creation_expr_meta_model, "elementType", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.array_creation_expr_meta_model.get_declared_property_meta_models().add(self.array_creation_expr_meta_model.elementTypePropertyMetaModel);
		self.array_creation_expr_meta_model.initializerPropertyMetaModel = PropertyMetaModel::new(self.array_creation_expr_meta_model, "initializer", com.github.javaparser.ast.expr.ArrayInitializerExpr.class, &Optional::of(self.array_initializer_expr_meta_model), true, false, false, false);
		self.array_creation_expr_meta_model.get_declared_property_meta_models().add(self.array_creation_expr_meta_model.initializerPropertyMetaModel);
		self.array_creation_expr_meta_model.levelsPropertyMetaModel = PropertyMetaModel::new(self.array_creation_expr_meta_model, "levels", com.github.javaparser.ast.ArrayCreationLevel.class, &Optional::of(self.array_creation_level_meta_model), false, true, true, false);
		self.array_creation_expr_meta_model.get_declared_property_meta_models().add(self.array_creation_expr_meta_model.levelsPropertyMetaModel);
		self.array_initializer_expr_meta_model.valuesPropertyMetaModel = PropertyMetaModel::new(self.array_initializer_expr_meta_model, "values", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.array_initializer_expr_meta_model.get_declared_property_meta_models().add(self.array_initializer_expr_meta_model.valuesPropertyMetaModel);
		self.assign_expr_meta_model.operatorPropertyMetaModel = PropertyMetaModel::new(self.assign_expr_meta_model, "operator", com.github.javaparser.ast.expr.AssignExpr.Operator.class, &Optional::empty(), false, false, false, false);
		self.assign_expr_meta_model.get_declared_property_meta_models().add(self.assign_expr_meta_model.operatorPropertyMetaModel);
		self.assign_expr_meta_model.targetPropertyMetaModel = PropertyMetaModel::new(self.assign_expr_meta_model, "target", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.assign_expr_meta_model.get_declared_property_meta_models().add(self.assign_expr_meta_model.targetPropertyMetaModel);
		self.assign_expr_meta_model.valuePropertyMetaModel = PropertyMetaModel::new(self.assign_expr_meta_model, "value", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.assign_expr_meta_model.get_declared_property_meta_models().add(self.assign_expr_meta_model.valuePropertyMetaModel);
		self.binary_expr_meta_model.leftPropertyMetaModel = PropertyMetaModel::new(self.binary_expr_meta_model, "left", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.binary_expr_meta_model.get_declared_property_meta_models().add(self.binary_expr_meta_model.leftPropertyMetaModel);
		self.binary_expr_meta_model.operatorPropertyMetaModel = PropertyMetaModel::new(self.binary_expr_meta_model, "operator", com.github.javaparser.ast.expr.BinaryExpr.Operator.class, &Optional::empty(), false, false, false, false);
		self.binary_expr_meta_model.get_declared_property_meta_models().add(self.binary_expr_meta_model.operatorPropertyMetaModel);
		self.binary_expr_meta_model.rightPropertyMetaModel = PropertyMetaModel::new(self.binary_expr_meta_model, "right", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.binary_expr_meta_model.get_declared_property_meta_models().add(self.binary_expr_meta_model.rightPropertyMetaModel);
		self.boolean_literal_expr_meta_model.valuePropertyMetaModel = PropertyMetaModel::new(self.boolean_literal_expr_meta_model, "value", bool.class, &Optional::empty(), false, false, false, false);
		self.boolean_literal_expr_meta_model.get_declared_property_meta_models().add(self.boolean_literal_expr_meta_model.valuePropertyMetaModel);
		self.cast_expr_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.cast_expr_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.cast_expr_meta_model.get_declared_property_meta_models().add(self.cast_expr_meta_model.expressionPropertyMetaModel);
		self.cast_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.cast_expr_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.cast_expr_meta_model.get_declared_property_meta_models().add(self.cast_expr_meta_model.typePropertyMetaModel);
		self.class_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.class_expr_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.class_expr_meta_model.get_declared_property_meta_models().add(self.class_expr_meta_model.typePropertyMetaModel);
		self.conditional_expr_meta_model.conditionPropertyMetaModel = PropertyMetaModel::new(self.conditional_expr_meta_model, "condition", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.conditional_expr_meta_model.get_declared_property_meta_models().add(self.conditional_expr_meta_model.conditionPropertyMetaModel);
		self.conditional_expr_meta_model.elseExprPropertyMetaModel = PropertyMetaModel::new(self.conditional_expr_meta_model, "elseExpr", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.conditional_expr_meta_model.get_declared_property_meta_models().add(self.conditional_expr_meta_model.elseExprPropertyMetaModel);
		self.conditional_expr_meta_model.thenExprPropertyMetaModel = PropertyMetaModel::new(self.conditional_expr_meta_model, "thenExpr", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.conditional_expr_meta_model.get_declared_property_meta_models().add(self.conditional_expr_meta_model.thenExprPropertyMetaModel);
		self.enclosed_expr_meta_model.innerPropertyMetaModel = PropertyMetaModel::new(self.enclosed_expr_meta_model, "inner", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.enclosed_expr_meta_model.get_declared_property_meta_models().add(self.enclosed_expr_meta_model.innerPropertyMetaModel);
		self.field_access_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.field_access_expr_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.field_access_expr_meta_model.get_declared_property_meta_models().add(self.field_access_expr_meta_model.namePropertyMetaModel);
		self.field_access_expr_meta_model.scopePropertyMetaModel = PropertyMetaModel::new(self.field_access_expr_meta_model, "scope", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.field_access_expr_meta_model.get_declared_property_meta_models().add(self.field_access_expr_meta_model.scopePropertyMetaModel);
		self.field_access_expr_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.field_access_expr_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.field_access_expr_meta_model.get_declared_property_meta_models().add(self.field_access_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.field_access_expr_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.field_access_expr_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.field_access_expr_meta_model.get_derived_property_meta_models().add(self.field_access_expr_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.instance_of_expr_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.instance_of_expr_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.instance_of_expr_meta_model.get_declared_property_meta_models().add(self.instance_of_expr_meta_model.expressionPropertyMetaModel);
		self.instance_of_expr_meta_model.patternPropertyMetaModel = PropertyMetaModel::new(self.instance_of_expr_meta_model, "pattern", com.github.javaparser.ast.expr.PatternExpr.class, &Optional::of(self.pattern_expr_meta_model), true, false, false, false);
		self.instance_of_expr_meta_model.get_declared_property_meta_models().add(self.instance_of_expr_meta_model.patternPropertyMetaModel);
		self.instance_of_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.instance_of_expr_meta_model, "type", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), false, false, false, false);
		self.instance_of_expr_meta_model.get_declared_property_meta_models().add(self.instance_of_expr_meta_model.typePropertyMetaModel);
		self.lambda_expr_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.lambda_expr_meta_model, "body", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.lambda_expr_meta_model.get_declared_property_meta_models().add(self.lambda_expr_meta_model.bodyPropertyMetaModel);
		self.lambda_expr_meta_model.isEnclosingParametersPropertyMetaModel = PropertyMetaModel::new(self.lambda_expr_meta_model, "isEnclosingParameters", bool.class, &Optional::empty(), false, false, false, false);
		self.lambda_expr_meta_model.get_declared_property_meta_models().add(self.lambda_expr_meta_model.isEnclosingParametersPropertyMetaModel);
		self.lambda_expr_meta_model.parametersPropertyMetaModel = PropertyMetaModel::new(self.lambda_expr_meta_model, "parameters", com.github.javaparser.ast.body.Parameter.class, &Optional::of(self.parameter_meta_model), false, false, true, false);
		self.lambda_expr_meta_model.get_declared_property_meta_models().add(self.lambda_expr_meta_model.parametersPropertyMetaModel);
		self.lambda_expr_meta_model.expressionBodyPropertyMetaModel = PropertyMetaModel::new(self.lambda_expr_meta_model, "expressionBody", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.lambda_expr_meta_model.get_derived_property_meta_models().add(self.lambda_expr_meta_model.expressionBodyPropertyMetaModel);
		self.member_value_pair_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.member_value_pair_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.member_value_pair_meta_model.get_declared_property_meta_models().add(self.member_value_pair_meta_model.namePropertyMetaModel);
		self.member_value_pair_meta_model.valuePropertyMetaModel = PropertyMetaModel::new(self.member_value_pair_meta_model, "value", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.member_value_pair_meta_model.get_declared_property_meta_models().add(self.member_value_pair_meta_model.valuePropertyMetaModel);
		self.method_call_expr_meta_model.argumentsPropertyMetaModel = PropertyMetaModel::new(self.method_call_expr_meta_model, "arguments", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.method_call_expr_meta_model.get_declared_property_meta_models().add(self.method_call_expr_meta_model.argumentsPropertyMetaModel);
		self.method_call_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.method_call_expr_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.method_call_expr_meta_model.get_declared_property_meta_models().add(self.method_call_expr_meta_model.namePropertyMetaModel);
		self.method_call_expr_meta_model.scopePropertyMetaModel = PropertyMetaModel::new(self.method_call_expr_meta_model, "scope", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.method_call_expr_meta_model.get_declared_property_meta_models().add(self.method_call_expr_meta_model.scopePropertyMetaModel);
		self.method_call_expr_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.method_call_expr_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.method_call_expr_meta_model.get_declared_property_meta_models().add(self.method_call_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.method_call_expr_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.method_call_expr_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.method_call_expr_meta_model.get_derived_property_meta_models().add(self.method_call_expr_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.method_reference_expr_meta_model.identifierPropertyMetaModel = PropertyMetaModel::new(self.method_reference_expr_meta_model, "identifier", java.lang.String.class, &Optional::empty(), false, true, false, false);
		self.method_reference_expr_meta_model.get_declared_property_meta_models().add(self.method_reference_expr_meta_model.identifierPropertyMetaModel);
		self.method_reference_expr_meta_model.scopePropertyMetaModel = PropertyMetaModel::new(self.method_reference_expr_meta_model, "scope", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.method_reference_expr_meta_model.get_declared_property_meta_models().add(self.method_reference_expr_meta_model.scopePropertyMetaModel);
		self.method_reference_expr_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.method_reference_expr_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.method_reference_expr_meta_model.get_declared_property_meta_models().add(self.method_reference_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.method_reference_expr_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.method_reference_expr_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.method_reference_expr_meta_model.get_derived_property_meta_models().add(self.method_reference_expr_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.name_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.name_expr_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.name_expr_meta_model.get_declared_property_meta_models().add(self.name_expr_meta_model.namePropertyMetaModel);
		self.name_meta_model.identifierPropertyMetaModel = PropertyMetaModel::new(self.name_meta_model, "identifier", java.lang.String.class, &Optional::empty(), false, true, false, false);
		self.name_meta_model.get_declared_property_meta_models().add(self.name_meta_model.identifierPropertyMetaModel);
		self.name_meta_model.qualifierPropertyMetaModel = PropertyMetaModel::new(self.name_meta_model, "qualifier", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), true, false, false, false);
		self.name_meta_model.get_declared_property_meta_models().add(self.name_meta_model.qualifierPropertyMetaModel);
		self.normal_annotation_expr_meta_model.pairsPropertyMetaModel = PropertyMetaModel::new(self.normal_annotation_expr_meta_model, "pairs", com.github.javaparser.ast.expr.MemberValuePair.class, &Optional::of(self.member_value_pair_meta_model), false, false, true, false);
		self.normal_annotation_expr_meta_model.get_declared_property_meta_models().add(self.normal_annotation_expr_meta_model.pairsPropertyMetaModel);
		self.object_creation_expr_meta_model.anonymousClassBodyPropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "anonymousClassBody", com.github.javaparser.ast.body.BodyDeclaration.class, &Optional::of(self.body_declaration_meta_model), true, false, true, true);
		self.object_creation_expr_meta_model.get_declared_property_meta_models().add(self.object_creation_expr_meta_model.anonymousClassBodyPropertyMetaModel);
		self.object_creation_expr_meta_model.argumentsPropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "arguments", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.object_creation_expr_meta_model.get_declared_property_meta_models().add(self.object_creation_expr_meta_model.argumentsPropertyMetaModel);
		self.object_creation_expr_meta_model.scopePropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "scope", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.object_creation_expr_meta_model.get_declared_property_meta_models().add(self.object_creation_expr_meta_model.scopePropertyMetaModel);
		self.object_creation_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "type", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, false, false);
		self.object_creation_expr_meta_model.get_declared_property_meta_models().add(self.object_creation_expr_meta_model.typePropertyMetaModel);
		self.object_creation_expr_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.object_creation_expr_meta_model.get_declared_property_meta_models().add(self.object_creation_expr_meta_model.typeArgumentsPropertyMetaModel);
		self.object_creation_expr_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.object_creation_expr_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.object_creation_expr_meta_model.get_derived_property_meta_models().add(self.object_creation_expr_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.pattern_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.pattern_expr_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.pattern_expr_meta_model.get_declared_property_meta_models().add(self.pattern_expr_meta_model.typePropertyMetaModel);
		self.record_pattern_expr_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.record_pattern_expr_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.record_pattern_expr_meta_model.get_declared_property_meta_models().add(self.record_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.record_pattern_expr_meta_model.patternListPropertyMetaModel = PropertyMetaModel::new(self.record_pattern_expr_meta_model, "patternList", com.github.javaparser.ast.expr.ComponentPatternExpr.class, &Optional::of(self.component_pattern_expr_meta_model), false, false, true, false);
		self.record_pattern_expr_meta_model.get_declared_property_meta_models().add(self.record_pattern_expr_meta_model.patternListPropertyMetaModel);
		self.single_member_annotation_expr_meta_model.memberValuePropertyMetaModel = PropertyMetaModel::new(self.single_member_annotation_expr_meta_model, "memberValue", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.single_member_annotation_expr_meta_model.get_declared_property_meta_models().add(self.single_member_annotation_expr_meta_model.memberValuePropertyMetaModel);
		self.simple_name_meta_model.identifierPropertyMetaModel = PropertyMetaModel::new(self.simple_name_meta_model, "identifier", java.lang.String.class, &Optional::empty(), false, true, false, false);
		self.simple_name_meta_model.get_declared_property_meta_models().add(self.simple_name_meta_model.identifierPropertyMetaModel);
		self.super_expr_meta_model.typeNamePropertyMetaModel = PropertyMetaModel::new(self.super_expr_meta_model, "typeName", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), true, false, false, false);
		self.super_expr_meta_model.get_declared_property_meta_models().add(self.super_expr_meta_model.typeNamePropertyMetaModel);
		self.switch_expr_meta_model.entriesPropertyMetaModel = PropertyMetaModel::new(self.switch_expr_meta_model, "entries", com.github.javaparser.ast.stmt.SwitchEntry.class, &Optional::of(self.switch_entry_meta_model), false, false, true, false);
		self.switch_expr_meta_model.get_declared_property_meta_models().add(self.switch_expr_meta_model.entriesPropertyMetaModel);
		self.switch_expr_meta_model.selectorPropertyMetaModel = PropertyMetaModel::new(self.switch_expr_meta_model, "selector", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.switch_expr_meta_model.get_declared_property_meta_models().add(self.switch_expr_meta_model.selectorPropertyMetaModel);
		self.this_expr_meta_model.typeNamePropertyMetaModel = PropertyMetaModel::new(self.this_expr_meta_model, "typeName", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), true, false, false, false);
		self.this_expr_meta_model.get_declared_property_meta_models().add(self.this_expr_meta_model.typeNamePropertyMetaModel);
		self.type_expr_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.type_expr_meta_model, "type", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.type_expr_meta_model.get_declared_property_meta_models().add(self.type_expr_meta_model.typePropertyMetaModel);
		self.type_pattern_expr_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.type_pattern_expr_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.type_pattern_expr_meta_model.get_declared_property_meta_models().add(self.type_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.type_pattern_expr_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.type_pattern_expr_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.type_pattern_expr_meta_model.get_declared_property_meta_models().add(self.type_pattern_expr_meta_model.namePropertyMetaModel);
		self.unary_expr_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.unary_expr_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.unary_expr_meta_model.get_declared_property_meta_models().add(self.unary_expr_meta_model.expressionPropertyMetaModel);
		self.unary_expr_meta_model.operatorPropertyMetaModel = PropertyMetaModel::new(self.unary_expr_meta_model, "operator", com.github.javaparser.ast.expr.UnaryExpr.Operator.class, &Optional::empty(), false, false, false, false);
		self.unary_expr_meta_model.get_declared_property_meta_models().add(self.unary_expr_meta_model.operatorPropertyMetaModel);
		self.unary_expr_meta_model.postfixPropertyMetaModel = PropertyMetaModel::new(self.unary_expr_meta_model, "postfix", bool.class, &Optional::empty(), false, false, false, false);
		self.unary_expr_meta_model.get_derived_property_meta_models().add(self.unary_expr_meta_model.postfixPropertyMetaModel);
		self.unary_expr_meta_model.prefixPropertyMetaModel = PropertyMetaModel::new(self.unary_expr_meta_model, "prefix", bool.class, &Optional::empty(), false, false, false, false);
		self.unary_expr_meta_model.get_derived_property_meta_models().add(self.unary_expr_meta_model.prefixPropertyMetaModel);
		self.match_all_pattern_expr_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.match_all_pattern_expr_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.match_all_pattern_expr_meta_model.get_declared_property_meta_models().add(self.match_all_pattern_expr_meta_model.modifiersPropertyMetaModel);
		self.variable_declaration_expr_meta_model.annotationsPropertyMetaModel = PropertyMetaModel::new(self.variable_declaration_expr_meta_model, "annotations", com.github.javaparser.ast.expr.AnnotationExpr.class, &Optional::of(self.annotation_expr_meta_model), false, false, true, false);
		self.variable_declaration_expr_meta_model.get_declared_property_meta_models().add(self.variable_declaration_expr_meta_model.annotationsPropertyMetaModel);
		self.variable_declaration_expr_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.variable_declaration_expr_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.variable_declaration_expr_meta_model.get_declared_property_meta_models().add(self.variable_declaration_expr_meta_model.modifiersPropertyMetaModel);
		self.variable_declaration_expr_meta_model.variablesPropertyMetaModel = PropertyMetaModel::new(self.variable_declaration_expr_meta_model, "variables", com.github.javaparser.ast.body.VariableDeclarator.class, &Optional::of(self.variable_declarator_meta_model), false, true, true, false);
		self.variable_declaration_expr_meta_model.get_declared_property_meta_models().add(self.variable_declaration_expr_meta_model.variablesPropertyMetaModel);
		self.variable_declaration_expr_meta_model.maximumCommonTypePropertyMetaModel = PropertyMetaModel::new(self.variable_declaration_expr_meta_model, "maximumCommonType", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, false, false);
		self.variable_declaration_expr_meta_model.get_derived_property_meta_models().add(self.variable_declaration_expr_meta_model.maximumCommonTypePropertyMetaModel);
		self.assert_stmt_meta_model.checkPropertyMetaModel = PropertyMetaModel::new(self.assert_stmt_meta_model, "check", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.assert_stmt_meta_model.get_declared_property_meta_models().add(self.assert_stmt_meta_model.checkPropertyMetaModel);
		self.assert_stmt_meta_model.messagePropertyMetaModel = PropertyMetaModel::new(self.assert_stmt_meta_model, "message", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.assert_stmt_meta_model.get_declared_property_meta_models().add(self.assert_stmt_meta_model.messagePropertyMetaModel);
		self.block_stmt_meta_model.statementsPropertyMetaModel = PropertyMetaModel::new(self.block_stmt_meta_model, "statements", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, true, false);
		self.block_stmt_meta_model.get_declared_property_meta_models().add(self.block_stmt_meta_model.statementsPropertyMetaModel);
		self.break_stmt_meta_model.labelPropertyMetaModel = PropertyMetaModel::new(self.break_stmt_meta_model, "label", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), true, false, false, false);
		self.break_stmt_meta_model.get_declared_property_meta_models().add(self.break_stmt_meta_model.labelPropertyMetaModel);
		self.catch_clause_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.catch_clause_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.catch_clause_meta_model.get_declared_property_meta_models().add(self.catch_clause_meta_model.bodyPropertyMetaModel);
		self.catch_clause_meta_model.parameterPropertyMetaModel = PropertyMetaModel::new(self.catch_clause_meta_model, "parameter", com.github.javaparser.ast.body.Parameter.class, &Optional::of(self.parameter_meta_model), false, false, false, false);
		self.catch_clause_meta_model.get_declared_property_meta_models().add(self.catch_clause_meta_model.parameterPropertyMetaModel);
		self.continue_stmt_meta_model.labelPropertyMetaModel = PropertyMetaModel::new(self.continue_stmt_meta_model, "label", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), true, false, false, false);
		self.continue_stmt_meta_model.get_declared_property_meta_models().add(self.continue_stmt_meta_model.labelPropertyMetaModel);
		self.do_stmt_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.do_stmt_meta_model, "body", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.do_stmt_meta_model.get_declared_property_meta_models().add(self.do_stmt_meta_model.bodyPropertyMetaModel);
		self.do_stmt_meta_model.conditionPropertyMetaModel = PropertyMetaModel::new(self.do_stmt_meta_model, "condition", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.do_stmt_meta_model.get_declared_property_meta_models().add(self.do_stmt_meta_model.conditionPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.argumentsPropertyMetaModel = PropertyMetaModel::new(self.explicit_constructor_invocation_stmt_meta_model, "arguments", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.explicit_constructor_invocation_stmt_meta_model.get_declared_property_meta_models().add(self.explicit_constructor_invocation_stmt_meta_model.argumentsPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.explicit_constructor_invocation_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.explicit_constructor_invocation_stmt_meta_model.get_declared_property_meta_models().add(self.explicit_constructor_invocation_stmt_meta_model.expressionPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.isThisPropertyMetaModel = PropertyMetaModel::new(self.explicit_constructor_invocation_stmt_meta_model, "isThis", bool.class, &Optional::empty(), false, false, false, false);
		self.explicit_constructor_invocation_stmt_meta_model.get_declared_property_meta_models().add(self.explicit_constructor_invocation_stmt_meta_model.isThisPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.explicit_constructor_invocation_stmt_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.explicit_constructor_invocation_stmt_meta_model.get_declared_property_meta_models().add(self.explicit_constructor_invocation_stmt_meta_model.typeArgumentsPropertyMetaModel);
		self.explicit_constructor_invocation_stmt_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.explicit_constructor_invocation_stmt_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.explicit_constructor_invocation_stmt_meta_model.get_derived_property_meta_models().add(self.explicit_constructor_invocation_stmt_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.expression_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.expression_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.expression_stmt_meta_model.get_declared_property_meta_models().add(self.expression_stmt_meta_model.expressionPropertyMetaModel);
		self.for_each_stmt_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.for_each_stmt_meta_model, "body", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.for_each_stmt_meta_model.get_declared_property_meta_models().add(self.for_each_stmt_meta_model.bodyPropertyMetaModel);
		self.for_each_stmt_meta_model.iterablePropertyMetaModel = PropertyMetaModel::new(self.for_each_stmt_meta_model, "iterable", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.for_each_stmt_meta_model.get_declared_property_meta_models().add(self.for_each_stmt_meta_model.iterablePropertyMetaModel);
		self.for_each_stmt_meta_model.variablePropertyMetaModel = PropertyMetaModel::new(self.for_each_stmt_meta_model, "variable", com.github.javaparser.ast.expr.VariableDeclarationExpr.class, &Optional::of(self.variable_declaration_expr_meta_model), false, false, false, false);
		self.for_each_stmt_meta_model.get_declared_property_meta_models().add(self.for_each_stmt_meta_model.variablePropertyMetaModel);
		self.for_stmt_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.for_stmt_meta_model, "body", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.for_stmt_meta_model.get_declared_property_meta_models().add(self.for_stmt_meta_model.bodyPropertyMetaModel);
		self.for_stmt_meta_model.comparePropertyMetaModel = PropertyMetaModel::new(self.for_stmt_meta_model, "compare", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.for_stmt_meta_model.get_declared_property_meta_models().add(self.for_stmt_meta_model.comparePropertyMetaModel);
		self.for_stmt_meta_model.initializationPropertyMetaModel = PropertyMetaModel::new(self.for_stmt_meta_model, "initialization", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.for_stmt_meta_model.get_declared_property_meta_models().add(self.for_stmt_meta_model.initializationPropertyMetaModel);
		self.for_stmt_meta_model.updatePropertyMetaModel = PropertyMetaModel::new(self.for_stmt_meta_model, "update", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.for_stmt_meta_model.get_declared_property_meta_models().add(self.for_stmt_meta_model.updatePropertyMetaModel);
		self.if_stmt_meta_model.conditionPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "condition", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.if_stmt_meta_model.get_declared_property_meta_models().add(self.if_stmt_meta_model.conditionPropertyMetaModel);
		self.if_stmt_meta_model.elseStmtPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "elseStmt", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), true, false, false, false);
		self.if_stmt_meta_model.get_declared_property_meta_models().add(self.if_stmt_meta_model.elseStmtPropertyMetaModel);
		self.if_stmt_meta_model.thenStmtPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "thenStmt", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.if_stmt_meta_model.get_declared_property_meta_models().add(self.if_stmt_meta_model.thenStmtPropertyMetaModel);
		self.if_stmt_meta_model.cascadingIfStmtPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "cascadingIfStmt", bool.class, &Optional::empty(), false, false, false, false);
		self.if_stmt_meta_model.get_derived_property_meta_models().add(self.if_stmt_meta_model.cascadingIfStmtPropertyMetaModel);
		self.if_stmt_meta_model.elseBlockPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "elseBlock", bool.class, &Optional::empty(), false, false, false, false);
		self.if_stmt_meta_model.get_derived_property_meta_models().add(self.if_stmt_meta_model.elseBlockPropertyMetaModel);
		self.if_stmt_meta_model.elseBranchPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "elseBranch", bool.class, &Optional::empty(), false, false, false, false);
		self.if_stmt_meta_model.get_derived_property_meta_models().add(self.if_stmt_meta_model.elseBranchPropertyMetaModel);
		self.if_stmt_meta_model.thenBlockPropertyMetaModel = PropertyMetaModel::new(self.if_stmt_meta_model, "thenBlock", bool.class, &Optional::empty(), false, false, false, false);
		self.if_stmt_meta_model.get_derived_property_meta_models().add(self.if_stmt_meta_model.thenBlockPropertyMetaModel);
		self.labeled_stmt_meta_model.labelPropertyMetaModel = PropertyMetaModel::new(self.labeled_stmt_meta_model, "label", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.labeled_stmt_meta_model.get_declared_property_meta_models().add(self.labeled_stmt_meta_model.labelPropertyMetaModel);
		self.labeled_stmt_meta_model.statementPropertyMetaModel = PropertyMetaModel::new(self.labeled_stmt_meta_model, "statement", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.labeled_stmt_meta_model.get_declared_property_meta_models().add(self.labeled_stmt_meta_model.statementPropertyMetaModel);
		self.local_class_declaration_stmt_meta_model.classDeclarationPropertyMetaModel = PropertyMetaModel::new(self.local_class_declaration_stmt_meta_model, "classDeclaration", com.github.javaparser.ast.body.ClassOrInterfaceDeclaration.class, &Optional::of(self.class_or_interface_declaration_meta_model), false, false, false, false);
		self.local_class_declaration_stmt_meta_model.get_declared_property_meta_models().add(self.local_class_declaration_stmt_meta_model.classDeclarationPropertyMetaModel);
		self.local_record_declaration_stmt_meta_model.recordDeclarationPropertyMetaModel = PropertyMetaModel::new(self.local_record_declaration_stmt_meta_model, "recordDeclaration", com.github.javaparser.ast.body.RecordDeclaration.class, &Optional::of(self.record_declaration_meta_model), false, false, false, false);
		self.local_record_declaration_stmt_meta_model.get_declared_property_meta_models().add(self.local_record_declaration_stmt_meta_model.recordDeclarationPropertyMetaModel);
		self.return_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.return_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.return_stmt_meta_model.get_declared_property_meta_models().add(self.return_stmt_meta_model.expressionPropertyMetaModel);
		self.switch_entry_meta_model.guardPropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "guard", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), true, false, false, false);
		self.switch_entry_meta_model.get_declared_property_meta_models().add(self.switch_entry_meta_model.guardPropertyMetaModel);
		self.switch_entry_meta_model.isDefaultPropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "isDefault", bool.class, &Optional::empty(), false, false, false, false);
		self.switch_entry_meta_model.get_declared_property_meta_models().add(self.switch_entry_meta_model.isDefaultPropertyMetaModel);
		self.switch_entry_meta_model.labelsPropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "labels", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.switch_entry_meta_model.get_declared_property_meta_models().add(self.switch_entry_meta_model.labelsPropertyMetaModel);
		self.switch_entry_meta_model.statementsPropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "statements", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, true, false);
		self.switch_entry_meta_model.get_declared_property_meta_models().add(self.switch_entry_meta_model.statementsPropertyMetaModel);
		self.switch_entry_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "type", com.github.javaparser.ast.stmt.SwitchEntry.Type.class, &Optional::empty(), false, false, false, false);
		self.switch_entry_meta_model.get_declared_property_meta_models().add(self.switch_entry_meta_model.typePropertyMetaModel);
		self.switch_entry_meta_model.switchStatementEntryPropertyMetaModel = PropertyMetaModel::new(self.switch_entry_meta_model, "switchStatementEntry", bool.class, &Optional::empty(), false, false, false, false);
		self.switch_entry_meta_model.get_derived_property_meta_models().add(self.switch_entry_meta_model.switchStatementEntryPropertyMetaModel);
		self.switch_stmt_meta_model.entriesPropertyMetaModel = PropertyMetaModel::new(self.switch_stmt_meta_model, "entries", com.github.javaparser.ast.stmt.SwitchEntry.class, &Optional::of(self.switch_entry_meta_model), false, false, true, false);
		self.switch_stmt_meta_model.get_declared_property_meta_models().add(self.switch_stmt_meta_model.entriesPropertyMetaModel);
		self.switch_stmt_meta_model.selectorPropertyMetaModel = PropertyMetaModel::new(self.switch_stmt_meta_model, "selector", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.switch_stmt_meta_model.get_declared_property_meta_models().add(self.switch_stmt_meta_model.selectorPropertyMetaModel);
		self.synchronized_stmt_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.synchronized_stmt_meta_model, "body", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.synchronized_stmt_meta_model.get_declared_property_meta_models().add(self.synchronized_stmt_meta_model.bodyPropertyMetaModel);
		self.synchronized_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.synchronized_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.synchronized_stmt_meta_model.get_declared_property_meta_models().add(self.synchronized_stmt_meta_model.expressionPropertyMetaModel);
		self.throw_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.throw_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.throw_stmt_meta_model.get_declared_property_meta_models().add(self.throw_stmt_meta_model.expressionPropertyMetaModel);
		self.try_stmt_meta_model.catchClausesPropertyMetaModel = PropertyMetaModel::new(self.try_stmt_meta_model, "catchClauses", com.github.javaparser.ast.stmt.CatchClause.class, &Optional::of(self.catch_clause_meta_model), false, false, true, false);
		self.try_stmt_meta_model.get_declared_property_meta_models().add(self.try_stmt_meta_model.catchClausesPropertyMetaModel);
		self.try_stmt_meta_model.finallyBlockPropertyMetaModel = PropertyMetaModel::new(self.try_stmt_meta_model, "finallyBlock", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), true, false, false, false);
		self.try_stmt_meta_model.get_declared_property_meta_models().add(self.try_stmt_meta_model.finallyBlockPropertyMetaModel);
		self.try_stmt_meta_model.resourcesPropertyMetaModel = PropertyMetaModel::new(self.try_stmt_meta_model, "resources", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, true, false);
		self.try_stmt_meta_model.get_declared_property_meta_models().add(self.try_stmt_meta_model.resourcesPropertyMetaModel);
		self.try_stmt_meta_model.tryBlockPropertyMetaModel = PropertyMetaModel::new(self.try_stmt_meta_model, "tryBlock", com.github.javaparser.ast.stmt.BlockStmt.class, &Optional::of(self.block_stmt_meta_model), false, false, false, false);
		self.try_stmt_meta_model.get_declared_property_meta_models().add(self.try_stmt_meta_model.tryBlockPropertyMetaModel);
		self.while_stmt_meta_model.bodyPropertyMetaModel = PropertyMetaModel::new(self.while_stmt_meta_model, "body", com.github.javaparser.ast.stmt.Statement.class, &Optional::of(self.statement_meta_model), false, false, false, false);
		self.while_stmt_meta_model.get_declared_property_meta_models().add(self.while_stmt_meta_model.bodyPropertyMetaModel);
		self.while_stmt_meta_model.conditionPropertyMetaModel = PropertyMetaModel::new(self.while_stmt_meta_model, "condition", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.while_stmt_meta_model.get_declared_property_meta_models().add(self.while_stmt_meta_model.conditionPropertyMetaModel);
		self.yield_stmt_meta_model.expressionPropertyMetaModel = PropertyMetaModel::new(self.yield_stmt_meta_model, "expression", com.github.javaparser.ast.expr.Expression.class, &Optional::of(self.expression_meta_model), false, false, false, false);
		self.yield_stmt_meta_model.get_declared_property_meta_models().add(self.yield_stmt_meta_model.expressionPropertyMetaModel);
		self.array_type_meta_model.componentTypePropertyMetaModel = PropertyMetaModel::new(self.array_type_meta_model, "componentType", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), false, false, false, false);
		self.array_type_meta_model.get_declared_property_meta_models().add(self.array_type_meta_model.componentTypePropertyMetaModel);
		self.array_type_meta_model.originPropertyMetaModel = PropertyMetaModel::new(self.array_type_meta_model, "origin", com.github.javaparser.ast.type.ArrayType.Origin.class, &Optional::empty(), false, false, false, false);
		self.array_type_meta_model.get_declared_property_meta_models().add(self.array_type_meta_model.originPropertyMetaModel);
		self.class_or_interface_type_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_type_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.class_or_interface_type_meta_model.get_declared_property_meta_models().add(self.class_or_interface_type_meta_model.namePropertyMetaModel);
		self.class_or_interface_type_meta_model.scopePropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_type_meta_model, "scope", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), true, false, false, false);
		self.class_or_interface_type_meta_model.get_declared_property_meta_models().add(self.class_or_interface_type_meta_model.scopePropertyMetaModel);
		self.class_or_interface_type_meta_model.typeArgumentsPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_type_meta_model, "typeArguments", com.github.javaparser.ast.type.Type.class, &Optional::of(self.type_meta_model), true, false, true, false);
		self.class_or_interface_type_meta_model.get_declared_property_meta_models().add(self.class_or_interface_type_meta_model.typeArgumentsPropertyMetaModel);
		self.class_or_interface_type_meta_model.usingDiamondOperatorPropertyMetaModel = PropertyMetaModel::new(self.class_or_interface_type_meta_model, "usingDiamondOperator", bool.class, &Optional::empty(), false, false, false, false);
		self.class_or_interface_type_meta_model.get_derived_property_meta_models().add(self.class_or_interface_type_meta_model.usingDiamondOperatorPropertyMetaModel);
		self.intersection_type_meta_model.elementsPropertyMetaModel = PropertyMetaModel::new(self.intersection_type_meta_model, "elements", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), false, true, true, false);
		self.intersection_type_meta_model.get_declared_property_meta_models().add(self.intersection_type_meta_model.elementsPropertyMetaModel);
		self.primitive_type_meta_model.typePropertyMetaModel = PropertyMetaModel::new(self.primitive_type_meta_model, "type", com.github.javaparser.ast.type.PrimitiveType.Primitive.class, &Optional::empty(), false, false, false, false);
		self.primitive_type_meta_model.get_declared_property_meta_models().add(self.primitive_type_meta_model.typePropertyMetaModel);
		self.type_parameter_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.type_parameter_meta_model, "name", com.github.javaparser.ast.expr.SimpleName.class, &Optional::of(self.simple_name_meta_model), false, false, false, false);
		self.type_parameter_meta_model.get_declared_property_meta_models().add(self.type_parameter_meta_model.namePropertyMetaModel);
		self.type_parameter_meta_model.typeBoundPropertyMetaModel = PropertyMetaModel::new(self.type_parameter_meta_model, "typeBound", com.github.javaparser.ast.type.ClassOrInterfaceType.class, &Optional::of(self.class_or_interface_type_meta_model), false, false, true, false);
		self.type_parameter_meta_model.get_declared_property_meta_models().add(self.type_parameter_meta_model.typeBoundPropertyMetaModel);
		self.union_type_meta_model.elementsPropertyMetaModel = PropertyMetaModel::new(self.union_type_meta_model, "elements", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), false, true, true, false);
		self.union_type_meta_model.get_declared_property_meta_models().add(self.union_type_meta_model.elementsPropertyMetaModel);
		self.wildcard_type_meta_model.extendedTypePropertyMetaModel = PropertyMetaModel::new(self.wildcard_type_meta_model, "extendedType", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), true, false, false, false);
		self.wildcard_type_meta_model.get_declared_property_meta_models().add(self.wildcard_type_meta_model.extendedTypePropertyMetaModel);
		self.wildcard_type_meta_model.superTypePropertyMetaModel = PropertyMetaModel::new(self.wildcard_type_meta_model, "superType", com.github.javaparser.ast.type.ReferenceType.class, &Optional::of(self.reference_type_meta_model), true, false, false, false);
		self.wildcard_type_meta_model.get_declared_property_meta_models().add(self.wildcard_type_meta_model.superTypePropertyMetaModel);
		self.module_exports_directive_meta_model.moduleNamesPropertyMetaModel = PropertyMetaModel::new(self.module_exports_directive_meta_model, "moduleNames", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, true, false);
		self.module_exports_directive_meta_model.get_declared_property_meta_models().add(self.module_exports_directive_meta_model.moduleNamesPropertyMetaModel);
		self.module_exports_directive_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_exports_directive_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_exports_directive_meta_model.get_declared_property_meta_models().add(self.module_exports_directive_meta_model.namePropertyMetaModel);
		self.module_opens_directive_meta_model.moduleNamesPropertyMetaModel = PropertyMetaModel::new(self.module_opens_directive_meta_model, "moduleNames", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, true, false);
		self.module_opens_directive_meta_model.get_declared_property_meta_models().add(self.module_opens_directive_meta_model.moduleNamesPropertyMetaModel);
		self.module_opens_directive_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_opens_directive_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_opens_directive_meta_model.get_declared_property_meta_models().add(self.module_opens_directive_meta_model.namePropertyMetaModel);
		self.module_provides_directive_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_provides_directive_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_provides_directive_meta_model.get_declared_property_meta_models().add(self.module_provides_directive_meta_model.namePropertyMetaModel);
		self.module_provides_directive_meta_model.withPropertyMetaModel = PropertyMetaModel::new(self.module_provides_directive_meta_model, "with", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, true, false);
		self.module_provides_directive_meta_model.get_declared_property_meta_models().add(self.module_provides_directive_meta_model.withPropertyMetaModel);
		self.module_requires_directive_meta_model.modifiersPropertyMetaModel = PropertyMetaModel::new(self.module_requires_directive_meta_model, "modifiers", com.github.javaparser.ast.Modifier.class, &Optional::of(self.modifier_meta_model), false, false, true, false);
		self.module_requires_directive_meta_model.get_declared_property_meta_models().add(self.module_requires_directive_meta_model.modifiersPropertyMetaModel);
		self.module_requires_directive_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_requires_directive_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_requires_directive_meta_model.get_declared_property_meta_models().add(self.module_requires_directive_meta_model.namePropertyMetaModel);
		self.module_uses_directive_meta_model.namePropertyMetaModel = PropertyMetaModel::new(self.module_uses_directive_meta_model, "name", com.github.javaparser.ast.expr.Name.class, &Optional::of(self.name_meta_model), false, false, false, false);
		self.module_uses_directive_meta_model.get_declared_property_meta_models().add(self.module_uses_directive_meta_model.namePropertyMetaModel);
	}

	pub fn get_node_meta_model(&self, c: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::Optional /**/ {
		for node_meta_model in self.node_meta_models {
			if node_meta_model.get_type_name().equals(&c.getSimpleName()) {
				return Optional::of(node_meta_model);
			}
		}
		return Optional::empty();
	}

	init {
	    initializeNodeMetaModels();
	    initializePropertyMetaModels();
	    initializeConstructorParameters();
	}
}