use crate::com::github::javaparser::utils::Utils::removeElementByObjectIdentity;
use crate::com::github::javaparser::utils::Utils::replaceElementByObjectIdentity;
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
use crate::com::github::javaparser::utils::Pair;
use java::util::ArrayList;
use java::util::List;
use java::util::Optional;

pub struct ModifierVisitor<A>;

impl<A> ModifierVisitor {
	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_members(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_members(members)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let default_value: Expression = n.get_default_value().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || type == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_default_value(default_value);
		n.set_name(name)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let index: Expression = n.get_index().accept(self, arg) as Expression;
		let name: Expression = n.get_name().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if index == null || name == null {
			return null;
		}
	
		n.set_index(index)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let element_type: Type = n.get_element_type().accept(self, arg) as Type;
		let initializer: ArrayInitializerExpr = n.get_initializer().map(|s|s.accept(self, arg) as ArrayInitializerExpr).orElse(null);
		let levels: NodeList<ArrayCreationLevel> = self.modify_list(&n.get_levels(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if element_type == null || levels.is_empty() {
			return null;
		}
	
		n.set_element_type(element_type)?;
		n.set_initializer(initializer);
		n.set_levels(levels)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let values: NodeList<Expression> = self.modify_list(&n.get_values(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_values(values)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let check: Expression = n.get_check().accept(self, arg) as Expression;
		let message: Expression = n.get_message().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if check == null {
			return null;
		}
	
		n.set_check(check)?;
		n.set_message(message);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let target: Expression = n.get_target().accept(self, arg) as Expression;
		let value: Expression = n.get_value().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if target == null || value == null {
			return null;
		}
	
		n.set_target(target)?;
		n.set_value(value)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let left: Expression = n.get_left().accept(self, arg) as Expression;
		let right: Expression = n.get_right().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if left == null {
			return right;
		}
	
		if right == null {
			return left;
		}
	
		n.set_left(left)?;
		n.set_right(right)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let statements: NodeList<Statement> = self.modify_list(&n.get_statements(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_statements(statements)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = n.get_label().map(|s|s.accept(self, arg) as SimpleName).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_label(label);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null || type == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = n.get_body().accept(self, arg) as BlockStmt;
		let parameter: Parameter = n.get_parameter().accept(self, arg) as Parameter;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || parameter == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_parameter(parameter)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if type == null {
			return null;
		}
	
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let extended_types: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_extended_types(), arg);
		let implemented_types: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_implemented_types(), arg);
		let permitted_types: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_permitted_types(), arg);
		let type_parameters: NodeList<TypeParameter> = self.modify_list(&n.get_type_parameters(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_members(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_extended_types(extended_types)?;
		n.set_implemented_types(implemented_types)?;
		n.set_permitted_types(permitted_types)?;
		n.set_type_parameters(type_parameters)?;
		n.set_members(members)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let implemented_types: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_implemented_types(), arg);
		let parameters: NodeList<Parameter> = self.modify_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = n.get_receiver_parameter().map(|s|s.accept(self, arg) as ReceiverParameter).orElse(null);
		let type_parameters: NodeList<TypeParameter> = self.modify_list(&n.get_type_parameters(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_members(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_implemented_types(implemented_types)?;
		n.set_parameters(parameters)?;
		n.set_receiver_parameter(receiver_parameter);
		n.set_type_parameters(type_parameters)?;
		n.set_members(members)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let scope: ClassOrInterfaceType = n.get_scope().map(|s|s.accept(self, arg) as ClassOrInterfaceType).orElse(null);
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations);
		n.set_name(name)?;
		n.set_scope(scope);
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let imports: NodeList<ImportDeclaration> = self.modify_list(&n.get_imports(), arg);
		let module: ModuleDeclaration = n.get_module().map(|s|s.accept(self, arg) as ModuleDeclaration).orElse(null);
		let package_declaration: PackageDeclaration = n.get_package_declaration().map(|s|s.accept(self, arg) as PackageDeclaration).orElse(null);
		let types: NodeList<TypeDeclaration<?>> = self.modify_list(&n.get_types(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_imports(imports)?;
		n.set_module(module);
		n.set_package_declaration(package_declaration);
		n.set_types(types)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let condition: Expression = n.get_condition().accept(self, arg) as Expression;
		let else_expr: Expression = n.get_else_expr().accept(self, arg) as Expression;
		let then_expr: Expression = n.get_then_expr().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if condition == null || else_expr == null || then_expr == null {
			return null;
		}
	
		n.set_condition(condition)?;
		n.set_else_expr(else_expr)?;
		n.set_then_expr(then_expr)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let body: BlockStmt = n.get_body().accept(self, arg) as BlockStmt;
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let parameters: NodeList<Parameter> = self.modify_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = n.get_receiver_parameter().map(|s|s.accept(self, arg) as ReceiverParameter).orElse(null);
		let thrown_exceptions: NodeList<ReferenceType> = self.modify_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.modify_list(&n.get_type_parameters(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_body(body)?;
		n.set_name(name)?;
		n.set_parameters(parameters)?;
		n.set_receiver_parameter(receiver_parameter);
		n.set_thrown_exceptions(thrown_exceptions)?;
		n.set_type_parameters(type_parameters)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let body: BlockStmt = n.get_body().accept(self, arg) as BlockStmt;
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let thrown_exceptions: NodeList<ReferenceType> = self.modify_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.modify_list(&n.get_type_parameters(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_body(body)?;
		n.set_name(name)?;
		n.set_thrown_exceptions(thrown_exceptions)?;
		n.set_type_parameters(type_parameters)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = n.get_label().map(|s|s.accept(self, arg) as SimpleName).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_label(label);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = n.get_body().accept(self, arg) as Statement;
		let condition: Expression = n.get_condition().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || condition == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_condition(condition)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let inner: Expression = n.get_inner().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if inner == null {
			return null;
		}
	
		n.set_inner(inner)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let arguments: NodeList<Expression> = self.modify_list(&n.get_arguments(), arg);
		let class_body: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_class_body(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_arguments(arguments)?;
		n.set_class_body(class_body)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let entries: NodeList<EnumConstantDeclaration> = self.modify_list(&n.get_entries(), arg);
		let implemented_types: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_implemented_types(), arg);
		let members: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_members(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_entries(entries)?;
		n.set_implemented_types(implemented_types)?;
		n.set_members(members)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let arguments: NodeList<Expression> = self.modify_list(&n.get_arguments(), arg);
		let expression: Expression = n.get_expression().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_arguments(arguments)?;
		n.set_expression(expression);
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let scope: Expression = n.get_scope().accept(self, arg) as Expression;
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || scope == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_scope(scope)?;
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let variables: NodeList<VariableDeclarator> = self.modify_list(&n.get_variables(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if variables.is_empty() {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_variables(variables)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = n.get_body().accept(self, arg) as Statement;
		let iterable: Expression = n.get_iterable().accept(self, arg) as Expression;
		let variable: VariableDeclarationExpr = n.get_variable().accept(self, arg) as VariableDeclarationExpr;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || iterable == null || variable == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_iterable(iterable)?;
		n.set_variable(variable)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = n.get_body().accept(self, arg) as Statement;
		let compare: Expression = n.get_compare().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let initialization: NodeList<Expression> = self.modify_list(&n.get_initialization(), arg);
		let update: NodeList<Expression> = self.modify_list(&n.get_update(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_compare(compare);
		n.set_initialization(initialization)?;
		n.set_update(update)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let condition: Expression = n.get_condition().accept(self, arg) as Expression;
		let else_stmt: Statement = n.get_else_stmt().map(|s|s.accept(self, arg) as Statement).orElse(null);
		let then_stmt: Statement = n.get_then_stmt().accept(self, arg) as Statement;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if condition == null || then_stmt == null {
			return null;
		}
	
		n.set_condition(condition)?;
		n.set_else_stmt(else_stmt);
		n.set_then_stmt(then_stmt)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let body: BlockStmt = n.get_body().accept(self, arg) as BlockStmt;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_body(body)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let pattern: PatternExpr = n.get_pattern().map(|s|s.accept(self, arg) as PatternExpr).orElse(null);
		let type: ReferenceType = n.get_type().accept(self, arg) as ReferenceType;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null || type == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_pattern(pattern);
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let label: SimpleName = n.get_label().accept(self, arg) as SimpleName;
		let statement: Statement = n.get_statement().accept(self, arg) as Statement;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if label == null || statement == null {
			return null;
		}
	
		n.set_label(label)?;
		n.set_statement(statement)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let value: Expression = n.get_value().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || value == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_value(value)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let arguments: NodeList<Expression> = self.modify_list(&n.get_arguments(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let scope: Expression = n.get_scope().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_arguments(arguments)?;
		n.set_name(name)?;
		n.set_scope(scope);
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let body: BlockStmt = n.get_body().map(|s|s.accept(self, arg) as BlockStmt).orElse(null);
		let type: Type = n.get_type().accept(self, arg) as Type;
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let parameters: NodeList<Parameter> = self.modify_list(&n.get_parameters(), arg);
		let receiver_parameter: ReceiverParameter = n.get_receiver_parameter().map(|s|s.accept(self, arg) as ReceiverParameter).orElse(null);
		let thrown_exceptions: NodeList<ReferenceType> = self.modify_list(&n.get_thrown_exceptions(), arg);
		let type_parameters: NodeList<TypeParameter> = self.modify_list(&n.get_type_parameters(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if type == null || name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_body(body);
		n.set_type(type)?;
		n.set_name(name)?;
		n.set_parameters(parameters)?;
		n.set_receiver_parameter(receiver_parameter);
		n.set_thrown_exceptions(thrown_exceptions)?;
		n.set_type_parameters(type_parameters)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let pairs: NodeList<MemberValuePair> = self.modify_list(&n.get_pairs(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_pairs(pairs)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let anonymous_class_body: NodeList<BodyDeclaration<?>> = self.modify_list(&n.get_anonymous_class_body(), arg);
		let arguments: NodeList<Expression> = self.modify_list(&n.get_arguments(), arg);
		let scope: Expression = n.get_scope().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let type: ClassOrInterfaceType = n.get_type().accept(self, arg) as ClassOrInterfaceType;
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if type == null {
			return null;
		}
	
		n.set_anonymous_class_body(anonymous_class_body);
		n.set_arguments(arguments)?;
		n.set_scope(scope);
		n.set_type(type)?;
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let var_args_annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_var_args_annotations(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || type == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_name(name)?;
		n.set_type(type)?;
		n.set_var_args_annotations(var_args_annotations)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let qualifier: Name = n.get_qualifier().map(|s|s.accept(self, arg) as Name).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_qualifier(qualifier);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let component_type: Type = n.get_component_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if component_type == null {
			return null;
		}
	
		n.set_annotations(annotations);
		n.set_component_type(component_type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let dimension: Expression = n.get_dimension().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations)?;
		n.set_dimension(dimension);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let elements: NodeList<ReferenceType> = self.modify_list(&n.get_elements(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if elements.is_empty() {
			return null;
		}
	
		n.set_annotations(annotations);
		n.set_elements(elements)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let elements: NodeList<ReferenceType> = self.modify_list(&n.get_elements(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if elements.is_empty() {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_elements(elements)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_expression(expression);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let member_value: Expression = n.get_member_value().accept(self, arg) as Expression;
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if member_value == null || name == null {
			return null;
		}
	
		n.set_member_value(member_value)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type_name: Name = n.get_type_name().map(|s|s.accept(self, arg) as Name).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_type_name(type_name);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let guard: Expression = n.get_guard().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let labels: NodeList<Expression> = self.modify_list(&n.get_labels(), arg);
		let statements: NodeList<Statement> = self.modify_list(&n.get_statements(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_guard(guard);
		n.set_labels(labels)?;
		n.set_statements(statements)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let entries: NodeList<SwitchEntry> = self.modify_list(&n.get_entries(), arg);
		let selector: Expression = n.get_selector().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if selector == null {
			return null;
		}
	
		n.set_entries(entries)?;
		n.set_selector(selector)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: BlockStmt = n.get_body().accept(self, arg) as BlockStmt;
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || expression == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_expression(expression)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type_name: Name = n.get_type_name().map(|s|s.accept(self, arg) as Name).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_type_name(type_name);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let catch_clauses: NodeList<CatchClause> = self.modify_list(&n.get_catch_clauses(), arg);
		let finally_block: BlockStmt = n.get_finally_block().map(|s|s.accept(self, arg) as BlockStmt).orElse(null);
		let resources: NodeList<Expression> = self.modify_list(&n.get_resources(), arg);
		let try_block: BlockStmt = n.get_try_block().accept(self, arg) as BlockStmt;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if try_block == null {
			return null;
		}
	
		n.set_catch_clauses(catch_clauses)?;
		n.set_finally_block(finally_block);
		n.set_resources(resources)?;
		n.set_try_block(try_block)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let class_declaration: ClassOrInterfaceDeclaration = n.get_class_declaration().accept(self, arg) as ClassOrInterfaceDeclaration;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if class_declaration == null {
			return null;
		}
	
		n.set_class_declaration(class_declaration)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let record_declaration: RecordDeclaration = n.get_record_declaration().accept(self, arg) as RecordDeclaration;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if record_declaration == null {
			return null;
		}
	
		n.set_record_declaration(record_declaration)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let type_bound: NodeList<ClassOrInterfaceType> = self.modify_list(&n.get_type_bound(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_name(name)?;
		n.set_type_bound(type_bound)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let variables: NodeList<VariableDeclarator> = self.modify_list(&n.get_variables(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if variables.is_empty() {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_modifiers(modifiers)?;
		n.set_variables(variables)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let initializer: Expression = n.get_initializer().map(|s|s.accept(self, arg) as Expression).orElse(null);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || type == null {
			return null;
		}
	
		n.set_initializer(initializer);
		n.set_name(name)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = n.get_body().accept(self, arg) as Statement;
		let condition: Expression = n.get_condition().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null || condition == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_condition(condition)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let extended_type: ReferenceType = n.get_extended_type().map(|s|s.accept(self, arg) as ReferenceType).orElse(null);
		let super_type: ReferenceType = n.get_super_type().map(|s|s.accept(self, arg) as ReferenceType).orElse(null);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations)?;
		n.set_extended_type(extended_type);
		n.set_super_type(super_type);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let body: Statement = n.get_body().accept(self, arg) as Statement;
		let parameters: NodeList<Parameter> = self.modify_list(&n.get_parameters(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if body == null {
			return null;
		}
	
		n.set_body(body)?;
		n.set_parameters(parameters)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let scope: Expression = n.get_scope().accept(self, arg) as Expression;
		let type_arguments: NodeList<Type> = self.modify_list(&n.get_type_arguments(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if scope == null {
			return null;
		}
	
		n.set_scope(scope)?;
		n.set_type_arguments(type_arguments);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if type == null {
			return null;
		}
	
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> com::github::javaparser::ast::visitor::visitable::Visitable {
		if n.is_empty() {
			return n;
		}
		/* final */ let change_list: List<Pair<Node, Node>> = ArrayList<>::new();
		/* final */ let list_copy: List<Node> = ArrayList<>::new(n);
		for node in list_copy {
			/* final */ let new_node: Node = node.accept(self, arg) as Node;
			change_list.add(Pair<>::new(node, new_node));
		}
		for change in change_list {
			if change.b == null {
				com::github::javaparser::utils::utils::Utils::remove_element_by_object_identity(n, change.a);
			} else {
				com::github::javaparser::utils::utils::Utils::replace_element_by_object_identity(n, change.a, change.b);
			}
		}
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	fn modify_list<N: com::github::javaparser::ast::node::Node>(&self, list: &com::github::javaparser::ast::node_list::NodeList, arg: &A) -> com::github::javaparser::ast::node_list::NodeList {
		return list.accept(self, arg) as NodeList<N>;
	}

	fn modify_list<N: com::github::javaparser::ast::node::Node>(&self, list: &/* Java */ java::util::Optional /**/, arg: &A) -> com::github::javaparser::ast::node_list::NodeList {
		return list.map(|ns|.modifyList(ns, arg)).orElse(null);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let directives: NodeList<ModuleDirective> = self.modify_list(&n.get_directives(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_directives(directives)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_modifiers(modifiers)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let module_names: NodeList<Name> = self.modify_list(&n.get_module_names(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_module_names(module_names)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = n.get_name().accept(self, arg) as Name;
		let with: NodeList<Name> = self.modify_list(&n.get_with(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_with(with)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let module_names: NodeList<Name> = self.modify_list(&n.get_module_names(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null {
			return null;
		}
	
		n.set_module_names(module_names)?;
		n.set_name(name)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let name: Name = n.get_name().accept(self, arg) as Name;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || type == null {
			return null;
		}
	
		n.set_annotations(annotations)?;
		n.set_name(name)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let annotations: NodeList<AnnotationExpr> = self.modify_list(&n.get_annotations(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_annotations(annotations)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let entries: NodeList<SwitchEntry> = self.modify_list(&n.get_entries(), arg);
		let selector: Expression = n.get_selector().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if selector == null {
			return null;
		}
	
		n.set_entries(entries)?;
		n.set_selector(selector)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let expression: Expression = n.get_expression().accept(self, arg) as Expression;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if expression == null {
			return null;
		}
	
		n.set_expression(expression)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let name: SimpleName = n.get_name().accept(self, arg) as SimpleName;
		let type: Type = n.get_type().accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if name == null || type == null {
			return null;
		}
	
		n.set_modifiers(modifiers)?;
		n.set_name(name)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let pattern_list: NodeList<ComponentPatternExpr> = self.modify_list(&n.get_pattern_list(), arg);
		let type: Type = n.get_type()?.accept(self, arg) as Type;
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		if type == null {
			return null;
		}
	
		n.set_modifiers(modifiers)?;
		n.set_pattern_list(pattern_list)?;
		n.set_type(type)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &A) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let modifiers: NodeList<Modifier> = self.modify_list(&n.get_modifiers(), arg);
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_modifiers(modifiers)?;
		n.set_comment(comment)?;
		return n;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &A) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::visitor::visitable::Visitable {
		let comment: Comment = n.get_comment().map(|s|s.accept(self, arg) as Comment).orElse(null);
		n.set_comment(comment)?;
		return n;
	}
}

impl<A> com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for ModifierVisitor<A> {}