use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::io;
use java::nio::charset::Charset;
use java::nio::file::Files;
use java::nio::file::Path;

pub struct Providers;

impl Providers {
	pub static UTF8: /* Java */ java::nio::charset::Charset /**/ = Charset::forName("utf-8");

	fn new() -> com::github::javaparser::providers::Providers {
	}

	pub fn provider(&self, reader: &/* Java */ java::io::Reader /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::provider::Provider {
		return StreamProvider::new(&com::github::javaparser::utils::utils::Utils::assert_not_null(reader)?);
	}

	pub fn provider(&self, input: &/* Java */ java::io::InputStream /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		com::github::javaparser::utils::utils::Utils::assert_not_null(input)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(encoding)?;
		let r0 = 'try0: {
			return StreamProvider::new(input, &encoding.name());
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				// and that's a fundamental problem, so runtime exception.
				break 'try0 Err(RuntimeException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn provider(&self, input: &/* Java */ java::io::InputStream /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::provider(input, self.UTF8)?;
	}

	pub fn provider(&self, file: &/* Java */ java::io::File /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.FileNotFoundException | java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::provider(FileInputStream::new(&com::github::javaparser::utils::utils::Utils::assert_not_null(file)?), &com::github::javaparser::utils::utils::Utils::assert_not_null(encoding)?)?;
	}

	pub fn provider(&self, file: &/* Java */ java::io::File /**/) /* thrown(java.io.FileNotFoundException | java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::provider(&com::github::javaparser::utils::utils::Utils::assert_not_null(file)?, self.UTF8)?;
	}

	pub fn provider(&self, path: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.io.IOException | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::provider(&Files::newInputStream(&com::github::javaparser::utils::utils::Utils::assert_not_null(path)?), &com::github::javaparser::utils::utils::Utils::assert_not_null(encoding)?)?;
	}

	pub fn provider(&self, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.lang.AssertionError | java.io.IOException | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::provider(&com::github::javaparser::utils::utils::Utils::assert_not_null(path)?, self.UTF8)?;
	}

	pub fn provider(&self, source: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::provider::Provider {
		return StringProvider::new(&com::github::javaparser::utils::utils::Utils::assert_not_null(source)?);
	}

	pub fn resource_provider(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/, path_to_resource: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.io.IOException | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		let resource_as_stream: InputStream = class_loader.getResourceAsStream(path_to_resource);
		if resource_as_stream == null {
			return Err(IOException::new("Cannot find " + path_to_resource));
		}
		return com::github::javaparser::providers::Providers::provider(resource_as_stream, encoding)?;
	}

	pub fn resource_provider(&self, path_to_resource: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.io.IOException | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		let class_loader: ClassLoader = Provider.class.getClassLoader();
		return com::github::javaparser::providers::Providers::resource_provider(class_loader, path_to_resource, encoding)?;
	}

	pub fn resource_provider(&self, path_to_resource: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | java.io.IOException | java.lang.RuntimeException) */ -> com::github::javaparser::provider::Provider {
		return com::github::javaparser::providers::Providers::resource_provider(path_to_resource, self.UTF8)?;
	}
}