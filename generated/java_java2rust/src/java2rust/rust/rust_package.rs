use javaparser_core::com::github::javaparser::ast::ImportDeclaration;
use javaparser_core::com::github::javaparser::ast::body::EnumDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedClassDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedInterfaceDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedRecordDeclaration;
use crate::java2rust::Java2Rust;
use crate::java2rust::JavaTranspiler;
use commons_lang3::org::apache::commons::lang3::StringUtils;
use commons_lang3::org::jspecify::annotations::NonNull;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::io::IOException;
use java::nio::file::Files;
use java::nio::file::Path;
use java::util;
use java::util::stream::Stream;

pub struct RustPackage {
	id: /* Java */ java::lang::String /**/,
	path: /* Java */ java::lang::String /**/,
	crate: java2rust::rust::rust_jar::RustJar,
	imports: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	subpackages: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	items: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl RustPackage {
	fn new(name: &/* Java */ java::lang::String /**/, module: &java2rust::rust::rust_package::RustPackage, visibility: &/* Java */ java2rust::rust::RustVisibility /**/, crate: &java2rust::rust::rust_jar::RustJar) -> java2rust::rust::rust_package::RustPackage {
		super(name, module, visibility);
		self.crate = crate;
		let tmp: List<String> = self.ancestors_without_top().map(RustPackage::use).toList().reversed();
		self.id = StringUtils::join(tmp, ".");
		self.path = StringUtils::join(tmp, "::");
	}

	pub fn is_empty(&self) -> bool {
		return self.items.isEmpty() && self.subpackages.isEmpty();
	}

	pub fn ancestors(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return Stream::iterate(self, Objects::nonNull, |m|m::module);
	}

	fn module_if_not_top(&self) -> java2rust::rust::rust_package::RustPackage {
		if  == null {
			return null;
		}
	
		return  if .module == null { null } else {  };
	}

	pub fn ancestors_without_top(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return Stream::iterate(self, Objects::nonNull, RustPackage::moduleIfNotTop);
	}

	pub fn use(&self) -> /* Java */ java::lang::String /**/ {
		if  == null {
			return "crate";
		}
		else {return ;
		}
	
	}

	pub fn locate(&self, path: &/* Java */ java::lang::String /**/) -> java2rust::rust::rust_package::RustPackage {
		if path.equals(self.id) {
			return self;
		}
	
		for sub in self.subpackages {
			let pkg: RustPackage = sub.locate(path);
			if pkg != null {
				return pkg;
			}
	
		}
		return null;
	}

	pub fn delete(&self) {
		.subpackages.remove(self);
	}

	pub fn parent(&self) -> java2rust::rust::rust_package::RustPackage {
		return ;
	}

	pub fn submodules(&self) -> /* Java */ java::util::List /**/ {
		return self.subpackages;
	}

	pub fn items(&self) -> /* Java */ java::util::List /**/ {
		return self.items;
	}

	pub fn qualified_name(&self) -> /* Java */ java::lang::String /**/ {
		return StringUtils::join(&self.ancestors().skip(1).map(RustPackage::use).toList().reversed(), ".");
	}

	pub fn submodule(&self, name: &/* Java */ java::lang::String /**/, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_package::RustPackage {
		let existing: Optional<RustPackage> = self.subpackages.stream().filter(|m|Objects::equals(m.name, name)).findFirst();
		if existing.isPresent() {
			return existing.get();
		}
	
		let mod: RustPackage = RustPackage::new(&Java2Rust::camel_case_to_snake_case(name), self, visibility, self.crate);
		self.subpackages.add(mod);
		return mod;
	}

	pub fn mod(&self, name: &/* Java */ java::lang::String /**/, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_package::RustPackage {
		let mod: RustPackage = RustPackage::new(&Java2Rust::camel_case_to_snake_case(name), self, visibility, self.crate);
		self.items.add(mod);
		return mod;
	}

	pub fn clazz(&self, name: &/* Java */ java::lang::String /**/, decl: &com::github::javaparser::resolution::declarations::resolved_class_declaration::ResolvedClassDeclaration, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_class::RustClass {
		//TODO: Ensure struct name is PascalCase
		let item: RustClass = RustClass::new(name, self, decl, visibility);
		self.items.add(item);
		return item;
	}

	pub fn record(&self, name: &/* Java */ java::lang::String /**/, decl: &com::github::javaparser::resolution::declarations::resolved_record_declaration::ResolvedRecordDeclaration, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_record::RustRecord {
		//TODO: Ensure struct name is PascalCase
		let item: RustRecord = RustRecord::new(name, self, decl, visibility);
		self.items.add(item);
		return item;
	}

	pub fn trait(&self, name: &/* Java */ java::lang::String /**/, decl: &com::github::javaparser::resolution::declarations::resolved_interface_declaration::ResolvedInterfaceDeclaration, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_interface::RustInterface {
		//TODO: Ensure trait name is PascalCase
		let item: RustInterface = RustInterface::new(name, self, decl, visibility);
		self.items.add(item);
		return item;
	}

	pub fn enumeration(&self, java: &com::github::javaparser::ast::body::enum_declaration::EnumDeclaration) -> java2rust::rust::rust_enum::RustEnum {
		//TODO: Ensure trait name is PascalCase
		let item: RustEnum = RustEnum::new(java, self);
		self.items.add(item);
		return item;
	}

	pub fn use(&self, java: &com::github::javaparser::ast::import_declaration::ImportDeclaration) -> java2rust::rust::rust_import::RustImport {
		let item: RustImport = RustImport::new(java, self);
		self.imports.add(item);
		return item;
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		super.analyze(transpiler);
		for item in self.items {
			item.analyze(transpiler);
		}
		for imp in self.imports {
			imp.analyze(transpiler);
		}
		for mod in self.subpackages {
			mod.analyze(transpiler);
		}
	}

	pub fn id(&self) -> /* Java */ java::lang::String /**/ {
		return self.path;
	}

	pub fn path(&self) -> /* Java */ java::lang::String /**/ {
		return self.path;
	}

	pub fn generate(&self, parent: &/* Java */ java::nio::file::Path /**/) {
		let dir: Path;
		let file: Path;
		if  == null {
			dir = parent;
			file = parent.resolve("lib.rs");
		} else if self.subpackages.isEmpty() {
			dir = parent;
			file = parent.resolve( + ".rs");
		} else {
			dir = parent.resolve();
			file = dir.resolve("mod.rs");
		}
		let r0 = 'try0: {
			Files::createDirectories(dir);
			Files::writeString(file, &self.to_string());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				System::err.println(&e.getLocalizedMessage());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		for mod in self.subpackages {
			mod.generate(dir);
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if !self.subpackages.isEmpty() {
			for submodule in self.subpackages {
				sb.append(&"%smod %s;\n".formatted(submodule.visibility, submodule.name));
			}
		}
		if !self.imports.isEmpty() {
			for imp in self.imports {
				sb.append(&"%s\n".formatted(&imp.to_string(self.crate)));
			}
			sb.append("\n");
		}
		if !self.items.isEmpty() {
			let items: StringJoiner = StringJoiner::new("\n\n");
			for item in self.items {
				items.add(&item.toString());
			}
			sb.append(items);
		}
		return sb.toString();
	}
}