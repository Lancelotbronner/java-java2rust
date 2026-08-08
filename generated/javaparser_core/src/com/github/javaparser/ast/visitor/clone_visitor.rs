use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments::BlockComment;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type;
use java::util::Optional;

pub struct CloneVisitor;

impl CloneVisitor {
	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let imports: NodeList<ImportDeclaration> = self.clone_list(&n.get_imports(), arg);
		let module: ModuleDeclaration = self.clone_node(&n.get_module(), arg);
		let package_declaration: PackageDeclaration = self.clone_node(&n.get_package_declaration(), arg);
		let types: NodeList<TypeDeclaration<?>> = self.clone_list(&n.get_types(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: CompilationUnit = CompilationUnit::new(&n.get_token_range().orElse(null), package_declaration, imports, types, module);
		n.get_storage().ifPresent(|s|r.set_storage(&s.get_path(), &s.get_encoding()));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: PackageDeclaration = PackageDeclaration::new(&n.get_token_range().orElse(null), annotations, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let type_bound: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_type_bound(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TypeParameter = TypeParameter::new(&n.get_token_range().orElse(null), name, type_bound, annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LineComment = LineComment::new(&n.get_token_range().orElse(null), &n.get_content());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: BlockComment = BlockComment::new(&n.get_token_range().orElse(null), &n.get_content());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let extended_types: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_extended_types(), arg);
		let implemented_types: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_implemented_types(), arg);
		let permitted_types: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_permitted_types(), arg);
		let type_parameters: NodeList<TypeParameter> = self.clone_list(&n.get_type_parameters(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_members(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ClassOrInterfaceDeclaration = ClassOrInterfaceDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, &n.is_interface(), name, type_parameters, extended_types, implemented_types, permitted_types, members, &n.is_compact());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let entries: NodeList<EnumConstantDeclaration> = self.clone_list(&n.get_entries(), arg);
		let implemented_types: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_implemented_types(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_members(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: EnumDeclaration = EnumDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, name, implemented_types, entries, members);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let arguments: NodeList<Expression> = self.clone_list(&n.get_arguments(), arg);
		let class_body: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_class_body(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: EnumConstantDeclaration = EnumConstantDeclaration::new(&n.get_token_range().orElse(null), annotations, name, arguments, class_body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let members: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_members(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: AnnotationDeclaration = AnnotationDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, name, members);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let default_value: Expression = self.clone_node(&n.get_default_value(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: AnnotationMemberDeclaration = AnnotationMemberDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, type, name, default_value);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let variables: NodeList<VariableDeclarator> = self.clone_list(&n.get_variables(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: FieldDeclaration = FieldDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, variables);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let initializer: Expression = self.clone_node(&n.get_initializer(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: VariableDeclarator = VariableDeclarator::new(&n.get_token_range().orElse(null), type, name, initializer);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let parameters: NodeList<Parameter> = self.clone_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = self.clone_node(&n.get_receiver_parameter(), arg);
		let thrown_exceptions: NodeList<ReferenceType> = self.clone_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.clone_list(&n.get_type_parameters(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ConstructorDeclaration = ConstructorDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, body, receiver_parameter);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let parameters: NodeList<Parameter> = self.clone_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = self.clone_node(&n.get_receiver_parameter(), arg);
		let thrown_exceptions: NodeList<ReferenceType> = self.clone_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.clone_list(&n.get_type_parameters(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MethodDeclaration = MethodDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, type_parameters, type, name, parameters, thrown_exceptions, body, receiver_parameter);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let var_args_annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_var_args_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: Parameter = Parameter::new(&n.get_token_range().orElse(null), modifiers, annotations, type, &n.is_var_args(), var_args_annotations, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: InitializerDeclaration = InitializerDeclaration::new(&n.get_token_range().orElse(null), &n.is_static(), body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TraditionalJavadocComment = TraditionalJavadocComment::new(&n.get_token_range().orElse(null), &n.get_content());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let scope: ClassOrInterfaceType = self.clone_node(&n.get_scope(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ClassOrInterfaceType = ClassOrInterfaceType::new(&n.get_token_range().orElse(null), scope, name, type_arguments, annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: PrimitiveType = PrimitiveType::new(&n.get_token_range().orElse(null), &n.get_type(), annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let component_type: Type = self.clone_node(&n.get_component_type(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ArrayType = ArrayType::new(&n.get_token_range().orElse(null), component_type, &n.get_origin(), annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let dimension: Expression = self.clone_node(&n.get_dimension(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ArrayCreationLevel = ArrayCreationLevel::new(&n.get_token_range().orElse(null), dimension, annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let elements: NodeList<ReferenceType> = self.clone_list(&n.get_elements(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: IntersectionType = IntersectionType::new(&n.get_token_range().orElse(null), elements);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let elements: NodeList<ReferenceType> = self.clone_list(&n.get_elements(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: UnionType = UnionType::new(&n.get_token_range().orElse(null), elements);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: VoidType = VoidType::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let extended_type: ReferenceType = self.clone_node(&n.get_extended_type(), arg);
		let super_type: ReferenceType = self.clone_node(&n.get_super_type(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: WildcardType = WildcardType::new(&n.get_token_range().orElse(null), extended_type, super_type, annotations);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: UnknownType = UnknownType::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let index: Expression = self.clone_node(&n.get_index(), arg);
		let name: Expression = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ArrayAccessExpr = ArrayAccessExpr::new(&n.get_token_range().orElse(null), name, index);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let element_type: Type = self.clone_node(&n.get_element_type(), arg);
		let initializer: ArrayInitializerExpr = self.clone_node(&n.get_initializer(), arg);
		let levels: NodeList<ArrayCreationLevel> = self.clone_list(&n.get_levels(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ArrayCreationExpr = ArrayCreationExpr::new(&n.get_token_range().orElse(null), element_type, levels, initializer);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let values: NodeList<Expression> = self.clone_list(&n.get_values(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ArrayInitializerExpr = ArrayInitializerExpr::new(&n.get_token_range().orElse(null), values);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let target: Expression = self.clone_node(&n.get_target(), arg);
		let value: Expression = self.clone_node(&n.get_value(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: AssignExpr = AssignExpr::new(&n.get_token_range().orElse(null), target, value, &n.get_operator());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let left: Expression = self.clone_node(&n.get_left(), arg);
		let right: Expression = self.clone_node(&n.get_right(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: BinaryExpr = BinaryExpr::new(&n.get_token_range().orElse(null), left, right, &n.get_operator());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: CastExpr = CastExpr::new(&n.get_token_range().orElse(null), type, expression);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ClassExpr = ClassExpr::new(&n.get_token_range().orElse(null), type);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let condition: Expression = self.clone_node(&n.get_condition(), arg);
		let else_expr: Expression = self.clone_node(&n.get_else_expr(), arg);
		let then_expr: Expression = self.clone_node(&n.get_then_expr(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ConditionalExpr = ConditionalExpr::new(&n.get_token_range().orElse(null), condition, then_expr, else_expr);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let inner: Expression = self.clone_node(&n.get_inner(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: EnclosedExpr = EnclosedExpr::new(&n.get_token_range().orElse(null), inner);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let scope: Expression = self.clone_node(&n.get_scope(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: FieldAccessExpr = FieldAccessExpr::new(&n.get_token_range().orElse(null), scope, type_arguments, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let pattern: PatternExpr = self.clone_node(&n.get_pattern(), arg);
		let type: ReferenceType = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: InstanceOfExpr = InstanceOfExpr::new(&n.get_token_range().orElse(null), expression, type, pattern);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: StringLiteralExpr = StringLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: IntegerLiteralExpr = IntegerLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LongLiteralExpr = LongLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: CharLiteralExpr = CharLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: DoubleLiteralExpr = DoubleLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: BooleanLiteralExpr = BooleanLiteralExpr::new(&n.get_token_range().orElse(null), &n.is_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: NullLiteralExpr = NullLiteralExpr::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let arguments: NodeList<Expression> = self.clone_list(&n.get_arguments(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let scope: Expression = self.clone_node(&n.get_scope(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MethodCallExpr = MethodCallExpr::new(&n.get_token_range().orElse(null), scope, type_arguments, name, arguments);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: NameExpr = NameExpr::new(&n.get_token_range().orElse(null), name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let anonymous_class_body: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_anonymous_class_body().orElse(null), arg);
		let arguments: NodeList<Expression> = self.clone_list(&n.get_arguments(), arg);
		let scope: Expression = self.clone_node(&n.get_scope(), arg);
		let type: ClassOrInterfaceType = self.clone_node(&n.get_type(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ObjectCreationExpr = ObjectCreationExpr::new(&n.get_token_range().orElse(null), scope, type, type_arguments, arguments, anonymous_class_body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let qualifier: Name = self.clone_node(&n.get_qualifier(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: Name = Name::new(&n.get_token_range().orElse(null), qualifier, &n.get_identifier());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SimpleName = SimpleName::new(&n.get_token_range().orElse(null), &n.get_identifier());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type_name: Name = self.clone_node(&n.get_type_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ThisExpr = ThisExpr::new(&n.get_token_range().orElse(null), type_name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type_name: Name = self.clone_node(&n.get_type_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SuperExpr = SuperExpr::new(&n.get_token_range().orElse(null), type_name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: UnaryExpr = UnaryExpr::new(&n.get_token_range().orElse(null), expression, &n.get_operator());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let variables: NodeList<VariableDeclarator> = self.clone_list(&n.get_variables(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: VariableDeclarationExpr = VariableDeclarationExpr::new(&n.get_token_range().orElse(null), modifiers, annotations, variables);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MarkerAnnotationExpr = MarkerAnnotationExpr::new(&n.get_token_range().orElse(null), name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let member_value: Expression = self.clone_node(&n.get_member_value(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SingleMemberAnnotationExpr = SingleMemberAnnotationExpr::new(&n.get_token_range().orElse(null), name, member_value);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let pairs: NodeList<MemberValuePair> = self.clone_list(&n.get_pairs(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: NormalAnnotationExpr = NormalAnnotationExpr::new(&n.get_token_range().orElse(null), name, pairs);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let value: Expression = self.clone_node(&n.get_value(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MemberValuePair = MemberValuePair::new(&n.get_token_range().orElse(null), name, value);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let arguments: NodeList<Expression> = self.clone_list(&n.get_arguments(), arg);
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ExplicitConstructorInvocationStmt = ExplicitConstructorInvocationStmt::new(&n.get_token_range().orElse(null), type_arguments, &n.is_this(), expression, arguments);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let class_declaration: ClassOrInterfaceDeclaration = self.clone_node(&n.get_class_declaration(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LocalClassDeclarationStmt = LocalClassDeclarationStmt::new(&n.get_token_range().orElse(null), class_declaration);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let record_declaration: RecordDeclaration = self.clone_node(&n.get_record_declaration(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LocalRecordDeclarationStmt = LocalRecordDeclarationStmt::new(&n.get_token_range().orElse(null), record_declaration);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let check: Expression = self.clone_node(&n.get_check(), arg);
		let message: Expression = self.clone_node(&n.get_message(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: AssertStmt = AssertStmt::new(&n.get_token_range().orElse(null), check, message);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let statements: NodeList<Statement> = self.clone_list(&n.get_statements(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: BlockStmt = BlockStmt::new(&n.get_token_range().orElse(null), statements);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = self.clone_node(&n.get_label(), arg);
		let statement: Statement = self.clone_node(&n.get_statement(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LabeledStmt = LabeledStmt::new(&n.get_token_range().orElse(null), label, statement);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: EmptyStmt = EmptyStmt::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ExpressionStmt = ExpressionStmt::new(&n.get_token_range().orElse(null), expression);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let entries: NodeList<SwitchEntry> = self.clone_list(&n.get_entries(), arg);
		let selector: Expression = self.clone_node(&n.get_selector(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SwitchStmt = SwitchStmt::new(&n.get_token_range().orElse(null), selector, entries);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let guard: Expression = self.clone_node(&n.get_guard(), arg);
		let labels: NodeList<Expression> = self.clone_list(&n.get_labels(), arg);
		let statements: NodeList<Statement> = self.clone_list(&n.get_statements(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SwitchEntry = SwitchEntry::new(&n.get_token_range().orElse(null), labels, &n.get_type(), statements, &n.is_default(), guard);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = self.clone_node(&n.get_label(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: BreakStmt = BreakStmt::new(&n.get_token_range().orElse(null), label);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ReturnStmt = ReturnStmt::new(&n.get_token_range().orElse(null), expression);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let condition: Expression = self.clone_node(&n.get_condition(), arg);
		let else_stmt: Statement = self.clone_node(&n.get_else_stmt(), arg);
		let then_stmt: Statement = self.clone_node(&n.get_then_stmt(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: IfStmt = IfStmt::new(&n.get_token_range().orElse(null), condition, then_stmt, else_stmt);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = self.clone_node(&n.get_body(), arg);
		let condition: Expression = self.clone_node(&n.get_condition(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: WhileStmt = WhileStmt::new(&n.get_token_range().orElse(null), condition, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = self.clone_node(&n.get_label(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ContinueStmt = ContinueStmt::new(&n.get_token_range().orElse(null), label);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = self.clone_node(&n.get_body(), arg);
		let condition: Expression = self.clone_node(&n.get_condition(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: DoStmt = DoStmt::new(&n.get_token_range().orElse(null), body, condition);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = self.clone_node(&n.get_body(), arg);
		let iterable: Expression = self.clone_node(&n.get_iterable(), arg);
		let variable: VariableDeclarationExpr = self.clone_node(&n.get_variable(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ForEachStmt = ForEachStmt::new(&n.get_token_range().orElse(null), variable, iterable, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = self.clone_node(&n.get_body(), arg);
		let compare: Expression = self.clone_node(&n.get_compare(), arg);
		let initialization: NodeList<Expression> = self.clone_list(&n.get_initialization(), arg);
		let update: NodeList<Expression> = self.clone_list(&n.get_update(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ForStmt = ForStmt::new(&n.get_token_range().orElse(null), initialization, compare, update, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ThrowStmt = ThrowStmt::new(&n.get_token_range().orElse(null), expression);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SynchronizedStmt = SynchronizedStmt::new(&n.get_token_range().orElse(null), expression, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let catch_clauses: NodeList<CatchClause> = self.clone_list(&n.get_catch_clauses(), arg);
		let finally_block: BlockStmt = self.clone_node(&n.get_finally_block(), arg);
		let resources: NodeList<Expression> = self.clone_list(&n.get_resources(), arg);
		let try_block: BlockStmt = self.clone_node(&n.get_try_block(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TryStmt = TryStmt::new(&n.get_token_range().orElse(null), resources, try_block, catch_clauses, finally_block);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let parameter: Parameter = self.clone_node(&n.get_parameter(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: CatchClause = CatchClause::new(&n.get_token_range().orElse(null), parameter, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = self.clone_node(&n.get_body(), arg);
		let parameters: NodeList<Parameter> = self.clone_list(&n.get_parameters(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: LambdaExpr = LambdaExpr::new(&n.get_token_range().orElse(null), parameters, body, &n.is_enclosing_parameters());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let scope: Expression = self.clone_node(&n.get_scope(), arg);
		let type_arguments: NodeList<Type> = self.clone_list(&n.get_type_arguments().orElse(null), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MethodReferenceExpr = MethodReferenceExpr::new(&n.get_token_range().orElse(null), scope, type_arguments, &n.get_identifier());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TypeExpr = TypeExpr::new(&n.get_token_range().orElse(null), type);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Object /**/) -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let new_nodes: NodeList<Node> = NodeList<>::new();
		for node in n {
			let result_node: Node = (node as Node).accept(self, arg) as Node;
			if result_node != null {
				new_nodes.add(result_node);
			}
		}
		return new_nodes;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ImportDeclaration = ImportDeclaration::new(&n.get_token_range().orElse(null), name, &n.is_static(), &n.is_asterisk(), &n.is_module());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let directives: NodeList<ModuleDirective> = self.clone_list(&n.get_directives(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleDeclaration = ModuleDeclaration::new(&n.get_token_range().orElse(null), annotations, name, &n.is_open(), directives);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleRequiresDirective = ModuleRequiresDirective::new(&n.get_token_range().orElse(null), modifiers, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	fn clone_node<T: com::github::javaparser::ast::node::Node>(&self, node: &/* Java */ java::util::Optional /**/, arg: &/* Java */ java::lang::Object /**/) -> T {
		if !node.isPresent() {
			return null;
		}
		let r: Node = node.get().accept(self, arg) as Node;
		if r == null {
			return null;
		}
		return r as T;
	}

	fn clone_node<T: com::github::javaparser::ast::node::Node>(&self, node: &T, arg: &/* Java */ java::lang::Object /**/) -> T {
		if node == null {
			return null;
		}
		let r: Node = node.accept(self, arg) as Node;
		if r == null {
			return null;
		}
		return r as T;
	}

	fn clone_list<N: com::github::javaparser::ast::node::Node>(&self, list: &com::github::javaparser::ast::node_list::NodeList, arg: &/* Java */ java::lang::Object /**/) -> com::github::javaparser::ast::node_list::NodeList {
		if list == null {
			return null;
		}
		return list.accept(self, arg) as NodeList<N>;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let module_names: NodeList<Name> = self.clone_list(&n.get_module_names(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleExportsDirective = ModuleExportsDirective::new(&n.get_token_range().orElse(null), name, module_names);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = self.clone_node(&n.get_name(), arg);
		let with: NodeList<Name> = self.clone_list(&n.get_with(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleProvidesDirective = ModuleProvidesDirective::new(&n.get_token_range().orElse(null), name, with);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleUsesDirective = ModuleUsesDirective::new(&n.get_token_range().orElse(null), name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let module_names: NodeList<Name> = self.clone_list(&n.get_module_names(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ModuleOpensDirective = ModuleOpensDirective::new(&n.get_token_range().orElse(null), name, module_names);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: UnparsableStmt = UnparsableStmt::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let name: Name = self.clone_node(&n.get_name(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: ReceiverParameter = ReceiverParameter::new(&n.get_token_range().orElse(null), annotations, type, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: VarType = VarType::new(&n.get_token_range().orElse(null));
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: Modifier = Modifier::new(&n.get_token_range().orElse(null), &n.get_keyword());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let entries: NodeList<SwitchEntry> = self.clone_list(&n.get_entries(), arg);
		let selector: Expression = self.clone_node(&n.get_selector(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: SwitchExpr = SwitchExpr::new(&n.get_token_range().orElse(null), selector, entries);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r);
		return r;
	}

	fn copy_data(&self, source: &com::github::javaparser::ast::node::Node, destination: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalStateException) */ {
		for data_key in source.get_data_keys() {
			destination.set_data(data_key, &source.get_data(data_key)?);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = self.clone_node(&n.get_expression(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: YieldStmt = YieldStmt::new(&n.get_token_range().orElse(null), expression);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TextBlockLiteralExpr = TextBlockLiteralExpr::new(&n.get_token_range().orElse(null), &n.get_value());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let type: Type = self.clone_node(&n.get_type(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: TypePatternExpr = TypePatternExpr::new(&n.get_token_range().orElse(null), modifiers, type, name);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let implemented_types: NodeList<ClassOrInterfaceType> = self.clone_list(&n.get_implemented_types(), arg);
		let parameters: NodeList<Parameter> = self.clone_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = self.clone_node(&n.get_receiver_parameter(), arg);
		let type_parameters: NodeList<TypeParameter> = self.clone_list(&n.get_type_parameters(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.clone_list(&n.get_members(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: RecordDeclaration = RecordDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, name, parameters, type_parameters, implemented_types, members, receiver_parameter);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = self.clone_node(&n.get_body(), arg);
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let name: SimpleName = self.clone_node(&n.get_name(), arg);
		let thrown_exceptions: NodeList<ReferenceType> = self.clone_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.clone_list(&n.get_type_parameters(), arg);
		let annotations: NodeList<AnnotationExpr> = self.clone_list(&n.get_annotations(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: CompactConstructorDeclaration = CompactConstructorDeclaration::new(&n.get_token_range().orElse(null), modifiers, annotations, type_parameters, name, thrown_exceptions, body);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let pattern_list: NodeList<ComponentPatternExpr> = self.clone_list(&n.get_pattern_list(), arg);
		let type: Type = self.clone_node(&n.get_type()?, arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: RecordPatternExpr = RecordPatternExpr::new(&n.get_token_range().orElse(null), modifiers, type, pattern_list);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.clone_list(&n.get_modifiers(), arg);
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MatchAllPatternExpr = MatchAllPatternExpr::new(&n.get_token_range().orElse(null), modifiers);
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = self.clone_node(&n.get_comment(), arg);
		let r: MarkdownComment = MarkdownComment::new(&n.get_token_range().orElse(null), &n.get_content());
		r.set_comment(comment)?;
		n.get_orphan_comments().stream().map(Comment::clone).forEach(r::addOrphanComment);
		self.copy_data(n, r)?;
		return r;
	}
}

impl com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for CloneVisitor {}