use java::util::ArrayList;
use java::util::Enumeration;
use java::util::HashMap;
use java::util::List;
use java::util::Map;
use java::util::Objects;
use java::util::Properties;
use crate::org::apache::commons::lang3::StringUtils;

pub struct StrSubstitutor {
	escape_char: u16,
	prefix_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher,
	suffix_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher,
	value_delimiter_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher,
	variable_resolver: org::apache::commons::lang3::text::str_lookup::StrLookup,
	enable_substitution_in_variables: bool,
	preserve_escapes: bool,
}

impl StrSubstitutor {
	pub static DEFAULT_ESCAPE: u16 = '$';

	pub static DEFAULT_PREFIX: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::string_matcher("${");

	pub static DEFAULT_SUFFIX: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::string_matcher("}");

	pub static DEFAULT_VALUE_DELIMITER: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::string_matcher(":-");

	pub fn replace<V>(&self, source: &/* Java */ java::lang::Object /**/, value_map: &/* Java */ java::util::Map /**/) -> /* Java */ java::lang::String /**/ {
		return StrSubstitutor::new(value_map).replace(source);
	}

	pub fn replace<V>(&self, source: &/* Java */ java::lang::Object /**/, value_map: &/* Java */ java::util::Map /**/, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return StrSubstitutor::new(value_map, prefix, suffix).replace(source);
	}

	pub fn replace(&self, source: &/* Java */ java::lang::Object /**/, value_properties: &/* Java */ java::util::Properties /**/) -> /* Java */ java::lang::String /**/ {
		if value_properties == null {
			return source.toString();
		}
		/* final */ let value_map: Map<String, String> = HashMap<>::new();
		/* final */ let prop_names: Enumeration<?> = value_properties.propertyNames();
		while prop_names.hasMoreElements() {
			/* final */ let prop_name: String = String::valueOf(&prop_names.nextElement());
			/* final */ let prop_value: String = value_properties.getProperty(prop_name);
			value_map.put(prop_name, prop_value);
		}
		return org::apache::commons::lang3::text::str_substitutor::StrSubstitutor::replace(source, value_map);
	}

	pub fn replace_system_properties(&self, source: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return StrSubstitutor::new(&StrLookup::system_properties_lookup()).replace(source);
	}

