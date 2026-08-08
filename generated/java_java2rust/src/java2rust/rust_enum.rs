use javaparser_core::com::github::javaparser::ast::body::EnumDeclaration;
use javaparser_core::com::github::javaparser::ast::expr::Expression;
use javaparser_core::com::github::javaparser::ast::type::Type;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedEnumDeclaration;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util::ArrayList;
use java::util::List;

pub struct RustEnum {
	java: com::github::javaparser::ast::body::enum_declaration::EnumDeclaration,
	resolved: com::github::javaparser::resolution::declarations::resolved_enum_declaration::ResolvedEnumDeclaration,
	fields: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl RustEnum {
	fn new(java: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration, module: &java2rust::rust_package::RustPackage) -> java2rust::rust_enum::RustEnum {
		super(&java.get_name_as_string(), module, &RustVisibility::pub(&java.is_public()));
		self.java = java;
		self.resolved = java.resolve();
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		super.analyze(transpiler);
		for field in self.fields {
			field.analyze(transpiler, self)?;
		}
	}

	pub fn id(&self) -> /* Java */ java::lang::String /**/ {
		return self.resolved.get_id();
	}

	pub fn path(&self) -> /* Java */ java::lang::String /**/ {
		return "%s::%s".formatted(.path, );
	}

	pub fn field(&self, name: &/* Java */ java::lang::String /**/, type: &com::github::javaparser::ast::type::type::Type, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> java2rust::rust_field::RustField {
		let field: RustField = RustField::new(name, type, initializer);
		self.fields.add(field);
		return field;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append();
		sb.append("enum ");
		sb.append();
		if self.fields.isEmpty() {
			sb.append(';');
			return sb.toString();
		}
		sb.append(" {\n");
		for field in self.fields {
			sb.append('\t');
			sb.append(field);
			sb.append(",\n");
		}
		sb.append('}');
		return sb.toString();
	}
}