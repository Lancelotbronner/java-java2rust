use java::util::Arrays::asList;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::UnaryExpr;
use java::io::IOException;
use java::io::Reader;
use java::util;
use java::util::function::Function;

pub struct Utils;

impl Utils {
	pub fn is_null_or_empty<E>(&self, collection: &/* Java */ java::util::Collection /**/) -> bool {
		return collection == null || collection.isEmpty();
	}

	pub fn assert_not_null<T>(&self, o: &T) /* thrown(java.lang.AssertionError) */ -> T {
		if o == null {
			return Err(AssertionError::new("A reference was unexpectedly null."));
		}
		return o;
	}

	pub fn assert_non_empty(&self, string: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> /* Java */ java::lang::String /**/ {
		if string == null || string.isEmpty() {
			return Err(AssertionError::new("A string was unexpectedly empty."));
		}
		return string;
	}

	pub fn assert_non_negative<T: /* Java */ java::lang::Number /**/>(&self, number: &T) /* thrown(java.lang.AssertionError) */ -> T {
		if number.longValue() < 0 {
			return Err(AssertionError::new("A number was unexpectedly negative."));
		}
		return number;
	}

	pub fn assert_positive<T: /* Java */ java::lang::Number /**/>(&self, number: &T) /* thrown(java.lang.AssertionError) */ -> T {
		if number.longValue() <= 0 {
			return Err(AssertionError::new("A number was unexpectedly non-positive."));
		}
		return number;
	}

	pub fn escape_end_of_lines(&self, string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let escaped_string: StringBuilder = StringBuilder::new();
		for c in string.toCharArray() {
			match c {
				'\n' =>  {
					escaped_string.append("\\n");
					break;
				}
				'\r' =>  {
					escaped_string.append("\\r");
					break;
				}
				_ => escaped_string.append(c),
			}
		}
		return escaped_string.toString();
	}

	pub fn reader_to_string(&self, reader: &/* Java */ java::io::Reader /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let result: StringBuilder = StringBuilder::new();
		/* final */ let buffer: [Option<char>; 8 * 1024] = [None; 8 * 1024];
		let num_chars: i32;
		while (num_chars = reader.read(buffer, 0, buffer.length)) > 0 {
			result.append(buffer, 0, num_chars);
		}
		return result.toString();
	}

	pub fn to_camel_case(&self, original: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::utils::Utils::screaming_to_camel_case(original);
	}

	pub fn screaming_to_camel_case(&self, original: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		let parts: Vec<String> = original.toLowerCase().split("_");
		 {
			let i: i32 = 0;
			while i < parts.length {
				{
					sb.append( if i == 0 { parts[i] } else { com::github::javaparser::utils::utils::Utils::capitalize(parts[i]) });
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	pub fn camel_case_to_screaming(&self, input: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if input.isEmpty() {
			return "";
		}
		let scream: StringBuilder = StringBuilder::new(&input.substring(0, 1).toUpperCase());
		for c in input.substring(1).toCharArray() {
			if Character::isUpperCase(c) {
				scream.append("_");
			}
			scream.append(&Character::toUpperCase(c));
		}
		return scream.toString();
	}

	pub fn next_word(&self, string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let index: i32 = 0;
		while index < string.length() && !Character::isWhitespace(&string.charAt(index)) {
			index += 1;
		}
		return string.substring(0, index);
	}

	pub fn indent(&self, builder: &/* Java */ java::lang::StringBuilder /**/, indent_level: i32) -> /* Java */ java::lang::StringBuilder /**/ {
		 {
			let i: i32 = 0;
			while i < indent_level {
				{
					builder.append("\t");
				}
				i += 1;
			 }
		 }
	
		return builder;
	}

	pub fn capitalize(&self, s: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::utils::Utils::string_transformer(s, "capitalize", String::toUpperCase)?;
	}

	pub fn decapitalize(&self, s: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::utils::Utils::string_transformer(s, "decapitalize", String::toLowerCase)?;
	}

	fn string_transformer(&self, s: &/* Java */ java::lang::String /**/, operation_description: &/* Java */ java::lang::String /**/, transformation: &/* Java */ java::util::function::Function /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if s.isEmpty() {
			return Err(IllegalArgumentException::new(&String::format("You cannot %s an empty string", operation_description)));
		}
		return transformation.apply(&s.substring(0, 1)) + s.substring(1);
	}

	pub fn value_is_null_or_empty(&self, mut value: &/* Java */ java::lang::Object /**/) -> bool {
		if value == null {
			return true;
		}
		if value instanceof Optional {
			if (value as Optional).isPresent() {
				value = (value as Optional).get();
			} else {
				return true;
			}
		}
		if value instanceof Collection {
			if (value as Collection).isEmpty() {
				return true;
			}
		}
		return false;
	}

	pub fn value_is_null_or_empty_string_or_optional(&self, value: &/* Java */ java::lang::Object /**/) -> bool {
		// is null?
		if value == null {
			return true;
		}
		//        return false;
		return  if value instanceof Optional { !(value as Optional).isPresent() } else { false };
	}

	pub fn replace_element_by_object_identity<E>(&self, list: &/* Java */ java::util::List /**/, old_object: &E, new_object: &E) {
		let index: i32 = com::github::javaparser::utils::utils::Utils::index_of_element_by_object_identity(list, old_object);
		if index == -1 {
			return;
		}
		list.set(index, new_object);
	}

	pub fn remove_element_by_object_identity<E>(&self, list: &/* Java */ java::util::List /**/, o: &E) {
		let index: i32 = com::github::javaparser::utils::utils::Utils::index_of_element_by_object_identity(list, o);
		if index == -1 {
			return;
		}
		list.remove(index);
	}

	pub fn index_of_element_by_object_identity<E>(&self, list: &/* Java */ java::util::List /**/, o: &E) -> i32 {
		 {
			let i: i32 = 0;
			while i < list.size() {
				{
					let list_o: Object = list.get(i);
					if o == list_o {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return -1;
	}

	pub fn set<T>(&self, items: &T) -> /* Java */ java::util::Set /**/ {
		return HashSet<>::new(&/* Java */ java::util::Arrays /**/::asList(items));
	}

	pub fn normalize_eol_in_text_block(&self, content: &/* Java */ java::lang::String /**/, desired_end_of_line_character: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return content.replaceAll("\\R", desired_end_of_line_character);
	}

	pub fn normalize_eol_in_text_block(&self, content: &/* Java */ java::lang::String /**/, desired_end_of_line_character: &com::github::javaparser::utils::line_separator::LineSeparator) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::utils::Utils::normalize_eol_in_text_block(content, &desired_end_of_line_character.as_raw_string());
	}

	pub fn remove_file_extension(&self, filename: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let extension_index: i32 = filename.lastIndexOf(".");
		if extension_index == -1 {
			return filename;
		}
	
		return filename.substring(0, extension_index);
	}

	pub fn trim_trailing_spaces(&self, mut line: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		while line.length() > 0 && line.charAt(line.length() - 1) <= 0x20 {
			line = line.substring(0, line.length() - 1);
		}
		return line;
	}

	pub fn has_unary_minus_as_parent(&self, n: &com::github::javaparser::ast::node::Node) -> bool {
		return n.get_parent_node().filter(|parent|parent instanceof UnaryExpr).map(|parent|parent as UnaryExpr).map(|unary_expr|unary_expr.get_operator() == UnaryExpr::com::github::javaparser::ast::expr::unary_expr::Operator::MINUS).orElse(false);
	}
}