	pub fn new() -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(null, self.DEFAULT_PREFIX, self.DEFAULT_SUFFIX, self.DEFAULT_ESCAPE);
	}

	pub fn new<V>(value_map: &/* Java */ java::util::Map /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(&StrLookup::map_lookup(value_map), self.DEFAULT_PREFIX, self.DEFAULT_SUFFIX, self.DEFAULT_ESCAPE);
	}

	pub fn new<V>(value_map: &/* Java */ java::util::Map /**/, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(&StrLookup::map_lookup(value_map), prefix, suffix, self.DEFAULT_ESCAPE);
	}

	pub fn new<V>(value_map: &/* Java */ java::util::Map /**/, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/, escape: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(&StrLookup::map_lookup(value_map), prefix, suffix, escape);
	}

	pub fn new<V>(value_map: &/* Java */ java::util::Map /**/, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/, escape: u16, value_delimiter: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(&StrLookup::map_lookup(value_map), prefix, suffix, escape, value_delimiter);
	}

	pub fn new(variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(variable_resolver, self.DEFAULT_PREFIX, self.DEFAULT_SUFFIX, self.DEFAULT_ESCAPE);
	}

	pub fn new(variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/, escape: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.set_variable_resolver(variable_resolver);
		self.set_variable_prefix(prefix);
		self.set_variable_suffix(suffix);
		self.set_escape_char(escape);
		self.set_value_delimiter_matcher(self.DEFAULT_VALUE_DELIMITER);
	}

	pub fn new(variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup, prefix: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::String /**/, escape: u16, value_delimiter: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.set_variable_resolver(variable_resolver);
		self.set_variable_prefix(prefix);
		self.set_variable_suffix(suffix);
		self.set_escape_char(escape);
		self.set_value_delimiter(value_delimiter);
	}

	pub fn new(variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup, prefix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, suffix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, escape: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		this(variable_resolver, prefix_matcher, suffix_matcher, escape, self.DEFAULT_VALUE_DELIMITER);
	}

	pub fn new(variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup, prefix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, suffix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, escape: u16, value_delimiter_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.set_variable_resolver(variable_resolver);
		self.set_variable_prefix_matcher(prefix_matcher);
		self.set_variable_suffix_matcher(suffix_matcher);
		self.set_escape_char(escape);
		self.set_value_delimiter_matcher(value_delimiter_matcher);
	}

	fn check_cyclic_substitution(&self, var_name: &/* Java */ java::lang::String /**/, prior_variables: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalStateException) */ {
		if !prior_variables.contains(var_name) {
			return;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(256);
		buf.append("Infinite loop in property interpolation of ");
		buf.append(&prior_variables.remove(0));
		buf.append(": ");
		buf.append_with_separators(prior_variables, "->");
		return Err(IllegalStateException::new(&buf.to_string()));
	}

	pub fn get_escape_char(&self) -> u16 {
		return self.escapeChar;
	}

	pub fn get_value_delimiter_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.value_delimiter_matcher;
	}

	pub fn get_variable_prefix_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.prefix_matcher;
	}

	pub fn get_variable_resolver(&self) -> org::apache::commons::lang3::text::str_lookup::StrLookup {
		return self.variableResolver;
	}

	pub fn get_variable_suffix_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.suffix_matcher;
	}

	pub fn is_enable_substitution_in_variables(&self) -> bool {
		return self.enable_substitution_in_variables;
	}

	pub fn is_preserve_escapes(&self) -> bool {
		return self.preserve_escapes;
	}

	pub fn replace(&self, source: &&[u16]) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(source.length).append(source);
		self.substitute(buf, 0, source.length);
		return buf.to_string();
	}

	pub fn replace(&self, source: &&[u16], offset: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		self.substitute(buf, 0, length);
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		return self.replace(source, 0, &source.length());
	}

	pub fn replace(&self, source: &/* Java */ java::lang::CharSequence /**/, offset: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		self.substitute(buf, 0, length);
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new().append(source);
		self.substitute(buf, 0, &buf.length());
		return buf.to_string();
	}

	pub fn replace(&self, source: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(&source.length()).append(source);
		self.substitute(buf, 0, &buf.length());
		return buf.to_string();
	}

	pub fn replace(&self, source: &org::apache::commons::lang3::text::str_builder::StrBuilder, offset: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		self.substitute(buf, 0, length);
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(source);
		if !self.substitute(buf, 0, &source.length()) {
			return source;
		}
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::String /**/, offset: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		if !self.substitute(buf, 0, length) {
			return source.substring(offset, offset + length);
		}
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(&source.length()).append(source);
		self.substitute(buf, 0, &buf.length());
		return buf.to_string();
	}

	pub fn replace(&self, source: &/* Java */ java::lang::StringBuffer /**/, offset: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if source == null {
			return null;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		self.substitute(buf, 0, length);
		return buf.to_string();
	}

	pub fn replace_in(&self, source: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> bool {
		if source == null {
			return false;
		}
		return self.substitute(source, 0, &source.length());
	}

	pub fn replace_in(&self, source: &org::apache::commons::lang3::text::str_builder::StrBuilder, offset: i32, length: i32) -> bool {
		if source == null {
			return false;
		}
		return self.substitute(source, offset, length);
	}

	pub fn replace_in(&self, source: &/* Java */ java::lang::StringBuffer /**/) -> bool {
		if source == null {
			return false;
		}
		return self.replace_in(source, 0, &source.length());
	}

	pub fn replace_in(&self, source: &/* Java */ java::lang::StringBuffer /**/, offset: i32, length: i32) -> bool {
		if source == null {
			return false;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		if !self.substitute(buf, 0, length) {
			return false;
		}
		source.replace(offset, offset + length, &buf.to_string());
		return true;
	}

	pub fn replace_in(&self, source: &/* Java */ java::lang::StringBuilder /**/) -> bool {
		if source == null {
			return false;
		}
		return self.replace_in(source, 0, &source.length());
	}

	pub fn replace_in(&self, source: &/* Java */ java::lang::StringBuilder /**/, offset: i32, length: i32) -> bool {
		if source == null {
			return false;
		}
		/* final */ let buf: StrBuilder = StrBuilder::new(length).append(source, offset, length)?;
		if !self.substitute(buf, 0, length) {
			return false;
		}
		source.replace(offset, offset + length, &buf.to_string());
		return true;
	}

	fn resolve_variable(&self, variable_name: &/* Java */ java::lang::String /**/, buf: &org::apache::commons::lang3::text::str_builder::StrBuilder, start_pos: i32, end_pos: i32) -> /* Java */ java::lang::String /**/ {
		/* final */ let resolver: StrLookup<?> = self.get_variable_resolver();
		if resolver == null {
			return null;
		}
		return resolver.lookup(variable_name);
	}

	pub fn set_enable_substitution_in_variables(&mut self, enable_substitution_in_variables: bool) {
		self.enableSubstitutionInVariables = enable_substitution_in_variables;
	}

	pub fn set_escape_char(&mut self, escape_character: u16) {
		self.escapeChar = escape_character;
	}

	pub fn set_preserve_escapes(&mut self, preserve_escapes: bool) {
		self.preserveEscapes = preserve_escapes;
	}

	pub fn set_value_delimiter(&self, value_delimiter: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		return self.set_value_delimiter_matcher(&StrMatcher::char_matcher(value_delimiter));
	}

	pub fn set_value_delimiter(&self, value_delimiter: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		if StringUtils::is_empty(value_delimiter) {
			self.set_value_delimiter_matcher(null);
			return self;
		}
		return self.set_value_delimiter_matcher(&StrMatcher::string_matcher(value_delimiter));
	}

	pub fn set_value_delimiter_matcher(&mut self, value_delimiter_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.valueDelimiterMatcher = value_delimiter_matcher;
		return self;
	}

	pub fn set_variable_prefix(&self, prefix: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		return self.set_variable_prefix_matcher(&StrMatcher::char_matcher(prefix));
	}

	pub fn set_variable_prefix(&self, prefix: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		return self.set_variable_prefix_matcher(&StrMatcher::string_matcher(&Objects::requireNonNull(prefix)));
	}

	pub fn set_variable_prefix_matcher(&mut self, prefix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.prefixMatcher = Objects::requireNonNull(prefix_matcher, "prefixMatcher");
		return self;
	}

	pub fn set_variable_resolver(&mut self, variable_resolver: &org::apache::commons::lang3::text::str_lookup::StrLookup) {
		self.variableResolver = variable_resolver;
	}

	pub fn set_variable_suffix(&self, suffix: u16) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		return self.set_variable_suffix_matcher(&StrMatcher::char_matcher(suffix));
	}

	pub fn set_variable_suffix(&self, suffix: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		return self.set_variable_suffix_matcher(&StrMatcher::string_matcher(&Objects::requireNonNull(suffix)));
	}

	pub fn set_variable_suffix_matcher(&mut self, suffix_matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_substitutor::StrSubstitutor {
		self.suffixMatcher = Objects::requireNonNull(suffix_matcher);
		return self;
	}

	fn substitute(&self, buf: &org::apache::commons::lang3::text::str_builder::StrBuilder, offset: i32, length: i32) -> bool {
		return self.substitute(buf, offset, length, null) > 0;
	}

	fn substitute(&self, buf: &org::apache::commons::lang3::text::str_builder::StrBuilder, offset: i32, length: i32, mut prior_variables: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalStateException | java.lang.StringIndexOutOfBoundsException) */ -> i32 {
		/* final */ let pfx_matcher: StrMatcher = self.get_variable_prefix_matcher();
		/* final */ let suff_matcher: StrMatcher = self.get_variable_suffix_matcher();
		/* final */ let escape: char = self.get_escape_char();
		/* final */ let value_delim_matcher: StrMatcher = self.get_value_delimiter_matcher();
		/* final */ let substitution_in_variables_enabled: bool = self.is_enable_substitution_in_variables();
		/* final */ let top: bool = prior_variables == null;
		let altered: bool = false;
		let length_change: i32 = 0;
		let chars: Vec<char> = buf.buffer;
		let buf_end: i32 = offset + length;
		let pos: i32 = offset;
		while pos < buf_end {
			/* final */ let start_match_len: i32 = pfx_matcher.is_match(chars, pos, offset, buf_end);
			if start_match_len == 0 {
				pos += 1;
			} else // found variable start marker
			if pos > offset && chars[pos - 1] == escape {
				// escaped
				if self.preserve_escapes {
					pos += 1;
					continue;
				}
				buf.delete_char_at(pos - 1)?;
				// in case buffer was altered
				chars = buf.buffer;
				length_change -= 1;
				altered = true;
				buf_end -= 1;
			} else {
				// find suffix
				/* final */ let start_pos: i32 = pos;
				pos += start_match_len;
				let end_match_len: i32;
				let nested_var_count: i32 = 0;
				while pos < buf_end {
					if substitution_in_variables_enabled && (end_match_len = pfx_matcher.is_match(chars, pos, offset, buf_end)) != 0 {
						// found a nested variable start
						nested_var_count += 1;
						pos += end_match_len;
						continue;
					}
					end_match_len = suff_matcher.is_match(chars, pos, offset, buf_end);
					if end_match_len == 0 {
						pos += 1;
					} else {
						// found variable end marker
						if nested_var_count == 0 {
							let var_name_expr: String = String::new(chars, start_pos + start_match_len, pos - start_pos - start_match_len);
							if substitution_in_variables_enabled {
								/* final */ let buf_name: StrBuilder = StrBuilder::new(var_name_expr);
								self.substitute(buf_name, 0, &buf_name.length());
								var_name_expr = buf_name.to_string();
							}
							pos += end_match_len;
							/* final */ let end_pos: i32 = pos;
							let var_name: String = var_name_expr;
							let var_default_value: String = null;
							if value_delim_matcher != null {
								/* final */ let var_name_expr_chars: Vec<char> = var_name_expr.toCharArray();
								let value_delimiter_match_len: i32;
								 {
									let i: i32 = 0;
									while i < var_name_expr_chars.length {
										{
											// if there's any nested variable when nested variable substitution disabled, then stop resolving name and default value.
											if !substitution_in_variables_enabled && pfx_matcher.is_match(var_name_expr_chars, i, i, var_name_expr_chars.length) != 0 {
												break;
											}
											if (value_delimiter_match_len = value_delim_matcher.is_match(var_name_expr_chars, i)) != 0 {
												var_name = var_name_expr.substring(0, i);
												var_default_value = var_name_expr.substring(i + value_delimiter_match_len);
												break;
											}
										}
										i += 1;
									 }
								 }
	
							}
							// on the first call initialize priorVariables
							if prior_variables == null {
								prior_variables = ArrayList<>::new();
								prior_variables.add(String::new(chars, offset, length));
							}
							// handle cyclic substitution
							self.check_cyclic_substitution(var_name, prior_variables)?;
							prior_variables.add(var_name);
							// resolve the variable
							let var_value: String = self.resolve_variable(var_name, buf, start_pos, end_pos);
							if var_value == null {
								var_value = var_default_value;
							}
							if var_value != null {
								// recursive replace
								/* final */ let var_len: i32 = var_value.length();
								buf.replace(start_pos, end_pos, var_value)?;
								altered = true;
								let change: i32 = self.substitute(buf, start_pos, var_len, prior_variables)?;
								change = change + var_len - (end_pos - start_pos);
								pos += change;
								buf_end += change;
								length_change += change;
								// in case buffer was altered
								chars = buf.buffer;
							}
							// remove variable from the cyclic stack
							prior_variables.remove(prior_variables.size() - 1);
							break;
						}
						nested_var_count -= 1;
						pos += end_match_len;
					}
				}
			}
		}
		if top {
			return  if altered { 1 } else { 0 };
		}
		return length_change;
	}
}