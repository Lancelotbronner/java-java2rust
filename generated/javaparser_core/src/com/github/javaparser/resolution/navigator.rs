use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body;
use crate::com::github::javaparser::ast::expr::MethodCallExpr;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::stmt::ReturnStmt;
use crate::com::github::javaparser::ast::stmt::SwitchStmt;
use java::util::Optional;
use java::util::function::Predicate;

pub struct Navigator;

impl Navigator {
	fn new() -> com::github::javaparser::resolution::navigator::Navigator {
	// prevent instantiation
	}

	pub fn demand_class(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		let cd: ClassOrInterfaceDeclaration = com::github::javaparser::resolution::navigator::Navigator::demand_class_or_interface(cu, qualified_name);
		if cd.is_interface() {
			return Err(IllegalStateException::new("Type is not a class"));
		}
		return cd;
	}

	pub fn demand_class_or_interface(&self, compilation_unit: &com::github::javaparser::ast::compilation_unit::CompilationUnit, qualified_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		return com::github::javaparser::resolution::navigator::Navigator::find_type(compilation_unit, qualified_name).map(|res|res.to_class_or_interface_declaration().orElseThrow(|()|IllegalStateException::new("Type is not a class or an interface, it is " + res.getClass().getCanonicalName()))).orElseThrow(|()|IllegalStateException::new("No type named '" + qualified_name + "'found"));
	}

