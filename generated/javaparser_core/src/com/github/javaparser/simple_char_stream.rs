pub struct SimpleCharStream {
	input_stream: com::github::javaparser::provider::Provider,
}

impl SimpleCharStream {
	fn stream_read(&self, a_buf: &&[u16], n_ofs: i32, n_len: i32) /* thrown(java.io.IOException) */ -> i32 {
		return self.input_stream.read(a_buf, n_ofs, n_len)?;
	}

	fn stream_close(&self) /* thrown(java.io.IOException) */ {
		self.input_stream.close();
	}

	pub fn new(dstream: &com::github::javaparser::provider::Provider, startline: i32, startcolumn: i32, buffersize: i32) -> com::github::javaparser::simple_char_stream::SimpleCharStream {
		super(startline, startcolumn, buffersize);
		self.input_stream = dstream;
	}

	pub fn new(dstream: &com::github::javaparser::provider::Provider, startline: i32, startcolumn: i32) -> com::github::javaparser::simple_char_stream::SimpleCharStream {
		this(dstream, startline, startcolumn, );
	}

	pub fn new(dstream: &com::github::javaparser::provider::Provider) -> com::github::javaparser::simple_char_stream::SimpleCharStream {
		this(dstream, 1, 1, );
	}

	pub fn re_init(&mut self, dstream: &com::github::javaparser::provider::Provider, startline: i32, startcolumn: i32, buffersize: i32) {
		self.input_stream = dstream;
		super.re_init(startline, startcolumn, buffersize);
	}

	pub fn re_init(&self, dstream: &com::github::javaparser::provider::Provider, startline: i32, startcolumn: i32) {
		self.re_init(dstream, startline, startcolumn, );
	}

	pub fn re_init(&self, dstream: &com::github::javaparser::provider::Provider) {
		self.re_init(dstream, 1, 1, );
	}
}

impl com::github::javaparser::char_stream::CharStream for SimpleCharStream {}