use javaparser_core::com::github::javaparser::ast::body::FieldDeclaration;
use javaparser_core::com::github::javaparser::ast::body::VariableDeclarator;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedFieldDeclaration;

pub struct RustStatic {
	parent: java2rust::rust_item::RustItem,
	visibility: /* Java */ java2rust::RustVisibility /**/,
	name: /* Java */ java::lang::String /**/,
	id: /* Java */ java::lang::String /**/,
	java: com::github::javaparser::ast::body::variable_declarator::VariableDeclarator,
	resolved: com::github::javaparser::resolution::declarations::resolved_field_declaration::ResolvedFieldDeclaration,
	rust_type: /* Java */ java::lang::String /**/,
	rust_initializer: /* Java */ java::lang::String /**/,
}

impl RustStatic {
	pub fn new(parent: &java2rust::rust_item::RustItem, field: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, declarator: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator) /* thrown(java.lang.UnsupportedOperationException) */ -> java2rust::rust_static::RustStatic {
		self.parent = parent;
		self.visibility = RustVisibility::pub(&field.is_public());
		self.name = declarator.get_name_as_string();
		//TODO: Pass other modifiers from FieldDeclaration
		self.java = declarator;
		self.resolved = declarator.resolve().as_field()?;
		self.rust_type = declarator.get_type_as_string();
		self.id = self.resolved.declaring_type().get_id() + "." + declarator.get_name_as_string();
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler, item: &java2rust::rust_item::RustItem) /* thrown(java.lang.UnsupportedOperationException) */ {
		self.rustType = transpiler.describe(&self.resolved.get_type())?;
		if self.java.get_initializer().isPresent() {
			self.rustInitializer = transpiler.describe(&self.java.get_initializer().get());
		}
	
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append(self.visibility);
		sb.append("static ");
		sb.append(self.name);
		sb.append(": ");
		sb.append(self.rust_type);
		if self.rust_initializer != null {
			sb.append(" = ");
			sb.append(self.rust_initializer);
		}
		sb.append(";");
		return sb.toString();
	}
}