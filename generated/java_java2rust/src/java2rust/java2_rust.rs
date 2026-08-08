use javaparser_core::com::github::javaparser::ParserConfiguration;
use javaparser_core::com::github::javaparser::StaticJavaParser;
use crate::java2rust::rust::RustJar;
use crate::java2rust::rust::RustUnit;
use commons_lang3::org::apache::commons::lang3::ArrayUtils;
use commons_lang3::org::junit::jupiter::api::Assertions::assertEquals;

pub struct Java2Rust;

impl Java2Rust {
	pub fn assert_conversion(&self, java: &/* Java */ java::lang::String /**/, rust: &/* Java */ java::lang::String /**/) {
		.assertEquals(&rust.trim(), &java2rust::java2_rust::Java2Rust::test(java).trim());
	}

	pub fn test(&self, java: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let transpiler: JavaTranspiler = JavaTranspiler::new();
		let config: ParserConfiguration = ParserConfiguration::new();
		config.set_language_level(ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_25);
		config.setSymbolResolver(transpiler::solver);
		StaticJavaParser::set_configuration(config)?;
		transpiler.add_source_code("test.java", java);
		transpiler.preanalyze();
		transpiler.analyze();
		let sb: StringBuilder = StringBuilder::new();
		let crate: RustJar = transpiler.crates.getFirst();
		for unit in crate.units {
			if crate.units.size() > 1 {
				sb.append("// ");
				sb.append(unit.path);
			}
			sb.append("\n");
			sb.append(unit);
			sb.append("\n");
		}
		return sb.toString().trim();
	}

	pub fn identifier(&self, java: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if Character::isLowerCase(&java.charAt(0)) {
			let sb: StringBuilder = StringBuilder::new();
			for c in java.toCharArray() {
				if Character::isUpperCase(c) {
					sb.append("_").append(&Character::toLowerCase(c));
				} else {
					sb.append(c);
				}
			}
			return sb.toString();
		}
		return java;
	}

	pub fn pascal_case_to_snake_case(&self, java: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		let chars: Vec<Character> = ArrayUtils.toObject(&java.toCharArray());
		let i: i32 = 0;
		for c in chars {
			if Character::isUpperCase(c) {
				if !sb.isEmpty() && !Character::isUpperCase(&ArrayUtils::get(chars, i + 1, '\0')) {
					sb.append('_');
				}
	
				sb.append(&Character::toLowerCase(c));
			} else {
				sb.append(c);
			}
			i += 1;
		}
		return sb.toString();
	}

	pub fn camel_case_to_snake_case(&self, java: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		let chars: Vec<Character> = ArrayUtils.toObject(&java.toCharArray());
		let i: i32 = 0;
		for c in chars {
			if Character::isUpperCase(c) {
				if !sb.isEmpty() && !Character::isUpperCase(&ArrayUtils::get(chars, i + 1, 'A')) {
					sb.append('_');
				}
	
				sb.append(&Character::toLowerCase(c));
			} else {
				sb.append(c);
			}
			i += 1;
		}
		return sb.toString();
	}
}