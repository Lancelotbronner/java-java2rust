use javaparser_core::com::github::javaparser::quality::NotNull;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedClassDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedRecordDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedReferenceType;
use java::util::ArrayList;
use java::util::List;
use java::util::Objects;
use java::util::StringJoiner;

pub struct RustImpls {
	item: java2rust::rust_item::RustItem,
	params: java2rust::rust_ty_params::RustTyParams,
	traits: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	problems: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl RustImpls {
	pub fn new(item: &java2rust::rust_item::RustItem, params: &java2rust::rust_ty_params::RustTyParams) -> java2rust::rust_impls::RustImpls {
		self.item = item;
		self.params = params;
	}

	pub fn is_empty(&self) -> bool {
		return self.traits.isEmpty() && self.problems.isEmpty();
	}

	pub fn analyze(&self, decl: &com::github::javaparser::resolution::declarations::resolved_class_declaration::ResolvedClassDeclaration, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		let r0 = 'try0: {
			self.analyze(&decl.get_all_interfaces(), transpiler);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("In RustImpl analysis: %s\n", &e.getLocalizedMessage());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn analyze(&self, implemented: &/* Java */ java::util::List /**/, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		Objects::requireNonNull(implemented);
		for i in implemented {
			let r0 = 'try0: {
				self.traits.add(&match transpiler.describe(i) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				});
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					System::err.printf("In RustImpl analysis: %s\n", &e.getLocalizedMessage());
					self.problems.add(&"%s".formatted(&e.getMessage()));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	pub fn analyze(&self, decl: &com::github::javaparser::resolution::declarations::resolved_record_declaration::ResolvedRecordDeclaration, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		// Note: As records cannot implement interfaces this is expected to always be null
		if decl.get_all_interfaces() != null {
			self.analyze(&decl.get_all_interfaces(), transpiler)?;
		}
	
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		let traits: StringJoiner = StringJoiner::new("\n\n");
		for trait in self.traits {
			traits.add(&"impl%s %s for %s%s {}".formatted(&self.params.to_bounds(), trait, self.item.name, &self.params.to_impl()));
		}
		sb.append(traits);
		let problems: StringJoiner = StringJoiner::new("\n", "/* ", " */");
		problems.setEmptyValue("");
		for problem in self.problems {
			problems.add(problem);
		}
		sb.append(problems);
		return sb.toString();
	}
}