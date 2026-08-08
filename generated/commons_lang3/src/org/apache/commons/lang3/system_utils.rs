use java::io::File;
use java::nio::file::Path;
use java::nio::file::Paths;

pub struct SystemUtils;

impl SystemUtils {
	static OS_NAME_WINDOWS_PREFIX: /* Java */ java::lang::String /**/ = "Windows";

	pub static FILE_ENCODING: /* Java */ java::lang::String /**/ = SystemProperties::get_file_encoding();

	pub static FILE_SEPARATOR: /* Java */ java::lang::String /**/ = SystemProperties::get_file_separator();

	pub static JAVA_AWT_FONTS: /* Java */ java::lang::String /**/ = SystemProperties::get_java_awt_fonts();

	pub static JAVA_AWT_GRAPHICSENV: /* Java */ java::lang::String /**/ = SystemProperties::get_java_awt_graphicsenv();

	pub static JAVA_AWT_HEADLESS: /* Java */ java::lang::String /**/ = SystemProperties::get_java_awt_headless();

	pub static JAVA_AWT_PRINTERJOB: /* Java */ java::lang::String /**/ = SystemProperties::get_java_awt_printerjob();

	pub static JAVA_CLASS_PATH: /* Java */ java::lang::String /**/ = SystemProperties::get_java_class_path();

	pub static JAVA_CLASS_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_class_version();

	pub static JAVA_COMPILER: /* Java */ java::lang::String /**/ = SystemProperties::get_java_compiler();

	pub static JAVA_ENDORSED_DIRS: /* Java */ java::lang::String /**/ = SystemProperties::get_java_endorsed_dirs();

	pub static JAVA_EXT_DIRS: /* Java */ java::lang::String /**/ = SystemProperties::get_java_ext_dirs();

	pub static JAVA_HOME: /* Java */ java::lang::String /**/ = SystemProperties::get_java_home();

	pub static JAVA_IO_TMPDIR: /* Java */ java::lang::String /**/ = SystemProperties::get_java_io_tmpdir();

	pub static JAVA_LIBRARY_PATH: /* Java */ java::lang::String /**/ = SystemProperties::get_java_library_path();

	pub static JAVA_RUNTIME_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_java_runtime_name();

	pub static JAVA_RUNTIME_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_runtime_version();

	pub static JAVA_SPECIFICATION_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_java_specification_name();

	pub static JAVA_SPECIFICATION_VENDOR: /* Java */ java::lang::String /**/ = SystemProperties::get_java_specification_vendor();

	pub static JAVA_SPECIFICATION_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_specification_version();

	static JAVA_SPECIFICATION_VERSION_AS_ENUM: org::apache::commons::lang3::java_version::JavaVersion = JavaVersion::get(JAVA_SPECIFICATION_VERSION);

	pub static JAVA_UTIL_PREFS_PREFERENCES_FACTORY: /* Java */ java::lang::String /**/ = SystemProperties::get_java_util_prefs_preferences_factory();

	pub static JAVA_VENDOR: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vendor();

	pub static JAVA_VENDOR_URL: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vendor_url();

	pub static JAVA_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_version();

	pub static JAVA_VM_INFO: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_info();

	pub static JAVA_VM_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_name();

	pub static JAVA_VM_SPECIFICATION_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_specification_name();

	pub static JAVA_VM_SPECIFICATION_VENDOR: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_specification_vendor();

	pub static JAVA_VM_SPECIFICATION_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_specification_version();

	pub static JAVA_VM_VENDOR: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_vendor();

	pub static JAVA_VM_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_java_vm_version();

	pub static LINE_SEPARATOR: /* Java */ java::lang::String /**/ = SystemProperties::get_line_separator();

	pub static OS_ARCH: /* Java */ java::lang::String /**/ = SystemProperties::get_os_arch();

	pub static OS_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_os_name();

	pub static OS_VERSION: /* Java */ java::lang::String /**/ = SystemProperties::get_os_version();

	pub static PATH_SEPARATOR: /* Java */ java::lang::String /**/ = SystemProperties::get_path_separator();

