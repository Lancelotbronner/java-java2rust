use java::util::ArrayList;
use java::util::Arrays;
use java::util::Collections;
use java::util::List;
use java::util::ListIterator;
use java::util::NoSuchElementException;
use java::util::StringTokenizer;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::StringUtils;

pub struct StrTokenizer {
	chars: &[u16],
	tokens: &[/* Java */ java::lang::String /**/],
	token_pos: i32,
	delim_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::split_matcher(),
	quote_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::none_matcher(),
	ignored_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::none_matcher(),
	trimmer_matcher: org::apache::commons::lang3::text::str_matcher::StrMatcher = StrMatcher::none_matcher(),
	empty_as_null: bool,
	ignore_empty_tokens: bool = true,
}

impl StrTokenizer {
	static CSV_TOKENIZER_PROTOTYPE: org::apache::commons::lang3::text::str_tokenizer::StrTokenizer = StrTokenizer::new().set_delimiter_matcher(&StrMatcher::comma_matcher()).set_quote_matcher(&StrMatcher::double_quote_matcher()).set_ignored_matcher(&StrMatcher::none_matcher()).set_trimmer_matcher(&StrMatcher::trim_matcher()).set_empty_token_as_null(false).set_ignore_empty_tokens(false);

	static TSV_TOKENIZER_PROTOTYPE: org::apache::commons::lang3::text::str_tokenizer::StrTokenizer = StrTokenizer::new().set_delimiter_matcher(&StrMatcher::tab_matcher()).set_quote_matcher(&StrMatcher::double_quote_matcher()).set_ignored_matcher(&StrMatcher::none_matcher()).set_trimmer_matcher(&StrMatcher::trim_matcher()).set_empty_token_as_null(false).set_ignore_empty_tokens(false);

	fn getcsv_clone(&self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.CSV_TOKENIZER_PROTOTYPE.clone() as StrTokenizer;
	}

	pub fn getcsv_instance(&self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::getcsv_clone();
	}

