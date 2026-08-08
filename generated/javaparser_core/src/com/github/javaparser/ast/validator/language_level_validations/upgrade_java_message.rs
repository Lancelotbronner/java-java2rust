use crate::com::github::javaparser::ParserConfiguration;

pub struct UpgradeJavaMessage {
	reason: /* Java */ java::lang::String /**/,
	level: com::github::javaparser::parser_configuration::LanguageLevel,
	upgrade_needed: bool,
}

impl UpgradeJavaMessage {
	fn new(reason: &/* Java */ java::lang::String /**/, level: &com::github::javaparser::parser_configuration::LanguageLevel) -> com::github::javaparser::ast::validator::language_level_validations::upgrade_java_message::UpgradeJavaMessage {
		this(reason, level, true);
	}

	fn new(reason: &/* Java */ java::lang::String /**/, level: &com::github::javaparser::parser_configuration::LanguageLevel, upgrade_needed: bool) -> com::github::javaparser::ast::validator::language_level_validations::upgrade_java_message::UpgradeJavaMessage {
		self.reason = reason;
		self.level = level;
		self.upgradeNeeded = upgrade_needed;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format( if self.upgrade_needed { "%s Pay attention that this feature is supported starting from '%s' language level." } else { "%s Pay attention that this feature is no longer supported since '%s' language level." }, self.reason, &self.level.toString()) + " If you need that feature the language level must be configured in the configuration before parsing the source files.";
	}
}