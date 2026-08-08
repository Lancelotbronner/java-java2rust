pub struct TokenMgrException {
	error_code: i32,
}

impl TokenMgrException {
	static serialVersionUID: i64 = 1;

	pub static LEXICAL_ERROR: i32 = 0;

	pub static STATIC_LEXER_ERROR: i32 = 1;

	pub static INVALID_LEXICAL_STATE: i32 = 2;

	pub static LOOP_DETECTED: i32 = 3;

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
								retval.append("\\u").append(&s.substring(s.length() - 4, &s.length()));
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

	fn lexical_err(&self, eof_seen: bool, lex_state: i32, error_line: i32, error_column: i32, error_after: &/* Java */ java::lang::String /**/, cur_char: i32) -> /* Java */ java::lang::String /**/ {
		let cur_char1: char = cur_char as char;
		return ("Lexical error at line " + error_line + ", column " + error_column + ".  Encountered: " + ( if eof_seen { "<EOF> " } else { ("\"" + com::github::javaparser::token_mgr_exception::TokenMgrException::add_escapes(&String::valueOf(cur_char1)) + "\"") + " (" + cur_char + "), " }) + "after : \"" + com::github::javaparser::token_mgr_exception::TokenMgrException::add_escapes(error_after) + "\"");
	}

	pub fn get_message(&self) -> /* Java */ java::lang::String /**/ {
		return super.getMessage();
	}

	pub fn new() -> com::github::javaparser::token_mgr_exception::TokenMgrException {
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, reason: i32) -> com::github::javaparser::token_mgr_exception::TokenMgrException {
		super(message);
		self.error_code = reason;
	}

	pub fn new(eof_seen: bool, lex_state: i32, error_line: i32, error_column: i32, error_after: &/* Java */ java::lang::String /**/, cur_char: i32, reason: i32) -> com::github::javaparser::token_mgr_exception::TokenMgrException {
		this(&com::github::javaparser::token_mgr_exception::TokenMgrException::lexical_err(eof_seen, lex_state, error_line, error_column, error_after, cur_char), reason);
	}
}

impl /* Java */ java::io::Serializable /**/ for TokenMgrException {}