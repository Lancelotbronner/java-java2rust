use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedInterfaceDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use java::util::ArrayList;
use java::util::List;
use java::util::StringJoiner;

pub struct RustInterface {
	decl: com::github::javaparser::resolution::declarations::resolved_interface_declaration::ResolvedInterfaceDeclaration,
	fields: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	params: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl RustInterface {
	fn new(name: &/* Java */ java::lang::String /**/, module: &java2rust::rust_package::RustPackage, decl: &com::github::javaparser::resolution::declarations::resolved_interface_declaration::ResolvedInterfaceDeclaration, visibility: &/* Java */ java2rust::RustVisibility /**/) -> java2rust::rust_interface::RustInterface {
		super(name, module, visibility);
		self.decl = decl;
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		super.analyze(transpiler);
		for field in self.fields {
			field.analyze(transpiler, self)?;
		}
		for param in self.decl.get_type_parameters() {
			let r0 = 'try0: {
				if !param.is_bounded() {
					self.params.add(&param.get_name());
					continue;
				}
				let sb: StringBuilder = StringBuilder::new();
				sb.append(&param.get_name());
				sb.append(": ");
				let bounds: StringJoiner = StringJoiner::new("+");
				for bound in param.get_bounds() {
					bounds.add(&match transpiler.describe(&bound.get_type()) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					});
				}
				sb.append(bounds);
				self.params.add(&sb.toString());
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					System::err.println(e);
					self.params.add(&"/* %s */ %s".formatted(&e.getMessage(), &param.get_name()));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	pub fn id(&self) -> /* Java */ java::lang::String /**/ {
		return self.decl.get_id();
	}

	pub fn path(&self) -> /* Java */ java::lang::String /**/ {
		return "%s::%s".formatted(.path, );
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append();
		sb.append("trait ");
		sb.append();
		let params: StringJoiner = StringJoiner::new(", ", "<", ">");
		params.setEmptyValue("");
		for param in self.params {
			params.add(param);
		}
		sb.append(params);
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