use java::io::ByteArrayOutputStream;
use java::io::IOException;
use java::io::ObjectInputStream;
use java::io::ObjectOutputStream;
use java::io::Serializable;
use java::lang::reflect::InvocationHandler;
use java::lang::reflect::InvocationTargetException;
use java::lang::reflect::Method;
use java::lang::reflect::Proxy;
use java::util::ArrayList;
use java::util::List;
use java::util::Objects;
use java::util::concurrent::CopyOnWriteArrayList;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::Validate;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;
use crate::org::apache::commons::lang3::function::FailableConsumer;

pub struct EventListenerSupport<L> {
	listeners: /* Java */ java::util::List /**/ = CopyOnWriteArrayList<>::new(),
	proxy: L,
	prototype_array: &[L],
}

impl<L> EventListenerSupport {
	static serialVersionUID: i64 = 3593265990380473632;

	pub fn create<T>(&self, listener_interface: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::event::event_listener_support::EventListenerSupport {
		return EventListenerSupport<>::new(listener_interface);
	}

	fn new() -> org::apache::commons::lang3::event::event_listener_support::EventListenerSupport {
	}

	pub fn new(listener_interface: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::event::event_listener_support::EventListenerSupport {
		this(listener_interface, &Thread::currentThread().getContextClassLoader());
	}

	pub fn new(listener_interface: &/* Java */ java::lang::Class /**/, class_loader: &/* Java */ java::lang::ClassLoader /**/) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::event::event_listener_support::EventListenerSupport {
		this();
		Objects::requireNonNull(listener_interface, "listenerInterface");
		Objects::requireNonNull(class_loader, "classLoader");
		Validate::is_true(&listener_interface.isInterface(), "Class %s is not an interface", &listener_interface.getName())?;
		self.initialize_transient_fields(listener_interface, class_loader);
	}

	pub fn add_listener(&self, listener: &L) {
		self.add_listener(listener, true);
	}

	pub fn add_listener(&self, listener: &L, allow_duplicate: bool) {
		Objects::requireNonNull(listener, "listener");
		if allow_duplicate || !self.listeners.contains(listener) {
			self.listeners.add(listener);
		}
	}

	fn create_invocation_handler(&self) -> /* Java */ java::lang::reflect::InvocationHandler /**/ {
		return ProxyInvocationHandler::new();
	}

	fn create_proxy(&mut self, listener_interface: &/* Java */ java::lang::Class /**/, class_loader: &/* Java */ java::lang::ClassLoader /**/) {
		self.proxy = listener_interface.cast(&Proxy::newProxyInstance(class_loader, : [Option<Class>; ] = [None; ], &self.create_invocation_handler()));
	}

	pub fn fire(&self) -> L {
		return self.proxy;
	}

	fn get_listener_count(&self) -> i32 {
		return self.listeners.size();
	}

	pub fn get_listeners(&self) -> &[L] {
		return self.listeners.toArray(self.prototype_array);
	}

	fn initialize_transient_fields(&mut self, listener_interface: &/* Java */ java::lang::Class /**/, class_loader: &/* Java */ java::lang::ClassLoader /**/) {
		// Will throw CCE here if not correct
		self.prototypeArray = ArrayUtils::new_instance(listener_interface, 0);
		self.create_proxy(listener_interface, class_loader);
	}

	fn read_object(&mut self, object_input_stream: &/* Java */ java::io::ObjectInputStream /**/) /* thrown(java.lang.ClassNotFoundException | java.io.IOException) */ {
		/* final */ let src_listeners: Vec<L> = object_input_stream.readObject() as Vec<L>;
		self.listeners = CopyOnWriteArrayList<>::new(src_listeners);
		/* final */ let listener_interface: Class<L> = ArrayUtils::get_component_type(src_listeners);
		self.initialize_transient_fields(listener_interface, &Thread::currentThread().getContextClassLoader());
	}

	pub fn remove_listener(&self, listener: &L) {
		self.listeners.remove(&Objects::requireNonNull(listener, "listener"));
	}

	fn write_object(&self, object_output_stream: &/* Java */ java::io::ObjectOutputStream /**/) /* thrown(java.io.IOException) */ {
		/* final */ let serializable_listeners: ArrayList<L> = ArrayList<>::new();
		// Don't just rely on instanceof Serializable:
		let test_object_output_stream: ObjectOutputStream = ObjectOutputStream::new(ByteArrayOutputStream::new());
		for /* final */ listener in self.listeners {
			let r0 = 'try0: {
				test_object_output_stream.writeObject(listener);
				serializable_listeners.add(listener);
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ IOException) => {
					//recreate test stream in case of indeterminate state
					test_object_output_stream = ObjectOutputStream::new(ByteArrayOutputStream::new());
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		// We can reconstitute everything we need from an array of our listeners,
		// which has the additional advantage of typically requiring less storage than a list:
		object_output_stream.writeObject(&serializable_listeners.toArray(self.prototype_array));
	}
}

impl<L> /* Java */ java::io::Serializable /**/ for EventListenerSupport<L> {}

struct ProxyInvocationHandler {
	handler: org::apache::commons::lang3::function::failable_consumer::FailableConsumer,
}

impl ProxyInvocationHandler {
	pub fn new() -> org::apache::commons::lang3::event::event_listener_support::ProxyInvocationHandler {
		this(ExceptionUtils::rethrow);
	}

	pub fn new(handler: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> org::apache::commons::lang3::event::event_listener_support::ProxyInvocationHandler {
		self.handler = Objects::requireNonNull(handler);
	}

	fn handle(&self, t: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException | java.lang.reflect.InvocationTargetException) */ {
		self.handler.accept(t);
	}

	pub fn invoke(&self, unused_proxy: &/* Java */ java::lang::Object /**/, method: &/* Java */ java::lang::reflect::Method /**/, args: &&[/* Java */ java::lang::Object /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		for /* final */ listener in  {
			let r0 = 'try0: {
				method.invoke(listener, args);
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					if let Err(e) = self.handle(t) {
						return Err(e);
					};
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		return null;
	}
}

impl /* Java */ java::lang::reflect::InvocationHandler /**/ for ProxyInvocationHandler {}