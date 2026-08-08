use java::io::BufferedReader;
use java::io::IOException;
use java::io::InputStream;
use java::io::InputStreamReader;
use java::io::Reader;

pub struct StreamProvider {
	m_a_reader: /* Java */ java::io::Reader /**/,
}

impl StreamProvider {
	pub fn new(stream: &/* Java */ java::io::InputStream /**/, charset_name: &/* Java */ java::lang::String /**/) /* thrown(java.io.IOException) */ -> com::github::javaparser::stream_provider::StreamProvider {
		this(BufferedReader::new(InputStreamReader::new(stream, charset_name)));
	}

	pub fn new(stream: &/* Java */ java::io::InputStream /**/, charset: &/* Java */ java::nio::charset::Charset /**/) -> com::github::javaparser::stream_provider::StreamProvider {
		this(BufferedReader::new(InputStreamReader::new(stream, charset)));
	}

	pub fn new(reader: &/* Java */ java::io::Reader /**/) -> com::github::javaparser::stream_provider::StreamProvider {
		self.m_a_reader = reader;
	}

	pub fn read(&self, a_dest: &&[u16], n_ofs: i32, n_len: i32) /* thrown(java.io.IOException) */ -> i32 {
		let result: i32 = self.m_a_reader.read(a_dest, n_ofs, n_len);
		/*  CBA -- Added 2014/03/29 -- 
	       This logic allows the generated Java code to be easily translated to C# (via sharpen) -
	       as in C# 0 represents end of file, and in Java, -1 represents end of file
	       See : http://msdn.microsoft.com/en-us/library/9kstw824(v=vs.110).aspx
	       ** Technically, this is not required for java but the overhead is extremely low compared to the code generation benefits.
		   */ 
		if result == 0 {
			if n_ofs < a_dest.length && n_len > 0 {
				result = -1;
			}
	
		}
	
		return result;
	}

	pub fn close(&self) /* thrown(java.io.IOException) */ {
		if self.m_a_reader != null {
			self.m_a_reader.close();
		}
	
	}
}

impl com::github::javaparser::provider::Provider for StreamProvider {}

impl /* Java */ java::io::Closeable /**/ for StreamProvider {}

impl /* Java */ java::lang::AutoCloseable /**/ for StreamProvider {}