use crate::com::github::javaparser::utils::LineSeparator;
use java::io::IOException;
use java::util::HashMap;
use java::util::Map;
use java::util::Optional;

pub struct LineEndingProcessingProvider {
	_input: com::github::javaparser::provider::Provider,
	_data: &[u16],
	_len: i32 = 0,
	_pos: i32 = 0,
	eol_counts: /* Java */ java::util::Map /**/ = HashMap<>::new(),
}

impl LineEndingProcessingProvider {
	static EOF: i32 = -1;

	static DEFAULT_BUFFER_SIZE: i32 = 2048;

	pub fn new(input: &com::github::javaparser::provider::Provider) -> com::github::javaparser::line_ending_processing_provider::LineEndingProcessingProvider {
		this(self.DEFAULT_BUFFER_SIZE, input);
	}

	pub fn new(buffer_size: i32, input: &com::github::javaparser::provider::Provider) -> com::github::javaparser::line_ending_processing_provider::LineEndingProcessingProvider {
		self._input = input;
		self._data = : [Option<char>; buffer_size] = [None; buffer_size];
	}

	pub fn close(&self) /* thrown(java.io.IOException) */ {
		self._input.close();
	}

	fn fill_buffer(&mut self) /* thrown(java.io.IOException) */ -> i32 {
		self._pos = 0;
		let direct: i32 = self._input.read(self._data, 0, self._data.length);
		if direct != 0 {
			self._len = direct;
		}
		return direct;
	}

	pub fn get_detected_line_ending(&self) -> com::github::javaparser::utils::line_separator::LineSeparator {
		return LineSeparator::get_line_ending(&self.eol_counts.getOrDefault(LineSeparator::CR, 0), &self.eol_counts.getOrDefault(LineSeparator::LF, 0), &self.eol_counts.getOrDefault(LineSeparator::CRLF, 0));
	}

	fn is_buffer_empty(&self) -> bool {
		return self._pos >= self._len;
	}

	fn next_buffered_char(&self) /* thrown(java.io.IOException) */ -> i32 {
		while self.is_buffer_empty() {
			let direct: i32 = self.fill_buffer()?;
			if direct < 0 {
				return self.EOF;
			}
		}
		return self._data[self._pos += 1 !!!check!!! post increment];
	}

	pub fn read(&self, mut buffer: &&[u16], offset: i32, len: i32) /* thrown(java.io.IOException) */ -> i32 {
		let pos: i32 = offset;
		let stop: i32 = offset + len;
		let previous_line_separator: LineSeparator = null;
		while pos < stop {
			let ch: i32 = self.next_buffered_char()?;
			if ch < 0 {
				if pos == offset {
					// Nothing read yet, this is the end of the stream.
					return self.EOF;
				}
				break;
			}
			let str: String = String::valueOf(ch as char);
			let lookup: Optional<LineSeparator> = LineSeparator::lookup(str);
			if lookup.isPresent() {
				let line_separator: LineSeparator = lookup.get();
				// Track the number of times this character is found..
				self.eol_counts.putIfAbsent(line_separator, 0);
				self.eol_counts.put(line_separator, self.eol_counts.get(line_separator) + 1);
				// line separator
				if line_separator == LineSeparator::LF {
					if previous_line_separator == LineSeparator::CR {
						self.eol_counts.putIfAbsent(LineSeparator::CRLF, 0);
						self.eol_counts.put(LineSeparator::CRLF, self.eol_counts.get(LineSeparator::CRLF) + 1);
					}
				}
				// If "this" (current) char <strong>is</strong> a line separator, set the next loop's "previous" to this
				previous_line_separator = line_separator;
			} else {
				// If "this" (current) char <strong>is not</strong> a line separator, set the next loop's "previous" to
				// null
				previous_line_separator = null;
			}
			buffer[pos += 1 !!!check!!! post increment] = ch as char;
		}
		return pos - offset;
	}
}

impl com::github::javaparser::provider::Provider for LineEndingProcessingProvider {}

impl /* Java */ java::io::Closeable /**/ for LineEndingProcessingProvider {}

impl /* Java */ java::lang::AutoCloseable /**/ for LineEndingProcessingProvider {}