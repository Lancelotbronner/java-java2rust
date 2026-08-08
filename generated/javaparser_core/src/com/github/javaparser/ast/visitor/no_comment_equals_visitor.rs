use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::comments::BlockComment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules;
use crate::com::github::javaparser::ast::stmt;
use crate::com::github::javaparser::ast::type;
use java::util::Optional;

pub struct NoCommentEqualsVisitor;

impl NoCommentEqualsVisitor {
	static SINGLETON: com::github::javaparser::ast::visitor::no_comment_equals_visitor::NoCommentEqualsVisitor = NoCommentEqualsVisitor::new();

	pub fn equals(&self, n: &com::github::javaparser::ast::node::Node, n2: &com::github::javaparser::ast::node::Node) -> bool {
		return self.SINGLETON.node_equals(n, n2);
	}

	fn nodes_equals<N: com::github::javaparser::ast::node::Node>(&self, n: &com::github::javaparser::ast::node_list::NodeList, n2: &com::github::javaparser::ast::node_list::NodeList) -> bool {
		if n == n2 {
			return true;
		}
		if n == null || n2 == null {
			return false;
		}
		if n.size() != n2.size() {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < n.size() {
				{
					if !self.node_equals(&n.get(i), &n2.get(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	fn node_equals<T: com::github::javaparser::ast::node::Node>(&self, n: &T, n2: &T) -> bool {
		if n == n2 {
			return true;
		}
		if n == null || n2 == null {
			return false;
		}
		if n.getClass() != n2.getClass() {
			return false;
		}
		return n.accept(self, n2);
	}

	fn node_equals<T: com::github::javaparser::ast::node::Node>(&self, n: &/* Java */ java::util::Optional /**/, n2: &/* Java */ java::util::Optional /**/) -> bool {
		return self.node_equals(&n.orElse(null), &n2.orElse(null));
	}

	fn nodes_equals<T: com::github::javaparser::ast::node::Node>(&self, n: &/* Java */ java::util::Optional /**/, n2: &/* Java */ java::util::Optional /**/) -> bool {
		return self.nodes_equals(&n.orElse(null), &n2.orElse(null));
	}

	fn obj_equals(&self, n: &/* Java */ java::lang::Object /**/, n2: &/* Java */ java::lang::Object /**/) -> bool {
		if n == n2 {
			return true;
		}
		if n == null || n2 == null {
			return false;
		}
		return n.equals(n2);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: CompilationUnit = arg as CompilationUnit;
		if !self.nodes_equals(&n.get_imports(), &n2.get_imports()) {
			return false;
		}
	
		if !self.node_equals(&n.get_module(), &n2.get_module()) {
			return false;
		}
	
		if !self.node_equals(&n.get_package_declaration(), &n2.get_package_declaration()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_types(), &n2.get_types()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: PackageDeclaration = arg as PackageDeclaration;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: TypeParameter = arg as TypeParameter;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_bound(), &n2.get_type_bound()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ClassOrInterfaceDeclaration = arg as ClassOrInterfaceDeclaration;
		if !self.nodes_equals(&n.get_extended_types(), &n2.get_extended_types()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_implemented_types(), &n2.get_implemented_types()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_compact(), &n2.is_compact()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_interface(), &n2.is_interface()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_permitted_types(), &n2.get_permitted_types()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_parameters(), &n2.get_type_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_members(), &n2.get_members()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: EnumDeclaration = arg as EnumDeclaration;
		if !self.nodes_equals(&n.get_entries(), &n2.get_entries()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_implemented_types(), &n2.get_implemented_types()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_members(), &n2.get_members()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: EnumConstantDeclaration = arg as EnumConstantDeclaration;
		if !self.nodes_equals(&n.get_arguments(), &n2.get_arguments()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_class_body(), &n2.get_class_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: AnnotationDeclaration = arg as AnnotationDeclaration;
		if !self.nodes_equals(&n.get_members(), &n2.get_members()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: AnnotationMemberDeclaration = arg as AnnotationMemberDeclaration;
		if !self.node_equals(&n.get_default_value(), &n2.get_default_value()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: FieldDeclaration = arg as FieldDeclaration;
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_variables(), &n2.get_variables()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: VariableDeclarator = arg as VariableDeclarator;
		if !self.node_equals(&n.get_initializer(), &n2.get_initializer()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ConstructorDeclaration = arg as ConstructorDeclaration;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_parameters(), &n2.get_parameters()) {
			return false;
		}
	
		if !self.node_equals(&n.get_receiver_parameter(), &n2.get_receiver_parameter()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_thrown_exceptions(), &n2.get_thrown_exceptions()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_parameters(), &n2.get_type_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MethodDeclaration = arg as MethodDeclaration;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_parameters(), &n2.get_parameters()) {
			return false;
		}
	
		if !self.node_equals(&n.get_receiver_parameter(), &n2.get_receiver_parameter()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_thrown_exceptions(), &n2.get_thrown_exceptions()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_parameters(), &n2.get_type_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: Parameter = arg as Parameter;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_var_args(), &n2.is_var_args()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_var_args_annotations(), &n2.get_var_args_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: InitializerDeclaration = arg as InitializerDeclaration;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_static(), &n2.is_static()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ClassOrInterfaceType = arg as ClassOrInterfaceType;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_scope(), &n2.get_scope()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: PrimitiveType = arg as PrimitiveType;
		if !self.obj_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ArrayType = arg as ArrayType;
		if !self.node_equals(&n.get_component_type(), &n2.get_component_type()) {
			return false;
		}
	
		if !self.obj_equals(&n.get_origin(), &n2.get_origin()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ArrayCreationLevel = arg as ArrayCreationLevel;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.node_equals(&n.get_dimension(), &n2.get_dimension()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: IntersectionType = arg as IntersectionType;
		if !self.nodes_equals(&n.get_elements(), &n2.get_elements()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: UnionType = arg as UnionType;
		if !self.nodes_equals(&n.get_elements(), &n2.get_elements()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: VoidType = arg as VoidType;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: WildcardType = arg as WildcardType;
		if !self.node_equals(&n.get_extended_type(), &n2.get_extended_type()) {
			return false;
		}
	
		if !self.node_equals(&n.get_super_type(), &n2.get_super_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: UnknownType = arg as UnknownType;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ArrayAccessExpr = arg as ArrayAccessExpr;
		if !self.node_equals(&n.get_index(), &n2.get_index()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ArrayCreationExpr = arg as ArrayCreationExpr;
		if !self.node_equals(&n.get_element_type(), &n2.get_element_type()) {
			return false;
		}
	
		if !self.node_equals(&n.get_initializer(), &n2.get_initializer()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_levels(), &n2.get_levels()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ArrayInitializerExpr = arg as ArrayInitializerExpr;
		if !self.nodes_equals(&n.get_values(), &n2.get_values()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: AssignExpr = arg as AssignExpr;
		if !self.obj_equals(&n.get_operator(), &n2.get_operator()) {
			return false;
		}
	
		if !self.node_equals(&n.get_target(), &n2.get_target()) {
			return false;
		}
	
		if !self.node_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: BinaryExpr = arg as BinaryExpr;
		if !self.node_equals(&n.get_left(), &n2.get_left()) {
			return false;
		}
	
		if !self.obj_equals(&n.get_operator(), &n2.get_operator()) {
			return false;
		}
	
		if !self.node_equals(&n.get_right(), &n2.get_right()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: CastExpr = arg as CastExpr;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ClassExpr = arg as ClassExpr;
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ConditionalExpr = arg as ConditionalExpr;
		if !self.node_equals(&n.get_condition(), &n2.get_condition()) {
			return false;
		}
	
		if !self.node_equals(&n.get_else_expr(), &n2.get_else_expr()) {
			return false;
		}
	
		if !self.node_equals(&n.get_then_expr(), &n2.get_then_expr()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: EnclosedExpr = arg as EnclosedExpr;
		if !self.node_equals(&n.get_inner(), &n2.get_inner()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: FieldAccessExpr = arg as FieldAccessExpr;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_scope(), &n2.get_scope()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: InstanceOfExpr = arg as InstanceOfExpr;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		if !self.node_equals(&n.get_pattern(), &n2.get_pattern()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: StringLiteralExpr = arg as StringLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: IntegerLiteralExpr = arg as IntegerLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: LongLiteralExpr = arg as LongLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: CharLiteralExpr = arg as CharLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: DoubleLiteralExpr = arg as DoubleLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: BooleanLiteralExpr = arg as BooleanLiteralExpr;
		if !self.obj_equals(&n.is_value(), &n2.is_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MethodCallExpr = arg as MethodCallExpr;
		if !self.nodes_equals(&n.get_arguments(), &n2.get_arguments()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_scope(), &n2.get_scope()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: NameExpr = arg as NameExpr;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ObjectCreationExpr = arg as ObjectCreationExpr;
		if !self.nodes_equals(&n.get_anonymous_class_body(), &n2.get_anonymous_class_body()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_arguments(), &n2.get_arguments()) {
			return false;
		}
	
		if !self.node_equals(&n.get_scope(), &n2.get_scope()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: Name = arg as Name;
		if !self.obj_equals(&n.get_identifier(), &n2.get_identifier()) {
			return false;
		}
	
		if !self.node_equals(&n.get_qualifier(), &n2.get_qualifier()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SimpleName = arg as SimpleName;
		if !self.obj_equals(&n.get_identifier(), &n2.get_identifier()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ThisExpr = arg as ThisExpr;
		if !self.node_equals(&n.get_type_name(), &n2.get_type_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SuperExpr = arg as SuperExpr;
		if !self.node_equals(&n.get_type_name(), &n2.get_type_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: UnaryExpr = arg as UnaryExpr;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		if !self.obj_equals(&n.get_operator(), &n2.get_operator()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: VariableDeclarationExpr = arg as VariableDeclarationExpr;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_variables(), &n2.get_variables()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MarkerAnnotationExpr = arg as MarkerAnnotationExpr;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SingleMemberAnnotationExpr = arg as SingleMemberAnnotationExpr;
		if !self.node_equals(&n.get_member_value(), &n2.get_member_value()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: NormalAnnotationExpr = arg as NormalAnnotationExpr;
		if !self.nodes_equals(&n.get_pairs(), &n2.get_pairs()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MemberValuePair = arg as MemberValuePair;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ExplicitConstructorInvocationStmt = arg as ExplicitConstructorInvocationStmt;
		if !self.nodes_equals(&n.get_arguments(), &n2.get_arguments()) {
			return false;
		}
	
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_this(), &n2.is_this()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: LocalClassDeclarationStmt = arg as LocalClassDeclarationStmt;
		if !self.node_equals(&n.get_class_declaration(), &n2.get_class_declaration()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: LocalRecordDeclarationStmt = arg as LocalRecordDeclarationStmt;
		if !self.node_equals(&n.get_record_declaration(), &n2.get_record_declaration()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: AssertStmt = arg as AssertStmt;
		if !self.node_equals(&n.get_check(), &n2.get_check()) {
			return false;
		}
	
		if !self.node_equals(&n.get_message(), &n2.get_message()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: BlockStmt = arg as BlockStmt;
		if !self.nodes_equals(&n.get_statements(), &n2.get_statements()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: LabeledStmt = arg as LabeledStmt;
		if !self.node_equals(&n.get_label(), &n2.get_label()) {
			return false;
		}
	
		if !self.node_equals(&n.get_statement(), &n2.get_statement()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ExpressionStmt = arg as ExpressionStmt;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SwitchStmt = arg as SwitchStmt;
		if !self.nodes_equals(&n.get_entries(), &n2.get_entries()) {
			return false;
		}
	
		if !self.node_equals(&n.get_selector(), &n2.get_selector()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SwitchEntry = arg as SwitchEntry;
		if !self.node_equals(&n.get_guard(), &n2.get_guard()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_default(), &n2.is_default()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_labels(), &n2.get_labels()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_statements(), &n2.get_statements()) {
			return false;
		}
	
		if !self.obj_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: BreakStmt = arg as BreakStmt;
		if !self.node_equals(&n.get_label(), &n2.get_label()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ReturnStmt = arg as ReturnStmt;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: IfStmt = arg as IfStmt;
		if !self.node_equals(&n.get_condition(), &n2.get_condition()) {
			return false;
		}
	
		if !self.node_equals(&n.get_else_stmt(), &n2.get_else_stmt()) {
			return false;
		}
	
		if !self.node_equals(&n.get_then_stmt(), &n2.get_then_stmt()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: WhileStmt = arg as WhileStmt;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_condition(), &n2.get_condition()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ContinueStmt = arg as ContinueStmt;
		if !self.node_equals(&n.get_label(), &n2.get_label()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: DoStmt = arg as DoStmt;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_condition(), &n2.get_condition()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ForEachStmt = arg as ForEachStmt;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_iterable(), &n2.get_iterable()) {
			return false;
		}
	
		if !self.node_equals(&n.get_variable(), &n2.get_variable()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ForStmt = arg as ForStmt;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_compare(), &n2.get_compare()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_initialization(), &n2.get_initialization()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_update(), &n2.get_update()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ThrowStmt = arg as ThrowStmt;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SynchronizedStmt = arg as SynchronizedStmt;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: TryStmt = arg as TryStmt;
		if !self.nodes_equals(&n.get_catch_clauses(), &n2.get_catch_clauses()) {
			return false;
		}
	
		if !self.node_equals(&n.get_finally_block(), &n2.get_finally_block()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_resources(), &n2.get_resources()) {
			return false;
		}
	
		if !self.node_equals(&n.get_try_block(), &n2.get_try_block()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: CatchClause = arg as CatchClause;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.node_equals(&n.get_parameter(), &n2.get_parameter()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: LambdaExpr = arg as LambdaExpr;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_enclosing_parameters(), &n2.is_enclosing_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_parameters(), &n2.get_parameters()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MethodReferenceExpr = arg as MethodReferenceExpr;
		if !self.obj_equals(&n.get_identifier(), &n2.get_identifier()) {
			return false;
		}
	
		if !self.node_equals(&n.get_scope(), &n2.get_scope()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_arguments(), &n2.get_type_arguments()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: TypeExpr = arg as TypeExpr;
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ImportDeclaration = arg as ImportDeclaration;
		if !self.obj_equals(&n.is_asterisk(), &n2.is_asterisk()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_module(), &n2.is_module()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_static(), &n2.is_static()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::node_list::NodeList, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return self.nodes_equals(n as NodeList<Node>, arg as NodeList<Node>);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleDeclaration = arg as ModuleDeclaration;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_directives(), &n2.get_directives()) {
			return false;
		}
	
		if !self.obj_equals(&n.is_open(), &n2.is_open()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleRequiresDirective = arg as ModuleRequiresDirective;
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleExportsDirective = arg as ModuleExportsDirective;
		if !self.nodes_equals(&n.get_module_names(), &n2.get_module_names()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleProvidesDirective = arg as ModuleProvidesDirective;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_with(), &n2.get_with()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleUsesDirective = arg as ModuleUsesDirective;
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ModuleOpensDirective = arg as ModuleOpensDirective;
		if !self.nodes_equals(&n.get_module_names(), &n2.get_module_names()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: ReceiverParameter = arg as ReceiverParameter;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: VarType = arg as VarType;
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: Modifier = arg as Modifier;
		if !self.obj_equals(&n.get_keyword(), &n2.get_keyword()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: SwitchExpr = arg as SwitchExpr;
		if !self.nodes_equals(&n.get_entries(), &n2.get_entries()) {
			return false;
		}
	
		if !self.node_equals(&n.get_selector(), &n2.get_selector()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: YieldStmt = arg as YieldStmt;
		if !self.node_equals(&n.get_expression(), &n2.get_expression()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: TextBlockLiteralExpr = arg as TextBlockLiteralExpr;
		if !self.obj_equals(&n.get_value(), &n2.get_value()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: TypePatternExpr = arg as TypePatternExpr;
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type(), &n2.get_type()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: RecordDeclaration = arg as RecordDeclaration;
		if !self.nodes_equals(&n.get_implemented_types(), &n2.get_implemented_types()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_parameters(), &n2.get_parameters()) {
			return false;
		}
	
		if !self.node_equals(&n.get_receiver_parameter(), &n2.get_receiver_parameter()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_parameters(), &n2.get_type_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_members(), &n2.get_members()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: CompactConstructorDeclaration = arg as CompactConstructorDeclaration;
		if !self.node_equals(&n.get_body(), &n2.get_body()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.node_equals(&n.get_name(), &n2.get_name()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_thrown_exceptions(), &n2.get_thrown_exceptions()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_type_parameters(), &n2.get_type_parameters()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_annotations(), &n2.get_annotations()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: RecordPatternExpr = arg as RecordPatternExpr;
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		if !self.nodes_equals(&n.get_pattern_list(), &n2.get_pattern_list()) {
			return false;
		}
	
		if !self.node_equals(&n.get_type()?, &n2.get_type()?) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MatchAllPatternExpr = arg as MatchAllPatternExpr;
		if !self.nodes_equals(&n.get_modifiers(), &n2.get_modifiers()) {
			return false;
		}
	
		return true;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &com::github::javaparser::ast::visitor::visitable::Visitable) -> /* Java */ java::lang::Boolean /**/ {
		/* final */ let n2: MarkdownComment = arg as MarkdownComment;
		if !self.obj_equals(&n.get_content(), &n2.get_content()) {
			return false;
		}
	
		return true;
	}
}

impl com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor for NoCommentEqualsVisitor {}