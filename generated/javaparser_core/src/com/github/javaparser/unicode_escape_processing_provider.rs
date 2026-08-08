use java::io::IOException;
use java::util::ArrayList;
use java::util::Collections;
use java::util::List;

pub struct UnicodeEscapeProcessingProvider {
	_data: &[u16],
	_len: i32 = 0,
	_pos: i32 = 0,
	_backslash_seen: bool,
	_input_line: com::github::javaparser::unicode_escape_processing_provider::LineCounter = LineCounter::new(),
	_output_line: com::github::javaparser::unicode_escape_processing_provider::LineCounter = LineCounter::new(),
	_mapping_builder: com::github::javaparser::unicode_escape_processing_provider::PositionMappingBuilder = PositionMappingBuilder::new(_outputLine, _inputLine),
	_input: com::github::javaparser::provider::Provider,
}

impl UnicodeEscapeProcessingProvider {
	static LF: u16 = '\n';

	static CR: u16 = '\r';

	static BACKSLASH: u16 = '\\';

	static EOF: i32 = -1;

	pub fn new(input: &com::github::javaparser::provider::Provider) -> com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider {
		this(2048, input);
	}

	pub fn new(buffer_size: i32, input: &com::github::javaparser::provider::Provider) -> com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider {
		self._input = input;
		self._data = : [Option<char>; buffer_size] = [None; buffer_size];
	}

	pub fn get_input_counter(&self) -> com::github::javaparser::unicode_escape_processing_provider::LineCounter {
		return self._inputLine;
	}

	pub fn get_output_counter(&self) -> com::github::javaparser::unicode_escape_processing_provider::LineCounter {
		return self._outputLine;
	}

	pub fn read(&self, mut buffer: &&[u16], offset: i32, len: i32) /* thrown(java.io.IOException) */ -> i32 {
		let pos: i32 = offset;
		let stop: i32 = offset + len;
		while pos < stop {
			let ch: i32 = self._outputLine.process(&self.next_output_char());
			if ch < 0 {
				if pos == offset {
					// Nothing read yet, this is the end of the stream.
					return self.EOF;
				}
				break;
			}
			self._mappingBuilder.update();
			buffer[pos += 1 !!!check!!! post increment] = ch as char;
		}
		return pos - offset;
	}

	pub fn close(&self) /* thrown(java.io.IOException) */ {
		self._input.close();
	}

	fn next_output_char(&self) /* thrown(java.io.IOException) */ -> i32 {
		let next: i32 = self.next_input_char();
		match next {
			self.EOF =>  {
				return self.EOF;
			}
			self.BACKSLASH =>  {
				{
					if self._backslashSeen {
						return self.clear_back_slash_seen(next);
					}
					return self.back_slash_seen();
				}
			}
			_ =>  {
				{
					// An arbitrary character.
					return self.clear_back_slash_seen(next);
				}
			}
		}
	}

	fn clear_back_slash_seen(&mut self, next: i32) -> i32 {
		self._backslashSeen = false;
		return next;
	}

	fn back_slash_seen(&mut self) /* thrown(java.io.IOException) */ -> i32 {
		self._backslashSeen = true;
		let next: i32 = self.next_input_char();
		match next {
			self.EOF =>  {
				// End of file after backslash produces the backslash itself.
				return self.BACKSLASH;
			}
			'u' =>  {
				{
					return self.unicode_start_seen();
				}
			}
			_ =>  {
				{
					self.push_back(next);
					return self.BACKSLASH;
				}
			}
		}
	}

	fn unicode_start_seen(&self) /* thrown(java.io.IOException) */ -> i32 {
		let u_cnt: i32 = 1;
		while true {
			let next: i32 = self.next_input_char();
			match next {
				self.EOF =>  {
					{
						self.push_back_us(u_cnt);
						return self.BACKSLASH;
					}
				}
				'u' =>  {
					{
						u_cnt += 1;
						continue;
					}
				}
				_ =>  {
					{
						return self.read_digits(u_cnt, next);
					}
				}
			}
		}
	}

