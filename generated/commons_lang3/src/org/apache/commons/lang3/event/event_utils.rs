use java::lang::reflect::InvocationHandler;
use java::lang::reflect::Method;
use java::lang::reflect::Proxy;
use java::util::Arrays;
use java::util::HashSet;
use java::util::Set;
use crate::org::apache::commons::lang3::reflect::MethodUtils;

pub struct EventUtils;

impl EventUtils {
	pub fn add_event_listener<L>(&self, event_source: &/* Java */ java::lang::Object /**/, listener_type: &/* Java */ java::lang::Class /**/, listener: &L) /* thrown(java.lang.IllegalArgumentException) */ {
		let r0 = 'try0: {
			MethodUtils::invoke_method(event_source, "add" + listener_type.getSimpleName(), listener);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ReflectiveOperationException) => {
				break 'try0 Err(IllegalArgumentException::new("Unable to add listener for class " + event_source.getClass().getName() + " and public add" + listener_type.getSimpleName() + " method which takes a parameter of type " + listener_type.getName() + "."));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn bind_events_to_method<L>(&self, target: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, event_source: &/* Java */ java::lang::Object /**/, listener_type: &/* Java */ java::lang::Class /**/, event_types: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		/* final */ let listener: L = listener_type.cast(&Proxy::newProxyInstance(&target.getClass().getClassLoader(), : [Option<Class>; ] = [None; ], EventBindingInvocationHandler::new(target, method_name, event_types)));
		org::apache::commons::lang3::event::event_utils::EventUtils::add_event_listener(event_source, listener_type, listener)?;
	}

	pub fn new() -> org::apache::commons::lang3::event::event_utils::EventUtils {
	// empty
	}
}

struct EventBindingInvocationHandler {
	target: /* Java */ java::lang::Object /**/,
	method_name: /* Java */ java::lang::String /**/,
	event_types: /* Java */ java::util::Set /**/,
}

impl EventBindingInvocationHandler {
	fn new(target: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, event_types: &&[/* Java */ java::lang::String /**/]) -> org::apache::commons::lang3::event::event_utils::EventBindingInvocationHandler {
		self.target = target;
		self.methodName = method_name;
		self.eventTypes = HashSet<>::new(&Arrays::asList(event_types));
	}

	fn has_matching_parameters_method(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> bool {
		return MethodUtils::get_accessible_method(&self.target.getClass(), self.method_name, &method.getParameterTypes()) != null;
	}

	pub fn invoke(&self, proxy: &/* Java */ java::lang::Object /**/, method: &/* Java */ java::lang::reflect::Method /**/, parameters: &&[/* Java */ java::lang::Object /**/]) /* thrown(java.lang.Throwable) */ -> /* Java */ java::lang::Object /**/ {
		if self.event_types.isEmpty() || self.event_types.contains(&method.getName()) {
			if self.has_matching_parameters_method(method) {
				return MethodUtils::invoke_method(self.target, self.method_name, parameters);
			}
			return MethodUtils::invoke_method(self.target, self.method_name);
		}
		return null;
	}
}

impl /* Java */ java::lang::reflect::InvocationHandler /**/ for EventBindingInvocationHandler {}