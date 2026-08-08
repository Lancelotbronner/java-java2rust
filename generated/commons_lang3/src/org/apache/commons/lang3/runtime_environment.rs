use java::io::IOException;
use java::nio::charset::Charset;
use java::nio::file::Files;
use java::nio::file::Paths;
use java::util::Arrays;

pub struct RuntimeEnvironment;

impl RuntimeEnvironment {
	fn file_exists(&self, path: &/* Java */ java::lang::String /**/) -> bool {
		return Files::exists(&Paths::get(path));
	}

	pub fn in_container(&self) -> /* Java */ java::lang::Boolean /**/ {
		return org::apache::commons::lang3::runtime_environment::RuntimeEnvironment::in_container(StringUtils::EMPTY);
	}

	fn in_container(&self, dir_prefix: &/* Java */ java::lang::String /**/) -> bool {
		/* final */ let value: String = org::apache::commons::lang3::runtime_environment::RuntimeEnvironment::read_file(dir_prefix + "/proc/1/environ", "container");
		if value != null {
			return !value.isEmpty();
		}
		return org::apache::commons::lang3::runtime_environment::RuntimeEnvironment::file_exists(dir_prefix + "/.dockerenv") || org::apache::commons::lang3::runtime_environment::RuntimeEnvironment::file_exists(dir_prefix + "/run/.containerenv");
	}

	fn read_file(&self, env_var_file: &/* Java */ java::lang::String /**/, key: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let r0 = 'try0: {
			/* final */ let bytes: Vec<i8> = Files::readAllBytes(&Paths::get(env_var_file));
			/* final */ let content: String = String::new(bytes, &Charset::defaultCharset());
			// Split by null byte character
			/* final */ let lines: Vec<String> = content.split(&String::valueOf(CharUtils::NUL));
			/* final */ let prefix: String = key + "=";
			// @formatter:off
			return Arrays::stream(lines).filter(|line|line.startsWith(prefix)).map(|line|line.split("=", 2)).map(|key_value|key_value[1]).findFirst().orElse(null);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				return null;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn new() -> org::apache::commons::lang3::runtime_environment::RuntimeEnvironment {
	// empty
	}
}