use javaparser_core::com::github::javaparser::ast::CompilationUnit;
use javaparser_core::com::github::javaparser::ast::ImportDeclaration;
use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::ast::body;
use javaparser_core::com::github::javaparser::ast::comments::Comment;
use javaparser_core::com::github::javaparser::ast::expr;
use javaparser_core::com::github::javaparser::ast::stmt;
use javaparser_core::com::github::javaparser::ast::type::ClassOrInterfaceType;
use javaparser_core::com::github::javaparser::ast::type::PrimitiveType;
use javaparser_core::com::github::javaparser::ast::type::ReferenceType;
use javaparser_core::com::github::javaparser::ast::type::Type;
use javaparser_core::com::github::javaparser::ast::visitor::VoidVisitorAdapter;
use javaparser_core::com::github::javaparser::utils::Pair;
use java::lang::reflect::Method;
use java::util::HashSet;
use java::util::List;
use java::util::Set;

pub struct IdTrackerVisitor {
	in_assign_target: bool = false,
}

impl IdTrackerVisitor {
	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &java2rust::id_tracker::IdTracker) {
		self.visit_comment(&n.get_comment().orElse(null), arg);
		self.in_assign_target = true;
		let r0 = 'try0: {
			n.get_target().accept(self, arg);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.in_assign_target = false;
	
		n.get_value().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::catch_clause::CatchClause, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		let r0 = 'try0: {
			super.visit(n, arg);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		arg.pop_block();
	
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		arg.add_declaration(&n.get_name().as_string(), Pair<>::new(null, n));
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, arg: &java2rust::id_tracker::IdTracker) {
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::compilation_unit::CompilationUnit, arg: &java2rust::id_tracker::IdTracker) {
		self.visit_comment(&n.get_comment().orElse(null), arg);
		if n.get_package_declaration().isPresent() {
			arg.set_package_name(&n.get_package_declaration().get().get_name().as_string());
			n.get_package_declaration().get().accept(self, arg);
		}
		if n.get_imports() != null {
			for /* final */ i in n.get_imports() {
				arg.add_import(Import::new(&i.get_name().to_string(), &i.is_static(), &i.is_asterisk()));
				i.accept(self, arg);
			}
		}
		if n.get_types() != null {
			for /* final */ type_declaration in n.get_types() {
				type_declaration.accept(self, arg);
			}
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		arg.add_declaration(&n.get_name().as_string(), Pair<>::new(null, n));
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::expression_stmt::ExpressionStmt, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		if n.get_expression() instanceof VariableDeclarationExpr {
			/* Java*/ ve/* */ .get_common_type()?;
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_each_stmt::ForEachStmt, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::for_stmt::ForStmt, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		arg.push_block(n)?;
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &java2rust::id_tracker::IdTracker) {
		if n.get_scope().isPresent() && n.get_scope().get() instanceof NameExpr {
			let clazz: Class = self.identifya_class(arg, &/* Java*/ ne/* */ .get_name().as_string());
			if clazz != null {
				let method_name: String = n.get_name().as_string();
				let ms: Vec<Method> = clazz.getMethods();
				let candidates: Set<Method> = HashSet<>::new();
				for m in ms {
					if m.getName().equals(method_name) {
						candidates.add(m);
					}
				}
				let resulting: Method = null;
				if candidates.size() == 1 {
					resulting = candidates.iterator().next();
				} else {
					let matching: List<Method> = candidates.stream().filter(|m|m.getParameterCount() == n.get_arguments().size() || m.isVarArgs() && m.getParameterCount() <= n.get_arguments().size()).toList();
					if matching.size() == 1 {
						resulting = matching.getFirst();
					}
				}
				if resulting != null {
					 {
						let i: i32 = 0;
						while i < resulting.getParameterCount() {
							{
								let p: java.lang.reflect.Parameter = resulting.getParameters()[i];
								if n.get_arguments().size() > i {
									arg.put_type(&n.get_arguments().get(i), &p.getType());
								}
							}
							i += 1;
						 }
					 }
	
				}
			}
		}
		arg.add_usage(&n.get_name().as_string(), n);
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ {
		let r0 = 'try0: {
			arg.add_declaration(&n.get_name().as_string(), Pair<>::new(null, n));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
			// ignore duplicate Methods with the same name. Let it be declared just once, so that self can be constructed.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		if n.get_thrown_exceptions() != null && !n.get_thrown_exceptions().is_empty() {
			arg.set_has_throws(&n.get_name().as_string());
		}
		if let Err(e) = arg.push_block(n) {
			return Err(e);
		};
		super.visit(n, arg);
		arg.pop_block();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &java2rust::id_tracker::IdTracker) {
		if self.in_assign_target {
			arg.add_change(&n.get_name().as_string(), n);
		} else {
			arg.add_usage(&n.get_name().as_string(), n);
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name::Name, arg: &java2rust::id_tracker::IdTracker) {
		if self.in_assign_target {
			arg.add_change(&n.get_qualifier().toString(), n);
		}
		super.visit(n, arg);
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::unary_expr::UnaryExpr, arg: &java2rust::id_tracker::IdTracker) {
		let r0 = 'try0: {
			match n.get_operator() {
				POSTFIX_INCREMENT =>  {
				}
				POSTFIX_DECREMENT =>  {
				}
				PREFIX_INCREMENT =>  {
				}
				PREFIX_DECREMENT =>  {
					self.in_assign_target = true;
					break;
				}
				_ => self.in_assign_target = false,
			}
			n.get_expression().accept(self, arg);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		self.in_assign_target = false;
	
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr, arg: &java2rust::id_tracker::IdTracker) {
		let type_descr: TypeDescription = self.get_type_description(arg, &n.get_common_type()?);
		let type: String = self.get_name_of_type(&n.get_common_type()?);
		if type_descr != null && arg.is_float(&type_descr.get_clazz()) {
			arg.put_type(n, &type_descr.get_clazz());
			if type_descr.get_array_count() > 0 {
				let r0 = 'try0: {
					let initializer: Node = n.get_child_nodes().get(1).get_child_nodes().get(1);
					if !(initializer instanceof MethodCallExpr) {
						let nodes: List<Node> = initializer.get_child_nodes();
						for child in nodes {
							arg.put_type(child, Double::TYPE);
						}
					}
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ RuntimeException) => {
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &java2rust::id_tracker::IdTracker) {
		let is_field: bool = n.get_parent_node().get().get_parent_node().get() instanceof FieldDeclaration;
		let clazz: TypeDescription = self.type_of(n, arg);
		arg.add_declaration(&n.get_name().as_string(), Pair<>::new(clazz, n));
		super.visit(n, arg);
	}

	fn identifya_class(&self, arg: &java2rust::id_tracker::IdTracker, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Class /**/ {
		let clazz: Class = null;
		if name != null {
			for i in arg.get_imports() {
				if !i.is_static_import() {
					if i.is_wildcard_import() {
						clazz = self.for_name(i.get_import_string() + "." + name);
					} else {
						if i.get_import_string().endsWith("." + name) {
							/* final */ let import_string: String = i.get_import_string();
							clazz = self.for_name(import_string);
						}
					}
				}
			}
			if clazz == null {
				clazz = self.for_name("java.lang." + name);
			}
			if clazz == null {
				clazz = self.for_name(arg.get_package_name() + "." + name);
			}
		}
		return clazz;
	}

	fn for_name(&self, import_string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Class /**/ {
		let clazz: Class = null;
		let r0 = 'try0: {
			clazz = Class::forName(import_string);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ClassNotFoundExceptionNoClassDefFoundError | ) => {
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return clazz;
	}

	fn visit_comment(&self, n: &com::github::javaparser::ast::comments::comment::Comment, arg: &java2rust::id_tracker::IdTracker) {
		if n != null {
			n.accept(self, arg);
		}
	}

	fn get_name_of_type(&self, t: &com::github::javaparser::ast::type::type::Type) -> /* Java */ java::lang::String /**/ {
		if t instanceof ClassOrInterfaceType {
			return (t as ClassOrInterfaceType).get_name().as_string();
		} else if t instanceof ReferenceType {
			return self.get_name_of_type(&/* Java*/ rtype/* */ .get_element_type());
		}
		return null;
	}

	fn type_of(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &java2rust::id_tracker::IdTracker) /* thrown(java.lang.AssertionError) */ -> java2rust::type_description::TypeDescription {
		let t: Type = null;
		if n.get_parent_node().get().get_parent_node().get() instanceof FieldDeclaration {
			t = /* Java*/ field_declaration/* */ .get_common_type()?;
		} else if n.get_parent_node().get() instanceof Parameter {
			t = /* Java*/ p/* */ .get_type();
		} else if n.get_parent_node().get().get_parent_node().get() instanceof VariableDeclarationExpr {
			t = /* Java*/ variable_declaration_expr/* */ .get_common_type()?;
		}
		if t != null {
			return self.get_type_description(arg, t);
		}
		return null;
	}

	fn get_type_description(&self, arg: &java2rust::id_tracker::IdTracker, t: &com::github::javaparser::ast::type::type::Type) -> java2rust::type_description::TypeDescription {
		let name: String = self.get_name_of_type(t);
		let clazz: Class = self.identifya_class(arg, name);
		if t instanceof ReferenceType {
			if clazz == null {
				clazz = self.get_potential_primitive_type(&/* Java*/ rtype/* */ .get_element_type());
			}
			if clazz != null {
				return TypeDescription::new(&/* Java*/ rtype/* */ .get_array_level(), clazz);
			}
	
		}
		if clazz == null {
			clazz = self.get_potential_primitive_type(t);
		}
		if clazz == null {
			return null;
		}
		else {
			return TypeDescription::new(0, clazz);
		}
	}

	fn get_potential_primitive_type(&self, t: &com::github::javaparser::ast::type::type::Type) -> /* Java */ java::lang::Class /**/ {
		if t instanceof PrimitiveType {
			match /* Java*/ pt/* */ .get_type().name() {
				"Byte" =>  {
					return Byte::TYPE;
				}
				"Short" =>  {
					return Short::TYPE;
				}
				"Int" =>  {
					return Integer::TYPE;
				}
				"Long" =>  {
					return Long::TYPE;
				}
				"Float" =>  {
					return Float::TYPE;
				}
				"Double" =>  {
					return Double::TYPE;
				}
				"Char" =>  {
					return Character::TYPE;
				}
				"Boolean" =>  {
					return Boolean::TYPE;
				}
				"void" =>  {
					return Void::TYPE;
				}
			}
		}
		return null;
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for IdTrackerVisitor {}