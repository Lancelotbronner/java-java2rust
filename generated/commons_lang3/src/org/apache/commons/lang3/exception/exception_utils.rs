use java::io::PrintStream;
use java::io::PrintWriter;
use java::io::StringWriter;
use java::lang::reflect::Method;
use java::lang::reflect::UndeclaredThrowableException;
use java::util::ArrayList;
use java::util::Collections;
use java::util::List;
use java::util::Objects;
use java::util::StringTokenizer;
use java::util::function::Consumer;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::reflect::MethodUtils;
use crate::org::apache::commons::lang3::util::IterableStringTokenizer;

pub struct ExceptionUtils;

impl ExceptionUtils {
	static CAUSE_METHOD_NAMES: &[/* Java */ java::lang::String /**/] = vec!["getCause", "getNextException", "getTargetException", "getException", "getSourceException", "getRootCause", "getCausedByException", "getNested", "getLinkedException", "getNestedException", "getLinkedCause", "getThrowable", ]
	;

	static NOT_FOUND: i32 = -1;

	static WRAPPED_MARKER: /* Java */ java::lang::String /**/ = " [wrapped] ";

	pub fn as_runtime_exception<T: /* Java */ java::lang::RuntimeException /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(T) */ -> T {
		// claim that the typeErasure invocation throws a RuntimeException
		return ExceptionUtils<T, RuntimeException>::erase_type(throwable)?;
	}

	fn erase_type<R, T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(T) */ -> R {
		return Err(throwable as T);
	}

	pub fn for_each(&self, throwable: &/* Java */ java::lang::Throwable /**/, consumer: &/* Java */ java::util::function::Consumer /**/) {
		org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::stream(throwable).forEach(consumer);
	}

