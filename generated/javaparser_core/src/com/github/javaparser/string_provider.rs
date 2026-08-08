use java::io::IOException;

pub struct StringProvider {
	m_s_str: /* Java */ java::lang::String /**/,
	m_n_pos: i32 = 0,
	m_n_len: i32,
}

impl StringProvider {
	pub fn new(s_str: &/* Java */ java::lang::String /**/) -> com::github::javaparser::string_provider::StringProvider {
		self.m_s_str = s_str;
		self.m_n_len = s_str.length();
	}

	pub fn read(&mut self, a_dest: &&[u16], n_ofs: i32, n_len: i32) /* thrown(java.io.IOException) */ -> i32 {
		/* final */ let n_left: i32 = self.m_n_len - self.m_n_pos;
		if n_left <= 0 {
			return -1;
		}
	
		let n_chars_read: i32 = a_dest.length - n_ofs;
		if n_len < n_chars_read {
			n_chars_read = n_len;
		}
	
		if n_left < n_chars_read {
			n_chars_read = n_left;
		}
	
		self.m_s_str.getChars(self.m_n_pos, self.m_n_pos + n_chars_read, a_dest, n_ofs);
		self.m_n_pos += n_chars_read;
		return n_chars_read;
	}

	pub fn close(&mut self) {
		self.m_s_str = null;
	}
}

impl com::github::javaparser::provider::Provider for StringProvider {}

impl /* Java */ java::io::Closeable /**/ for StringProvider {}

impl /* Java */ java::lang::AutoCloseable /**/ for StringProvider {}