pub struct ParseException {
	current_token: com::github::javaparser::token::Token,
	expected_token_sequences: &[&[i32]],
	token_image: &[/* Java */ java::lang::String /**/],
}

impl ParseException {
	static INDENT: /* Java */ java::lang::String /**/ = "    ";

	static EOL: /* Java */ java::lang::String /**/ = "\n";

	pub fn new(current_token_val: &com::github::javaparser::token::Token, expected_token_sequences_val: &&[&[i32]], token_image_val: &&[/* Java */ java::lang::String /**/]) -> com::github::javaparser::parse_exception::ParseException {
		this(current_token_val, expected_token_sequences_val, token_image_val, null);
	}

	pub fn new(current_token_val: &com::github::javaparser::token::Token, expected_token_sequences_val: &&[&[i32]], token_image_val: &&[/* Java */ java::lang::String /**/], lexical_state_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::parse_exception::ParseException {
		super(&com::github::javaparser::parse_exception::ParseException::_initialise(current_token_val, expected_token_sequences_val, token_image_val, lexical_state_name));
		self.current_token = current_token_val;
		self.expected_token_sequences = expected_token_sequences_val;
		self.token_image = token_image_val;
	}

	pub fn new() -> com::github::javaparser::parse_exception::ParseException {
		super();
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> com::github::javaparser::parse_exception::ParseException {
		super(message);
	}

	fn _initialise(&self, current_token: &com::github::javaparser::token::Token, expected_token_sequences: &&[&[i32]], token_image: &&[/* Java */ java::lang::String /**/], lexical_state_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let expected: StringBuilder = StringBuilder::new();
		let max_size: i32 = 0;
		let sorted_options: java.util.TreeSet<String> = java.util.TreeSet<String>::new();
		 {
			let i: i32 = 0;
			while i < expected_token_sequences.length {
				{
					if max_size < expected_token_sequences[i].length {
						max_size = expected_token_sequences[i].length;
					}
	
					 {
						let j: i32 = 0;
						while j < expected_token_sequences[i].length {
							{
								sorted_options.add(token_image[expected_token_sequences[i][j]]);
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		for option in sorted_options {
			expected.append(self.INDENT).append(option).append(self.EOL);
		}
		let sb: StringBuilder = StringBuilder::new();
		sb.append("Encountered unexpected token:");
		let tok: Token = current_token.next;
		 {
			let i: i32 = 0;
			while i < max_size {
				{
					let token_text: String = tok.image;
					let escaped_token_text: String = com::github::javaparser::parse_exception::ParseException::add_escapes(token_text);
					if i != 0 {
						sb.append(' ');
					}
	
					if tok.kind == 0 {
						sb.append(token_image[0]);
						break;
					}
					sb.append(" \"");
					sb.append(escaped_token_text);
					sb.append("\"");
					sb.append(" " + token_image[tok.kind]);
					tok = tok.next;
				}
				i += 1;
			 }
		 }
	
		sb.append(self.EOL).append(self.INDENT).append("at line ").append(current_token.next.beginLine).append(", column ").append(current_token.next.beginColumn);
		sb.append(".").append(self.EOL);
		if expected_token_sequences.length == 0 {
		// Nothing to add here
		} else {
			sb.append(self.EOL).append("Was expecting").append( if expected_token_sequences.length == 1 { ":" } else { " one of:" }).append(self.EOL).append(self.EOL).append(expected);
		}
		return sb.toString();
	}

	fn add_escapes(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let retval: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = 0;
			while i < str.length() {
				{
					/* final */ let ch: char = str.charAt(i);
					match ch {
						'\b' =>  {
							retval.append("\\b");
							continue;
						}
						'\t' =>  {
							retval.append("\\t");
							continue;
						}
						'\n' =>  {
							retval.append("\\n");
							continue;
						}
						'\f' =>  {
							retval.append("\\f");
							continue;
						}
						'\r' =>  {
							retval.append("\\r");
							continue;
						}
						'\"' =>  {
							retval.append("\\\"");
							continue;
						}
						'\'' =>  {
							retval.append("\\\'");
							continue;
						}
						'\\' =>  {
							retval.append("\\\\");
							continue;
						}
						_ =>  {
							if ch < 0x20 || ch > 0x7e {
								let s: String = "0000" + Integer::toString(ch, 16);
								retval.append("\\u" + s.substring(s.length() - 4, &s.length()));
							} else {
								retval.append(ch);
							}
							continue;
						}
					}
				}
				i += 1;
			 }
		 }
	
		return retval.toString();
	}
}

impl /* Java */ java::io::Serializable /**/ for ParseException {}