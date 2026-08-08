use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use java::io::IOException;
use java::io::PrintWriter;
use java::io::StringWriter;
use java::util::function::Supplier;

pub struct Log;

impl Log {
	static CURRENT_ADAPTER: com::github::javaparser::utils::log::Adapter = SilentAdapter::new();

	pub fn set_adapter(&mut self, adapter: &com::github::javaparser::utils::log::Adapter) {
		self.CURRENT_ADAPTER = adapter;
	}

	pub fn trace(&self, format: &/* Java */ java::lang::String /**/, args: &/* Java */ java::util::function::Supplier /**/) {
		self.CURRENT_ADAPTER.trace(&com::github::javaparser::utils::log::Log::make_formatting_supplier(format, args));
	}

	fn make_formatting_supplier(&self, format: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::util::function::Supplier /**/]) -> /* Java */ java::util::function::Supplier /**/ {
		return |()|{
			let objects: [Option<Object>; args.length] = [None; args.length];
			 {
				let i: i32 = 0;
				while i < args.length {
					{
						objects[i] = args[i].get();
					}
					i += 1;
				 }
			 }
	
			return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f(format, objects);
		};
	}

	pub fn info(&self, format: &/* Java */ java::lang::String /**/, args: &/* Java */ java::util::function::Supplier /**/) {
		self.CURRENT_ADAPTER.info(&com::github::javaparser::utils::log::Log::make_formatting_supplier(format, args));
	}

	pub fn error(&self, throwable: &/* Java */ java::lang::Throwable /**/) {
		self.CURRENT_ADAPTER.error(|()|throwable, |()|null);
	}

	pub fn error(&self, throwable: &/* Java */ java::lang::Throwable /**/, format: &/* Java */ java::lang::String /**/, args: &/* Java */ java::util::function::Supplier /**/) {
		self.CURRENT_ADAPTER.error(|()|throwable, &com::github::javaparser::utils::log::Log::make_formatting_supplier(format, args));
	}

	pub fn error(&self, format: &/* Java */ java::lang::String /**/, args: &/* Java */ java::util::function::Supplier /**/) {
		self.CURRENT_ADAPTER.error(|()|null, &com::github::javaparser::utils::log::Log::make_formatting_supplier(format, args));
	}
}

pub struct StandardOutStandardErrorAdapter;

impl StandardOutStandardErrorAdapter {
	pub fn info(&self, message_supplier: &/* Java */ java::util::function::Supplier /**/) {
		System::out.println(&message_supplier.get());
	}

	pub fn trace(&self, message_supplier: &/* Java */ java::util::function::Supplier /**/) {
		System::out.println(&message_supplier.get());
	}

	pub fn error(&self, throwable_supplier: &/* Java */ java::util::function::Supplier /**/, message_supplier: &/* Java */ java::util::function::Supplier /**/) /* thrown(java.lang.AssertionError) */ {
		let throwable: Throwable = throwable_supplier.get();
		let message: String = message_supplier.get();
		if message == null {
			System::err.println(&throwable.getMessage());
			self.print_stack_trace(throwable)?;
		} else if throwable == null {
			System::err.println(message);
		} else {
			System::err.println(message + ":" + throwable.getMessage());
			self.print_stack_trace(throwable)?;
		}
	}

	fn print_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.AssertionError) */ {
		let r0 = 'try0: {
			(let sw: StringWriter = StringWriter::new();
				let pw: PrintWriter = PrintWriter::new(sw)) throwable.printStackTrace(pw);
			self.trace(sw::toString);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				break 'try0 Err(AssertionError::new("Error in logging library"));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}

impl com::github::javaparser::utils::log::Adapter for StandardOutStandardErrorAdapter {}

pub struct SilentAdapter;

impl SilentAdapter {
	pub fn info(&self, message_supplier: &/* Java */ java::util::function::Supplier /**/) {
	}

	pub fn trace(&self, message_supplier: &/* Java */ java::util::function::Supplier /**/) {
	}

	pub fn error(&self, throwable_supplier: &/* Java */ java::util::function::Supplier /**/, message_supplier: &/* Java */ java::util::function::Supplier /**/) {
	}
}

impl com::github::javaparser::utils::log::Adapter for SilentAdapter {}

pub trait Adapter;