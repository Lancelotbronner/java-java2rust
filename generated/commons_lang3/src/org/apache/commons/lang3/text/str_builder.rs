use java::io::IOException;
use java::io::Reader;
use java::io::Serializable;
use java::io::Writer;
use java::nio::CharBuffer;
use java::util::Arrays;
use java::util::Iterator;
use java::util::List;
use java::util::Objects;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::CharUtils;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Strings;
use crate::org::apache::commons::lang3::builder::Builder;

pub struct StrBuilder {
	buffer: &[u16],
	size: i32,
	new_line: /* Java */ java::lang::String /**/,
	null_text: /* Java */ java::lang::String /**/,
}

impl StrBuilder {
	static CAPACITY: i32 = 32;

	static serialVersionUID: i64 = 7628716375283629643;

	pub fn new() -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		this(self.CAPACITY);
	}

	pub fn new(mut initial_capacity: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if initial_capacity <= 0 {
			initial_capacity = self.CAPACITY;
		}
		self.buffer = : [Option<char>; initial_capacity] = [None; initial_capacity];
	}

	pub fn new(str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			self.buffer = : [Option<char>; self.CAPACITY] = [None; self.CAPACITY];
		} else {
			self.buffer = : [Option<char>; str.length() + self.CAPACITY] = [None; str.length() + self.CAPACITY];
			self.append(str);
		}
	}

	pub fn append(&mut self, value: bool) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if value {
			self.ensure_capacity(self.size + 4);
			self.buffer[self.size += 1 !!!check!!! post increment] = 't';
			self.buffer[self.size += 1 !!!check!!! post increment] = 'r';
			self.buffer[self.size += 1 !!!check!!! post increment] = 'u';
		} else {
			self.ensure_capacity(self.size + 5);
			self.buffer[self.size += 1 !!!check!!! post increment] = 'f';
			self.buffer[self.size += 1 !!!check!!! post increment] = 'a';
			self.buffer[self.size += 1 !!!check!!! post increment] = 'l';
			self.buffer[self.size += 1 !!!check!!! post increment] = 's';
		}
		self.buffer[self.size += 1 !!!check!!! post increment] = 'e';
		return self;
	}

	pub fn append(&mut self, ch: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let len: i32 = self.length();
		self.ensure_capacity(len + 1);
		self.buffer[self.size += 1 !!!check!!! post increment] = ch;
		return self;
	}

	pub fn append(&mut self, chars: &&[u16]) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if chars == null {
			return self.append_null();
		}
		/* final */ let str_len: i32 = chars.length;
		if str_len > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + str_len);
			System::arraycopy(chars, 0, self.buffer, len, str_len);
			self.size += str_len;
		}
		return self;
	}

	pub fn append(&mut self, chars: &&[u16], start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if chars == null {
			return self.append_null();
		}
		if start_index < 0 || start_index > chars.length {
			return Err(StringIndexOutOfBoundsException::new("Invalid startIndex: " + length));
		}
		if length < 0 || start_index + length > chars.length {
			return Err(StringIndexOutOfBoundsException::new("Invalid length: " + length));
		}
		if length > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			System::arraycopy(chars, start_index, self.buffer, len, length);
			self.size += length;
		}
		return self;
	}

	pub fn append(&mut self, buf: &/* Java */ java::nio::CharBuffer /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if buf == null {
			return self.append_null();
		}
		if buf.hasArray() {
			/* final */ let length: i32 = buf.remaining();
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			System::arraycopy(&buf.array(), buf.arrayOffset() + buf.position(), self.buffer, len, length);
			self.size += length;
		} else {
			self.append(&buf.toString());
		}
		return self;
	}

	pub fn append(&mut self, buf: &/* Java */ java::nio::CharBuffer /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if buf == null {
			return self.append_null();
		}
		if buf.hasArray() {
			/* final */ let total_length: i32 = buf.remaining();
			if start_index < 0 || start_index > total_length {
				return Err(StringIndexOutOfBoundsException::new("startIndex must be valid"));
			}
			if length < 0 || start_index + length > total_length {
				return Err(StringIndexOutOfBoundsException::new("length must be valid"));
			}
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			System::arraycopy(&buf.array(), buf.arrayOffset() + buf.position() + start_index, self.buffer, len, length);
			self.size += length;
		} else {
			self.append(&buf.toString(), start_index, length)?;
		}
		return self;
	}

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if seq == null {
			return self.append_null();
		}
		if seq instanceof StrBuilder {
			return self.append(seq as StrBuilder);
		}
		if seq instanceof StringBuilder {
			return self.append(seq as StringBuilder);
		}
		if seq instanceof StringBuffer {
			return self.append(seq as StringBuffer);
		}
		if seq instanceof CharBuffer {
			return self.append(seq as CharBuffer);
		}
		return self.append(&seq.toString());
	}

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if seq == null {
			return self.append_null();
		}
		return self.append(&seq.toString(), start_index, length)?;
	}

	pub fn append(&self, value: f64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(&String::valueOf(value));
	}

	pub fn append(&self, value: f32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(&String::valueOf(value));
	}

	pub fn append(&self, value: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(&String::valueOf(value));
	}

	pub fn append(&self, value: i64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(&String::valueOf(value));
	}

	pub fn append(&self, obj: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if obj == null {
			return self.append_null();
		}
		if obj instanceof CharSequence {
			return self.append(obj as CharSequence);
		}
		return self.append(&obj.toString());
	}

	pub fn append(&mut self, str: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		/* final */ let str_len: i32 = str.length();
		if str_len > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + str_len);
			System::arraycopy(str.buffer, 0, self.buffer, len, str_len);
			self.size += str_len;
		}
		return self;
	}

	pub fn append(&mut self, str: &org::apache::commons::lang3::text::str_builder::StrBuilder, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		if start_index < 0 || start_index > str.length() {
			return Err(StringIndexOutOfBoundsException::new("startIndex must be valid"));
		}
		if length < 0 || start_index + length > str.length() {
			return Err(StringIndexOutOfBoundsException::new("length must be valid"));
		}
		if length > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			str.get_chars(start_index, start_index + length, self.buffer, len)?;
			self.size += length;
		}
		return self;
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		/* final */ let str_len: i32 = str.length();
		if str_len > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + str_len);
			str.getChars(0, str_len, self.buffer, len);
			self.size += str_len;
		}
		return self;
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::String /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		if start_index < 0 || start_index > str.length() {
			return Err(StringIndexOutOfBoundsException::new("startIndex must be valid"));
		}
		if length < 0 || start_index + length > str.length() {
			return Err(StringIndexOutOfBoundsException::new("length must be valid"));
		}
		if length > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			str.getChars(start_index, start_index + length, self.buffer, len);
			self.size += length;
		}
		return self;
	}

	pub fn append(&self, format: &/* Java */ java::lang::String /**/, objs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(&String::format(format, objs));
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::StringBuffer /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		/* final */ let str_len: i32 = str.length();
		if str_len > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + str_len);
			str.getChars(0, str_len, self.buffer, len);
			self.size += str_len;
		}
		return self;
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::StringBuffer /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		if start_index < 0 || start_index > str.length() {
			return Err(StringIndexOutOfBoundsException::new("startIndex must be valid"));
		}
		if length < 0 || start_index + length > str.length() {
			return Err(StringIndexOutOfBoundsException::new("length must be valid"));
		}
		if length > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			str.getChars(start_index, start_index + length, self.buffer, len);
			self.size += length;
		}
		return self;
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::StringBuilder /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		/* final */ let str_len: i32 = str.length();
		if str_len > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + str_len);
			str.getChars(0, str_len, self.buffer, len);
			self.size += str_len;
		}
		return self;
	}

	pub fn append(&mut self, str: &/* Java */ java::lang::StringBuilder /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if str == null {
			return self.append_null();
		}
		if start_index < 0 || start_index > str.length() {
			return Err(StringIndexOutOfBoundsException::new("startIndex must be valid"));
		}
		if length < 0 || start_index + length > str.length() {
			return Err(StringIndexOutOfBoundsException::new("length must be valid"));
		}
		if length > 0 {
			/* final */ let len: i32 = self.length();
			self.ensure_capacity(len + length);
			str.getChars(start_index, start_index + length, self.buffer, len);
			self.size += length;
		}
		return self;
	}

	pub fn append_all(&self, iterable: &/* Java */ java::lang::Iterable /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if iterable != null {
			iterable.forEach(self::append);
		}
		return self;
	}

	pub fn append_all(&self, it: &/* Java */ java::util::Iterator /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if it != null {
			it.forEachRemaining(self::append);
		}
		return self;
	}

	pub fn append_all<T>(&self, array: &T) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* 
	         * @SuppressWarnings used to hide warning about vararg usage. We cannot
	         * use @SafeVarargs, since this method is not final. Using @SuppressWarnings
	         * is fine, because it isn't inherited by subclasses, so each subclass must
	         * vouch for itself whether its use of 'array' is safe.
	         */ 
		if ArrayUtils::is_not_empty(array) {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append_fixed_width_pad_left(&self, value: i32, width: i32, pad_char: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append_fixed_width_pad_left(&String::valueOf(value), width, pad_char);
	}

	pub fn append_fixed_width_pad_left(&mut self, obj: &/* Java */ java::lang::Object /**/, width: i32, pad_char: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if width > 0 {
			self.ensure_capacity(self.size + width);
			let str: String = ObjectUtils::to_string(obj, self::getNullText);
			if str == null {
				str = StringUtils::EMPTY;
			}
			/* final */ let str_len: i32 = str.length();
			if str_len >= width {
				str.getChars(str_len - width, str_len, self.buffer, self.size);
			} else {
				/* final */ let pad_len: i32 = width - str_len;
				/* final */ let to_index: i32 = self.size + pad_len;
				Arrays::fill(self.buffer, self.size, to_index, pad_char);
				str.getChars(0, str_len, self.buffer, to_index);
			}
			self.size += width;
		}
		return self;
	}

	pub fn append_fixed_width_pad_right(&self, value: i32, width: i32, pad_char: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append_fixed_width_pad_right(&String::valueOf(value), width, pad_char);
	}

	pub fn append_fixed_width_pad_right(&mut self, obj: &/* Java */ java::lang::Object /**/, width: i32, pad_char: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if width > 0 {
			self.ensure_capacity(self.size + width);
			let str: String = ObjectUtils::to_string(obj, self::getNullText);
			if str == null {
				str = StringUtils::EMPTY;
			}
			/* final */ let str_len: i32 = str.length();
			if str_len >= width {
				str.getChars(0, width, self.buffer, self.size);
			} else {
				str.getChars(0, str_len, self.buffer, self.size);
				/* final */ let from_index: i32 = self.size + str_len;
				Arrays::fill(self.buffer, from_index, from_index + width - str_len, pad_char);
			}
			self.size += width;
		}
		return self;
	}

	pub fn appendln(&self, value: bool) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(value).append_new_line();
	}

	pub fn appendln(&self, ch: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(ch).append_new_line();
	}

	pub fn appendln(&self, chars: &&[u16]) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(chars).append_new_line();
	}

	pub fn appendln(&self, chars: &&[u16], start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(chars, start_index, length)?.append_new_line();
	}

	pub fn appendln(&self, value: f64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(value).append_new_line();
	}

	pub fn appendln(&self, value: f32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(value).append_new_line();
	}

	pub fn appendln(&self, value: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(value).append_new_line();
	}

	pub fn appendln(&self, value: i64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(value).append_new_line();
	}

	pub fn appendln(&self, obj: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(obj).append_new_line();
	}

	pub fn appendln(&self, str: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str).append_new_line();
	}

	pub fn appendln(&self, str: &org::apache::commons::lang3::text::str_builder::StrBuilder, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str, start_index, length)?.append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str).append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::String /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str, start_index, length)?.append_new_line();
	}

	pub fn appendln(&self, format: &/* Java */ java::lang::String /**/, objs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(format, objs).append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::StringBuffer /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str).append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::StringBuffer /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str, start_index, length)?.append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::StringBuilder /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str).append_new_line();
	}

	pub fn appendln(&self, str: &/* Java */ java::lang::StringBuilder /**/, start_index: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append(str, start_index, length)?.append_new_line();
	}

	pub fn append_new_line(&self) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.new_line == null {
			self.append(&System::lineSeparator());
			return self;
		}
		return self.append(self.new_line);
	}

	pub fn append_null(&self) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.null_text == null {
			return self;
		}
		return self.append(self.null_text);
	}

	pub fn append_padding(&mut self, length: i32, pad_char: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if length >= 0 {
			self.ensure_capacity(self.size + length);
			 {
				let i: i32 = 0;
				while i < length {
					{
						self.buffer[self.size += 1 !!!check!!! post increment] = pad_char;
					}
					i += 1;
				 }
			 }
	
		}
		return self;
	}

	pub fn append_separator(&self, separator: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.is_not_empty() {
			self.append(separator);
		}
		return self;
	}

	pub fn append_separator(&self, standard: u16, default_if_empty: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.is_not_empty() {
			self.append(standard);
		} else {
			self.append(default_if_empty);
		}
		return self;
	}

	pub fn append_separator(&self, separator: u16, loop_index: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if loop_index > 0 {
			self.append(separator);
		}
		return self;
	}

	pub fn append_separator(&self, separator: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.append_separator(separator, null);
	}

	pub fn append_separator(&self, separator: &/* Java */ java::lang::String /**/, loop_index: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if separator != null && loop_index > 0 {
			self.append(separator);
		}
		return self;
	}

	pub fn append_separator(&self, standard: &/* Java */ java::lang::String /**/, default_if_empty: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let str: String =  if self.is_empty() { default_if_empty } else { standard };
		if str != null {
			self.append(str);
		}
		return self;
	}

	pub fn append_to(&self, appendable: &/* Java */ java::lang::Appendable /**/) /* thrown(java.io.IOException) */ {
		if appendable instanceof Writer {
			(appendable as Writer).write(self.buffer, 0, self.size);
		} else if appendable instanceof StringBuilder {
			(appendable as StringBuilder).append(self.buffer, 0, self.size);
		} else if appendable instanceof StringBuffer {
			(appendable as StringBuffer).append(self.buffer, 0, self.size);
		} else if appendable instanceof CharBuffer {
			(appendable as CharBuffer).put(self.buffer, 0, self.size);
		} else {
			appendable.append(self);
		}
	}

	pub fn append_with_separators(&self, iterable: &/* Java */ java::lang::Iterable /**/, separator: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if iterable != null {
			/* final */ let sep: String = Objects::toString(separator, "");
			/* final */ let it: Iterator<?> = iterable.iterator();
			while it.hasNext() {
				self.append(&it.next());
				if it.hasNext() {
					self.append(sep);
				}
			}
		}
		return self;
	}

	pub fn append_with_separators(&self, it: &/* Java */ java::util::Iterator /**/, separator: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if it != null {
			/* final */ let sep: String = Objects::toString(separator, "");
			while it.hasNext() {
				self.append(&it.next());
				if it.hasNext() {
					self.append(sep);
				}
			}
		}
		return self;
	}

	pub fn append_with_separators(&self, array: &&[/* Java */ java::lang::Object /**/], separator: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if array != null && array.length > 0 {
			/* final */ let sep: String = Objects::toString(separator, "");
			self.append(array[0]);
			 {
				let i: i32 = 1;
				while i < array.length {
					{
						self.append(sep);
						self.append(array[i]);
					}
					i += 1;
				 }
			 }
	
		}
		return self;
	}

	pub fn as_reader(&self) -> /* Java */ java::io::Reader /**/ {
		return StrBuilderReader::new();
	}

	pub fn as_tokenizer(&self) -> org::apache::commons::lang3::text::str_tokenizer::StrTokenizer {
		return StrBuilderTokenizer::new();
	}

	pub fn as_writer(&self) -> /* Java */ java::io::Writer /**/ {
		return StrBuilderWriter::new();
	}

	pub fn build(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_string();
	}

	pub fn capacity(&self) -> i32 {
		return self.buffer.length;
	}

	pub fn char_at(&self, index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> u16 {
		if index < 0 || index >= self.length() {
			return Err(StringIndexOutOfBoundsException::new(index));
		}
		return self.buffer[index];
	}

	pub fn clear(&mut self) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.size = 0;
		return self;
	}

	pub fn contains(&self, ch: u16) -> bool {
		/* final */ let this_buf: Vec<char> = self.buffer;
		 {
			let i: i32 = 0;
			while i < self.size {
				{
					if this_buf[i] == ch {
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return false;
	}

	pub fn contains(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return self.index_of(str, 0) >= 0;
	}

	pub fn contains(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> bool {
		return self.index_of(matcher, 0) >= 0;
	}

	pub fn delete(&self, start_index: i32, mut end_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		end_index = self.validate_range(start_index, end_index)?;
		/* final */ let len: i32 = end_index - start_index;
		if len > 0 {
			self.delete_impl(start_index, end_index, len);
		}
		return self;
	}

	pub fn delete_all(&self, ch: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		 {
			let i: i32 = 0;
			while i < self.size {
				{
					if self.buffer[i] == ch {
						/* final */ let start: i32 = i;
						while i += 1 < self.size {
							if self.buffer[i] != ch {
								break;
							}
						}
						/* final */ let len: i32 = i - start;
						self.delete_impl(start, i, len);
						i -= len;
					}
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn delete_all(&self, str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let len: i32 = StringUtils::length(str);
		if len > 0 {
			let index: i32 = self.index_of(str, 0);
			while index >= 0 {
				self.delete_impl(index, index + len, len);
				index = self.index_of(str, index);
			}
		}
		return self;
	}

	pub fn delete_all(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.replace(matcher, null, 0, self.size, -1);
	}

	pub fn delete_char_at(&self, index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if index < 0 || index >= self.size {
			return Err(StringIndexOutOfBoundsException::new(index));
		}
		self.delete_impl(index, index + 1, 1);
		return self;
	}

	pub fn delete_first(&self, ch: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		 {
			let i: i32 = 0;
			while i < self.size {
				{
					if self.buffer[i] == ch {
						self.delete_impl(i, i + 1, 1);
						break;
					}
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn delete_first(&self, str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let len: i32 = StringUtils::length(str);
		if len > 0 {
			/* final */ let index: i32 = self.index_of(str, 0);
			if index >= 0 {
				self.delete_impl(index, index + len, len);
			}
		}
		return self;
	}

	pub fn delete_first(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.replace(matcher, null, 0, self.size, 1);
	}

	fn delete_impl(&mut self, start_index: i32, end_index: i32, len: i32) {
		System::arraycopy(self.buffer, end_index, self.buffer, start_index, self.size - end_index);
		self.size -= len;
	}

	pub fn ends_with(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		if str == null {
			return false;
		}
		/* final */ let len: i32 = str.length();
		if len == 0 {
			return true;
		}
		if len > self.size {
			return false;
		}
		let pos: i32 = self.size - len;
		 {
			let i: i32 = 0;
			while i < len {
				{
					if self.buffer[pos] != str.charAt(i) {
						return false;
					}
				}
				i += 1;
				pos += 1;
			 }
		 }
	
		return true;
	}

	pub fn ensure_capacity(&mut self, capacity: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if capacity > self.buffer.length {
			self.buffer = ArrayUtils::arraycopy(self.buffer, 0, 0, self.size, |()|: [Option<char>; capacity * 2] = [None; capacity * 2]);
		}
		return self;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj instanceof StrBuilder && self.equals(obj as StrBuilder);
	}

	pub fn equals(&self, other: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> bool {
		if self == other {
			return true;
		}
		if other == null {
			return false;
		}
		if self.size != other.size {
			return false;
		}
		/* final */ let this_buf: Vec<char> = self.buffer;
		/* final */ let other_buf: Vec<char> = other.buffer;
		 {
			let i: i32 = self.size - 1;
			while i >= 0 {
				{
					if this_buf[i] != other_buf[i] {
						return false;
					}
				}
				i -= 1;
			 }
		 }
	
		return true;
	}

	pub fn equals_ignore_case(&self, other: &org::apache::commons::lang3::text::str_builder::StrBuilder) -> bool {
		if self == other {
			return true;
		}
		if self.size != other.size {
			return false;
		}
		/* final */ let this_buf: Vec<char> = self.buffer;
		/* final */ let other_buf: Vec<char> = other.buffer;
		 {
			let i: i32 = self.size - 1;
			while i >= 0 {
				{
					/* final */ let c1: char = this_buf[i];
					/* final */ let c2: char = other_buf[i];
					if c1 != c2 && Character::toUpperCase(c1) != Character::toUpperCase(c2) {
						return false;
					}
				}
				i -= 1;
			 }
		 }
	
		return true;
	}

	pub fn get_chars(&self, mut destination: &&[u16]) -> &[u16] {
		/* final */ let len: i32 = self.length();
		if destination == null || destination.length < len {
			destination = : [Option<char>; len] = [None; len];
		}
		return ArrayUtils::arraycopy(self.buffer, 0, destination, 0, len);
	}

	pub fn get_chars(&self, start_index: i32, end_index: i32, destination: &&[u16], destination_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ {
		if start_index < 0 {
			return Err(StringIndexOutOfBoundsException::new(start_index));
		}
		if end_index < 0 || end_index > self.length() {
			return Err(StringIndexOutOfBoundsException::new(end_index));
		}
		if start_index > end_index {
			return Err(StringIndexOutOfBoundsException::new("end < start"));
		}
		System::arraycopy(self.buffer, start_index, destination, destination_index, end_index - start_index);
	}

	pub fn get_new_line_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.new_line;
	}

	pub fn get_null_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.null_text;
	}

	pub fn hash_code(&self) -> i32 {
		/* final */ let buf: Vec<char> = self.buffer;
		let hash: i32 = 0;
		 {
			let i: i32 = self.size - 1;
			while i >= 0 {
				{
					hash = 31 * hash + buf[i];
				}
				i -= 1;
			 }
		 }
	
		return hash;
	}

	pub fn index_of(&self, ch: u16) -> i32 {
		return self.index_of(ch, 0);
	}

	pub fn index_of(&self, ch: u16, mut start_index: i32) -> i32 {
		start_index = Math::max(start_index, 0);
		if start_index >= self.size {
			return -1;
		}
		/* final */ let this_buf: Vec<char> = self.buffer;
		 {
			let i: i32 = start_index;
			while i < self.size {
				{
					if this_buf[i] == ch {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return -1;
	}

	pub fn index_of(&self, str: &/* Java */ java::lang::String /**/) -> i32 {
		return self.index_of(str, 0);
	}

	pub fn index_of(&self, str: &/* Java */ java::lang::String /**/, start_index: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.index_of(self, str, start_index);
	}

	pub fn index_of(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> i32 {
		return self.index_of(matcher, 0);
	}

	pub fn index_of(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, mut start_index: i32) -> i32 {
		start_index = Math::max(start_index, 0);
		if matcher == null || start_index >= self.size {
			return -1;
		}
		/* final */ let len: i32 = self.size;
		/* final */ let buf: Vec<char> = self.buffer;
		 {
			let i: i32 = start_index;
			while i < len {
				{
					if matcher.is_match(buf, i, start_index, len) > 0 {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return -1;
	}

	pub fn insert(&mut self, mut index: i32, value: bool) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.validate_index(index)?;
		if value {
			self.ensure_capacity(self.size + 4);
			System::arraycopy(self.buffer, index, self.buffer, index + 4, self.size - index);
			self.buffer[index += 1 !!!check!!! post increment] = 't';
			self.buffer[index += 1 !!!check!!! post increment] = 'r';
			self.buffer[index += 1 !!!check!!! post increment] = 'u';
			self.buffer[index] = 'e';
			self.size += 4;
		} else {
			self.ensure_capacity(self.size + 5);
			System::arraycopy(self.buffer, index, self.buffer, index + 5, self.size - index);
			self.buffer[index += 1 !!!check!!! post increment] = 'f';
			self.buffer[index += 1 !!!check!!! post increment] = 'a';
			self.buffer[index += 1 !!!check!!! post increment] = 'l';
			self.buffer[index += 1 !!!check!!! post increment] = 's';
			self.buffer[index] = 'e';
			self.size += 5;
		}
		return self;
	}

	pub fn insert(&mut self, mut index: i32, value: u16) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.validate_index(index)?;
		self.ensure_capacity(self.size + 1);
		System::arraycopy(self.buffer, index, self.buffer, index + 1, self.size - index);
		self.buffer[index] = value;
		self.size += 1;
		return self;
	}

	pub fn insert(&mut self, index: i32, chars: &&[u16]) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.validate_index(index)?;
		if chars == null {
			return self.insert(index, self.null_text);
		}
		/* final */ let len: i32 = chars.length;
		if len > 0 {
			self.ensure_capacity(self.size + len);
			System::arraycopy(self.buffer, index, self.buffer, index + len, self.size - index);
			System::arraycopy(chars, 0, self.buffer, index, len);
			self.size += len;
		}
		return self;
	}

	pub fn insert(&mut self, index: i32, chars: &&[u16], offset: i32, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.validate_index(index)?;
		if chars == null {
			return self.insert(index, self.null_text);
		}
		if offset < 0 || offset > chars.length {
			return Err(StringIndexOutOfBoundsException::new("Invalid offset: " + offset));
		}
		if length < 0 || offset + length > chars.length {
			return Err(StringIndexOutOfBoundsException::new("Invalid length: " + length));
		}
		if length > 0 {
			self.ensure_capacity(self.size + length);
			System::arraycopy(self.buffer, index, self.buffer, index + length, self.size - index);
			System::arraycopy(chars, offset, self.buffer, index, length);
			self.size += length;
		}
		return self;
	}

	pub fn insert(&self, index: i32, value: f64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.insert(index, &String::valueOf(value));
	}

	pub fn insert(&self, index: i32, value: f32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.insert(index, &String::valueOf(value));
	}

	pub fn insert(&self, index: i32, value: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.insert(index, &String::valueOf(value));
	}

	pub fn insert(&self, index: i32, value: i64) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.insert(index, &String::valueOf(value));
	}

	pub fn insert(&self, index: i32, obj: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if obj == null {
			return self.insert(index, self.null_text);
		}
		return self.insert(index, &obj.toString());
	}

	pub fn insert(&mut self, index: i32, mut str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.validate_index(index)?;
		if str == null {
			str = self.null_text;
		}
		if str != null {
			/* final */ let str_len: i32 = str.length();
			if str_len > 0 {
				/* final */ let new_size: i32 = self.size + str_len;
				self.ensure_capacity(new_size);
				System::arraycopy(self.buffer, index, self.buffer, index + str_len, self.size - index);
				self.size = new_size;
				str.getChars(0, str_len, self.buffer, index);
			}
		}
		return self;
	}

	pub fn is_empty(&self) -> bool {
		return self.size == 0;
	}

	pub fn is_not_empty(&self) -> bool {
		return self.size > 0;
	}

	pub fn last_index_of(&self, ch: u16) -> i32 {
		return self.last_index_of(ch, self.size - 1);
	}

	pub fn last_index_of(&self, ch: u16, mut start_index: i32) -> i32 {
		start_index =  if start_index >= self.size { self.size - 1 } else { start_index };
		if start_index < 0 {
			return -1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if self.buffer[i] == ch {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return -1;
	}

	pub fn last_index_of(&self, str: &/* Java */ java::lang::String /**/) -> i32 {
		return self.last_index_of(str, self.size - 1);
	}

	pub fn last_index_of(&self, str: &/* Java */ java::lang::String /**/, start_index: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.last_index_of(self, str, start_index);
	}

	pub fn last_index_of(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher) -> i32 {
		return self.last_index_of(matcher, self.size);
	}

	pub fn last_index_of(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, mut start_index: i32) -> i32 {
		start_index =  if start_index >= self.size { self.size - 1 } else { start_index };
		if matcher == null || start_index < 0 {
			return -1;
		}
		/* final */ let buf: Vec<char> = self.buffer;
		/* final */ let end_index: i32 = start_index + 1;
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if matcher.is_match(buf, i, 0, end_index) > 0 {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return -1;
	}

	pub fn left_string(&self, length: i32) -> /* Java */ java::lang::String /**/ {
		if length <= 0 {
			return StringUtils::EMPTY;
		}
		if length >= self.size {
			return String::new(self.buffer, 0, self.size);
		}
		return String::new(self.buffer, 0, length);
	}

	pub fn length(&self) -> i32 {
		return self.size;
	}

	pub fn mid_string(&self, mut index: i32, length: i32) -> /* Java */ java::lang::String /**/ {
		if index < 0 {
			index = 0;
		}
		if length <= 0 || index >= self.size {
			return StringUtils::EMPTY;
		}
		if self.size <= index + length {
			return String::new(self.buffer, index, self.size - index);
		}
		return String::new(self.buffer, index, length);
	}

	pub fn minimize_capacity(&mut self) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.buffer.length > self.length() {
			self.buffer = ArrayUtils::arraycopy(self.buffer, 0, 0, self.size, |()|: [Option<char>; self.length()] = [None; self.length()]);
		}
		return self;
	}

	pub fn read_from(&mut self, readable: &/* Java */ java::lang::Readable /**/) /* thrown(java.io.IOException) */ -> i32 {
		/* final */ let old_size: i32 = self.size;
		if readable instanceof Reader {
			/* final */ let r: Reader = readable as Reader;
			self.ensure_capacity(self.size + 1);
			let read: i32;
			while (read = r.read(self.buffer, self.size, self.buffer.length - self.size)) != -1 {
				self.size += read;
				self.ensure_capacity(self.size + 1);
			}
		} else if readable instanceof CharBuffer {
			/* final */ let cb: CharBuffer = readable as CharBuffer;
			/* final */ let remaining: i32 = cb.remaining();
			self.ensure_capacity(self.size + remaining);
			cb.get(self.buffer, self.size, remaining);
			self.size += remaining;
		} else {
			while true {
				self.ensure_capacity(self.size + 1);
				/* final */ let buf: CharBuffer = CharBuffer::wrap(self.buffer, self.size, self.buffer.length - self.size);
				/* final */ let read: i32 = readable.read(buf);
				if read == -1 {
					break;
				}
				self.size += read;
			}
		}
		return self.size - old_size;
	}

	pub fn replace(&self, start_index: i32, mut end_index: i32, replace_str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		end_index = self.validate_range(start_index, end_index)?;
		/* final */ let insert_len: i32 = StringUtils::length(replace_str);
		self.replace_impl(start_index, end_index, end_index - start_index, replace_str, insert_len);
		return self;
	}

	pub fn replace(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, replace_str: &/* Java */ java::lang::String /**/, start_index: i32, mut end_index: i32, replace_count: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		end_index = self.validate_range(start_index, end_index)?;
		return self.replace_impl(matcher, replace_str, start_index, end_index, replace_count);
	}

	pub fn replace_all(&mut self, search: u16, replace: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if search != replace {
			 {
				let i: i32 = 0;
				while i < self.size {
					{
						if self.buffer[i] == search {
							self.buffer[i] = replace;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return self;
	}

	pub fn replace_all(&self, search_str: &/* Java */ java::lang::String /**/, replace_str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let search_len: i32 = StringUtils::length(search_str);
		if search_len > 0 {
			/* final */ let replace_len: i32 = StringUtils::length(replace_str);
			let index: i32 = self.index_of(search_str, 0);
			while index >= 0 {
				self.replace_impl(index, index + search_len, search_len, replace_str, replace_len);
				index = self.index_of(search_str, index + replace_len);
			}
		}
		return self;
	}

	pub fn replace_all(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, replace_str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.replace(matcher, replace_str, 0, self.size, -1)?;
	}

	pub fn replace_first(&mut self, search: u16, replace: u16) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if search != replace {
			 {
				let i: i32 = 0;
				while i < self.size {
					{
						if self.buffer[i] == search {
							self.buffer[i] = replace;
							break;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return self;
	}

	pub fn replace_first(&self, search_str: &/* Java */ java::lang::String /**/, replace_str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		/* final */ let search_len: i32 = StringUtils::length(search_str);
		if search_len > 0 {
			/* final */ let index: i32 = self.index_of(search_str, 0);
			if index >= 0 {
				/* final */ let replace_len: i32 = StringUtils::length(replace_str);
				self.replace_impl(index, index + search_len, search_len, replace_str, replace_len);
			}
		}
		return self;
	}

	pub fn replace_first(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, replace_str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		return self.replace(matcher, replace_str, 0, self.size, 1)?;
	}

	fn replace_impl(&mut self, start_index: i32, end_index: i32, remove_len: i32, insert_str: &/* Java */ java::lang::String /**/, insert_len: i32) {
		/* final */ let new_size: i32 = self.size - remove_len + insert_len;
		if insert_len != remove_len {
			self.ensure_capacity(new_size);
			System::arraycopy(self.buffer, end_index, self.buffer, start_index + insert_len, self.size - end_index);
			self.size = new_size;
		}
		if insert_len > 0 {
			insert_str.getChars(0, insert_len, self.buffer, start_index);
		}
	}

	fn replace_impl(&self, matcher: &org::apache::commons::lang3::text::str_matcher::StrMatcher, replace_str: &/* Java */ java::lang::String /**/, from: i32, mut to: i32, replace_count: i32) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if matcher == null || self.size == 0 {
			return self;
		}
		/* final */ let replace_len: i32 = StringUtils::length(replace_str);
		 {
			let i: i32 = from;
			while i < to && replace_count != 0 {
				{
					/* final */ let buf: Vec<char> = self.buffer;
					/* final */ let remove_len: i32 = matcher.is_match(buf, i, from, to);
					if remove_len > 0 {
						self.replace_impl(i, i + remove_len, remove_len, replace_str, replace_len);
						to = to - remove_len + replace_len;
						i = i + replace_len - 1;
						if replace_count > 0 {
							replace_count -= 1;
						}
					}
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn reverse(&self) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.size == 0 {
			return self;
		}
		/* final */ let half: i32 = self.size / 2;
		/* final */ let buf: Vec<char> = self.buffer;
		 {
			let left_idx: i32 = 0; let right_idx: i32 = self.size - 1;
			while left_idx < half {
				{
					/* final */ let swap: char = buf[left_idx];
					buf[left_idx] = buf[right_idx];
					buf[right_idx] = swap;
				}
				left_idx += 1;
				right_idx -= 1;
			 }
		 }
	
		return self;
	}

	pub fn right_string(&self, length: i32) -> /* Java */ java::lang::String /**/ {
		if length <= 0 {
			return StringUtils::EMPTY;
		}
		if length >= self.size {
			return String::new(self.buffer, 0, self.size);
		}
		return String::new(self.buffer, self.size - length, length);
	}

	pub fn set_char_at(&mut self, mut index: i32, ch: u16) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if index < 0 || index >= self.length() {
			return Err(StringIndexOutOfBoundsException::new(index));
		}
		self.buffer[index] = ch;
		return self;
	}

	pub fn set_length(&mut self, length: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if length < 0 {
			return Err(StringIndexOutOfBoundsException::new(length));
		}
		if length < self.size {
			self.size = length;
		} else if length > self.size {
			self.ensure_capacity(length);
			Arrays::fill(self.buffer, self.size, length, CharUtils::NUL);
			self.size = length;
		}
		return self;
	}

	pub fn set_new_line_text(&mut self, new_line: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		self.newLine = new_line;
		return self;
	}

	pub fn set_null_text(&mut self, mut null_text: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if StringUtils::is_empty(null_text) {
			null_text = null;
		}
		self.nullText = null_text;
		return self;
	}

	pub fn size(&self) -> i32 {
		return self.size;
	}

	pub fn starts_with(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		if str == null {
			return false;
		}
		/* final */ let len: i32 = str.length();
		if len == 0 {
			return true;
		}
		if len > self.size {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < len {
				{
					if self.buffer[i] != str.charAt(i) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn sub_sequence(&self, start_index: i32, end_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> /* Java */ java::lang::CharSequence /**/ {
		if start_index < 0 {
			return Err(StringIndexOutOfBoundsException::new(start_index));
		}
		if end_index > self.size {
			return Err(StringIndexOutOfBoundsException::new(end_index));
		}
		if start_index > end_index {
			return Err(StringIndexOutOfBoundsException::new(end_index - start_index));
		}
		return self.substring(start_index, end_index);
	}

	pub fn substring(&self, start: i32) -> /* Java */ java::lang::String /**/ {
		return self.substring(start, self.size);
	}

	pub fn substring(&self, start_index: i32, mut end_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> /* Java */ java::lang::String /**/ {
		end_index = self.validate_range(start_index, end_index)?;
		return String::new(self.buffer, start_index, end_index - start_index);
	}

	pub fn to_char_array(&self) -> &[u16] {
		if self.size == 0 {
			return ArrayUtils::EMPTY_CHAR_ARRAY;
		}
		return ArrayUtils.arraycopy(self.buffer, 0, 0, self.size, Vec<char>::new);
	}

	pub fn to_char_array(&self, start_index: i32, mut end_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> &[u16] {
		end_index = self.validate_range(start_index, end_index)?;
		/* final */ let len: i32 = end_index - start_index;
		if len == 0 {
			return ArrayUtils::EMPTY_CHAR_ARRAY;
		}
		return ArrayUtils.arraycopy(self.buffer, start_index, 0, len, Vec<char>::new);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::new(self.buffer, 0, self.size);
	}

	pub fn to_string_buffer(&self) -> /* Java */ java::lang::StringBuffer /**/ {
		return StringBuffer::new(self.size).append(self.buffer, 0, self.size);
	}

	pub fn to_string_builder(&self) -> /* Java */ java::lang::StringBuilder /**/ {
		return StringBuilder::new(self.size).append(self.buffer, 0, self.size);
	}

	pub fn trim(&self) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> org::apache::commons::lang3::text::str_builder::StrBuilder {
		if self.size == 0 {
			return self;
		}
		let len: i32 = self.size;
		/* final */ let buf: Vec<char> = self.buffer;
		let pos: i32 = 0;
		while pos < len && buf[pos] <= ' ' {
			pos += 1;
		}
		while pos < len && buf[len - 1] <= ' ' {
			len -= 1;
		}
		if len < self.size {
			self.delete(len, self.size)?;
		}
		if pos > 0 {
			self.delete(0, pos)?;
		}
		return self;
	}

	fn validate_index(&self, index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ {
		if index < 0 || index > self.size {
			return Err(StringIndexOutOfBoundsException::new(index));
		}
	}

	fn validate_range(&self, start_index: i32, mut end_index: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> i32 {
		if start_index < 0 {
			return Err(StringIndexOutOfBoundsException::new(start_index));
		}
		if end_index > self.size {
			end_index = self.size;
		}
		if start_index > end_index {
			return Err(StringIndexOutOfBoundsException::new("end < start"));
		}
		return end_index;
	}
}

impl /* Java */ java::lang::CharSequence /**/ for StrBuilder {}

impl /* Java */ java::lang::Appendable /**/ for StrBuilder {}

impl /* Java */ java::io::Serializable /**/ for StrBuilder {}

impl org::apache::commons::lang3::builder::builder::Builder for StrBuilder {}

struct StrBuilderReader {
	pos: i32,
	mark: i32,
}

impl StrBuilderReader {
	fn new() -> org::apache::commons::lang3::text::str_builder::StrBuilderReader {
	}

	pub fn close(&self) {
	// do nothing
	}

	pub fn mark(&mut self, read_ahead_limit: i32) {
		self.mark = self.pos;
	}

	pub fn mark_supported(&self) -> bool {
		return true;
	}

	pub fn read(&self) /* thrown(java.lang.StringIndexOutOfBoundsException) */ -> i32 {
		if !self.ready() {
			return -1;
		}
		return self.char_at(self.pos += 1 !!!check!!! post increment)?;
	}

	pub fn read(&mut self, b: &&[u16], off: i32, mut len: i32) /* thrown(java.lang.IndexOutOfBoundsException | java.lang.StringIndexOutOfBoundsException) */ -> i32 {
		if off < 0 || len < 0 || off > b.length || off + len > b.length || off + len < 0 {
			return Err(IndexOutOfBoundsException::new());
		}
		if len == 0 {
			return 0;
		}
		if self.pos >= self.size() {
			return -1;
		}
		if self.pos + len > self.size() {
			len = self.size() - self.pos;
		}
		StrBuilder.get_chars(self.pos, self.pos + len, b, off)?;
		self.pos += len;
		return len;
	}

	pub fn ready(&self) -> bool {
		return self.pos < self.size();
	}

	pub fn reset(&mut self) {
		self.pos = self.mark;
	}

	pub fn skip(&mut self, mut n: i64) -> i64 {
		if self.pos + n > self.size() {
			n = self.size() - self.pos;
		}
		if n < 0 {
			return 0;
		}
		self.pos = Math::addExact(self.pos, &Math::toIntExact(n));
		return n;
	}
}

impl /* Java */ java::lang::Readable /**/ for StrBuilderReader {}

impl /* Java */ java::io::Closeable /**/ for StrBuilderReader {}

impl /* Java */ java::lang::AutoCloseable /**/ for StrBuilderReader {}

struct StrBuilderTokenizer;

impl StrBuilderTokenizer {
	fn new() -> org::apache::commons::lang3::text::str_builder::StrBuilderTokenizer {
	}

	pub fn get_content(&self) -> /* Java */ java::lang::String /**/ {
		/* final */ let str: String = super.get_content();
		if str == null {
			return StrBuilder.to_string();
		}
		return str;
	}

	fn tokenize(&self, chars: &&[u16], offset: i32, count: i32) -> /* Java */ java::util::List /**/ {
		if chars == null {
			return super.tokenize(StrBuilder.buffer, 0, &StrBuilder.size());
		}
		return super.tokenize(chars, offset, count);
	}
}

impl /* Java */ java::util::ListIterator /**/ for StrBuilderTokenizer {}

impl /* Java */ java::util::Iterator /**/ for StrBuilderTokenizer {}

impl /* Java */ java::lang::Cloneable /**/ for StrBuilderTokenizer {}

struct StrBuilderWriter;

impl StrBuilderWriter {
	fn new() -> org::apache::commons::lang3::text::str_builder::StrBuilderWriter {
	}

	pub fn close(&self) {
	// do nothing
	}

	pub fn flush(&self) {
	// do nothing
	}

	pub fn write(&self, cbuf: &&[u16]) {
		StrBuilder.append(cbuf);
	}

	pub fn write(&self, cbuf: &&[u16], off: i32, len: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ {
		StrBuilder.append(cbuf, off, len)?;
	}

	pub fn write(&self, c: i32) {
		StrBuilder.append(c as char);
	}

	pub fn write(&self, str: &/* Java */ java::lang::String /**/) {
		StrBuilder.append(str);
	}

	pub fn write(&self, str: &/* Java */ java::lang::String /**/, off: i32, len: i32) /* thrown(java.lang.StringIndexOutOfBoundsException) */ {
		StrBuilder.append(str, off, len)?;
	}
}

impl /* Java */ java::lang::Appendable /**/ for StrBuilderWriter {}

impl /* Java */ java::io::Closeable /**/ for StrBuilderWriter {}

impl /* Java */ java::lang::AutoCloseable /**/ for StrBuilderWriter {}

impl /* Java */ java::io::Flushable /**/ for StrBuilderWriter {}