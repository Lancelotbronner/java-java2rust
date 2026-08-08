use javaparser_core::com::github::javaparser::ast::expr::Expression;
use javaparser_core::com::github::javaparser::ast::type::Type;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedRecordDeclaration;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util::ArrayList;
use java::util::List;
use java::util::StringJoiner;

pub struct RustRecord {
	decl: com::github::javaparser::resolution::declarations::resolved_record_declaration::ResolvedRecordDeclaration,
	fields: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	impls: java2rust::rust_impls::RustImpls,
	typarams: java2rust::rust_ty_params::RustTyParams = RustTyParams::new(),
}

impl RustRecord {
	fn new(name: &/* Java */ java::lang::String /**/, module: &java2rust::rust_package::RustPackage, decl: &com::github::javaparser::resolution::declarations::resolved_record_declaration::ResolvedRecordDeclaration, visibility: &/* Java */ java2rust::RustVisibility /**/) -> java2rust::rust_record::RustRecord {
		super(name, module, visibility);
		self.decl = decl;
		self.impls = RustImpls::new(self, self.typarams);
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		super.analyze(transpiler);
		for field in self.fields {
			field.analyze(transpiler, self);
		}
		self.typarams.analyze(self.decl, transpiler);
		self.impls.analyze(self.decl, transpiler)?;
	}

	pub fn id(&self) -> /* Java */ java::lang::String /**/ {
		return self.decl.get_id();
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
		sb.append("#[derive(Debug, Hash, Eq, PartialEq)]\n");
		sb.append();
		sb.append("struct ");
		sb.append();
		sb.append(self.typarams);
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
		let impl: StringJoiner = StringJoiner::new("\n\n", &"\n\nimpl%s %s {\n".formatted(self.typarams, ), "}");
		impl.setEmptyValue("");
		for field in  {
			impl.add("\t" + field.to_string().replace("\n", "\n\t"));
		}
		for method in  {
			impl.add(&method.toString());
		}
		sb.append(impl);
		if !self.impls.is_empty() {
			sb.append("\n\n");
			sb.append(self.impls);
		}
		return sb.toString();
	}
}