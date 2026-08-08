use crate::com::github::javaparser::utils::Utils::capitalize;
use crate::com::github::javaparser::utils::Utils::decapitalize;
use java::io::File;
use java::net::URISyntaxException;
use java::nio::file::Path;
use java::nio::file::Paths;

pub struct CodeGenerationUtils;

impl CodeGenerationUtils {
	fn new() -> com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils {
	}

	pub fn getter_name(&self, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if name.startsWith("is") && bool.class.equals(type) {
			return name;
		}
		if Boolean::TYPE.equals(type) {
			return "is" + com::github::javaparser::utils::utils::Utils::capitalize(name);
		}
		return "get" + com::github::javaparser::utils::utils::Utils::capitalize(name);
	}

	pub fn getter_to_property_name(&self, getter_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if getter_name.startsWith("is") {
			return com::github::javaparser::utils::utils::Utils::decapitalize(&getter_name.substring(&"is".length()));
		}
		if getter_name.startsWith("get") {
			return com::github::javaparser::utils::utils::Utils::decapitalize(&getter_name.substring(&"get".length()));
		}
		if getter_name.startsWith("has") {
			return com::github::javaparser::utils::utils::Utils::decapitalize(&getter_name.substring(&"has".length()));
		}
		return Err(IllegalArgumentException::new("Unexpected getterName '" + getter_name + "'"));
	}

	pub fn setter_name(&self, field_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if field_name.startsWith("is") {
			return "set" + field_name.substring(2);
		}
		return "set" + com::github::javaparser::utils::utils::Utils::capitalize(field_name);
	}

	pub fn optional_of(&self, text: &/* Java */ java::lang::String /**/, is_optional: bool) -> /* Java */ java::lang::String /**/ {
		if is_optional {
			return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("Optional.of(%s)", text);
		}
		return "Optional.empty()";
	}

	pub fn f(&self, format: &/* Java */ java::lang::String /**/, params: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return String::format(format, params);
	}

	pub fn file_in_package_absolute_path(&self, root: &/* Java */ java::lang::String /**/, mut pkg: &/* Java */ java::lang::String /**/, file: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::file::Path /**/ {
		pkg = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_to_path(pkg);
		return Paths::get(root, pkg, file).normalize();
	}

	pub fn file_in_package_absolute_path(&self, root: &/* Java */ java::nio::file::Path /**/, pkg: &/* Java */ java::lang::String /**/, file: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::file::Path /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::file_in_package_absolute_path(&root.toString(), pkg, file);
	}

	pub fn file_in_package_relative_path(&self, mut pkg: &/* Java */ java::lang::String /**/, file: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::file::Path /**/ {
		pkg = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_to_path(pkg);
		return Paths::get(pkg, file).normalize();
	}

	pub fn package_to_path(&self, pkg: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return pkg.replace('.', File::separatorChar);
	}

	pub fn package_absolute_path(&self, root: &/* Java */ java::lang::String /**/, mut pkg: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::file::Path /**/ {
		pkg = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_to_path(pkg);
		return Paths::get(root, pkg).normalize();
	}

	pub fn package_absolute_path(&self, root: &/* Java */ java::nio::file::Path /**/, pkg: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::file::Path /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_absolute_path(&root.toString(), pkg);
	}

	pub fn class_loader_root(&self, c: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.AssertionError) */ -> /* Java */ java::nio::file::Path /**/ {
		let r0 = 'try0: {
			return Paths::get(&c.getProtectionDomain().getCodeSource().getLocation().toURI());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ URISyntaxException) => {
				break 'try0 Err(AssertionError::new("Bug in JavaParser, please report.", e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn maven_module_root(&self, c: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.AssertionError) */ -> /* Java */ java::nio::file::Path /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::class_loader_root(c)?.resolve(&Paths::get("..", "..")).normalize();
	}

	pub fn subtract_paths(&self, mut full: &/* Java */ java::nio::file::Path /**/, mut difference: &/* Java */ java::nio::file::Path /**/) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::nio::file::Path /**/ {
		while difference != null {
			if difference.getFileName().equals(&full.getFileName()) {
				difference = difference.getParent();
				full = full.getParent();
			} else {
				return Err(RuntimeException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("'%s' could not be subtracted from '%s'", difference, full)));
			}
		}
		return full;
	}
}