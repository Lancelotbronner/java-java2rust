pub struct AbstractCharStream {
	m_n_tab_size: i32 = 1,
	buffer: &[u16],
	bufsize: i32,
	bufpos: i32,
	available: i32,
	token_begin: i32,
	in_buf: i32,
	max_next_char_ind: i32,
	m_a_buf_line: &[i32],
	m_a_buf_column: &[i32],
	m_n_line_no: i32,
	m_n_column_no: i32,
	m_b_prev_char_iscr: bool,
	m_b_prev_char_islf: bool,
	m_b_track_line_column: bool = true,
}

impl AbstractCharStream {
	pub static DEFAULT_BUF_SIZE: i32 = 4096;

	fn hexval(&self, c: u16) /* thrown(java.io.IOException) */ -> i32 {
		match c {
			'0' =>  {
				return 0;
			}
			'1' =>  {
				return 1;
			}
			'2' =>  {
				return 2;
			}
			'3' =>  {
				return 3;
			}
			'4' =>  {
				return 4;
			}
			'5' =>  {
				return 5;
			}
			'6' =>  {
				return 6;
			}
			'7' =>  {
				return 7;
			}
			'8' =>  {
				return 8;
			}
			'9' =>  {
				return 9;
			}
			'a' =>  {
			}
			'A' =>  {
				return 10;
			}
			'b' =>  {
			}
			'B' =>  {
				return 11;
			}
			'c' =>  {
			}
			'C' =>  {
				return 12;
			}
			'd' =>  {
			}
			'D' =>  {
				return 13;
			}
			'e' =>  {
			}
			'E' =>  {
				return 14;
			}
			'f' =>  {
			}
			'F' =>  {
				return 15;
			}
			_ =>  {
				return Err(java.io.IOException::new("Invalid hex char '" + c + "' (=" + c as i32 + ") provided!"));
			}
		}
	}

	pub fn new(n_start_line: i32, n_start_column: i32, n_buffer_size: i32) -> com::github::javaparser::abstract_char_stream::AbstractCharStream {
		self.re_init(n_start_line, n_start_column, n_buffer_size);
	}

	pub fn re_init(&mut self, n_start_line: i32, n_start_column: i32, n_buffer_size: i32) {
		self.m_n_line_no = n_start_line;
		self.m_n_column_no = n_start_column - 1;
		self.m_b_prev_char_is_c_r = false;
		self.m_b_prev_char_is_l_f = false;
		if self.buffer == null || n_buffer_size != self.buffer.length {
			self.bufsize = n_buffer_size;
			self.available = n_buffer_size;
			self.buffer = : [Option<char>; n_buffer_size] = [None; n_buffer_size];
			self.m_a_buf_line = : [i32; n_buffer_size] = [0; n_buffer_size];
			self.m_a_buf_column = : [i32; n_buffer_size] = [0; n_buffer_size];
		}
		self.max_next_char_ind = 0;
		self.in_buf = 0;
		self.token_begin = 0;
		self.bufpos = -1;
	}

	fn stream_read(&self, a_buf: &&[u16], n_ofs: i32, n_len: i32) /* thrown(java.io.IOException) */ -> i32 ;

	fn stream_close(&self) /* thrown(java.io.IOException) */ ;

	fn get_buf_size_after_expansion(&self) -> i32 {
		// Double the size by default
		return self.bufsize * 2;
	}

	fn expand_buff(&mut self, b_wrap_around: bool) {
		// Get the new buffer size
		/* final */ let n_new_buf_size: i32 = self.get_buf_size_after_expansion();
		/* final */ let newbuffer: [Option<char>; n_new_buf_size] = [None; n_new_buf_size];
		/* final */ let newbufline: [i32; n_new_buf_size] = [0; n_new_buf_size];
		/* final */ let newbufcolumn: [i32; n_new_buf_size] = [0; n_new_buf_size];
		// Number of chars to be preserved
		/* final */ let n_preserved_chars: i32 = self.bufsize - self.token_begin;
		if b_wrap_around {
			// Move from offset "tokenBegin" to offset 0
			// arraycopy(src, srcPos, dest, destPos, length)
			// copy the "tail end" to the "start" (index 0) of the new buffer array 
			System::arraycopy(self.buffer, self.token_begin, newbuffer, 0, n_preserved_chars);
			// copy the remaining "wrap around" content of the buffer from the start of the original buffer (starting at srcPos index 0) 
			System::arraycopy(self.buffer, 0, newbuffer, n_preserved_chars, self.bufpos);
			// swap the new buffer in place of the old buffer
			self.buffer = newbuffer;
			System::arraycopy(self.m_a_buf_line, self.token_begin, newbufline, 0, n_preserved_chars);
			System::arraycopy(self.m_a_buf_line, 0, newbufline, n_preserved_chars, self.bufpos);
			self.m_a_buf_line = newbufline;
			System::arraycopy(self.m_a_buf_column, self.token_begin, newbufcolumn, 0, n_preserved_chars);
			System::arraycopy(self.m_a_buf_column, 0, newbufcolumn, n_preserved_chars, self.bufpos);
			self.m_a_buf_column = newbufcolumn;
			self.bufpos += n_preserved_chars;
			self.max_next_char_ind = self.bufpos;
		} else {
			// Move from offset "tokenBegin" to offset 0
			System::arraycopy(self.buffer, self.token_begin, newbuffer, 0, n_preserved_chars);
			self.buffer = newbuffer;
			System::arraycopy(self.m_a_buf_line, self.token_begin, newbufline, 0, n_preserved_chars);
			self.m_a_buf_line = newbufline;
			System::arraycopy(self.m_a_buf_column, self.token_begin, newbufcolumn, 0, n_preserved_chars);
			self.m_a_buf_column = newbufcolumn;
			self.bufpos -= self.token_begin;
			self.max_next_char_ind = self.bufpos;
		}
		// Increase buffer size
		self.bufsize = n_new_buf_size;
		self.available = n_new_buf_size;
		self.token_begin = 0;
	}

