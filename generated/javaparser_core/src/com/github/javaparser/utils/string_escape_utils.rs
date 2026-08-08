use java::io::IOException;
use java::io::StringWriter;
use java::io::Writer;
use java::util::HashMap;
use java::util::HashSet;

pub struct StringEscapeUtils;

impl StringEscapeUtils {
	static JAVA_CTRL_CHARS_UNESCAPE: com::github::javaparser::utils::string_escape_utils::LookupTranslator = LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]);

	static JAVA_CTRL_CHARS_ESCAPE: com::github::javaparser::utils::string_escape_utils::LookupTranslator = LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]);

	static ESCAPE_JAVA: com::github::javaparser::utils::string_escape_utils::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]), JAVA_CTRL_CHARS_ESCAPE);

	static UNESCAPE_JAVA: com::github::javaparser::utils::string_escape_utils::CharSequenceTranslator = AggregateTranslator::new(OctalUnescaper::new(), UnicodeUnescaper::new(), JAVA_CTRL_CHARS_UNESCAPE, LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]));

	static UNESCAPE_JAVA_TEXT_BLOCK: com::github::javaparser::utils::string_escape_utils::CharSequenceTranslator = AggregateTranslator::new(OctalUnescaper::new(), UnicodeUnescaper::new(), JAVA_CTRL_CHARS_UNESCAPE, LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]));

	fn new() -> com::github::javaparser::utils::string_escape_utils::StringEscapeUtils {
	}

	pub fn escape_java(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_JAVA.translate(input)?;
	}

	pub fn unescape_java(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_JAVA.translate(input)?;
	}

	pub fn unescape_java_text_block(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_JAVA_TEXT_BLOCK.translate(input)?;
	}
}

struct CharSequenceTranslator;

impl CharSequenceTranslator {
	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 ;

	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if input == null {
			return null;
		}
		let r0 = 'try0: {
			/* final */ let writer: StringWriter = StringWriter::new(input.length() * 2);
			if let Err(e) = self.translate(input, writer) {
				return Err(e);
			};
			return writer.toString();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				// this should never ever happen while writing to a StringWriter
				break 'try0 Err(RuntimeException::new(ioe));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException) */ {
		if out == null {
			return Err(IllegalArgumentException::new("The Writer must not be null"));
		}
		if input == null {
			return;
		}
		let pos: i32 = 0;
		/* final */ let len: i32 = input.length();
		while pos < len {
			/* final */ let consumed: i32 = self.translate(input, pos, out)?;
			if consumed == 0 {
				// inlined implementation of Character.toChars(Character.codePointAt(input, pos))
				// avoids allocating temp char arrays and duplicate checks
				let c1: char = input.charAt(pos);
				out.write(c1);
				pos += 1;
				if Character::isHighSurrogate(c1) && pos < len {
					let c2: char = input.charAt(pos);
					if Character::isLowSurrogate(c2) {
						out.write(c2);
						pos += 1;
					}
				}
				continue;
			}
			// and they just took care of a surrogate pair
			 {
				let pt: i32 = 0;
				while pt < consumed {
					{
						pos += Character::charCount(&Character::codePointAt(input, pos));
					}
					pt += 1;
				 }
			 }
	
		}
	}
}

struct LookupTranslator {
	lookup_map: /* Java */ java::util::HashMap /**/,
	prefix_set: /* Java */ java::util::HashSet /**/,
	shortest: i32,
	longest: i32,
}