	pub static USER_COUNTRY: /* Java */ java::lang::String /**/ = SystemProperties::get_property(SystemProperties::USER_COUNTRY, |()|SystemProperties::get_property(SystemProperties::USER_REGION));

	pub static USER_DIR: /* Java */ java::lang::String /**/ = SystemProperties::get_user_dir();

	pub static USER_HOME: /* Java */ java::lang::String /**/ = SystemProperties::get_user_home();

	pub static USER_LANGUAGE: /* Java */ java::lang::String /**/ = SystemProperties::get_user_language();

	pub static USER_NAME: /* Java */ java::lang::String /**/ = SystemProperties::get_user_name();

	pub static USER_TIMEZONE: /* Java */ java::lang::String /**/ = SystemProperties::get_user_timezone();

	pub static IS_JAVA_1_1: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.1");

	pub static IS_JAVA_1_2: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.2");

	pub static IS_JAVA_1_3: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.3");

	pub static IS_JAVA_1_4: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.4");

	pub static IS_JAVA_1_5: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.5");

	pub static IS_JAVA_1_6: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.6");

	pub static IS_JAVA_1_7: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.7");

	pub static IS_JAVA_1_8: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("1.8");

	pub static IS_JAVA_1_9: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("9");

	pub static IS_JAVA_9: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("9");

	pub static IS_JAVA_10: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("10");

	pub static IS_JAVA_11: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("11");

	pub static IS_JAVA_12: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("12");

	pub static IS_JAVA_13: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("13");

	pub static IS_JAVA_14: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("14");

	pub static IS_JAVA_15: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("15");

	pub static IS_JAVA_16: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("16");

	pub static IS_JAVA_17: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("17");

	pub static IS_JAVA_18: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("18");

	pub static IS_JAVA_19: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("19");

	pub static IS_JAVA_20: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("20");

	pub static IS_JAVA_21: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("21");

	pub static IS_JAVA_22: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("22");

	pub static IS_JAVA_23: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("23");

	pub static IS_JAVA_24: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("24");

	pub static IS_JAVA_25: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("25");

	pub static IS_JAVA_26: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_java_version_matches("26");

	pub static IS_OS_AIX: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("AIX");

	pub static IS_OS_ANDROID: bool = Strings::org::apache::commons::lang3::strings::Strings::CS.contains(&SystemProperties::get_java_vendor(), "Android");

	pub static IS_OS_HP_UX: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("HP-UX");

	pub static IS_OS_400: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("OS/400");

	pub static IS_OS_IRIX: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Irix");

	pub static IS_OS_LINUX: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Linux");

	pub static IS_OS_MAC: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Mac");

	pub static IS_OS_MAC_OSX: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Mac OS X");

	pub static IS_OS_MAC_OSX_CHEETAH: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.0");

	pub static IS_OS_MAC_OSX_PUMA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.1");

	pub static IS_OS_MAC_OSX_JAGUAR: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.2");

	pub static IS_OS_MAC_OSX_PANTHER: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.3");

	pub static IS_OS_MAC_OSX_TIGER: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.4");

	pub static IS_OS_MAC_OSX_LEOPARD: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.5");

	pub static IS_OS_MAC_OSX_SNOW_LEOPARD: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.6");

	pub static IS_OS_MAC_OSX_LION: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.7");

	pub static IS_OS_MAC_OSX_MOUNTAIN_LION: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.8");

	pub static IS_OS_MAC_OSX_MAVERICKS: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.9");

	pub static IS_OS_MAC_OSX_YOSEMITE: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.10");

	pub static IS_OS_MAC_OSX_EL_CAPITAN: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.11");

	pub static IS_OS_MAC_OSX_SIERRA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.12");

	pub static IS_OS_MAC_OSX_HIGH_SIERRA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.13");

	pub static IS_OS_MAC_OSX_MOJAVE: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.14");