	fn read_digits(&self, u_cnt: i32, next3: i32) /* thrown(java.io.IOException) */ -> i32 {
		let digit3: i32 = com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider::digit(next3);
		if digit3 < 0 {
			self.push_back(next3);
			self.push_back_us(u_cnt);
			return self.BACKSLASH;
		}
		let next2: i32 = self.next_input_char();
		let digit2: i32 = com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider::digit(next2);
		if digit2 < 0 {
			self.push_back(next2);
			self.push_back(next3);
			self.push_back_us(u_cnt);
			return self.BACKSLASH;
		}
		let next1: i32 = self.next_input_char();
		let digit1: i32 = com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider::digit(next1);
		if digit1 < 0 {
			self.push_back(next1);
			self.push_back(next2);
			self.push_back(next3);
			self.push_back_us(u_cnt);
			return self.BACKSLASH;
		}
		let next0: i32 = self.next_input_char();
		let digit0: i32 = com::github::javaparser::unicode_escape_processing_provider::UnicodeEscapeProcessingProvider::digit(next0);
		if digit0 < 0 {
			self.push_back(next0);
			self.push_back(next1);
			self.push_back(next2);
			self.push_back(next3);
			self.push_back_us(u_cnt);
			return self.BACKSLASH;
		}
		let ch: i32 = digit3 << 12 | digit2 << 8 | digit1 << 4 | digit0;
		return self.clear_back_slash_seen(ch);
	}

	fn push_back_us(&self, cnt: i32) {
		 {
			let n: i32 = 0;
			while n < cnt {
				{
					self.push_back('u');
				}
				n += 1;
			 }
		 }
	
	}

	fn digit(&self, ch: i32) -> i32 {
		if ch >= '0' && ch <= '9' {
			return ch - '0';
		}
		if ch >= 'A' && ch <= 'F' {
			return 10 + ch - 'A';
		}
		if ch >= 'a' && ch <= 'f' {
			return 10 + ch - 'a';
		}
		return -1;
	}

	fn next_input_char(&self) /* thrown(java.io.IOException) */ -> i32 {
		let result: i32 = self.next_buffered_char();
		return self._inputLine.process(result);
	}

	fn next_buffered_char(&self) /* thrown(java.io.IOException) */ -> i32 {
		while self.is_buffer_empty() {
			let direct: i32 = self.fill_buffer();
			if direct < 0 {
				return self.EOF;
			}
		}
		return self._data[self._pos += 1 !!!check!!! post increment];
	}

	fn is_buffer_empty(&self) -> bool {
		return self._pos >= self._len;
	}

	fn fill_buffer(&mut self) /* thrown(java.io.IOException) */ -> i32 {
		self._pos = 0;
		let direct: i32 = self._input.read(self._data, 0, self._data.length);
		if direct != 0 {
			self._len = direct;
		}
		return direct;
	}

	fn push_back(&mut self, ch: i32) {
		if ch < 0 {
			return;
		}
		if self.is_buffer_empty() {
			self._pos = self._data.length;
			self._len = self._data.length;
		} else if self._pos == 0 {
			if self._len == self._data.length {
				// Buffer is completely full, no push possible, enlarge buffer.
				let new_data: [Option<char>; self._data.length + 1024] = [None; self._data.length + 1024];
				self._len = new_data.length;
				self._pos = new_data.length - self._data.length;
				System::arraycopy(self._data, 0, new_data, self._pos, self._data.length);
				self._data = new_data;
			} else {
				// Move contents to the right.
				let cnt: i32 = self._len - self._pos;
				self._pos = self._data.length - self._len;
				self._len = self._data.length;
				System::arraycopy(self._data, 0, self._data, self._pos, cnt);
			}
		}
		self._data[self._pos -= 1] = ch as char;
	}

	pub fn get_position_mapping(&self) -> com::github::javaparser::unicode_escape_processing_provider::PositionMapping {
		return self._mappingBuilder.get_mapping();
	}
}

impl com::github::javaparser::provider::Provider for UnicodeEscapeProcessingProvider {}

impl /* Java */ java::io::Closeable /**/ for UnicodeEscapeProcessingProvider {}

impl /* Java */ java::lang::AutoCloseable /**/ for UnicodeEscapeProcessingProvider {}