impl LookupTranslator {
	fn new(lookup: &&[/* Java */ java::lang::CharSequence /**/]) -> com::github::javaparser::utils::string_escape_utils::LookupTranslator {
		self.lookup_map = HashMap<>::new();
		self.prefix_set = HashSet<>::new();
		let _shortest: i32 = Integer::MAX_VALUE;
		let _longest: i32 = 0;
		if lookup != null {
			for /* final */ seq in lookup {
				self.lookupMap.put(&seq[0].toString(), &seq[1].toString());
				self.prefixSet.add(&seq[0].charAt(0));
				/* final */ let sz: i32 = seq[0].length();
				if sz < _shortest {
					_shortest = sz;
				}
				if sz > _longest {
					_longest = sz;
				}
			}
		}
		self.shortest = _shortest;
		self.longest = _longest;
	}

	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		// check if translation exists for the input at position index
		if self.prefix_set.contains(&input.charAt(index)) {
			let max: i32 = self.longest;
			if index + self.longest > input.length() {
				max = input.length() - index;
			}
			// implement greedy algorithm by trying maximum match first
			 {
				let i: i32 = max;
				while i >= self.shortest {
					{
						/* final */ let sub_seq: CharSequence = input.subSequence(index, index + i);
						/* final */ let result: String = self.lookup_map.get(&sub_seq.toString());
						if result != null {
							out.write(result);
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
		}
		return 0;
	}
}

struct AggregateTranslator {
	translators: &[com::github::javaparser::utils::string_escape_utils::CharSequenceTranslator],
}

impl AggregateTranslator {
	fn new(translators: &com::github::javaparser::utils::string_escape_utils::CharSequenceTranslator) -> com::github::javaparser::utils::string_escape_utils::AggregateTranslator {
		self.translators =  if translators == null { null } else { translators.clone() };
	}

	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		for /* final */ translator in self.translators {
			/* final */ let consumed: i32 = translator.translate(input, index, out)?;
			if consumed != 0 {
				return consumed;
			}
		}
		return 0;
	}
}

struct OctalUnescaper;

impl OctalUnescaper {
	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		// how many characters left, ignoring the first \
		/* final */ let remaining: i32 = input.length() - index - 1;
		/* final */ let builder: StringBuilder = StringBuilder::new();
		if input.charAt(index) == '\\' && remaining > 0 && self.is_octal_digit(&input.charAt(index + 1)) {
			/* final */ let next: i32 = index + 1;
			/* final */ let next2: i32 = index + 2;
			/* final */ let next3: i32 = index + 3;
			// we know this is good as we checked it in the if block above
			builder.append(&input.charAt(next));
			if remaining > 1 && self.is_octal_digit(&input.charAt(next2)) {
				builder.append(&input.charAt(next2));
				if remaining > 2 && self.is_zero_to_three(&input.charAt(next)) && self.is_octal_digit(&input.charAt(next3)) {
					builder.append(&input.charAt(next3));
				}
			}
			out.write(&Integer::parseInt(&builder.toString(), 8));
			return 1 + builder.length();
		}
		return 0;
	}

	fn is_octal_digit(&self, ch: u16) -> bool {
		return ch >= '0' && ch <= '7';
	}

	fn is_zero_to_three(&self, ch: u16) -> bool {
		return ch >= '0' && ch <= '3';
	}
}

struct UnicodeUnescaper;

impl UnicodeUnescaper {
	fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException) */ -> i32 {
		if input.charAt(index) == '\\' && index + 1 < input.length() && input.charAt(index + 1) == 'u' {
			// consume optional additional 'u' chars
			let i: i32 = 2;
			while index + i < input.length() && input.charAt(index + i) == 'u' {
				i += 1;
			}
			if index + i < input.length() && input.charAt(index + i) == '+' {
				i += 1;
			}
			if index + i + 4 <= input.length() {
				// Get 4 hex digits
				/* final */ let unicode: CharSequence = input.subSequence(index + i, index + i + 4);
				let r0 = 'try0: {
					/* final */ let value: i32 = Integer::parseInt(&unicode.toString(), 16);
					out.write(value as char);
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ NumberFormatException) => {
						break 'try0 Err(IllegalArgumentException::new("Unable to parse unicode value: " + unicode, nfe));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
				return i + 4;
			}
			return Err(IllegalArgumentException::new("Less than 4 hex digits in unicode value: '" + input.subSequence(index, &input.length()) + "' due to end of CharSequence"));
		}
		return 0;
	}
}