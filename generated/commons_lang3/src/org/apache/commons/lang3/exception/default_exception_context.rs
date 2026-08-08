use java::io::Serializable;
use java::util::ArrayList;
use java::util::List;
use java::util::Objects;
use java::util::Set;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::Strings;
use crate::org::apache::commons::lang3::tuple::ImmutablePair;
use crate::org::apache::commons::lang3::tuple::Pair;

pub struct DefaultExceptionContext {
	context_values: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl DefaultExceptionContext {
	static serialVersionUID: i64 = 20110706;

	pub fn new() -> org::apache::commons::lang3::exception::default_exception_context::DefaultExceptionContext {
	// empty
	}

	pub fn add_context_value(&self, label: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::exception::default_exception_context::DefaultExceptionContext {
		self.context_values.add(ImmutablePair<>::new(label, value));
		return self;
	}

	pub fn get_context_entries(&self) -> /* Java */ java::util::List /**/ {
		return self.context_values;
	}

	pub fn get_context_labels(&self) -> /* Java */ java::util::Set /**/ {
		return self.stream().map(Pair::getKey).collect(&Collectors::toSet());
	}

	pub fn get_context_values(&self, label: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::List /**/ {
		return self.stream().filter(|pair|Strings::org::apache::commons::lang3::strings::Strings::CS.equals(label, &pair.get_key())).map(Pair::getValue).collect(&Collectors::toList());
	}

	pub fn get_first_context_value(&self, label: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Object /**/ {
		return self.stream().filter(|pair|Strings::org::apache::commons::lang3::strings::Strings::CS.equals(label, &pair.get_key())).findFirst().map(Pair::getValue).orElse(null);
	}

	pub fn get_formatted_exception_message(&self, base_message: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let buffer: StringBuilder = StringBuilder::new(256);
		if base_message != null {
			buffer.append(base_message);
		}
		if !self.context_values.isEmpty() {
			if buffer.length() > 0 {
				buffer.append('\n');
			}
			buffer.append("Exception Context:\n");
			let i: i32 = 0;
			for /* final */ pair in self.context_values {
				buffer.append("\t[");
				buffer.append(i += 1);
				buffer.append(':');
				buffer.append(&pair.get_key());
				buffer.append("=");
				/* final */ let value: Object = pair.get_value();
				let r0 = 'try0: {
					buffer.append(&Objects::toString(value));
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ Exception) => {
						buffer.append("Exception thrown on toString(): ");
						buffer.append(&ExceptionUtils::get_stack_trace(e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
				buffer.append("]\n");
			}
			buffer.append("---------------------------------");
		}
		return buffer.toString();
	}

	pub fn set_context_value(&self, label: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::exception::default_exception_context::DefaultExceptionContext {
		self.context_values.removeIf(|p|Strings::org::apache::commons::lang3::strings::Strings::CS.equals(label, &p.get_key()));
		self.add_context_value(label, value);
		return self;
	}

	fn stream(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return self.context_values.stream();
	}
}

impl org::apache::commons::lang3::exception::exception_context::ExceptionContext for DefaultExceptionContext {}

impl /* Java */ java::io::Serializable /**/ for DefaultExceptionContext {}