use javaparser_core::com::github::javaparser::ast;
use javaparser_core::com::github::javaparser::ast::body;
use javaparser_core::com::github::javaparser::ast::comments;
use javaparser_core::com::github::javaparser::ast::expr;
use javaparser_core::com::github::javaparser::ast::modules;
use javaparser_core::com::github::javaparser::ast::stmt;
use javaparser_core::com::github::javaparser::ast::type;
use javaparser_core::com::github::javaparser::ast::visitor::VoidVisitorAdapter;
use javaparser_core::com::github::javaparser::resolution::UnsolvedSymbolException;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedType;
use javaparser_core::com::github::javaparser::symbolsolver::javaparsermodel::declarations::JavaParserEnumConstantDeclaration;
use javaparser_core::com::github::javaparser::symbolsolver::javaparsermodel::declarations::JavaParserFieldDeclaration;
use javaparser_core::com::github::javaparser::symbolsolver::javaparsermodel::declarations::JavaParserParameterDeclaration;
use javaparser_core::com::github::javaparser::symbolsolver::javaparsermodel::declarations::JavaParserVariableDeclaration;
use javaparser_core::com::github::javaparser::symbolsolver::reflectionmodel::ReflectionEnumConstantDeclaration;
use commons_lang3::org::apache::commons::lang3::StringUtils;
use commons_lang3::org::apache::commons::lang3::Strings;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util;
use java::util::function::Function;
use java::util::stream::Collectors;
use javaparser_core::com::github::javaparser::utils::PositionUtils::sortByBeginPosition;
use javaparser_core::com::github::javaparser::utils::Utils::isNullOrEmpty;
use java::util::Collections::reverse;

pub struct RustVisitor {
	transpiler: java2rust::java_transpiler::JavaTranspiler,
	printer: java2rust::rust_printer::RustPrinter = RustPrinter::new("\t"),
	try_block: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	item: java2rust::rust_item::RustItem,
	method: java2rust::i_rust_function::IRustFunction,
	is_var_decl_stmt: bool = true,
	is_mutating: bool = false,
}

impl RustVisitor {
	pub fn new(transpiler: &java2rust::java_transpiler::JavaTranspiler) -> java2rust::rust_visitor::RustVisitor {
		self.transpiler = transpiler;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.printer.to_string();
	}

	pub fn get_source(&self) -> /* Java */ java::lang::String /**/ {
		return self.printer.get_source();
	}

	fn print_javadoc(&self, javadoc: &com::github::javaparser::ast::comments::javadoc_comment::JavadocComment, arg: &/* Java */ java::lang::Object /**/) {
		if javadoc != null {
			javadoc.accept(self, arg);
		}
	}

