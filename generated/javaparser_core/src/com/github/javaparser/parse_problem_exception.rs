use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::Collections::singletonList;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::List;

pub struct ParseProblemException {
	problems: /* Java */ java::util::List /**/,
}

impl ParseProblemException {
	pub fn new(problems: &/* Java */ java::util::List /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_problem_exception::ParseProblemException {
		super(&com::github::javaparser::parse_problem_exception::ParseProblemException::create_message(&com::github::javaparser::utils::utils::Utils::assert_not_null(problems)?));
		self.problems = problems;
	}

	pub fn new(throwable: &/* Java */ java::lang::Throwable /**/) -> com::github::javaparser::parse_problem_exception::ParseProblemException {
		this(&/* Java */ java::util::Collections /**/::singletonList(Problem::new(&throwable.getMessage(), null, throwable)));
	}

	fn create_message(&self, problems: &/* Java */ java::util::List /**/) -> /* Java */ java::lang::String /**/ {
		let message: StringBuilder = StringBuilder::new();
		for problem in problems {
			message.append(&problem.to_string()).append(LineSeparator::SYSTEM);
		}
		return message.toString();
	}

	pub fn get_problems(&self) -> /* Java */ java::util::List /**/ {
		return self.problems;
	}
}

impl /* Java */ java::io::Serializable /**/ for ParseProblemException {}