	fn internal_adjust_buff_size(&mut self) {
		/* final */ let n_half_buffer_size: i32 = self.bufsize / 2;
		if self.available == self.bufsize {
			if self.token_begin < 0 {
				// If this method is called from "beginToken()"
				// Just refill the buffer from the start
				self.bufpos = 0;
				self.max_next_char_ind = 0;
			} else if self.token_begin > n_half_buffer_size {
				// The token started in the second half - fill the front part 
				self.bufpos = 0;
				self.max_next_char_ind = 0;
				// Available bytes are > 50%
				self.available = self.token_begin;
			} else {
				// Token starts in the first half
				// just append to existing buffer
				self.expand_buff(false);
			}
		} else {
			// A token was read across array boundaries 
			if self.available > self.token_begin {
				self.available = self.bufsize;
			} else if (self.token_begin - self.available) < n_half_buffer_size {
				self.expand_buff(true);
			} else {
				self.available = self.token_begin;
			}
		}
	}

	fn fill_buff(&mut self) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ {
		if self.max_next_char_ind == self.available {
			self.internal_adjust_buff_size();
		}
	
		let r0 = 'try0: {
			// Read from underlying stream
			/* final */ let n_chars_read: i32 = match self.stream_read(self.buffer, self.max_next_char_ind, self.available - self.max_next_char_ind) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			if n_chars_read == -1 {
				// We reached the end of the file
				if let Err(e) = self.stream_close() {
					return Err(e);
				};
				// Caught down below and re-thrown
				break 'try0 Err(java.io.IOException::new("PGCC end of stream"));
			}
			self.max_next_char_ind += n_chars_read;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ java.io.IOException) => {
				self.bufpos -= 1;
				// ?What is the reason of this? Backup of 0 does nothing
				self.backup(0)?;
				if self.token_begin == -1 {
					// Error occurred in "beginToken()"
					self.token_begin = self.bufpos;
				}
				return Err(ex);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn internal_set_buf_line_column(&mut self, n_line: i32, n_column: i32) {
		self.m_a_buf_line[self.bufpos] = n_line;
		self.m_a_buf_column[self.bufpos] = n_column;
	}

	fn internal_update_line_column(&mut self, c: u16) {
		self.m_n_column_no += 1;
		if self.m_b_prev_char_is_l_f {
			// It's a "\r\n" or "\n"
			// Start of a new line
			self.m_b_prev_char_is_l_f = false;
			self.m_n_column_no = 1;
			self.m_n_line_no += 1;
		} else if self.m_b_prev_char_is_c_r {
			self.m_b_prev_char_is_c_r = false;
			if c == '\n' {
				// It's a "\r\n"
				self.m_b_prev_char_is_l_f = true;
			} else {
				// It's only a "\r"
				self.m_n_column_no = 1;
				self.m_n_line_no += 1;
			}
		}
		match c {
			'\r' =>  {
				self.m_b_prev_char_is_c_r = true;
				break;
			}
			'\n' =>  {
				self.m_b_prev_char_is_l_f = true;
				break;
			}
			'\t' =>  {
				self.m_n_column_no -= 1;
				self.m_n_column_no += (self.m_n_tab_size - (self.m_n_column_no % self.m_n_tab_size));
				break;
			}
		}
		self.internal_set_buf_line_column(self.m_n_line_no, self.m_n_column_no);
	}

	pub fn read_char(&mut self) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> u16 {
		if self.in_buf > 0 {
			// Something is left from last backup
			self.in_buf -= 1;
			self.bufpos += 1;
			if self.bufpos == self.bufsize {
				// Buffer overflow
				self.bufpos = 0;
			}
			return self.buffer[self.bufpos];
		}
		self.bufpos += 1;
		if self.bufpos >= self.max_next_char_ind {
			self.fill_buff()?;
		}
	
		/* final */ let c: char = self.buffer[self.bufpos];
		if self.m_b_track_line_column {
			self.internal_update_line_column(c);
		}
	
		return c;
	}

	pub fn begin_token(&mut self) /* thrown(java.io.IOException) */ -> u16 {
		self.token_begin = -1;
		/* final */ let c: char = self.read_char()?;
		self.token_begin = self.bufpos;
		return c;
	}

	pub fn get_begin_column(&self) -> i32 {
		return self.m_a_buf_column[self.token_begin];
	}

	pub fn get_begin_line(&self) -> i32 {
		return self.m_a_buf_line[self.token_begin];
	}

	pub fn get_end_column(&self) -> i32 {
		return self.m_a_buf_column[self.bufpos];
	}

	pub fn get_end_line(&self) -> i32 {
		return self.m_a_buf_line[self.bufpos];
	}

	pub fn backup(&mut self, n_amount: i32) /* thrown(java.lang.IllegalStateException) */ {
		if n_amount > self.bufsize {
			return Err(IllegalStateException::new("Cannot back " + n_amount + " chars which is larger than the internal buffer size (" + self.bufsize + ")"));
		}
	
		self.in_buf += n_amount;
		self.bufpos -= n_amount;
		if self.bufpos < 0 {
			// Buffer underflow (modulo)
			self.bufpos += self.bufsize;
		}
	}

	pub fn get_image(&self) -> /* Java */ java::lang::String /**/ {
		if self.bufpos >= self.token_begin {
			// from tokenBegin to bufpos
			return String::new(self.buffer, self.token_begin, self.bufpos - self.token_begin + 1);
		}
		// from tokenBegin to bufsize, and from 0 to bufpos
		return String::new(self.buffer, self.token_begin, self.bufsize - self.token_begin) + String::new(self.buffer, 0, self.bufpos + 1);
	}

	pub fn get_suffix(&self, len: i32) -> &[u16] {
		let ret: [Option<char>; len] = [None; len];
		if (self.bufpos + 1) >= len {
			// one piece
			System::arraycopy(self.buffer, self.bufpos - len + 1, ret, 0, len);
		} else {
			// Wrap around
			/* final */ let n_part1: i32 = len - self.bufpos - 1;
			System::arraycopy(self.buffer, self.bufsize - n_part1, ret, 0, n_part1);
			System::arraycopy(self.buffer, 0, ret, n_part1, self.bufpos + 1);
		}
		return ret;
	}

	pub fn done(&mut self) {
		self.buffer = null;
		self.m_a_buf_line = null;
		self.m_a_buf_column = null;
	}

	pub fn get_tab_size(&self) -> i32 {
		return self.m_n_tab_size;
	}

	pub fn set_tab_size(&mut self, n_tab_size: i32) {
		self.m_n_tab_size = n_tab_size;
	}

	pub fn adjust_begin_line_column(&mut self, n_new_line: i32, new_col: i32) {
		let start: i32 = self.token_begin;
		let new_line: i32 = n_new_line;
		let len: i32;
		if self.bufpos >= self.token_begin {
			len = self.bufpos - self.token_begin + self.in_buf + 1;
		} else {
			len = self.bufsize - self.token_begin + self.bufpos + 1 + self.in_buf;
		}
		let i: i32 = 0;
		let j: i32 = 0;
		let k: i32 = 0;
		let next_col_diff: i32 = 0;
		let column_diff: i32 = 0;
		// TODO disassemble meaning and split up
		while i < len && self.m_a_buf_line[j = start % self.bufsize] == self.m_a_buf_line[k = start += 1 % self.bufsize] {
			self.m_a_buf_line[j] = new_line;
			next_col_diff = column_diff + self.m_a_buf_column[k] - self.m_a_buf_column[j];
			self.m_a_buf_column[j] = new_col + column_diff;
			column_diff = next_col_diff;
			i += 1;
		}
		if i < len {
			self.m_a_buf_line[j] = new_line += 1 !!!check!!! post increment;
			self.m_a_buf_column[j] = new_col + column_diff;
			while i += 1 !!!check!!! post increment < len {
				// TODO disassemble meaning and split up
				if self.m_a_buf_line[j = start % self.bufsize] != self.m_a_buf_line[start += 1 % self.bufsize] {
					self.m_a_buf_line[j] = new_line += 1 !!!check!!! post increment;
				}
				else {self.m_a_buf_line[j] = new_line;
				}
	
			}
		}
		self.m_n_line_no = self.m_a_buf_line[j];
		self.m_n_column_no = self.m_a_buf_column[j];
	}

	fn get_line(&self) -> i32 {
		return self.m_n_line_no;
	}

	fn get_column(&self) -> i32 {
		return self.m_n_column_no;
	}

	pub fn is_track_line_column(&self) -> bool {
		return self.m_b_track_line_column;
	}

	pub fn set_track_line_column(&mut self, b_track_line_column: bool) {
		self.m_b_track_line_column = b_track_line_column;
	}
}

impl com::github::javaparser::char_stream::CharStream for AbstractCharStream {}