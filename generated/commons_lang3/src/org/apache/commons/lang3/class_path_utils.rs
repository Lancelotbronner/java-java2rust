use java::util::Objects;

pub struct ClassPathUtils;

impl ClassPathUtils {
	pub fn package_to_path(&self, path: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::requireNonNull(path, "path").replace('.', '/');
	}

	pub fn path_to_package(&self, path: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::requireNonNull(path, "path").replace('/', '.');
	}

	pub fn to_fully_qualified_name(&self, context: &/* Java */ java::lang::Class /**/, resource_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(context, "context");
		Objects::requireNonNull(resource_name, "resourceName");
		return org::apache::commons::lang3::class_path_utils::ClassPathUtils::to_fully_qualified_name(&context.getPackage(), resource_name);
	}

	pub fn to_fully_qualified_name(&self, context: &/* Java */ java::lang::Package /**/, resource_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(context, "context");
		Objects::requireNonNull(resource_name, "resourceName");
		return context.getName() + "." + resource_name;
	}

	pub fn to_fully_qualified_path(&self, context: &/* Java */ java::lang::Class /**/, resource_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(context, "context");
		Objects::requireNonNull(resource_name, "resourceName");
		return org::apache::commons::lang3::class_path_utils::ClassPathUtils::to_fully_qualified_path(&context.getPackage(), resource_name);
	}

	pub fn to_fully_qualified_path(&self, context: &/* Java */ java::lang::Package /**/, resource_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(context, "context");
		Objects::requireNonNull(resource_name, "resourceName");
		return org::apache::commons::lang3::class_path_utils::ClassPathUtils::package_to_path(&context.getName()) + "/" + resource_name;
	}

	pub fn new() -> org::apache::commons::lang3::class_path_utils::ClassPathUtils {
	// empty
	}
}