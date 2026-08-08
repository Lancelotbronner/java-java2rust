use java::net::URL;
use java::net::URLClassLoader;
use java::util::Arrays;
use java::util::Objects;

pub struct ClassLoaderUtils;

impl ClassLoaderUtils {
	static EMPTY_URL_ARRAY: &[/* Java */ java::net::URL /**/] = ;

	pub fn get_systemur_ls(&self) -> &[/* Java */ java::net::URL /**/] {
		return org::apache::commons::lang3::class_loader_utils::ClassLoaderUtils::getur_ls(&ClassLoader::getSystemClassLoader());
	}

	pub fn get_threadur_ls(&self) -> &[/* Java */ java::net::URL /**/] {
		return org::apache::commons::lang3::class_loader_utils::ClassLoaderUtils::getur_ls(&Thread::currentThread().getContextClassLoader());
	}

	fn getur_ls(&self, cl: &/* Java */ java::lang::ClassLoader /**/) -> &[/* Java */ java::net::URL /**/] {
		return  if cl instanceof URLClassLoader { (cl as URLClassLoader).getURLs() } else { self.EMPTY_URL_ARRAY };
	}

	pub fn to_string(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/) -> /* Java */ java::lang::String /**/ {
		if class_loader instanceof URLClassLoader {
			return org::apache::commons::lang3::class_loader_utils::ClassLoaderUtils::to_string(class_loader as URLClassLoader);
		}
		return Objects::toString(class_loader);
	}

	pub fn to_string(&self, class_loader: &/* Java */ java::net::URLClassLoader /**/) -> /* Java */ java::lang::String /**/ {
		return  if class_loader != null { class_loader + Arrays::toString(&class_loader.getURLs()) } else { "null" };
	}

	pub fn new() -> org::apache::commons::lang3::class_loader_utils::ClassLoaderUtils {
	// empty
	}
}