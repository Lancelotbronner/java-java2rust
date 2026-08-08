pub struct Preconditions;

impl Preconditions {
	fn new() -> com::github::javaparser::quality::preconditions::Preconditions {
	// This constructor hide the public one.
	}

	pub fn check_argument(&self, expression: bool, message: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(&String::valueOf(message)));
		}
	}

	pub fn check_argument(&self, expression: bool) /* thrown(java.lang.IllegalArgumentException) */ {
		com::github::javaparser::quality::preconditions::Preconditions::check_argument(expression, "Invalid argument provided.")?;
	}

	pub fn check_not_null(&self, reference: &/* Java */ java::lang::Object /**/, message: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		com::github::javaparser::quality::preconditions::Preconditions::check_argument(reference != null, message)?;
	}

	pub fn check_not_null(&self, reference: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		com::github::javaparser::quality::preconditions::Preconditions::check_not_null(reference, "A null value is not allowed here.")?;
	}
}