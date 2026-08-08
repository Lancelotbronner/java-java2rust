use crate::com::github::javaparser::ast::comments::CommentsCollection;
use crate::com::github::javaparser::utils::LineSeparator;
use java::nio::file::Path;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ParseResult<T> {
	result: T,
	problems: /* Java */ java::util::List /**/,
	comments_collection: com::github::javaparser::ast::comments::comments_collection::CommentsCollection,
	source_path: /* Java */ java::nio::file::Path /**/,
}

impl<T> ParseResult {
	pub fn new(result: &T, problems: &/* Java */ java::util::List /**/, comments_collection: &com::github::javaparser::ast::comments::comments_collection::CommentsCollection) -> com::github::javaparser::parse_result::ParseResult {
		self.commentsCollection = comments_collection;
		self.result = result;
		self.problems = problems;
	}

	pub fn is_successful(&self) -> bool {
		return self.problems.isEmpty() && self.result != null;
	}

	pub fn if_successful(&self, consumer: &/* Java */ java::util::function::Consumer /**/) {
		if self.is_successful() {
			consumer.accept(self.result);
		}
	}

	pub fn set_source_path(&mut self, source_path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::parse_result::ParseResult {
		self.sourcePath = source_path;
		return self;
	}

	pub fn get_problems(&self) -> /* Java */ java::util::List /**/ {
		return self.problems;
	}

	pub fn get_problem(&self, i: i32) -> com::github::javaparser::problem::Problem {
		return self.get_problems().get(i);
	}

	pub fn get_comments_collection(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.comments_collection);
	}

	pub fn get_result(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.result);
	}

	pub fn get_source_path(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.sourcePath);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		if self.is_successful() {
			return "Parsing successful";
		}
		let message: StringBuilder = StringBuilder::new("Parsing failed");
		if self.source_path != null {
			message.append(" for ").append(self.source_path);
		}
		message.append(":").append(LineSeparator::SYSTEM);
		for problem in self.problems {
			message.append(&problem.to_string()).append(LineSeparator::SYSTEM);
		}
		return message.toString();
	}
}