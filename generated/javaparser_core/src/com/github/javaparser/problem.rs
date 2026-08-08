use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::Comparator;
use java::util::Optional;

pub struct Problem {
	message: /* Java */ java::lang::String /**/,
	location: com::github::javaparser::token_range::TokenRange,
	cause: /* Java */ java::lang::Throwable /**/,
}

impl Problem {
	pub static PROBLEM_BY_BEGIN_POSITION: /* Java */ java::util::Comparator /**/ = |(a, b)|{
		/* final */ let a_begin: Optional<Position> = a.get_location().flatMap(|l|l.get_begin().get_range().map(|r|r.begin));
		/* final */ let b_begin: Optional<Position> = b.get_location().flatMap(|l|l.get_begin().get_range().map(|r|r.begin));
		if a_begin.isPresent() && b_begin.isPresent() {
			return a_begin.get().compare_to(&b_begin.get())?;
		}
		if a.get_location().isPresent() || b.get_location().isPresent() {
			if a.get_location().isPresent() {
				return 1;
			}
			return -1;
		}
		return 0;
	};

	pub fn new(message: &/* Java */ java::lang::String /**/, location: &com::github::javaparser::token_range::TokenRange, cause: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::problem::Problem {
		com::github::javaparser::utils::utils::Utils::assert_not_null(message)?;
		self.message = message;
		self.location = location;
		self.cause = cause;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		/* final */ let str: StringBuilder = StringBuilder::new(&self.get_verbose_message());
		if self.cause != null {
			str.append(LineSeparator::SYSTEM).append("Problem stacktrace : ").append(LineSeparator::SYSTEM);
			 {
				let i: i32 = 0;
				while i < self.cause.getStackTrace().length {
					{
						let ste: StackTraceElement = self.cause.getStackTrace()[i];
						str.append("  ").append(&ste.toString());
						if i + 1 != self.cause.getStackTrace().length {
							str.append(LineSeparator::SYSTEM);
						}
	
					}
					i += 1;
				 }
			 }
	
		}
		return str.toString();
	}

	pub fn get_message(&self) -> /* Java */ java::lang::String /**/ {
		return self.message;
	}

	pub fn get_verbose_message(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_location().map(|l|l.get_begin().get_range().map(|r|r.begin.to_string()).orElse("(line ?,col ?)") + " " + self.message).orElse(self.message);
	}

	pub fn get_location(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.location);
	}

	pub fn get_cause(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.cause);
	}
}