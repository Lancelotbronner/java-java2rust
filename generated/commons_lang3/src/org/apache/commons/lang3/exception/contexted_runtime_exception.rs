use java::util::List;
use java::util::Set;
use crate::org::apache::commons::lang3::tuple::Pair;

pub struct ContextedRuntimeException {
	exception_context: org::apache::commons::lang3::exception::exception_context::ExceptionContext,
}

impl ContextedRuntimeException {
	static serialVersionUID: i64 = 20110706;

	pub fn new() -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		self.exception_context = DefaultExceptionContext::new();
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		super(message);
		self.exception_context = DefaultExceptionContext::new();
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		super(message, cause);
		self.exception_context = DefaultExceptionContext::new();
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/, mut context: &org::apache::commons::lang3::exception::exception_context::ExceptionContext) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		super(message, cause);
		if context == null {
			context = DefaultExceptionContext::new();
		}
		self.exception_context = context;
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		super(cause);
		self.exception_context = DefaultExceptionContext::new();
	}

	pub fn add_context_value(&self, label: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		self.exception_context.add_context_value(label, value);
		return self;
	}

	pub fn get_context_entries(&self) -> /* Java */ java::util::List /**/ {
		return self.exceptionContext.get_context_entries();
	}

	pub fn get_context_labels(&self) -> /* Java */ java::util::Set /**/ {
		return self.exception_context.get_context_labels();
	}

	pub fn get_context_values(&self, label: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::List /**/ {
		return self.exceptionContext.get_context_values(label);
	}

	pub fn get_first_context_value(&self, label: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Object /**/ {
		return self.exceptionContext.get_first_context_value(label);
	}

	pub fn get_formatted_exception_message(&self, base_message: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.exception_context.get_formatted_exception_message(base_message);
	}

	pub fn get_message(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_formatted_exception_message(&super.getMessage());
	}

	pub fn get_raw_message(&self) -> /* Java */ java::lang::String /**/ {
		return super.getMessage();
	}

	pub fn set_context_value(&self, label: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::exception::contexted_runtime_exception::ContextedRuntimeException {
		self.exception_context.set_context_value(label, value);
		return self;
	}
}

impl org::apache::commons::lang3::exception::exception_context::ExceptionContext for ContextedRuntimeException {}

impl /* Java */ java::io::Serializable /**/ for ContextedRuntimeException {}