	pub fn demand_constructor(&self, td: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration, index: i32) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		// TODO: Refactor to use `td.findAll(ConstructorDeclaration.class);` - potential difference re: searching only
		// immediate children?
		let found: ConstructorDeclaration = null;
		let i: i32 = 0;
		for bd in td.get_members() {
			if bd instanceof ConstructorDeclaration {
				let cd: ConstructorDeclaration = bd as ConstructorDeclaration;
				if i == index {
					found = cd;
					break;
				}
				i += 1;
			}
		}
		if found == null {
			return Err(IllegalStateException::new("No constructor with index " + index));
		}
		return found;
	}

	pub fn demand_enum(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		let res: Optional<TypeDeclaration<?>> = com::github::javaparser::resolution::navigator::Navigator::find_type(cu, qualified_name);
		if !res.isPresent() {
			return Err(IllegalStateException::new("No type found"));
		}
		if !(res.get() instanceof EnumDeclaration) {
			return Err(IllegalStateException::new("Type is not an enum"));
		}
		return res.get() as EnumDeclaration;
	}

	pub fn demand_field(&self, cd: &com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		for bd in cd.get_members() {
			if bd instanceof FieldDeclaration {
				let fd: FieldDeclaration = bd as FieldDeclaration;
				for vd in fd.get_variables() {
					if vd.get_name().get_id().equals(name) {
						return vd;
					}
				}
			}
		}
		return Err(IllegalStateException::new("No field with given name"));
	}

	pub fn demand_interface(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::class_or_interface_declaration::ClassOrInterfaceDeclaration {
		let cd: ClassOrInterfaceDeclaration = com::github::javaparser::resolution::navigator::Navigator::demand_class_or_interface(cu, qualified_name);
		if !cd.is_interface() {
			return Err(IllegalStateException::new("Type is not an interface"));
		}
		return cd;
	}

	pub fn demand_method(&self, cd: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		let found: MethodDeclaration = null;
		for bd in cd.get_members() {
			if bd instanceof MethodDeclaration {
				let md: MethodDeclaration = bd as MethodDeclaration;
				if md.get_name_as_string().equals(name) {
					if found != null {
						return Err(IllegalStateException::new("Ambiguous getName"));
					}
					found = md;
				}
			}
		}
		if found == null {
			return Err(IllegalStateException::new("No method called " + name));
		}
		return found;
	}

	pub fn demand_node_of_given_class<N: com::github::javaparser::ast::node::Node>(&self, node: &com::github::javaparser::ast::node::Node, clazz: &/* Java */ java::lang::Class /**/) -> N {
		return node.find_first(clazz).orElseThrow(IllegalArgumentException::new);
	}

	pub fn demand_parent_node(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::Node {
		return node.get_parent_node().orElseThrow(|()|IllegalStateException::new("Parent not found, the node does not appear to be inserted in a correct AST"));
	}

	pub fn demand_parent_node(&self, node: &com::github::javaparser::ast::node::Node, is_accepted_parent_node: &/* Java */ java::util::function::Predicate /**/) -> com::github::javaparser::ast::node::Node {
		let parent: Node = node;
		loop { {
			parent = com::github::javaparser::resolution::navigator::Navigator::demand_parent_node(parent);
		}if !(!is_accepted_parent_node.test(parent)) break;}
		return parent;
	}

	pub fn demand_return_stmt(&self, method: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return com::github::javaparser::resolution::navigator::Navigator::demand_node_of_given_class(method, ReturnStmt.class);
	}

	pub fn demand_switch(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt {
		return com::github::javaparser::resolution::navigator::Navigator::find_switch_helper(node).orElseThrow(IllegalArgumentException::new);
	}

	pub fn demand_variable_declaration(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return node.find_first(VariableDeclarator.class, |n|n.get_name_as_string().equals(name));
	}

	pub fn find_method_call(&self, node: &com::github::javaparser::ast::node::Node, method_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return node.find_first(MethodCallExpr.class, |n|n.get_name_as_string().equals(method_name));
	}

	pub fn find_name_expression(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return node.find_first(NameExpr.class, |n|n.get_name_as_string().equals(name));
	}

	pub fn find_node_of_given_class<N: com::github::javaparser::ast::node::Node>(&self, node: &com::github::javaparser::ast::node::Node, clazz: &/* Java */ java::lang::Class /**/) -> N {
		return com::github::javaparser::resolution::navigator::Navigator::demand_node_of_given_class(node, clazz);
	}

	pub fn find_return_stmt(&self, method: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return com::github::javaparser::resolution::navigator::Navigator::demand_return_stmt(method);
	}

	pub fn find_simple_name(&self, node: &com::github::javaparser::ast::node::Node, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		return node.find_first(SimpleName.class, |n|n.as_string().equals(name));
	}

	pub fn find_switch(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::stmt::switch_stmt::SwitchStmt {
		return com::github::javaparser::resolution::navigator::Navigator::demand_switch(node);
	}

	fn find_switch_helper(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::util::Optional /**/ {
		if node instanceof SwitchStmt {
			return Optional::of(node as SwitchStmt);
		}
		return node.find_first(SwitchStmt.class);
	}

	pub fn find_type(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, qualified_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		if cu.get_types().is_empty() {
			return Optional::empty();
		}
		/* final */ let type_name: String = com::github::javaparser::resolution::navigator::Navigator::get_outer_type_name(qualified_name);
		let type: Optional<TypeDeclaration<?>> = cu.get_types().stream().filter(|(t)|t.get_name().get_id().equals(type_name)).findFirst();
		/* final */ let inner_type_name: String = com::github::javaparser::resolution::navigator::Navigator::get_inner_type_name(qualified_name);
		if type.isPresent() && !inner_type_name.isEmpty() {
			return com::github::javaparser::resolution::navigator::Navigator::find_type(&type.get(), inner_type_name);
		}
		return type;
	}

	pub fn find_type(&self, td: &com::github::javaparser::ast::body::type_declaration::TypeDeclaration, qualified_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		/* final */ let type_name: String = com::github::javaparser::resolution::navigator::Navigator::get_outer_type_name(qualified_name);
		let type: Optional<TypeDeclaration<?>> = Optional::empty();
		for n in td.get_members() {
			if n instanceof TypeDeclaration && (n as TypeDeclaration<?>).get_name().get_id().equals(type_name) {
				type = Optional::of(n as TypeDeclaration<?>);
				break;
			}
		}
		/* final */ let inner_type_name: String = com::github::javaparser::resolution::navigator::Navigator::get_inner_type_name(qualified_name);
		if type.isPresent() && !inner_type_name.isEmpty() {
			return com::github::javaparser::resolution::navigator::Navigator::find_type(&type.get(), inner_type_name);
		}
		return type;
	}

	fn get_inner_type_name(&self, qualified_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if qualified_name.contains(".") {
			return qualified_name.split("\\.", 2)[1];
		}
		return "";
	}

	fn get_outer_type_name(&self, qualified_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return qualified_name.split("\\.", 2)[0];
	}

	pub fn require_parent_node(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::ast::node::Node {
		return com::github::javaparser::resolution::navigator::Navigator::demand_parent_node(node);
	}
}