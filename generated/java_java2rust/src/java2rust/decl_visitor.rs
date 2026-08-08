use javaparser_core::com::github::javaparser::ast::ImportDeclaration;
use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::ast::body;
use javaparser_core::com::github::javaparser::ast::expr;
use javaparser_core::com::github::javaparser::ast::stmt::ThrowStmt;
use javaparser_core::com::github::javaparser::ast::visitor::VoidVisitorAdapter;
use javaparser_core::com::github::javaparser::resolution::UnsolvedSymbolException;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedType;
use crate::java2rust::rust;
use java::util::Objects;
use java::util::Optional;
use java::util::Stack;

pub struct DeclVisitor {
	transpiler: java2rust::java_transpiler::JavaTranspiler,
	modules: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	items: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	fields: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	functions: /* Java */ java::util::Stack /**/ = Stack<>::new(),
	is_mutable_scope: bool,
}

impl DeclVisitor {
	pub fn new(transpiler: &java2rust::java_transpiler::JavaTranspiler, module: &java2rust::rust::rust_package::RustPackage) -> java2rust::decl_visitor::DeclVisitor {
		self.transpiler = transpiler;
		self.modules.push(module);
	}

	pub fn visit(&mut self, n: &com::github::javaparser::ast::expr::assign_expr::AssignExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.is_mutable_scope = true;
		n.get_target().accept(self, arg);
		self.is_mutable_scope = false;
		n.get_value().accept(self, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		let item: RustItem;
		if n.is_interface() {
			item = self.modules.peek().trait(&n.get_name_as_string(), &n.resolve().as_interface()?, &RustVisibility::pub(&n.is_public()));
		} else {
			item = self.modules.peek().clazz(&n.get_name_as_string(), &n.resolve().as_class()?, &RustVisibility::pub(&n.is_public()));
		}
		self.transpiler.register(item);
		self.items.push(item);
		super.visit(n, arg);
		self.items.pop();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		let r0 = 'try0: {
			let method: RustConstructor = self.items.peek().constructor(n);
			self.transpiler.register_name(method.id, method.name);
			self.functions.push(method);
			super.visit(n, arg);
			self.functions.pop();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In ConstructorDeclaration: %s\n", &e.getLocalizedMessage());
			//TODO: push error method.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		let item: RustItem = self.modules.peek().enumeration(n);
		self.transpiler.register(item);
		self.items.push(item);
		super.visit(n, arg);
		self.items.pop();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		self.fields.push(n);
		super.visit(n, arg);
		self.fields.pop();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		let r0 = 'try0: {
			let method: RustInitializer = self.items.peek().initializer(n);
			self.functions.push(method);
			super.visit(n, arg);
			self.functions.pop();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In InitializerDeclaration: %s\n", &e.getLocalizedMessage());
			//TODO: push error method.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::method_call_expr::MethodCallExpr, arg: &/* Java */ java::lang::Object /**/) {
		if self.functions.isEmpty() {
			let n1: Optional<Node> = n.get_parent_node();
			while n1.isPresent() && !(n1.get() instanceof BodyDeclaration<?>) n1 = n1.get().get_parent_node();
			if n1.isEmpty() {
				System::err.printf("Unhandled method call context: <none>\n");
			}
			else if n1.get() instanceof EnumConstantDeclaration {
				//TODO: Handle enum constant decls
				return;
			}
			else {System::err.printf("Unhandled method call context: %s\n", &n1.map(|n2|n2.get_meta_model().get_type_name()).orElse("UNKNOWN"));
			}
	
			return;
		}
		let function: IRustFunction = self.functions.peek();
		if function == null {
			//TODO: handle lambdas
			return;
		}
	
		function.calls().add_callee(n);
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		let r0 = 'try0: {
			let method: RustMethod = self.items.peek().method(n);
			self.transpiler.register(method);
			self.transpiler.register_name(method.id, method.name);
			self.functions.push(method);
			super.visit(n, arg);
			self.functions.pop();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In MethodDeclaration: %s\n", &e.getLocalizedMessage());
			//TODO: push error method.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::name_expr::NameExpr, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		if !self.is_mutable_scope {
			return;
		}
	
		//TODO: global name metadata? method signature + name
		let r0 = 'try0: {
			let resolved: ResolvedValueDeclaration = n.resolve();
			if resolved.is_parameter() {
				let param: RustParam = self.functions.peek().params().java(&n.get_name_as_string());
				if param != null {
					param.isMutable = true;
				}
	
			}
			if resolved.is_field() {
				if Objects::equals(&self.items.peek().id(), &match resolved.as_field() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				}.declaring_type().get_id()) {
					self.functions.peek().params().mutate_self();
				}
	
			}
	
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ UnsolvedSymbolException) => {
			//TODO: This name is a type name, like Optional in Optional.map
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::this_expr::ThisExpr, arg: &/* Java */ java::lang::Object /**/) {
		if !self.is_mutable_scope {
			return;
		}
	
		//TODO: handle type name
		self.functions.peek().params().mutate_self();
		super.visit(n, arg);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt, arg: &/* Java */ java::lang::Object /**/) {
		super.visit(n, arg);
		let method: IRustFunction = self.functions.peek();
		let r0 = 'try0: {
			let ty: ResolvedType = n.get_expression().calculate_resolved_type();
			method.thrown().add(ty);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In ThrowStmt: %s\n", &e.getLocalizedMessage());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, arg: &/* Java */ java::lang::Object /**/) {
		if self.items.isEmpty() {
			assert!( false);
			return;
		}
		if self.fields.isEmpty() {
			assert!( false);
			return;
		}
		if self.fields.peek().is_static() {
			let field: RustStatic = self.items.peek().static_field(&self.fields.peek(), n);
			self.transpiler.register_name(field.id, self.items.peek().path() + "::" + field.name);
			return;
		}
		let field: RustField = self.items.peek().field(&n.get_name_as_string(), &n.get_type(), &n.get_initializer().orElse(null));
		let id: String = "%s.%s".formatted(&self.items.peek().id(), &n.get_name());
		self.transpiler.register_name(id, field.name);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::expr::lambda_expr::LambdaExpr, arg: &/* Java */ java::lang::Object /**/) {
		self.functions.push(null);
		super.visit(n, arg);
		self.functions.pop();
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::import_declaration::ImportDeclaration, arg: &/* Java */ java::lang::Object /**/) {
		// We'll be using this for name resolution.
		self.modules.peek().use(n);
	}

	pub fn visit(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, arg: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		self.items.push(&self.modules.peek().record(&n.get_name_as_string(), &n.resolve().as_record()?, &RustVisibility::pub(&n.is_public())));
		super.visit(n, arg);
		self.items.pop();
	}
}

impl com::github::javaparser::ast::visitor::void_visitor::VoidVisitor for DeclVisitor {}