pub struct Import {
	wildcard_import: bool,
	static_import: bool,
	import_string: /* Java */ java::lang::String /**/,
}

impl Import {
	pub fn new(import_string: &/* Java */ java::lang::String /**/, static_import: bool, wildcard_import: bool) -> java2rust::import::Import {
		self.importString = import_string;
		self.staticImport = static_import;
		self.wildcardImport = wildcard_import;
	}

	pub fn get_import_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.import_string;
	}

	pub fn is_static_import(&self) -> bool {
		return self.static_import;
	}

	pub fn is_wildcard_import(&self) -> bool {
		return self.wildcard_import;
	}
}