	pub static IS_OS_MAC_OSX_CATALINA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "10.15");

	pub static IS_OS_MAC_OSX_BIG_SUR: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "11");

	pub static IS_OS_MAC_OSX_MONTEREY: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "12");

	pub static IS_OS_MAC_OSX_VENTURA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "13");

	pub static IS_OS_MAC_OSX_SONOMA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "14");

	pub static IS_OS_MAC_OSX_SEQUOIA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_matches("Mac OS X", "15");

	pub static IS_OS_FREE_BSD: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("FreeBSD");

	pub static IS_OS_OPEN_BSD: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("OpenBSD");

	pub static IS_OS_NET_BSD: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("NetBSD");

	pub static IS_OS_NETWARE: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Netware");

	pub static IS_OS_OS2: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("OS/2");

	pub static IS_OS_SOLARIS: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("Solaris");

	pub static IS_OS_SUN_OS: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("SunOS");

	pub static IS_OS_UNIX: bool = IS_OS_AIX || IS_OS_HP_UX || IS_OS_IRIX || IS_OS_LINUX || IS_OS_MAC_OSX || IS_OS_SOLARIS || IS_OS_SUN_OS || IS_OS_FREE_BSD || IS_OS_OPEN_BSD || IS_OS_NET_BSD;

	pub static IS_OS_WINDOWS: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX);

	pub static IS_OS_WINDOWS_2000: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 2000");

	pub static IS_OS_WINDOWS_2003: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 2003");

	pub static IS_OS_WINDOWS_2008: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " Server 2008");

	pub static IS_OS_WINDOWS_2012: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " Server 2012");

	pub static IS_OS_WINDOWS_95: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 95");

	pub static IS_OS_WINDOWS_98: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 98");

	pub static IS_OS_WINDOWS_ME: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " Me");

	pub static IS_OS_WINDOWS_NT: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " NT");

	pub static IS_OS_WINDOWS_XP: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " XP");

	pub static IS_OS_WINDOWS_VISTA: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " Vista");

	pub static IS_OS_WINDOWS_7: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 7");

	pub static IS_OS_WINDOWS_8: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 8");

	pub static IS_OS_WINDOWS_10: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 10");

	pub static IS_OS_WINDOWS_11: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches(OS_NAME_WINDOWS_PREFIX + " 11");

	pub static IS_OS_ZOS: bool = org::apache::commons::lang3::system_utils::SystemUtils::get_os_name_matches("z/OS");

	pub static USER_HOME_KEY: /* Java */ java::lang::String /**/ = "user.home";

	pub static USER_NAME_KEY: /* Java */ java::lang::String /**/ = "user.name";

	pub static USER_DIR_KEY: /* Java */ java::lang::String /**/ = "user.dir";

	pub static JAVA_IO_TMPDIR_KEY: /* Java */ java::lang::String /**/ = "java.io.tmpdir";

	pub static JAVA_HOME_KEY: /* Java */ java::lang::String /**/ = "java.home";

	pub static AWT_TOOLKIT: /* Java */ java::lang::String /**/ = SystemProperties::get_awt_toolkit();

	pub fn get_environment_variable(&self, name: &/* Java */ java::lang::String /**/, default_value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let r0 = 'try0: {
			/* final */ let value: String = System::getenv(name);
			return  if value == null { default_value } else { value };
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ SecurityException) => {
				// System.err.println("Caught a SecurityException reading the environment variable '" + name + "'.");
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_host_name(&self) -> /* Java */ java::lang::String /**/ {
		return  if self.IS_OS_WINDOWS { System::getenv("COMPUTERNAME") } else { System::getenv("HOSTNAME") };
	}

	pub fn get_java_home(&self) -> /* Java */ java::io::File /**/ {
		return File::new(&SystemProperties::get_java_home());
	}

	pub fn get_java_home_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return Paths::get(&SystemProperties::get_java_home());
	}

	pub fn get_java_io_tmp_dir(&self) -> /* Java */ java::io::File /**/ {
		return File::new(&SystemProperties::get_java_io_tmpdir());
	}

	pub fn get_java_io_tmp_dir_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return Paths::get(&SystemProperties::get_java_io_tmpdir());
	}

	fn get_java_version_matches(&self, version_prefix: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::system_utils::SystemUtils::is_java_version_match(self.JAVA_SPECIFICATION_VERSION, version_prefix);
	}

	fn get_os_matches(&self, os_name_prefix: &/* Java */ java::lang::String /**/, os_version_prefix: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::system_utils::SystemUtils::is_os_match(self.OS_NAME, self.OS_VERSION, os_name_prefix, os_version_prefix);
	}

	fn get_os_name_matches(&self, os_name_prefix: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::system_utils::SystemUtils::is_os_name_match(self.OS_NAME, os_name_prefix);
	}

	pub fn get_user_dir(&self) -> /* Java */ java::io::File /**/ {
		return File::new(&SystemProperties::get_user_dir());
	}

	pub fn get_user_dir_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return Paths::get(&SystemProperties::get_user_dir());
	}

	pub fn get_user_home(&self) -> /* Java */ java::io::File /**/ {
		return File::new(&SystemProperties::get_user_home());
	}

	pub fn get_user_home_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return Paths::get(&SystemProperties::get_user_home());
	}

	pub fn get_user_name(&self) -> /* Java */ java::lang::String /**/ {
		return SystemProperties::get_user_name();
	}

	pub fn get_user_name(&self, default_value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return SystemProperties::get_user_name(default_value);
	}

	pub fn is_java_awt_headless(&self) -> bool {
		return Boolean::TRUE.toString().equals(self.JAVA_AWT_HEADLESS);
	}

	pub fn is_java_version_at_least(&self, required_version: &org::apache::commons::lang3::java_version::JavaVersion) -> bool {
		return self.JAVA_SPECIFICATION_VERSION_AS_ENUM != null && self.JAVA_SPECIFICATION_VERSION_AS_ENUM.at_least(required_version);
	}

	pub fn is_java_version_at_most(&self, required_version: &org::apache::commons::lang3::java_version::JavaVersion) -> bool {
		return self.JAVA_SPECIFICATION_VERSION_AS_ENUM != null && self.JAVA_SPECIFICATION_VERSION_AS_ENUM.at_most(required_version);
	}

	fn is_java_version_match(&self, version: &/* Java */ java::lang::String /**/, version_prefix: &/* Java */ java::lang::String /**/) -> bool {
		if version == null {
			return false;
		}
		return version.startsWith(version_prefix);
	}

	fn is_os_match(&self, os_name: &/* Java */ java::lang::String /**/, os_version: &/* Java */ java::lang::String /**/, os_name_prefix: &/* Java */ java::lang::String /**/, os_version_prefix: &/* Java */ java::lang::String /**/) -> bool {
		if os_name == null || os_version == null {
			return false;
		}
		return org::apache::commons::lang3::system_utils::SystemUtils::is_os_name_match(os_name, os_name_prefix) && org::apache::commons::lang3::system_utils::SystemUtils::is_os_version_match(os_version, os_version_prefix);
	}

	fn is_os_name_match(&self, os_name: &/* Java */ java::lang::String /**/, os_name_prefix: &/* Java */ java::lang::String /**/) -> bool {
		if os_name == null {
			return false;
		}
		return Strings::org::apache::commons::lang3::strings::Strings::CI.starts_with(os_name, os_name_prefix);
	}

	fn is_os_version_match(&self, os_version: &/* Java */ java::lang::String /**/, os_version_prefix: &/* Java */ java::lang::String /**/) -> bool {
		if StringUtils::is_empty(os_version) {
			return false;
		}
		// Compare parts of the version string instead of using String.startsWith(String) because otherwise
		// osVersionPrefix 10.1 would also match osVersion 10.10
		/* final */ let version_prefix_parts: Vec<String> = JavaVersion::split(os_version_prefix);
		/* final */ let version_parts: Vec<String> = JavaVersion::split(os_version);
		 {
			let i: i32 = 0;
			while i < Math::min(version_prefix_parts.length, version_parts.length) {
				{
					if !version_prefix_parts[i].equals(version_parts[i]) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn new() -> org::apache::commons::lang3::system_utils::SystemUtils {
	}
}