	fn accept_and_copy(&self, n: &com::github::javaparser::ast::node::Node, arg: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		let mark: i32 = self.printer.push();
		n.accept(self, arg);
		let result: String = self.printer.get_mark(mark);
		self.printer.drop();
		return result;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("AnnotationDeclaration");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::annotation_member_declaration::AnnotationMemberDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("AnnotationMemberDeclaration");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_access_expr::ArrayAccessExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_name().accept(self, arg);
		self.printer.print("[");
		n.get_index().accept(self, arg);
		self.printer.print("]");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_levels()) {
			let type: String = self.accept_and_cut(&n.get_element_type(), arg);
			let type_or_default_value: String = self.default_value(type);
			if type_or_default_value.equals("None") {
				type = "Option<" + type + ">";
			}
	
			let dims: List<String> = n.get_levels().stream().map(|e|self.accept_and_cut(e, arg)).collect(&Collectors::toList());
			self.printer.print(": ");
			self.printer.print(&self.get_array_declaration(type, dims));
			self.printer.print(" = ");
			self.printer.print(&self.get_array_declaration(type_or_default_value, dims));
		} else {
			self.printer.print(" ");
			if n.get_initializer().isPresent() {
				n.get_initializer().get().accept(self, &n.get_element_type());
			}
	
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, arg: &/* Java */ java::lang::Object /**/) {
		let t: Type =  if arg instanceof Type { arg as Type } else { null };
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_values()) {
			if t != null {
				let dims: List<Integer> = self.get_dimensions(n, t);
				let sb: StringBuilder = StringBuilder::new();
				sb.append(&self.accept_and_cut(t, arg));
				/* Java */ java::util::Collections /**/::reverse(dims);
				for i in dims {
					sb.insert(0, "vec![");
					sb.append("; ").append(i).append("]");
				}
				self.printer.print(": ");
				self.printer.print(&sb.toString());
				self.printer.print(" = ");
			}
			self.printer.print("vec![");
			for val in n.get_values() {
				val.accept(self, null);
				self.printer.print(", ");
			}
			self.printer.println("]");
		}
	}

	fn get_dimensions(&self, mut n: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr, t: &com::github::javaparser::ast::type::type::Type) -> /* Java */ java::util::List /**/ {
		let dimensions: List<Integer> = ArrayList<>::new();
		let actsize: Integer = n.get_values().size();
		while n != null {
			dimensions.add(actsize);
			actsize = null;
			let first_value: Expression = n.get_values().get(0);
			if first_value instanceof ArrayInitializerExpr {
				let size: Integer = null;
				for e in n.get_values() {
					let ai: ArrayInitializerExpr = e as ArrayInitializerExpr;
					if size == null {
						size = ai.get_values().size();
						n = ai;
					} else {
						if size < ai.get_values().size() {
							size = ai.get_values().size();
							n = ai;
						}
					}
				}
				actsize = size;
			} else {
				n = null;
			}
		}
		return dimensions;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::assert_stmt::AssertStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("assert!( ");
		n.get_check().accept(self, arg);
		if n.get_message().isPresent() {
			self.printer.print(" : ");
			n.get_message().get().accept(self, arg);
		}
		self.printer.print(");");
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.is_mutating = true;
		n.get_target().accept(self, arg);
		self.is_mutating = false;
		self.printer.print(&" %s ".formatted(&java2rust::rust_visitor::RustVisitor::description_of(&n.get_operator())));
		n.get_value().accept(self, arg);
	}

	fn description_of(&self, op: &com::github::javaparser::ast::expr::assign_expr::Operator) -> /* Java */ java::lang::String /**/ {
		return match op {
		ASSIGN => "=",
		BINARY_AND => "&=",
		BINARY_OR => "|=",
		XOR => "^=",
		PLUS => "+=",
		MINUS => "-=",
		REMAINDER => "%=",
		DIVIDE => "/=",
		MULTIPLY => "*=",
		LEFT_SHIFT => "<<=",
		SIGNED_RIGHT_SHIFT => "/* signed */ >>=",
		UNSIGNED_RIGHT_SHIFT => "/* unsigned */ >>=",
		};
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_left().accept(self, arg);
		self.printer.print(&" %s ".formatted(&java2rust::rust_visitor::RustVisitor::description_of(&n.get_operator())));
		n.get_right().accept(self, arg);
	}

	fn description_of(&self, op: &com::github::javaparser::ast::expr::binary_expr::Operator) -> /* Java */ java::lang::String /**/ {
		return match op {
		OR => "||",
		AND => "&&",
		BINARY_OR => "|",
		BINARY_AND => "&",
		XOR => "^",
		EQUALS => "==",
		NOT_EQUALS => "!=",
		LESS => "<",
		GREATER => ">",
		LESS_EQUALS => "<=",
		GREATER_EQUALS => ">=",
		LEFT_SHIFT => "<<",
		SIGNED_RIGHT_SHIFT => "/* signed */ >>",
		UNSIGNED_RIGHT_SHIFT => "/* unsigned */ >>",
		PLUS => "+",
		MINUS => "-",
		MULTIPLY => "*",
		DIVIDE => "/",
		REMAINDER => "%",
		};
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::block_comment::BlockComment, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.comment(&n.get_content());
		self.printer.end_comment();
		self.printer.println();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.println("{");
		if n.get_statements() != null {
			self.printer.indent();
			for /* final */ s in n.get_statements() {
				s.accept(self, arg);
				self.printer.println();
			}
			self.printer.unindent();
		}
		self.print_orphan_comments_ending(n);
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&String::valueOf(&n.get_value()));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::break_stmt::BreakStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("break");
		if n.get_label().isPresent() {
			self.printer.print(" '");
			self.printer.print(&n.get_label().get().as_string());
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::cast_expr::CastExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_expression().accept(self, arg);
		self.printer.print(" as ");
		n.get_type().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(" catch (");
		n.get_parameter().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::char_literal_expr::CharLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("'");
		self.printer.print(&n.get_value());
		self.printer.print("'");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::class_expr::ClassExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_type().accept(self, arg);
		self.printer.print(".class");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		/* final */ let static_searched: vec![Vec<bool>; 1] = vec![true, ]
		;
		let select_field_declaration_boolean_function: Function<BodyDeclaration<?>, Boolean> = |mem|{
			if mem instanceof FieldDeclaration {
				return fd.is_static() == static_searched[0];
			} else {
				return false;
			}
		};
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_members()) {
			self.print_members(&n.get_members(), arg, select_field_declaration_boolean_function);
		}
	
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_implemented_types()) {
			self.printer.print("#[derive(");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_implemented_types().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.println(")]");
		}
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		if n.is_interface() {
			self.printer.print("trait ");
		} else {
			self.printer.print("struct ");
		}
		self.printer.print(&n.get_name().as_string());
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_extended_types()) {
			if n.is_interface() {
				self.printer.print(" : ");
				let first: bool = true;
				for i in n.get_extended_types() {
					if first {
						first = false;
					}
					else {self.printer.print(" + ");
					}
	
					i.accept(self, arg);
				}
				self.printer.println(" {");
				self.printer.indent();
			} else {
				self.printer.println(" {");
				self.printer.indent();
				let count: i32 =  if n.get_extended_types().size() > 1 { 0 } else { -1 };
				for /* final */ c in n.get_extended_types() {
					self.printer.print("base" + ( if count >= 0 { count += 1 + "" } else { "" }) + ": ");
					c.accept(self, arg);
					self.printer.println(",");
				}
			}
		} else {
			self.printer.println(" {");
			self.printer.indent();
		}
		static_searched[0] = false;
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_members()) {
			self.print_members(&n.get_members(), arg, select_field_declaration_boolean_function);
		}
		self.print_orphan_comments_ending(n);
		if !n.is_interface() {
			self.printer.unindent();
			self.printer.println("}");
			self.printer.println("");
			self.printer.print("impl ");
			self.printer.print(&n.get_name().as_string());
			self.printer.println(" {");
			self.printer.indent();
		}
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_members()) {
			self.print_members(&n.get_members(), arg, |mem|!(mem instanceof FieldDeclaration));
		}
		self.printer.unindent();
		self.printer.println("}");
	}

	fn print_members(&self, members: &/* Java */ java::util::List /**/, arg: &/* Java */ java::lang::Object /**/, filter: &/* Java */ java::util::function::Function /**/) {
		for /* final */ member in members {
			if filter == null || filter.apply(member) {
				member.accept(self, arg);
			}
		}
	}

	fn print_type_parameters(&self, args: &/* Java */ java::util::List /**/, arg: &/* Java */ java::lang::Object /**/) {
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(args) {
			self.printer.print("<");
			 {
				/* final */ let i: Iterator<TypeParameter> = args.iterator();
				while i.hasNext(){
					/* final */ let t: TypeParameter = i.next();
					t.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.print(">");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print(&n.get_name().as_string());
		if n.is_using_diamond_operator() {
			self.printer.print("<>");
		} else {
			if n.get_type_arguments().isPresent() {
				self.print_type_args(&n.get_type_arguments().get(), arg);
			}
	
		}
	}

	fn print_type_args(&self, args: &/* Java */ java::util::List /**/, arg: &/* Java */ java::lang::Object /**/) {
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(args) {
			self.printer.print("<");
			 {
				/* final */ let i: Iterator<Type> = args.iterator();
				while i.hasNext(){
					/* final */ let t: Type = i.next();
					t.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.print(">");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_package_declaration().isPresent() {
			n.get_package_declaration().get().accept(self, arg);
		}
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_types()) {
			 {
				/* final */ let i: Iterator<TypeDeclaration<?>> = n.get_types().iterator();
				while i.hasNext(){
					i.next().accept(self, arg);
					self.printer.println();
					if i.hasNext() {
						self.printer.println();
					}
				}
			 }
	
		}
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(" if ");
		n.get_condition().accept(self, arg);
		self.printer.print(" { ");
		n.get_then_expr().accept(self, arg);
		self.printer.print(" } else { ");
		n.get_else_expr().accept(self, arg);
		self.printer.print(" }");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if !n.get_type_parameters().is_empty() {
			self.printer.print(" ");
		}
		self.printer.print("fn new");
		self.printer.print("(");
		if !n.get_parameters().is_empty() {
			 {
				/* final */ let i: Iterator<Parameter> = n.get_parameters().iterator();
				while i.hasNext(){
					/* final */ let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(") -> ");
		self.printer.print(&n.get_name().as_string());
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
			self.printer.print(" throws ");
			 {
				/* final */ let i: Iterator<ReferenceType> = n.get_thrown_exceptions().iterator();
				while i.hasNext(){
					/* final */ let reference_type: ReferenceType = i.next();
					reference_type.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(" ");
		n.get_body().accept(self, arg);
		self.printer.println("\n");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::continue_stmt::ContinueStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("continue");
		if n.get_label().isPresent() {
			self.printer.print(" '");
			self.printer.print(&n.get_label().get().as_string());
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::do_stmt::DoStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("loop { ");
		n.get_body().accept(self, arg);
		self.printer.print("if !(");
		n.get_condition().accept(self, arg);
		self.printer.print(") break;");
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::double_literal_expr::DoubleLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let value: String = n.get_value();
		if !StringUtils::contains_any(value, '.', 'e', 'E', 'x', 'X') {
			value = value + ".0";
		}
	
		self.printer.print(&self.remove_plus_and_suffix(value, "D", "d"));
	}

	fn remove_plus_and_suffix(&self, mut value: &/* Java */ java::lang::String /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if value.startsWith("+") {
			value = value.substring(1);
		}
		if value.startsWith(".") {
			value = "0" + value;
		}
		if Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with_any(value, search_strings) {
			value = value.substring(0, value.length() - 1);
		}
		if value.endsWith(".") {
			value = value + "0";
		}
		value = value.replace("d.", ".");
		return value;
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("(");
		if n.get_inner() != null {
			n.get_inner().accept(self, arg);
		}
	
		self.printer.print(")");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&n.get_name().as_string());
		if n.get_arguments() != null {
			self.print_arguments(&n.get_arguments(), arg);
		}
		if !n.get_class_body().is_empty() {
			self.printer.println(" {");
			self.printer.indent();
			self.print_members(&n.get_class_body(), arg, null);
			self.printer.unindent();
			self.printer.println("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		self.printer.print("enum ");
		self.printer.print(&n.get_name().as_string());
		if !n.get_implemented_types().is_empty() {
			self.printer.print(" implements ");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_implemented_types().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.println(" {");
		self.printer.indent();
		if n.get_entries() != null {
			self.printer.println();
			 {
				/* final */ let i: Iterator<EnumConstantDeclaration> = n.get_entries().iterator();
				while i.hasNext(){
					/* final */ let e: EnumConstantDeclaration = i.next();
					e.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		if !n.get_members().is_empty() {
			self.printer.println(";");
			self.print_members(&n.get_members(), arg, null);
		} else {
			if !n.get_entries().is_empty() {
				self.printer.println();
			}
		}
		self.printer.unindent();
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.is_this() {
			if n.get_type_arguments().isPresent() {
				self.print_type_args(&n.get_type_arguments().get(), arg);
			}
	
			self.printer.print("this");
		} else {
			if n.get_expression().isPresent() {
				n.get_expression().get().accept(self, arg);
				self.printer.print(".");
			}
			if n.get_type_arguments().isPresent() {
				self.print_type_args(&n.get_type_arguments().get(), arg);
			}
	
			self.printer.print("super");
		}
		self.print_arguments(&n.get_arguments(), arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_expression().accept(self, arg);
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::field_access_expr::FieldAccessExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_scope().accept(self, arg);
		let access: String = ".";
		let name: String = n.get_name_as_string();
		let r0 = 'try0: {
			let value: ResolvedValueDeclaration = n.resolve();
			let is_static: bool;
			if value.is_field() {
				is_static = match value.as_field() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				}.is_static();
			}
			else if value.is_variable() {
				is_static = false;
			}
			else if value.is_enum_constant() {
				is_static = true;
			}
			else if value.is_type_pattern() {
				is_static = true;
			}
			else if value.is_parameter() {
				is_static = false;
			}
			else if value.is_type() {
				is_static = true;
			}
			else if value.is_method() {
				is_static = false;
			}
			else {//TODO: when there's a way to detect array length, do that and throw an exception on else
				// this is an array `.length` access
				is_static = false;
			}
	
			//			else
			//				throw new UnsupportedOperationException(
			//					"Unsupported FieldAccessExpr: Unknown value '%s' (%s)".formatted(
			//						value.getName(),
			//						value));
			access =  if is_static { "::" } else { "." };
			let ty: ResolvedType = value.get_type();
			if ty.is_reference_type() {
				let id: String = "%s.%s".formatted(&match ty.as_reference_type() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				}.get_id(), &n.get_name_as_string());
				name = self.transpiler.name_of(id, name);
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ UnsolvedSymbolException) => {
				// Assume it's a local type instead
				let r1 = 'try1: {
					let scope: ResolvedType = n.get_scope().calculate_resolved_type();
					access = "::";
					name = self.transpiler.name_of(scope.describe() + "." + name, name);
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ Throwable) => {
						if e.getLocalizedMessage() != null {
							System::err.printf("In unsolved FieldAccessExpr: %s\n", &e.getLocalizedMessage());
						}
						else {System::err.printf("In unsolved FieldAccessExpr: %s\n", e);
						}
	
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			},
			Err(e @ Throwable) => {
				if e.getLocalizedMessage() != null {
					System::err.printf("In FieldAccessExpr: %s\n", &e.getLocalizedMessage());
				}
				else {System::err.printf("In FieldAccessExpr: %s\n", e);
				}
	
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.printer.print(access + name);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		// indent if necessary
		self.printer.print("");
		 {
			/* final */ let i: Iterator<VariableDeclarator> = n.get_variables().iterator();
			while i.hasNext(){
				/* final */ let var: VariableDeclarator = i.next();
				var.accept(self, &n.get_common_type()?);
				if i.hasNext() {
					self.printer.print(", ");
				}
			}
		 }
	
		self.printer.println(",");
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("for ");
		self.isVarDeclStmt = false;
		n.get_variable().accept(self, arg);
		self.isVarDeclStmt = true;
		self.printer.print(" in ");
		n.get_iterable().accept(self, arg);
		self.printer.print(" ");
		self.encapsulate_if_not_block(&n.get_body(), arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_initialization() != null && !n.get_initialization().is_empty() {
			self.printer.println(" {");
			self.printer.indent();
			for /* final */ e in n.get_initialization() {
				e.accept(self, arg);
				self.printer.println(";");
			}
		}
		if n.get_compare().isPresent() {
			self.printer.print("while ");
			n.get_compare().get().accept(self, arg);
		} else {
			self.printer.print("loop ");
		}
		if n.get_update() != null && !n.get_update().is_empty() {
			self.printer.println(" {");
			self.printer.indent();
		}
		self.encapsulate_if_not_block(&n.get_body(), arg);
		self.printer.println();
		if n.get_update() != null && !n.get_update().is_empty() {
			for /* final */ e in n.get_update() {
				e.accept(self, arg);
				self.printer.println(";");
			}
			self.printer.unindent();
			self.printer.println(" }");
		}
		if n.get_initialization() != null && !n.get_initialization().is_empty() {
			self.printer.unindent();
			self.printer.println(" }");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::if_stmt::IfStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("if ");
		n.get_condition().accept(self, arg);
		/* final */ let then_block: bool = n.get_then_stmt() instanceof BlockStmt;
		if // block statement should start on the same line
		then_block {
			self.printer.print(" ");
		}
		else {
			self.printer.println(" {");
			self.printer.indent();
		}
		n.get_then_stmt().accept(self, arg);
		if !then_block {
			self.printer.unindent();
			self.printer.println();
			self.printer.println("}");
		}
		if n.get_else_stmt().isPresent() {
			if then_block {
				self.printer.print(" ");
			}
	
			/* final */ let else_if: bool = n.get_else_stmt().get() instanceof IfStmt;
			/* final */ let else_block: bool = n.get_else_stmt().get() instanceof BlockStmt;
			if // put chained if and start of block statement on a same level
			else_if || else_block {
				self.printer.print("else ");
			}
			else {
				self.printer.print("else {");
				self.printer.indent();
			}
			n.get_else_stmt().get().accept(self, arg);
			if !(else_if || else_block) {
				self.printer.unindent();
				self.printer.println();
				self.printer.println("}");
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.is_static() {
			self.printer.print("static ");
		}
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_expression().accept(self, arg);
		self.printer.print(" instanceof ");
		n.get_type().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::integer_literal_expr::IntegerLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let output: String = self.remove_plus_and_suffix(&n.get_value());
		if self.is_float_in_history(n) {
			self.printer.print(output + ".0");
		} else {
			self.printer.print(output);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("TraditionalJavadocComment");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::labeled_stmt::LabeledStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("'");
		self.printer.print(&n.get_label().as_string());
		self.printer.print(": ");
		n.get_statement().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::line_comment::LineComment, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.print("//");
		let tmp: String = n.get_content();
		tmp = tmp.replace('\r', ' ');
		tmp = tmp.replace('\n', ' ');
		self.printer.println(tmp);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::long_literal_expr::LongLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&self.remove_plus_and_suffix(&n.get_value(), "l", "L"));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("MarkerAnnotationExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::member_value_pair::MemberValuePair, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&n.get_name().as_string());
		self.printer.print(" = ");
		n.get_value().accept(self, arg);
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let access: String = ".";
		let name: String = n.get_name_as_string();
		let scope: String = null;
		let thrown: Set<ResolvedType> = null;
		let is_void: bool = false;
		let is_within_try: bool = !self.try_block.isEmpty();
		let r0 = 'try0: {
			let resolved: ResolvedMethodDeclaration = n.resolve();
			access =  if resolved.is_static() { "::" } else { "." };
			name = self.transpiler.name_of(&resolved.get_qualified_signature(), name);
			is_void = resolved.get_return_type().is_void();
			let method: RustMethod = self.transpiler.method(resolved);
			if method != null && !method.thrown().isEmpty() {
				thrown = method.thrown();
			}
	
			if method != null {
				self.is_mutating = method.params().is_mut_self();
			}
	
			if n.get_scope().isEmpty() {
				scope =  if resolved.is_static() { self.transpiler.describe(&resolved.declaring_type()) } else { "self" };
			}
	
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In MethodCallExpr: %s\n", &e.getLocalizedMessage());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		if is_within_try && thrown != null {
			self.printer.print( if is_void { "if let Err(e) = " } else { "match " });
		}
	
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
		}
	
		self.is_mutating = false;
		if n.get_type_arguments().isPresent() {
			self.print_type_args(&n.get_type_arguments().get(), arg);
		}
	
		if scope != null {
			self.printer.print(scope);
		}
	
		self.printer.print(access + name);
		self.print_arguments(&n.get_arguments(), arg);
		if !is_within_try && thrown != null {
			self.printer.print("?");
		}
	
		if !is_within_try || thrown == null {
			return;
		}
	
		self.printer.println(" {");
		self.printer.indent();
		if is_void {
			self.printer.println("return Err(e);");
		} else {
			let ret: String =  if self.try_block.isEmpty() { "return" } else { "break %s".formatted(&self.try_block.peek()) };
			self.printer.println(&"Err(e) => %s Err(e),".formatted(ret));
			self.printer.println("Ok(s) => s,");
		}
		self.printer.unindent();
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.RuntimeException) */ {
		self.print_orphan_comments_before_this_child_node(n)?;
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		for a in n.get_annotations() {
			if a.get_name().get_identifier().equals("Test") {
				self.printer.println("#[test]");
			}
		}
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		self.printer.print("fn ");
		if n.is_default() {
			self.printer.print("default ");
		}
		self.print_type_parameters(&n.get_type_parameters(), arg);
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_parameters()) {
			self.printer.print(" ");
		}
		let mark: i32 = self.printer.push();
		n.get_type().accept(self, arg);
		let type_string: String = self.printer.get_mark(mark);
		self.printer.pop();
		self.printer.print(&self.to_snake_if_necessary(&n.get_name().as_string()));
		self.printer.print("(");
		if !n.is_static() {
			self.printer.print("&self");
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
				self.printer.print(", ");
			}
	
		}
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_parameters()) {
			 {
				/* final */ let i: Iterator<Parameter> = n.get_parameters().iterator();
				while i.hasNext(){
					/* final */ let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(") ");
		if !type_string.equals("void") {
			self.printer.print("-> ");
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
				self.replace_throws(n, arg, type_string);
			} else {
				self.printer.print(type_string);
			}
			self.printer.print(" ");
		} else {
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_thrown_exceptions()) {
				self.printer.print(" -> ");
				self.replace_throws(n, arg, "Void");
			}
		}
		if n.get_body().isEmpty() {
			self.printer.print(";");
		} else {
			n.get_body().get().accept(self, arg);
		}
		self.printer.println("\n");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let name: String = self.to_snake_if_necessary(&n.get_name_as_string());
		let error: bool = false;
		let r0 = 'try0: {
			let resolved: ResolvedValueDeclaration = n.resolve();
			if resolved.is_field() && self.item != null {
				if Objects::equals(&self.item.id(), &match resolved.as_field() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				}.declaring_type().get_id()) {
					self.printer.print("self.");
					self.printer.print(name);
				}
			} else if resolved instanceof JavaParserEnumConstantDeclaration || resolved instanceof ReflectionEnumConstantDeclaration {
				//TODO: Print the type access first?
				self.printer.print(&n.get_name_as_string());
			} else if resolved instanceof JavaParserParameterDeclaration {
				let param: RustParam = self.method.params().java(&java.getWrappedNode());
				if param != null {
					if self.isMutating {
						param.isMutable = true;
					}
	
					self.printer.print(param.name);
				} else {
					let parent: Node = java.getWrappedNode().getParentNode().orElse(null);
					if parent instanceof LambdaExpr {
					} else {
						//TODO: This may happen in sub-method declarations such as anonymous classes, use a method stack instead.
						//TODO: This may happen in catch clauses
						System::err.printf("In NameExpr: could not match parameter '%s' to method.\n\t method: %s\n", &java.getWrappedNode(), self.method);
					}
					self.printer.print(name);
				}
			} else if resolved instanceof JavaParserVariableDeclaration || resolved instanceof JavaParserFieldDeclaration {
				//TODO: Get their values and add later
				self.printer.print(name);
			} else {
				System::err.printf("In NameExpr: unknown resolved value: %s\n", &resolved.get_type());
				error = true;
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ UnsolvedSymbolException) => {
				//TODO: Check that this name actually resolves to a type first
				self.printer.print(&n.get_name_as_string());
			},
			Err(e @ Throwable) => {
				if t.getLocalizedMessage() != null {
					System::err.printf("In NameExpr: %s\n", &t.getLocalizedMessage());
				}
				else {System::err.printf("In NameExpr: %s\n", t);
				}
	
				error = true;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		if error {
			self.printer.print(name);
		}
	
		/* 
			Optional<Pair<TypeDescription, Node>> b = idTracker.findDeclarationNodeFor(
				n.getName()
					.asString(), n);
	
			if (b.isPresent() && (NodeEvaluator.isNonStaticFieldDeclaration(b.get().b) && idTracker.isOutsideConstructor() || NodeEvaluator.isNonStaticMethodDeclaration(
				b.get().b))) {
				printer.print("self.");
			}
			*/ 
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("NormalAnnotationExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("null");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::object_creation_expr::ObjectCreationExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_scope().isPresent() {
			n.get_scope().get().accept(self, arg);
			self.printer.print(".");
		}
		if n.get_type_arguments().isPresent() {
			self.print_type_args(&n.get_type_arguments().get(), arg);
			if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_arguments().get()) {
				self.printer.print(" ");
			}
		}
		n.get_type().accept(self, arg);
		self.printer.print("::new");
		self.print_arguments(&n.get_arguments(), arg);
		if n.get_anonymous_class_body().isPresent() {
			self.printer.println(" {");
			self.printer.indent();
			self.print_members(&n.get_anonymous_class_body().get(), arg, null);
			self.printer.unindent();
			self.printer.print("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::package_declaration::PackageDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("// package ");
		n.get_name().accept(self, arg);
		self.printer.println(";");
		self.printer.println();
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::parameter::Parameter, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_modifiers().accept(self, arg);
		if n.is_var_args() {
			self.printer.comment("... ");
		}
	
		self.printer.end_comment();
		n.get_name().accept(self, arg);
		if n.get_type().is_unknown_type() {
			return;
		}
	
		self.printer.print(": ");
		if n.get_type().is_reference_type() {
			self.printer.print("&");
		}
	
		n.get_type().accept(self, arg);
	}

	pub fn description_of(&self, primitive: &com::github::javaparser::ast::type::primitive_type::Primitive) -> /* Java */ java::lang::String /**/ {
		return match primitive {
		BOOLEAN => "bool",
		BYTE => "i8",
		CHAR => "char",
		DOUBLE => "f64",
		FLOAT => "f32",
		INT => "i32",
		LONG => "i64",
		SHORT => "i16",
		};
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::primitive_type::PrimitiveType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&java2rust::rust_visitor::RustVisitor::description_of(&n.get_type()));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_qualifier().isPresent() {
			n.get_qualifier().get().accept(self, arg);
			self.printer.print("::");
		}
		self.printer.print(&n.get_identifier());
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::simple_name::SimpleName, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&self.to_snake_if_necessary(&n.get_identifier()));
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::array_type::ArrayType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		 {
			let i: i32 = 0;
			while i < n.get_array_level() {
				{
					self.printer.print("Vec<");
				}
				i += 1;
			 }
		 }
	
		n.get_element_type().accept(self, arg);
		 {
			let i: i32 = 0;
			while i < n.get_array_level() {
				{
					self.printer.print(">");
				}
				i += 1;
			 }
		 }
	
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::array_creation_level::ArrayCreationLevel, arg: &/* Java */ java::lang::Object /**/) {
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::intersection_type::IntersectionType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let is_first: bool = true;
		for element in n.get_elements() {
			element.accept(self, arg);
			if is_first {
				is_first = false;
			} else {
				self.printer.print(" & ");
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::union_type::UnionType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let is_first: bool = true;
		for element in n.get_elements() {
			element.accept(self, arg);
			if is_first {
				is_first = false;
			} else {
				self.printer.print(" | ");
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::return_stmt::ReturnStmt, arg: &/* Java */ java::lang::Object /**/) {
		//TODO: get method associated to this return, ensure the return expression matches the return type
		// The final expression is either `Ok(EXPR)`, `EXPR`, or `EXPR.into()`
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("return");
		if n.get_expression().isPresent() {
			self.printer.print(" ");
			//			if (idTracker.hasThrows()) {
			//				printer.print("Ok(");
			//			}
			n.get_expression().get().accept(self, arg);
		//			if (idTracker.hasThrows()) {
		//				printer.print(")");
		//			}
		}
		self.printer.print(";");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("SingleMemberAnnotationExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::string_literal_expr::StringLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("\"");
		self.printer.print(&n.get_value());
		self.printer.print("\"");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::super_expr::SuperExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
			self.printer.print(".");
		}
		self.printer.print("super");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_entry::SwitchEntry, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException) */ {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_labels().is_non_empty() {
			n.get_labels().get_first().orElseThrow().accept(self, arg);
			n.get_labels().stream().skip(1).forEach(|label|{
				self.printer.print(" | ");
				label.accept(self, arg);
			});
		}
		if n.is_default() {
			self.printer.print("_");
		}
	
		if n.get_guard().isPresent() {
			self.printer.print(" if ");
			n.get_guard().get().accept(self, arg);
		}
		self.printer.print(" => ");
		let first: Statement = n.get_statements().get_first().orElse(null);
		if n.get_statements().size() == 1 && first != null && first.is_expression_stmt() {
			first.as_expression_stmt()?.get_expression().accept(self, arg);
			self.printer.println(",");
		} else {
			self.printer.println(" {");
			self.printer.indent();
			for /* final */ s in n.get_statements() {
				s.accept(self, arg);
				self.printer.println();
			}
			self.printer.unindent();
			self.printer.println("}");
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("match ");
		n.get_selector().accept(self, arg);
		self.printer.println(" {");
		if n.get_entries() != null {
			self.printer.indent();
			for /* final */ e in n.get_entries() {
				e.accept(self, arg);
			}
			self.printer.unindent();
		}
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::synchronized_stmt::SynchronizedStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("synchronized (");
		n.get_expression().accept(self, arg);
		self.printer.print(") ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_type_name().isPresent() {
			n.get_type_name().get().accept(self, arg);
		} else {
			self.printer.print("self");
		//			if (idTracker.isOutsideConstructor())
		//				printer.print("self");
		//			else {
		//				printer.print("let ");
		//			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print( if self.try_block.isEmpty() { "return " } else { "break %s ".formatted(&self.try_block.pop()) });
		self.printer.print("Err(");
		n.get_expression().accept(self, arg);
		self.printer.print(");");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::try_stmt::TryStmt, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException) */ {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let label: String = "'try%s".formatted(&self.try_block.size());
		let result: String = "r%s".formatted(&self.try_block.size());
		self.try_block.push(label);
		self.printer.println(&"let %s = %s: {".formatted(result, label));
		self.printer.indent();
		if !n.get_resources().is_empty() {
			//TODO: review using resources and ensuring they're dropped at the end of this block?
			self.printer.print("(");
			let resources: Iterator<Expression> = n.get_resources().iterator();
			let first: bool = true;
			while resources.hasNext() {
				self.visit(&resources.next().as_variable_declaration_expr()?, arg);
				if resources.hasNext() {
					self.printer.print(";");
					self.printer.println();
					if first {
						self.printer.indent();
					}
				}
				first = false;
			}
			if n.get_resources().size() > 1 {
				self.printer.unindent();
			}
			self.printer.print(") ");
		}
		for stmt in n.get_try_block().get_statements() {
			stmt.accept(self, arg);
			self.printer.println();
		}
		self.printer.println(&"break %s Ok(());".formatted(label));
		self.printer.unindent();
		self.printer.println("};");
		if n.get_catch_clauses() != null {
			self.printer.println(&"match %s {".formatted(result));
			self.printer.indent();
			for /* final */ c in n.get_catch_clauses() {
				self.printer.print("Err(e @ ");
				c.get_parameter().get_type().accept(self, arg);
				self.printer.print(") => ");
				c.get_body().accept(self, arg);
				self.printer.println(",");
			}
			self.printer.println("Err(e) => Err(e)?,");
			self.printer.println("Ok => (),");
			self.printer.unindent();
			self.printer.print("}");
		}
		if n.get_finally_block().isPresent() {
			self.printer.println();
			for stmt in n.get_finally_block().get().get_statements() {
				stmt.accept(self, arg);
				self.printer.println();
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_class_declaration_stmt::LocalClassDeclarationStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("LocalClassDeclarationStmt");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::local_record_declaration_stmt::LocalRecordDeclarationStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("LocalRecordDeclarationStmt");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::type_parameter::TypeParameter, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print(&n.get_name().as_string());
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(&n.get_type_bound()) {
			self.printer.print(" extends ");
			 {
				/* final */ let i: Iterator<ClassOrInterfaceType> = n.get_type_bound().iterator();
				while i.hasNext(){
					/* final */ let c: ClassOrInterfaceType = i.next();
					c.accept(self, arg);
					if i.hasNext() {
						self.printer.print(" & ");
					}
				}
			 }
	
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let unary_suffix: String = "";
		match n.get_operator() {
			PREFIX_INCREMENT => unary_suffix = " += 1",
			POSTFIX_INCREMENT =>  {
				if unary_suffix.isEmpty() {
					unary_suffix = " += 1" + ( if self.is_embedded_in_stmt(n) { " !!!check!!! post increment" } else { "" });
				}
	
			}
			PREFIX_DECREMENT =>  {
				if unary_suffix.isEmpty() {
					unary_suffix = " -= 1";
				}
	
			}
			POSTFIX_DECREMENT =>  {
				if unary_suffix.isEmpty() {
					unary_suffix = " -= 1" + ( if self.is_embedded_in_stmt(n) { " !!!check!!! post decrement" } else { "" });
				}
	
			}
			PLUS =>  {
				n.get_expression().accept(self, arg);
				self.printer.print(unary_suffix);
				break;
			}
			_ => self.org_visit(n, arg),
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::unknown_type::UnknownType, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("UnknownType");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.AssertionError) */ {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		n.get_modifiers().accept(self, arg);
		self.printer.end_comment();
		self.printer.print("");
		 {
			/* final */ let i: Iterator<VariableDeclarator> = n.get_variables().iterator();
			while i.hasNext(){
				/* final */ let v: VariableDeclarator = i.next();
				v.accept(self, &n.get_common_type()?);
				if i.hasNext() {
					self.printer.print("; ");
				}
	
			}
		 }
	
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let name: String = self.accept_and_cut(&n.get_name(), arg);
		if !self.isVarDeclStmt {
			self.printer.print(name);
			return;
		}
		let is_constant: bool = false;
		if Character::isUpperCase(&name.charAt(0)) {
			self.printer.print("const ");
			is_constant = true;
		} else {
			self.printer.print("let ");
		//			if (idTracker.isChanged(name, n)) {
		//				printer.print("mut ");
		//			}
		}
		self.printer.print(name);
		let is_initialized_array: bool = n.get_initializer().isPresent() && (n.get_initializer().get() instanceof ArrayInitializerExpr || n.get_initializer().get() instanceof ArrayCreationExpr);
		if arg instanceof Type && !is_initialized_array {
			self.printer.print(": ");
			let tmp: String = self.accept_and_cut(t, null);
			if is_constant && tmp.equals("String") {
				self.printer.print("&'static str");
			} else {
				self.printer.print(tmp);
			}
		}
		if n.get_initializer().isPresent() {
			if !is_initialized_array {
				self.printer.print(" = ");
			}
	
			n.get_initializer().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::void_type::VoidType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("void");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::while_stmt::WhileStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("while ");
		n.get_condition().accept(self, arg);
		self.printer.print(" ");
		n.get_body().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::wildcard_type::WildcardType, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("?");
		if n.get_extended_type().isPresent() {
			self.printer.print(" extends ");
			n.get_extended_type().get().accept(self, arg);
		}
		if n.get_super_type().isPresent() {
			self.printer.print(" super ");
			n.get_super_type().get().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Object /**/) {
		//TODO: Push lambda (as RustLambda?) to stack, use it in param resolution
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let parameters: List<Parameter> = n.get_parameters();
		let print_par: bool = n.is_enclosing_parameters();
		self.printer.print("|");
		if print_par {
			self.printer.print("(");
		}
		if parameters != null {
			 {
				let i: Iterator<Parameter> = parameters.iterator();
				while i.hasNext(){
					let p: Parameter = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		if print_par {
			self.printer.print(")");
		}
		self.printer.print("|");
		let body: Statement = n.get_body();
		if body instanceof ExpressionStmt {
			// Print the expression directly
			stmt.get_expression().accept(self, arg);
		} else {
			body.accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_reference_expr::MethodReferenceExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		let scope: Expression = n.get_scope();
		let identifier: String = n.get_identifier();
		if scope != null {
			n.get_scope().accept(self, arg);
		}
		self.printer.print("::");
		if n.get_type_arguments().isPresent() {
			self.printer.print("<");
			 {
				let i: Iterator<Type> = n.get_type_arguments().get().iterator();
				while i.hasNext(){
					let p: Type = i.next();
					p.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
			self.printer.print(">");
		}
		if identifier != null {
			self.printer.print(identifier);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_expr::TypeExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		if n.get_type() != null {
			n.get_type().accept(self, arg);
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		self.printer.print("use ");
		if n.is_static() {
			self.printer.comment("static");
		}
		n.get_name().accept(self, arg);
		if n.is_asterisk() {
			self.printer.print("::*");
		}
		self.printer.println(";");
		self.print_orphan_comments_ending(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleDeclaration");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_requires_directive::ModuleRequiresDirective, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleRequiresDirective");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_exports_directive::ModuleExportsDirective, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleExportsDirective");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_provides_directive::ModuleProvidesDirective, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleProvidesDirective");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_uses_directive::ModuleUsesDirective, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleUsesDirective");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modules::module_opens_directive::ModuleOpensDirective, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ModuleOpensDirective");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("UnparsableStmt");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("ReceiverParameter");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::var_type::VarType, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("VarType");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::modifier::Modifier, arg: &/* Java */ java::lang::Object /**/) {
		match n.get_keyword() {
			DEFAULT =>  {
				self.printer.comment("default ");
				break;
			}
			PUBLIC =>  {
				self.printer.end_comment();
				self.printer.print("pub ");
				break;
			}
			PROTECTED =>  {
				self.printer.comment("protected ");
				break;
			}
			PRIVATE =>  {
				// everything is private by default in Rust
				break;
			}
			ABSTRACT =>  {
				self.printer.comment("abstract ");
				break;
			}
			STATIC =>  {
				self.printer.comment("static ");
				break;
			}
			FINAL =>  {
				self.printer.comment("final ");
				break;
			}
			TRANSIENT =>  {
				self.printer.comment("transient ");
				break;
			}
			VOLATILE =>  {
				self.printer.comment("volatile ");
				break;
			}
			SYNCHRONIZED =>  {
				self.printer.comment("synchronized ");
				break;
			}
			NATIVE =>  {
				self.printer.comment("native ");
				break;
			}
			STRICTFP =>  {
				self.printer.comment("strictfp ");
				break;
			}
			TRANSITIVE =>  {
				self.printer.comment("transitive ");
				break;
			}
			SEALED =>  {
				self.printer.comment("sealed ");
				break;
			}
			NON_SEALED =>  {
				self.printer.comment("nonsealed ");
				break;
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::switch_expr::SwitchExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.print("match ");
		n.get_selector().accept(self, arg);
		self.printer.println(" {");
		n.get_entries().forEach(|p|p.accept(self, arg));
		self.printer.print("}");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::text_block_literal_expr::TextBlockLiteralExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.println("r#\"");
		let lines: List<String> = n.strip_indent_of_lines().collect(&Collectors::toList());
		lines.removeLast();
		lines.forEach(printer::println);
		self.printer.print("\"#");
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::yield_stmt::YieldStmt, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("YieldStmt");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("TypePatternExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("RecordDeclaration");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("CompactConstructorDeclaration");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::record_pattern_expr::RecordPatternExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("RecordPatternExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::match_all_pattern_expr::MatchAllPatternExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.start_comment();
		self.printer.print("MatchAllPatternExpr");
		self.printer.println(&n.to_string());
		self.printer.end_comment();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::comments::markdown_comment::MarkdownComment, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.print("/**");
		self.printer.print(&n.get_content());
		self.printer.println("*/");
	}

	fn is_embedded_in_stmt(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr) -> bool {
		let parent: Node = n.get_parent_node().get();
		return !(parent instanceof ExpressionStmt) && !(parent instanceof ForStmt);
	}

	fn org_visit(&self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.print_java_comment(&n.get_comment().orElse(null), arg);
		match n.get_operator() {
			PLUS =>  {
				self.printer.print("+");
				break;
			}
			MINUS =>  {
				self.printer.print("-");
				break;
			}
			BITWISE_COMPLEMENT =>  {
				self.printer.print("~");
				break;
			}
			LOGICAL_COMPLEMENT =>  {
				self.printer.print("!");
				break;
			}
			PREFIX_INCREMENT =>  {
				self.printer.print("++");
				break;
			}
			PREFIX_DECREMENT =>  {
				self.printer.print("--");
				break;
			}
			_ =>  {
			}
		}
		n.get_expression().accept(self, arg);
		match n.get_operator() {
			POSTFIX_INCREMENT =>  {
				self.printer.print("++");
				break;
			}
			POSTFIX_DECREMENT =>  {
				self.printer.print("--");
				break;
			}
			_ =>  {
			}
		}
	}

	fn to_snake_if_necessary(&self, n: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let name: String = self.transpiler.name_of(n);
		if name != null {
			return name;
		}
	
		return Java2Rust::identifier(n);
	}

	fn replace_throws(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Object /**/, type_string: &/* Java */ java::lang::String /**/) {
		self.printer.start_comment();
		self.printer.comment("throws ");
		 {
			/* final */ let i: Iterator<ReferenceType> = n.get_thrown_exceptions().iterator();
			while i.hasNext(){
				/* final */ let name: ReferenceType = i.next();
				name.accept(self, arg);
				if i.hasNext() {
					self.printer.comment(", ");
				}
			}
		 }
	
		self.printer.end_comment();
		self.printer.print("Result<");
		self.printer.print(type_string);
		self.printer.print(", Rc<Exception>>");
	}

	fn encapsulate_if_not_block(&self, n: &com::github::javaparser::ast::stmt::statement::Statement, arg: &/* Java */ java::lang::Object /**/) {
		if n instanceof BlockStmt {
			n.accept(self, arg);
		} else {
			self.printer.println("{");
			self.printer.indent();
			n.accept(self, arg);
			self.printer.unindent();
			self.printer.println();
			self.printer.print("}");
		}
	}

	fn print_arguments(&self, args: &/* Java */ java::util::List /**/, arg: &/* Java */ java::lang::Object /**/) {
		self.printer.print("(");
		if !com::github::javaparser::utils::utils::Utils::is_null_or_empty(args) {
			 {
				/* final */ let i: Iterator<Expression> = args.iterator();
				while i.hasNext(){
					/* final */ let e: Expression = i.next();
					if e instanceof NameExpr {
					/* 
						Optional<Pair<TypeDescription, Node>> decl = idTracker.findDeclarationNodeFor(
							ne.getName()
								.asString(), ne);
						if (decl.isPresent() && decl.get().b != null) {
							final TypeDescription left = decl.get().a;
							if (left != null && (!left.clazz.isPrimitive() || left.getArrayCount() > 0)) {
								printer.print("&");
							}
						}
						 */ 
					} else if e instanceof MethodCallExpr {
						self.printer.print("&");
					}
					e.accept(self, arg);
					if i.hasNext() {
						self.printer.print(", ");
					}
				}
			 }
	
		}
		self.printer.print(")");
	}

	fn print_orphan_comments_before_this_child_node(&self, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.RuntimeException) */ {
		if node instanceof Comment {
			return;
		}
	
		if node.get_parent_node().isEmpty() {
			return;
		}
	
		let parent: Node = node.get_parent_node().get();
		let everything: List<Node> = LinkedList<Node>::new();
		everything.addAll(&parent.get_child_nodes());
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(everything);
		let position_of_the_child: i32 = -1;
		 {
			let i: i32 = 0;
			while i < everything.size() {
				{
					if everything.get(i) == node {
						position_of_the_child = i;
					}
	
				}
				i += 1;
			 }
		 }
	
		if position_of_the_child == -1 {
			return Err(RuntimeException::new("My index not found!!! " + node));
		}
	
		let position_of_previous_child: i32 = -1;
		 {
			let i: i32 = position_of_the_child - 1;
			while i >= 0 && position_of_previous_child == -1 {
				{
					if !(everything.get(i) instanceof Comment) {
						position_of_previous_child = i;
					}
	
				}
				i -= 1;
			 }
		 }
	
		 {
			let i: i32 = position_of_previous_child + 1;
			while i < position_of_the_child {
				{
					let node_to_print: Node = everything.get(i);
					if !(node_to_print instanceof Comment) {
						return Err(RuntimeException::new(&"Expected comment, instead %s. Position of previous child: %d, position of child %d".formatted(&node_to_print.getClass(), position_of_previous_child, position_of_the_child)));
					}
	
					node_to_print.accept(self, null);
				}
				i += 1;
			 }
		 }
	
	}

	fn print_orphan_comments_ending(&self, node: &com::github::javaparser::ast::node::Node) {
		let everything: List<Node> = LinkedList<Node>::new(&node.get_child_nodes());
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(everything);
		if everything.isEmpty() {
			return;
		}
		let comments_at_end: i32 = 0;
		let finding_comments: bool = true;
		while finding_comments && comments_at_end < everything.size() {
			let last: Node = everything.get(everything.size() - 1 - comments_at_end);
			finding_comments = (last instanceof Comment);
			if finding_comments {
				comments_at_end += 1;
			}
		}
		 {
			let i: i32 = 0;
			while i < comments_at_end {
				{
					everything.get(everything.size() - comments_at_end + i).accept(self, null);
				}
				i += 1;
			 }
		 }
	
	}

	fn accept_and_cut(&self, n: &com::github::javaparser::ast::node::Node, arg: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		let mark: i32 = self.printer.push();
		n.accept(self, arg);
		let result: String = self.printer.get_mark(mark);
		self.printer.pop();
		return result;
	}

	fn default_value(&self, type: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return match type {
		"f64" | "f32" => "0.0",
		"u64" | "u32" | "u16" | "u8" | "usize" | "i64" | "i32" | "i16" | "i8" => "0",
		"bool" => "false",
		_ => "None",
		};
	}

	fn get_array_declaration(&self, type_or_default_value: &/* Java */ java::lang::String /**/, dims: &/* Java */ java::util::List /**/) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append(type_or_default_value);
		/* Java */ java::util::Collections /**/::reverse(dims);
		for s in dims {
			sb.insert(0, "[");
			sb.append("; ").append(s).append("]");
		}
		/* Java */ java::util::Collections /**/::reverse(dims);
		return sb.toString();
	}

	fn print_java_comment(&self, javacomment: &com::github::javaparser::ast::comments::comment::Comment, arg: &/* Java */ java::lang::Object /**/) {
		if javacomment != null {
			javacomment.accept(self, arg);
		}
	}

	fn replace_length_at_end(&self, field_access: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if field_access.equals("length") {
			return "len()";
		}
		else {return field_access;
		}
	
	}

	fn gen_string_expr_sequence(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr) -> /* Java */ java::util::List /**/ {
		let result: List<Node> = ArrayList<>::new();
		if n.get_operator() == BinaryExpr::com::github::javaparser::ast::expr::binary_expr::Operator::PLUS {
			self.gen_string_part(&n.get_left(), result);
			self.gen_string_part(&n.get_right(), result);
		} else {
			result.add(n);
			return result;
		}
		return result;
	}

	fn gen_string_part(&self, n: &com::github::javaparser::ast::node::Node, result: &/* Java */ java::util::List /**/) {
		if n instanceof BinaryExpr {
			result.addAll(&self.gen_string_expr_sequence((n as BinaryExpr)));
		} else {
			result.add(n);
		}
	}

	fn print_string_expression(&self, n: &com::github::javaparser::ast::expr::binary_expr::BinaryExpr, arg: &/* Java */ java::lang::Object /**/) {
		let bin_chain: List<Node> = self.gen_string_expr_sequence(n);
		self.printer.print("format!(\"");
		for node in bin_chain {
			if node instanceof StringLiteralExpr {
				let value: String = (node as StringLiteralExpr).get_value();
				self.printer.print(value);
			} else {
				self.printer.print("{}");
			}
		}
		self.printer.print("\"");
		for node in bin_chain {
			if !(node instanceof StringLiteralExpr) && node != n {
				self.printer.print(", ");
				node.accept(self, arg);
			}
		}
		self.printer.print(")");
	}

	fn is_float_in_siblings(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		if n == null || n.get_parent_node().isEmpty() {
			return false;
		}
	
		if self.stop_history_search(&n.get_parent_node().get()) {
			return false;
		}
	
		let siblings: List<Node> = n.get_parent_node().get().get_child_nodes();
		for sibling in siblings {
		//			if (idTracker.isFloat(sibling)) {
		//				return true;
		//			}
		}
		return false;
	}

	fn is_float_in_history(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		if self.stop_history_search(n) {
			return false;
		}
	
		if n == null {
			return false;
		}
	
		if self.is_float_in_siblings(n) {
			return true;
		}
	
		//		} else {
		return self.is_float_in_history(&n.get_parent_node().get());
	//		}
	}

	fn stop_history_search(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		return n instanceof VariableDeclarator || n instanceof MethodCallExpr || n instanceof Statement || n instanceof ArrayAccessExpr;
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for RustVisitor {}