	pub fn get_cause(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::lang::Throwable /**/ {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_cause(throwable, null);
	}

	pub fn get_cause(&self, throwable: &/* Java */ java::lang::Throwable /**/, mut method_names: &&[/* Java */ java::lang::String /**/]) -> /* Java */ java::lang::Throwable /**/ {
		if throwable == null {
			return null;
		}
		if method_names == null {
			/* final */ let cause: Throwable = throwable.getCause();
			if cause != null {
				return cause;
			}
			method_names = self.CAUSE_METHOD_NAMES;
		}
		return Stream::of(method_names).map(|m|org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_cause_using_method_name(throwable, m)).filter(Objects::nonNull).findFirst().orElse(null);
	}

	fn get_cause_using_method_name(&self, throwable: &/* Java */ java::lang::Throwable /**/, method_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Throwable /**/ {
		if method_name != null {
			/* final */ let method: Method = MethodUtils::get_method_object(&throwable.getClass(), method_name);
			if method != null && Throwable.class.isAssignableFrom(&method.getReturnType()) {
				let r0 = 'try0: {
					return method.invoke(throwable) as Throwable;
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ ReflectiveOperationException) => {
					// exception ignored
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		return null;
	}

	pub fn get_default_cause_method_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return ArrayUtils::clone(self.CAUSE_METHOD_NAMES);
	}

	pub fn get_message(&self, th: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::lang::String /**/ {
		if th == null {
			return StringUtils::EMPTY;
		}
		/* final */ let cls_name: String = ClassUtils::get_short_class_name(th, null);
		return cls_name + ": " + StringUtils::default_string(&th.getMessage());
	}

	pub fn get_root_cause(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::lang::Throwable /**/ {
		/* final */ let list: List<Throwable> = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwable_list(throwable);
		return  if list.isEmpty() { null } else { list.get(list.size() - 1) };
	}

	pub fn get_root_cause_message(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let root: Throwable = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_root_cause(throwable);
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_message( if root == null { throwable } else { root });
	}

	pub fn get_root_cause_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_root_cause_stack_trace_list(throwable).toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn get_root_cause_stack_trace_list(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::util::List /**/ {
		if throwable == null {
			return Collections::emptyList();
		}
		/* final */ let throwables: Vec<Throwable> = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwables(throwable);
		/* final */ let count: i32 = throwables.length;
		/* final */ let frames: List<String> = ArrayList<>::new();
		let next_trace: List<String> = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_stack_frame_list(throwables[count - 1]);
		 {
			let i: i32 = count;
			while i -= 1 >= 0{
				/* final */ let trace: List<String> = next_trace;
				if i != 0 {
					next_trace = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_stack_frame_list(throwables[i - 1]);
					org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::remove_common_frames(trace, next_trace);
				}
				if i == count - 1 {
					frames.add(&throwables[i].toString());
				} else {
					frames.add(self.WRAPPED_MARKER + throwables[i].toString());
				}
				frames.addAll(trace);
			}
		 }
	
		return frames;
	}

	fn get_stack_frame_list(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::util::List /**/ {
		/* final */ let stack_trace: String = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_stack_trace(throwable);
		/* final */ let linebreak: String = System::lineSeparator();
		/* final */ let frames: StringTokenizer = StringTokenizer::new(stack_trace, linebreak);
		/* final */ let list: List<String> = ArrayList<>::new();
		let trace_started: bool = false;
		while frames.hasMoreTokens() {
			/* final */ let token: String = frames.nextToken();
			// Determine if the line starts with "<whitespace>at"
			/* final */ let at: i32 = token.indexOf("at");
			if at != self.NOT_FOUND && token.substring(0, at).trim().isEmpty() {
				trace_started = true;
				list.add(token);
			} else if trace_started {
				break;
			}
		}
		return list;
	}

	fn get_stack_frames(&self, stack_trace: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return IterableStringTokenizer::new(stack_trace, &System::lineSeparator()).to_array();
	}

	pub fn get_stack_frames(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> &[/* Java */ java::lang::String /**/] {
		if throwable == null {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_stack_frames(&org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_stack_trace(throwable));
	}

	pub fn get_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::lang::String /**/ {
		if throwable == null {
			return StringUtils::EMPTY;
		}
		/* final */ let sw: StringWriter = StringWriter::new();
		throwable.printStackTrace(PrintWriter::new(sw, true));
		return sw.toString();
	}

	pub fn get_throwable_count(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> i32 {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwable_list(throwable).size();
	}

	pub fn get_throwable_list(&self, mut throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::util::List /**/ {
		/* final */ let list: List<Throwable> = ArrayList<>::new();
		while throwable != null && !list.contains(throwable) {
			list.add(throwable);
			throwable = throwable.getCause();
		}
		return list;
	}

	pub fn get_throwables(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> &[/* Java */ java::lang::Throwable /**/] {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwable_list(throwable).toArray(ArrayUtils::EMPTY_THROWABLE_ARRAY);
	}

	pub fn has_cause(&self, mut chain: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/) -> bool {
		if chain instanceof UndeclaredThrowableException {
			chain = chain.getCause();
		}
		return type.isInstance(chain);
	}

	fn index_of(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/, mut from_index: i32, subclass: bool) -> i32 {
		if throwable == null || type == null {
			return self.NOT_FOUND;
		}
		if from_index < 0 {
			from_index = 0;
		}
		/* final */ let throwables: Vec<Throwable> = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwables(throwable);
		if from_index >= throwables.length {
			return self.NOT_FOUND;
		}
		if subclass {
			 {
				let i: i32 = from_index;
				while i < throwables.length {
					{
						if type.isAssignableFrom(&throwables[i].getClass()) {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
		} else {
			 {
				let i: i32 = from_index;
				while i < throwables.length {
					{
						if type.equals(&throwables[i].getClass()) {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return self.NOT_FOUND;
	}

	pub fn index_of_throwable(&self, throwable: &/* Java */ java::lang::Throwable /**/, clazz: &/* Java */ java::lang::Class /**/) -> i32 {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::index_of(throwable, clazz, 0, false);
	}

	pub fn index_of_throwable(&self, throwable: &/* Java */ java::lang::Throwable /**/, clazz: &/* Java */ java::lang::Class /**/, from_index: i32) -> i32 {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::index_of(throwable, clazz, from_index, false);
	}

	pub fn index_of_type(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/) -> i32 {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::index_of(throwable, type, 0, true);
	}

	pub fn index_of_type(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/, from_index: i32) -> i32 {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::index_of(throwable, type, from_index, true);
	}

	pub fn is_checked(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> bool {
		return throwable != null && !(throwable instanceof Error) && !(throwable instanceof RuntimeException);
	}

	pub fn is_unchecked(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> bool {
		return throwable != null && (throwable instanceof Error || throwable instanceof RuntimeException);
	}

	pub fn print_root_cause_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/) {
		org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::print_root_cause_stack_trace(throwable, System::err);
	}

	pub fn print_root_cause_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/, print_stream: &/* Java */ java::io::PrintStream /**/) {
		if throwable == null {
			return;
		}
		Objects::requireNonNull(print_stream, "printStream");
		org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_root_cause_stack_trace_list(throwable).forEach(printStream::println);
		print_stream.flush();
	}

	pub fn print_root_cause_stack_trace(&self, throwable: &/* Java */ java::lang::Throwable /**/, print_writer: &/* Java */ java::io::PrintWriter /**/) {
		if throwable == null {
			return;
		}
		Objects::requireNonNull(print_writer, "printWriter");
		org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_root_cause_stack_trace_list(throwable).forEach(printWriter::println);
		print_writer.flush();
	}

	pub fn remove_common_frames(&self, cause_frames: &/* Java */ java::util::List /**/, wrapper_frames: &/* Java */ java::util::List /**/) {
		Objects::requireNonNull(cause_frames, "causeFrames");
		Objects::requireNonNull(wrapper_frames, "wrapperFrames");
		let cause_frame_index: i32 = cause_frames.size() - 1;
		let wrapper_frame_index: i32 = wrapper_frames.size() - 1;
		while cause_frame_index >= 0 && wrapper_frame_index >= 0 {
			// Remove the frame from the cause trace if it is the same
			// as in the wrapper trace
			/* final */ let cause_frame: String = cause_frames.get(cause_frame_index);
			/* final */ let wrapper_frame: String = wrapper_frames.get(wrapper_frame_index);
			if cause_frame.equals(wrapper_frame) {
				cause_frames.remove(cause_frame_index);
			}
			cause_frame_index -= 1;
			wrapper_frame_index -= 1;
		}
	}

	pub fn rethrow<T>(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(T) */ -> T {
		// claim that the typeErasure invocation throws a RuntimeException
		return ExceptionUtils<T, RuntimeException>::erase_type(throwable)?;
	}

	pub fn stream(&self, throwable: &/* Java */ java::lang::Throwable /**/) -> /* Java */ java::util::stream::Stream /**/ {
		// No point building a custom Iterable as it would keep track of visited elements to avoid infinite loops
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwable_list(throwable).stream();
	}

	fn throwable_of<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/, mut from_index: i32, subclass: bool) -> T {
		if throwable == null || type == null {
			return null;
		}
		if from_index < 0 {
			from_index = 0;
		}
		/* final */ let throwables: Vec<Throwable> = org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::get_throwables(throwable);
		if from_index >= throwables.length {
			return null;
		}
		if subclass {
			 {
				let i: i32 = from_index;
				while i < throwables.length {
					{
						if type.isAssignableFrom(&throwables[i].getClass()) {
							return type.cast(throwables[i]);
						}
					}
					i += 1;
				 }
			 }
	
		} else {
			 {
				let i: i32 = from_index;
				while i < throwables.length {
					{
						if type.equals(&throwables[i].getClass()) {
							return type.cast(throwables[i]);
						}
					}
					i += 1;
				 }
			 }
	
		}
		return null;
	}

	pub fn throwable_of_throwable<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/, clazz: &/* Java */ java::lang::Class /**/) -> T {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::throwable_of(throwable, clazz, 0, false);
	}

	pub fn throwable_of_throwable<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/, clazz: &/* Java */ java::lang::Class /**/, from_index: i32) -> T {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::throwable_of(throwable, clazz, from_index, false);
	}

	pub fn throwable_of_type<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/) -> T {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::throwable_of(throwable, type, 0, true);
	}

	pub fn throwable_of_type<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &/* Java */ java::lang::Throwable /**/, type: &/* Java */ java::lang::Class /**/, from_index: i32) -> T {
		return org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::throwable_of(throwable, type, from_index, true);
	}

	pub fn throw_unchecked<T>(&self, throwable: &T) /* thrown(java.lang.Error | java.lang.RuntimeException) */ -> T {
		if throwable instanceof RuntimeException {
			return Err(throwable as RuntimeException);
		}
		if throwable instanceof Error {
			return Err(throwable as Error);
		}
		return throwable;
	}

	pub fn throw_unchecked<T: /* Java */ java::lang::Throwable /**/>(&self, throwable: &T) /* thrown(T | java.lang.Throwable) */ -> T {
		if org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::is_unchecked(throwable) {
			return Err(org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::as_runtime_exception(throwable)?);
		}
		return throwable;
	}

	pub fn wrap_and_throw<R>(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(T | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ -> R {
		return Err(UndeclaredThrowableException::new(&org::apache::commons::lang3::exception::exception_utils::ExceptionUtils::throw_unchecked(throwable)?));
	}

	pub fn new() -> org::apache::commons::lang3::exception::exception_utils::ExceptionUtils {
	// empty
	}
}