pub struct UnsolvedSymbolException {
	name: /* Java */ java::lang::String /**/,
	context: /* Java */ java::lang::String /**/,
	cause: /* Java */ java::lang::Throwable /**/,
}

impl UnsolvedSymbolException {
	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::unsolved_symbol_exception::UnsolvedSymbolException {
		this(name, null, null);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, context: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::unsolved_symbol_exception::UnsolvedSymbolException {
		this(name, context, null);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> com::github::javaparser::resolution::unsolved_symbol_exception::UnsolvedSymbolException {
		this(name, null, cause);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, context: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> com::github::javaparser::resolution::unsolved_symbol_exception::UnsolvedSymbolException {
		super("Unsolved symbol" + ( if context != null { " in " + context } else { "" }) + " : " + name, cause);
		self.name = name;
		self.context = context;
		self.cause = cause;
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "UnsolvedSymbolException{" + "context='" + self.context + "'" + ", name='" + self.name + "'" + ", cause='" + self.cause + "'" + "}";
	}
}

impl /* Java */ java::io::Serializable /**/ for UnsolvedSymbolException {}