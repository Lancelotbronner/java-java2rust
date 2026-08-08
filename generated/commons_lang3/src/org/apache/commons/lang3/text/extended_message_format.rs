use java::text::Format;
use java::text::MessageFormat;
use java::text::ParsePosition;
use java::util::ArrayList;
use java::util::Collection;
use java::util::Locale;
use java::util::Map;
use java::util::Objects;
use crate::org::apache::commons::lang3::LocaleUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Validate;

pub struct ExtendedMessageFormat {
	to_pattern: /* Java */ java::lang::String /**/,
	registry: /* Java */ java::util::Map /**/,
}

impl ExtendedMessageFormat {
	static serialVersionUID: i64 = -2362048321261811743;

	static EMPTY_PATTERN: /* Java */ java::lang::String /**/ = StringUtils::EMPTY;

	static START_FMT: u16 = ',';

	static END_FE: u16 = '}';

	static START_FE: u16 = '{';

	static QUOTE: u16 = '\'';

	pub fn new(pattern: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::extended_message_format::ExtendedMessageFormat {
		this(pattern, &Locale::getDefault());
	}

	pub fn new(pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::text::extended_message_format::ExtendedMessageFormat {
		this(pattern, locale, null);
	}

	pub fn new(pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/, registry: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::text::extended_message_format::ExtendedMessageFormat {
		super(self.EMPTY_PATTERN);
		self.setLocale(&LocaleUtils::to_locale(locale));
		self.registry = registry;
		self.apply_pattern(pattern)?;
	}

	pub fn new(pattern: &/* Java */ java::lang::String /**/, registry: &/* Java */ java::util::Map /**/) -> org::apache::commons::lang3::text::extended_message_format::ExtendedMessageFormat {
		this(pattern, &Locale::getDefault(), registry);
	}

	fn append_quoted_string(&self, pattern: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, append_to: &/* Java */ java::lang::StringBuilder /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::StringBuilder /**/ {
		assert!( pattern.toCharArray()[pos.getIndex()] == self.QUOTE : "Quoted string must start with quote character");
		// handle quote character at the beginning of the string
		if append_to != null {
			append_to.append(self.QUOTE);
		}
		self.next(pos);
		/* final */ let start: i32 = pos.getIndex();
		/* final */ let c: Vec<char> = pattern.toCharArray();
		 {
			let i: i32 = pos.getIndex();
			while i < pattern.length() {
				{
					if c[pos.getIndex()] == self.QUOTE {
						self.next(pos);
						return  if append_to == null { null } else { append_to.append(c, start, pos.getIndex() - start) };
					}
					self.next(pos);
				}
				i += 1;
			 }
		 }
	
		return Err(IllegalArgumentException::new("Unterminated quoted string at position " + start));
	}

	pub fn apply_pattern(&mut self, pattern: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if self.registry == null {
			super.applyPattern(pattern);
			self.to_pattern = super.toPattern();
			return;
		}
		/* final */ let found_formats: ArrayList<Format> = ArrayList<>::new();
		/* final */ let found_descriptions: ArrayList<String> = ArrayList<>::new();
		/* final */ let strip_custom: StringBuilder = StringBuilder::new(&pattern.length());
		/* final */ let pos: ParsePosition = ParsePosition::new(0);
		/* final */ let c: Vec<char> = pattern.toCharArray();
		let fmt_count: i32 = 0;
		while pos.getIndex() < pattern.length() {
			match c[pos.getIndex()] {
				self.QUOTE =>  {
					self.append_quoted_string(pattern, pos, strip_custom)?;
					break;
				}
				self.START_FE =>  {
					fmt_count += 1;
					self.seek_non_ws(pattern, pos);
					/* final */ let start: i32 = pos.getIndex();
					/* final */ let index: i32 = self.read_argument_index(pattern, &self.next(pos))?;
					strip_custom.append(self.START_FE).append(index);
					self.seek_non_ws(pattern, pos);
					let format: Format = null;
					let format_description: String = null;
					if c[pos.getIndex()] == self.START_FMT {
						format_description = self.parse_format_description(pattern, &self.next(pos))?;
						format = self.get_format(format_description);
						if format == null {
							strip_custom.append(self.START_FMT).append(format_description);
						}
					}
					found_formats.add(format);
					found_descriptions.add( if format == null { null } else { format_description });
					Validate::is_true(found_formats.size() == fmt_count)?;
					Validate::is_true(found_descriptions.size() == fmt_count)?;
					if c[pos.getIndex()] != self.END_FE {
						return Err(IllegalArgumentException::new("Unreadable format element at position " + start));
					}
				}
				// falls-through
				_ =>  {
					strip_custom.append(c[pos.getIndex()]);
					self.next(pos);
				}
			}
		}
		super.applyPattern(&strip_custom.toString());
		self.to_pattern = self.insert_formats(&super.toPattern(), found_descriptions);
		if self.contains_elements(found_formats) {
			/* final */ let orig_formats: Vec<Format> = self.getFormats();
			// only loop over what we know we have, as MessageFormat on Java 1.3
			// seems to provide an extra format element:
			let i: i32 = 0;
			for /* final */ f in found_formats {
				if f != null {
					orig_formats[i] = f;
				}
				i += 1;
			}
			super.setFormats(orig_formats);
		}
	}

	fn contains_elements(&self, coll: &/* Java */ java::util::Collection /**/) -> bool {
		if coll == null || coll.isEmpty() {
			return false;
		}
		return coll.stream().anyMatch(Objects::nonNull);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if !super.equals(obj) {
			return false;
		}
		if !(obj instanceof ExtendedMessageFormat) {
			return false;
		}
		/* final */ let other: ExtendedMessageFormat = obj as ExtendedMessageFormat;
		return Objects::equals(self.registry, other.registry) && Objects::equals(self.to_pattern, other.toPattern);
	}

	fn get_format(&self, desc: &/* Java */ java::lang::String /**/) -> /* Java */ java::text::Format /**/ {
		if self.registry != null {
			let name: String = desc;
			let args: String = null;
			/* final */ let i: i32 = desc.indexOf(self.START_FMT);
			if i > 0 {
				name = desc.substring(0, i).trim();
				args = desc.substring(i + 1).trim();
			}
			/* final */ let factory: FormatFactory = self.registry.get(name);
			if factory != null {
				return factory.get_format(name, args, &self.getLocale());
			}
		}
		return null;
	}

	fn get_quoted_string(&self, pattern: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		self.append_quoted_string(pattern, pos, null)?;
	}

	pub fn hash_code(&self) -> i32 {
		/* final */ let prime: i32 = 31;
		/* final */ let result: i32 = super.hashCode();
		return prime * result + Objects::hash(self.registry, self.to_pattern);
	}

	fn insert_formats(&self, pattern: &/* Java */ java::lang::String /**/, custom_patterns: &/* Java */ java::util::ArrayList /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if !self.contains_elements(custom_patterns) {
			return pattern;
		}
		/* final */ let sb: StringBuilder = StringBuilder::new(pattern.length() * 2);
		/* final */ let pos: ParsePosition = ParsePosition::new(0);
		let fe: i32 = -1;
		let depth: i32 = 0;
		while pos.getIndex() < pattern.length() {
			/* final */ let c: char = pattern.charAt(&pos.getIndex());
			match c {
				self.QUOTE =>  {
					self.append_quoted_string(pattern, pos, sb)?;
					break;
				}
				self.START_FE =>  {
					depth += 1;
					sb.append(self.START_FE).append(&self.read_argument_index(pattern, &self.next(pos))?);
					// do not look for custom patterns when they are embedded, e.g. in a choice
					if depth == 1 {
						fe += 1;
						/* final */ let custom_pattern: String = custom_patterns.get(fe);
						if custom_pattern != null {
							sb.append(self.START_FMT).append(custom_pattern);
						}
					}
					break;
				}
				self.END_FE => depth -= 1,
				// falls-through
				_ =>  {
					sb.append(c);
					self.next(pos);
				}
			}
		}
		return sb.toString();
	}

	fn next(&self, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::text::ParsePosition /**/ {
		pos.setIndex(pos.getIndex() + 1);
		return pos;
	}

	fn parse_format_description(&self, pattern: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let start: i32 = pos.getIndex();
		self.seek_non_ws(pattern, pos);
		/* final */ let text: i32 = pos.getIndex();
		let depth: i32 = 1;
		while pos.getIndex() < pattern.length() {
			{
				match pattern.charAt(&pos.getIndex()) {
					self.START_FE =>  {
						depth += 1;
						break;
					}
					self.END_FE =>  {
						depth -= 1;
						if depth == 0 {
							return pattern.substring(text, &pos.getIndex());
						}
						break;
					}
					self.QUOTE =>  {
						self.get_quoted_string(pattern, pos)?;
						break;
					}
					_ =>  {
						break;
					}
				}
			}
			self.next(pos);
		 }
	
		return Err(IllegalArgumentException::new("Unterminated format element at position " + start));
	}

	fn read_argument_index(&self, pattern: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		/* final */ let start: i32 = pos.getIndex();
		self.seek_non_ws(pattern, pos);
		/* final */ let result: StringBuilder = StringBuilder::new();
		let error: bool = false;
		while !error && pos.getIndex() < pattern.length() {
			{
				let c: char = pattern.charAt(&pos.getIndex());
				if Character::isWhitespace(c) {
					self.seek_non_ws(pattern, pos);
					c = pattern.charAt(&pos.getIndex());
					if c != self.START_FMT && c != self.END_FE {
						error = true;
						continue;
					}
				}
				if (c == self.START_FMT || c == self.END_FE) && result.length() > 0 {
					let r0 = 'try0: {
						return Integer::parseInt(&result.toString());
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ NumberFormatException) => {
						// we've already ensured only digits, so unless something
						// outlandishly large was specified we should be okay.
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				error = !Character::isDigit(c);
				result.append(c);
			}
			self.next(pos);
		 }
	
		if error {
			break 'try0 Err(IllegalArgumentException::new("Invalid format argument index at position " + start + ": " + pattern.substring(start, &pos.getIndex())));
		}
		return Err(IllegalArgumentException::new("Unterminated format element at position " + start));
	}

	fn seek_non_ws(&self, pattern: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) {
		let len: i32;
		/* final */ let buffer: Vec<char> = pattern.toCharArray();
		loop { {
			len = StrMatcher::split_matcher().is_match(buffer, &pos.getIndex());
			pos.setIndex(pos.getIndex() + len);
		}if !(len > 0 && pos.getIndex() < pattern.length()) break;}
	}

	pub fn set_format(&self, format_element_index: i32, new_format: &/* Java */ java::text::Format /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn set_format_by_argument_index(&self, argument_index: i32, new_format: &/* Java */ java::text::Format /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn set_formats(&self, new_formats: &&[/* Java */ java::text::Format /**/]) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn set_formats_by_argument_index(&self, new_formats: &&[/* Java */ java::text::Format /**/]) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn to_pattern(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_pattern;
	}
}

impl /* Java */ java::io::Serializable /**/ for ExtendedMessageFormat {}

impl /* Java */ java::lang::Cloneable /**/ for ExtendedMessageFormat {}