pub struct PositionMapping {
	_deltas: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl PositionMapping {
	pub fn new() -> com::github::javaparser::unicode_escape_processing_provider::PositionMapping {
		super();
	}

	pub fn is_empty(&self) -> bool {
		return self._deltas.isEmpty();
	}

	fn add(&self, line: i32, column: i32, line_delta: i32, column_delta: i32) {
		self._deltas.add(DeltaInfo::new(line, column, line_delta, column_delta));
	}

	pub fn lookup(&self, position: &com::github::javaparser::position::Position) -> com::github::javaparser::unicode_escape_processing_provider::PositionUpdate {
		let result: i32 = Collections::binarySearch(self._deltas, position);
		if result >= 0 {
			return self._deltas.get(result);
		}
		let insert_index: i32 = -result - 1;
		if insert_index == 0 {
			// Before the first delta info, identity mapping.
			return PositionUpdate.com::github::javaparser::unicode_escape_processing_provider::PositionUpdate::NONE;
		}
		return self._deltas.get(insert_index - 1);
	}

	pub fn transform(&self, pos: &com::github::javaparser::position::Position) -> com::github::javaparser::position::Position {
		return self.lookup(pos).transform(pos);
	}

	pub fn transform(&self, range: &com::github::javaparser::range::Range) -> com::github::javaparser::range::Range {
		let begin: Position = self.transform(range.begin);
		let end: Position = self.transform(range.end);
		if begin == range.begin && end == range.end {
			// No change.
			return range;
		}
		return Range::new(begin, end);
	}
}

pub trait PositionUpdate;

struct DeltaInfo {
	_line_delta: i32,
	_column_delta: i32,
}

impl DeltaInfo {
	pub fn new(line: i32, column: i32, line_delta: i32, column_delta: i32) -> com::github::javaparser::unicode_escape_processing_provider::DeltaInfo {
		super(line, column);
		self._lineDelta = line_delta;
		self._columnDelta = column_delta;
	}

	pub fn transform_line(&self, source_line: i32) -> i32 {
		return source_line + self._lineDelta;
	}

	pub fn transform_column(&self, source_column: i32) -> i32 {
		return source_column + self._columnDelta;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "(" +  + ", " +  + ": " + self._lineDelta + ", " + self._columnDelta + ")";
	}
}

impl com::github::javaparser::unicode_escape_processing_provider::PositionUpdate for DeltaInfo {}

impl /* Java */ java::lang::Comparable /**/ for DeltaInfo {}

struct PositionMappingBuilder {
	_left: com::github::javaparser::unicode_escape_processing_provider::LineCounter,
	_right: com::github::javaparser::unicode_escape_processing_provider::LineCounter,
	_mapping: com::github::javaparser::unicode_escape_processing_provider::PositionMapping = PositionMapping::new(),
	_line_delta: i32 = 0,
	_column_delta: i32 = 0,
}

impl PositionMappingBuilder {
	pub fn new(left: &com::github::javaparser::unicode_escape_processing_provider::LineCounter, right: &com::github::javaparser::unicode_escape_processing_provider::LineCounter) -> com::github::javaparser::unicode_escape_processing_provider::PositionMappingBuilder {
		self._left = left;
		self._right = right;
		self.update();
	}

	pub fn get_mapping(&self) -> com::github::javaparser::unicode_escape_processing_provider::PositionMapping {
		return self._mapping;
	}

	pub fn update(&mut self) {
		let line_delta: i32 = self._right.get_line() - self._left.get_line();
		let column_delta: i32 = self._right.get_column() - self._left.get_column();
		if line_delta != self._lineDelta || column_delta != self._columnDelta {
			self._mapping.add(&self._left.get_line(), &self._left.get_column(), line_delta, column_delta);
			self._lineDelta = line_delta;
			self._columnDelta = column_delta;
		}
	}
}

pub struct LineCounter {
	_cr_seen: bool,
	_line: i32 = 1,
	_column: i32 = 1,
}

impl LineCounter {
	pub fn new() -> com::github::javaparser::unicode_escape_processing_provider::LineCounter {
		super();
	}

	pub fn get_line(&self) -> i32 {
		return self._line;
	}

	pub fn get_column(&self) -> i32 {
		return self._column;
	}

	pub fn get_position(&self) -> com::github::javaparser::position::Position {
		return Position::new(&self.get_line(), &self.get_column());
	}

	pub fn process(&mut self, ch: i32) -> i32 {
		match ch {
			 =>  {
				{
					break;
				}
			}
			 =>  {
				{
					self.inc_line();
					self._crSeen = true;
					break;
				}
			}
			 =>  {
				{
					// CR LF does only count as a single line terminator.
					if self._crSeen {
						self._crSeen = false;
					} else {
						self.inc_line();
					}
					break;
				}
			}
			_ =>  {
				{
					self._crSeen = false;
					self._column += 1;
				}
			}
		}
		return ch;
	}

	fn inc_line(&mut self) {
		self._line += 1;
		self._column = 1;
	}
}