	pub fn getcsv_instance(&self, input: &&[u16]) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		/* final */ let tok: StrTokenizer = org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::getcsv_clone();
		tok.reset(input);
		return tok;
	}

	pub fn getcsv_instance(&self, input: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		/* final */ let tok: StrTokenizer = org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::getcsv_clone();
		tok.reset(input);
		return tok;
	}

	fn gettsv_clone(&self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.TSV_TOKENIZER_PROTOTYPE.clone() as StrTokenizer;
	}

	pub fn gettsv_instance(&self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::gettsv_clone();
	}

	pub fn gettsv_instance(&self, input: &&[u16]) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		/* final */ let tok: StrTokenizer = org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::gettsv_clone();
		tok.reset(input);
		return tok;
	}

	pub fn gettsv_instance(&self, input: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		/* final */ let tok: StrTokenizer = org::apache::commons::lang3::text::str_tokenizer::StrTokenizer::gettsv_clone();
		tok.reset(input);
		return tok;
	}

	pub fn new() -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.chars = null;
	}

	pub fn new(input: &&[u16]) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.chars = ArrayUtils.clone(input);
	}

	pub fn new(input: &&[u16], delim: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_char(delim);
	}

	pub fn new(input: &&[u16], delim: u16, quote: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input, delim);
		self.set_quote_char(quote);
	}

	pub fn new(input: &&[u16], delim: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_string(delim);
	}

	pub fn new(input: &&[u16], delim: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_matcher(delim);
	}

	pub fn new(input: &&[u16], delim: &org::apache::commons::lang3::text::str_matcher::StrMatcher, quote: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input, delim);
		self.set_quote_matcher(quote);
	}

	pub fn new(input: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		if input != null {
			self.chars = input.toCharArray();
		} else {
			self.chars = null;
		}
	}

	pub fn new(input: &/* Java */ java::lang::String /**/, delim: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_char(delim);
	}

	pub fn new(input: &/* Java */ java::lang::String /**/, delim: u16, quote: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input, delim);
		self.set_quote_char(quote);
	}

	pub fn new(input: &/* Java */ java::lang::String /**/, delim: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_string(delim);
	}

	pub fn new(input: &/* Java */ java::lang::String /**/, delim: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input);
		self.set_delimiter_matcher(delim);
	}

	pub fn new(input: &/* Java */ java::lang::String /**/, delim: &org::apache::commons::lang3::text::str_matcher::StrMatcher, quote: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		this(input, delim);
		self.set_quote_matcher(quote);
	}

	pub fn add(&self, obj: &/* Java */ java::lang::String /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new("add() is unsupported"));
	}

	fn add_token(&self, list: &/* Java */ java::util::List /**/, mut tok: &/* Java */ java::lang::String /**/) {
		if StringUtils::is_empty(tok) {
			if self.is_ignore_empty_tokens() {
				return;
			}
			if self.is_empty_token_as_null() {
				tok = null;
			}
		}
		list.add(tok);
	}

	fn check_tokenized(&mut self) {
		if self.tokens == null {
			if self.chars == null {
				// still call tokenize as subclass may do some work
				/* final */ let split: List<String> = self.tokenize(null, 0, 0);
				self.tokens = split.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
			} else {
				/* final */ let split: List<String> = self.tokenize(self.chars, 0, self.chars.length);
				self.tokens = split.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
			}
		}
	}

	pub fn clone(&self) -> /* Java */ java::lang::Object /**/ {
		let r0 = 'try0: {
			return self.clone_reset();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ CloneNotSupportedException) => {
				return null;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn clone_reset(&self) /* thrown(java.lang.CloneNotSupportedException) */ -> /* Java */ java::lang::Object /**/ {
		// this method exists to enable 100% test coverage
		/* final */ let cloned: StrTokenizer = super.clone() as StrTokenizer;
		if cloned.chars != null {
			cloned.chars = cloned.chars.clone();
		}
		cloned.reset();
		return cloned;
	}

	pub fn get_content(&self) -> /* Java */ java::lang::String /**/ {
		if self.chars == null {
			return null;
		}
		return String::new(self.chars);
	}

	pub fn get_delimiter_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.delimMatcher;
	}

	pub fn get_ignored_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.ignored_matcher;
	}

	pub fn get_quote_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.quote_matcher;
	}

	pub fn get_token_array(&self) -> &[/* Java */ java::lang::String /**/] {
		self.check_tokenized();
		return self.tokens.clone();
	}

	pub fn get_token_list(&self) -> /* Java */ java::util::List /**/ {
		self.check_tokenized();
		/* final */ let list: List<String> = ArrayList<>::new(self.tokens.length);
		list.addAll(&Arrays::asList(self.tokens));
		return list;
	}

	pub fn get_trimmer_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.trimmer_matcher;
	}

	pub fn has_next(&self) -> bool {
		self.check_tokenized();
		return self.token_pos < self.tokens.length;
	}

	pub fn has_previous(&self) -> bool {
		self.check_tokenized();
		return self.token_pos > 0;
	}

	pub fn is_empty_token_as_null(&self) -> bool {
		return self.emptyAsNull;
	}

	pub fn is_ignore_empty_tokens(&self) -> bool {
		return self.ignore_empty_tokens;
	}

	fn is_quote(&self, src_chars: &&[u16], pos: i32, len: i32, quote_start: i32, quote_len: i32) -> bool {
		 {
			let i: i32 = 0;
			while i < quote_len {
				{
					if pos + i >= len || src_chars[pos + i] != src_chars[quote_start + i] {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn next(&self) /* thrown(java.util.NoSuchElementException) */ -> /* Java */ java::lang::String /**/ {
		if self.has_next() {
			return self.tokens[self.token_pos += 1 !!!check!!! post increment];
		}
		return Err(NoSuchElementException::new());
	}

	pub fn next_index(&self) -> i32 {
		return self.token_pos;
	}

	pub fn next_token(&self) -> /* Java */ java::lang::String /**/ {
		if self.has_next() {
			return self.tokens[self.token_pos += 1 !!!check!!! post increment];
		}
		return null;
	}

	pub fn previous(&self) /* thrown(java.util.NoSuchElementException) */ -> /* Java */ java::lang::String /**/ {
		if self.has_previous() {
			return self.tokens[self.token_pos -= 1];
		}
		return Err(NoSuchElementException::new());
	}

	pub fn previous_index(&self) -> i32 {
		return self.token_pos - 1;
	}

	pub fn previous_token(&self) -> /* Java */ java::lang::String /**/ {
		if self.has_previous() {
			return self.tokens[self.token_pos -= 1];
		}
		return null;
	}

	fn read_next_token(&self, src_chars: &&[u16], mut start: i32, len: i32, work_area: &org::apache::commons::lang3::text::str_builder::StrBuilder, token_list: &/* Java */ java::util::List /**/) -> i32 {
		// field delimiter or the quote character
		while start < len {
			/* final */ let remove_len: i32 = Math::max(&self.get_ignored_matcher().is_match(src_chars, start, start, len), &self.get_trimmer_matcher().is_match(src_chars, start, start, len));
			if remove_len == 0 || self.get_delimiter_matcher().is_match(src_chars, start, start, len) > 0 || self.get_quote_matcher().is_match(src_chars, start, start, len) > 0 {
				break;
			}
			start += remove_len;
		}
		// handle reaching end
		if start >= len {
			self.add_token(token_list, StringUtils::EMPTY);
			return -1;
		}
		// handle empty token
		/* final */ let delim_len: i32 = self.get_delimiter_matcher().is_match(src_chars, start, start, len);
		if delim_len > 0 {
			self.add_token(token_list, StringUtils::EMPTY);
			return start + delim_len;
		}
		// handle found token
		/* final */ let quote_len: i32 = self.get_quote_matcher().is_match(src_chars, start, start, len);
		if quote_len > 0 {
			return self.read_with_quotes(src_chars, start + quote_len, len, work_area, token_list, start, quote_len);
		}
		return self.read_with_quotes(src_chars, start, len, work_area, token_list, 0, 0);
	}

	fn read_with_quotes(&self, src_chars: &&[u16], start: i32, len: i32, work_area: &org::apache::commons::lang3::text::str_builder::StrBuilder, token_list: &/* Java */ java::util::List /**/, quote_start: i32, quote_len: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> i32 {
		// Loop until we've found the end of the quoted
		// string or the end of the input
		work_area.clear();
		let pos: i32 = start;
		let quoting: bool = quote_len > 0;
		let trim_start: i32 = 0;
		while pos < len {
			// encounter a non-quoted delimiter, or end of string
			if quoting {
				// rather than end the token.
				if self.is_quote(src_chars, pos, len, quote_start, quote_len) {
					if self.is_quote(src_chars, pos + quote_len, len, quote_start, quote_len) {
						// matched pair of quotes, thus an escaped quote
						work_area.append(src_chars, pos, quote_len)?;
						pos += quote_len * 2;
						trim_start = work_area.size();
						continue;
					}
					// end of quoting
					quoting = false;
					pos += quote_len;
					continue;
				}
			} else {
				// Not in quoting mode
				// check for delimiter, and thus end of token
				/* final */ let delim_len: i32 = self.get_delimiter_matcher().is_match(src_chars, pos, start, len);
				if delim_len > 0 {
					// return condition when end of token found
					self.add_token(token_list, &work_area.substring(0, trim_start)?);
					return pos + delim_len;
				}
				// check for quote, and thus back into quoting mode
				if quote_len > 0 && self.is_quote(src_chars, pos, len, quote_start, quote_len) {
					quoting = true;
					pos += quote_len;
					continue;
				}
				// check for ignored (outside quotes), and ignore
				/* final */ let ignored_len: i32 = self.get_ignored_matcher().is_match(src_chars, pos, start, len);
				if ignored_len > 0 {
					pos += ignored_len;
					continue;
				}
				// check for trimmed character
				// don't yet know if it's at the end, so copy to workArea
				// use trimStart to keep track of trim at the end
				/* final */ let trimmed_len: i32 = self.get_trimmer_matcher().is_match(src_chars, pos, start, len);
				if trimmed_len > 0 {
					work_area.append(src_chars, pos, trimmed_len)?;
					pos += trimmed_len;
					continue;
				}
			}
			// copy regular character from inside quotes
			work_area.append(src_chars[pos += 1 !!!check!!! post increment]);
			trim_start = work_area.size();
		}
		// return condition when end of string found
		self.add_token(token_list, &work_area.substring(0, trim_start)?);
		return -1;
	}

	pub fn remove(&self) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new("remove() is unsupported"));
	}

	pub fn reset(&mut self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.token_pos = 0;
		self.tokens = null;
		return self;
	}

	pub fn reset(&mut self, input: &&[u16]) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.reset();
		self.chars = ArrayUtils::clone(input);
		return self;
	}

	pub fn reset(&mut self, input: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.reset();
		if input != null {
			self.chars = input.toCharArray();
		} else {
			self.chars = null;
		}
		return self;
	}

	pub fn set(&self, obj: &/* Java */ java::lang::String /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new("set() is unsupported"));
	}

	pub fn set_delimiter_char(&self, delim: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.set_delimiter_matcher(&StrMatcher::char_matcher(delim));
	}

	pub fn set_delimiter_matcher(&mut self, delim: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		if delim == null {
			self.delimMatcher = StrMatcher::none_matcher();
		} else {
			self.delimMatcher = delim;
		}
		return self;
	}

	pub fn set_delimiter_string(&self, delim: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.set_delimiter_matcher(&StrMatcher::string_matcher(delim));
	}

	pub fn set_empty_token_as_null(&mut self, empty_as_null: bool) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.emptyAsNull = empty_as_null;
		return self;
	}

	pub fn set_ignored_char(&self, ignored: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.set_ignored_matcher(&StrMatcher::char_matcher(ignored));
	}

	pub fn set_ignored_matcher(&mut self, ignored: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		if ignored != null {
			self.ignoredMatcher = ignored;
		}
		return self;
	}

	pub fn set_ignore_empty_tokens(&mut self, ignore_empty_tokens: bool) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		self.ignoreEmptyTokens = ignore_empty_tokens;
		return self;
	}

	pub fn set_quote_char(&self, quote: u16) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return self.set_quote_matcher(&StrMatcher::char_matcher(quote));
	}

	pub fn set_quote_matcher(&mut self, quote: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		if quote != null {
			self.quoteMatcher = quote;
		}
		return self;
	}

	pub fn set_trimmer_matcher(&mut self, trimmer: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		if trimmer != null {
			self.trimmerMatcher = trimmer;
		}
		return self;
	}

	pub fn size(&self) -> i32 {
		self.check_tokenized();
		return self.tokens.length;
	}

	fn tokenize(&self, src_chars: &&[u16], offset: i32, count: i32) -> /* Java */ java::util::List /**/ {
		if ArrayUtils::is_empty(src_chars) {
			return Collections::emptyList();
		}
		/* final */ let buf: StrBuilder = StrBuilder::new();
		/* final */ let token_list: List<String> = ArrayList<>::new();
		let pos: i32 = offset;
		// loop around the entire buffer
		while pos >= 0 && pos < count {
			// find next token
			pos = self.read_next_token(src_chars, pos, count, buf, token_list);
			// handle case where end of string is a delimiter
			if pos >= count {
				self.add_token(token_list, StringUtils::EMPTY);
			}
		}
		return token_list;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		if self.tokens == null {
			return "StrTokenizer[not tokenized yet]";
		}
		return "StrTokenizer" + self.get_token_list();
	}
}

impl /* Java */ java::util::ListIterator /**/ for StrTokenizer {}

impl /* Java */ java::util::Iterator /**/ for StrTokenizer {}

impl /* Java */ java::lang::Cloneable /**/ for StrTokenizer {}