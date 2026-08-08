pub struct MethodAmbiguityException;

impl MethodAmbiguityException {
	pub fn new(description: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::method_ambiguity_exception::MethodAmbiguityException {
		super(description);
	}
}

impl /* Java */ java::io::Serializable /**/ for MethodAmbiguityException {}