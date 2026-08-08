use crate::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;

pub struct Value {
	type: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
	name: /* Java */ java::lang::String /**/,
}

impl Value {
	pub fn new(type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::model::value::Value {
		self.type = type;
		self.name = name;
	}

	pub fn from(&self, decl: &com::github::javaparser::resolution::declarations::resolved_value_declaration::ResolvedValueDeclaration) -> com::github::javaparser::resolution::model::value::Value {
		let type: ResolvedType = decl.get_type();
		return Value::new(type, &decl.get_name());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Value{" + "type=" + self.type + ", name='" + self.name + '\'' + '}';
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
	}

	pub fn get_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.type;
	}
}