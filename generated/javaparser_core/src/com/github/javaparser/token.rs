pub struct Token {
	kind: i32,
	begin_line: i32,
	begin_column: i32,
	end_line: i32,
	end_column: i32,
	image: /* Java */ java::lang::String /**/,
	next: com::github::javaparser::token::Token,
	special_token: com::github::javaparser::token::Token,
}

impl Token {
	static serialVersionUID: i64 = 1;

	pub fn new() -> com::github::javaparser::token::Token {
	}

	pub fn new(n_kind: i32) -> com::github::javaparser::token::Token {
		this(n_kind, null);
	}

	pub fn new(n_kind: i32, s_image: &/* Java */ java::lang::String /**/) -> com::github::javaparser::token::Token {
		self.kind = n_kind;
		self.image = s_image;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Object /**/ {
		return null;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.image;
	}

	pub fn new_token(&self, of_kind: i32, image: &/* Java */ java::lang::String /**/) -> com::github::javaparser::token::Token {
		match of_kind {
			_ =>  {
				return Token::new(of_kind, image);
			}
		}
	}

	pub fn new_token(&self, of_kind: i32) -> com::github::javaparser::token::Token {
		return com::github::javaparser::token::Token::new_token(of_kind, null);
	}
}

impl /* Java */ java::io::Serializable /**/ for Token {}