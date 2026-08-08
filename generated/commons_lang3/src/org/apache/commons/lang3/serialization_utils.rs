use java::io::ByteArrayInputStream;
use java::io::ByteArrayOutputStream;
use java::io::IOException;
use java::io::InputStream;
use java::io::ObjectInputStream;
use java::io::ObjectOutputStream;
use java::io::ObjectStreamClass;
use java::io::OutputStream;
use java::io::Serializable;
use java::util::Objects;

pub struct SerializationUtils;

impl SerializationUtils {
	pub fn clone<T: /* Java */ java::io::Serializable /**/>(&self, object: &T) /* thrown(org.apache.commons.lang3.SerializationException) */ -> T {
		if object == null {
			return null;
		}
		/* final */ let bais: ByteArrayInputStream = ByteArrayInputStream::new(&org::apache::commons::lang3::serialization_utils::SerializationUtils::serialize(object));
		/* final */ let cls: Class<T> = ObjectUtils::get_class(object);
		let r0 = 'try0: {
			(let in: ClassLoaderAwareObjectInputStream = ClassLoaderAwareObjectInputStream::new(bais, &cls.getClassLoader())) // same type as the original serialized object
			return in.readObject() as T;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ClassNotFoundExceptionIOException | ) => {
				break 'try0 Err(SerializationException::new(&String::format("%s while reading cloned object data", &ex.getClass().getSimpleName()), ex));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn deserialize<T>(&self, object_data: &&[i8]) /* thrown(org.apache.commons.lang3.SerializationException) */ -> T {
		Objects::requireNonNull(object_data, "objectData");
		return org::apache::commons::lang3::serialization_utils::SerializationUtils::deserialize(ByteArrayInputStream::new(object_data))?;
	}

	pub fn deserialize<T>(&self, input_stream: &/* Java */ java::io::InputStream /**/) /* thrown(org.apache.commons.lang3.SerializationException) */ -> T {
		Objects::requireNonNull(input_stream, "inputStream");
		let r0 = 'try0: {
			(let in: ObjectInputStream = ObjectInputStream::new(input_stream)) /* final */ let obj: T = in.readObject() as T;
			return obj;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ClassNotFoundExceptionIOException | NegativeArraySizeException | ) => {
				break 'try0 Err(SerializationException::new(ex));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn roundtrip<T: /* Java */ java::io::Serializable /**/>(&self, obj: &T) /* thrown(org.apache.commons.lang3.SerializationException) */ -> T {
		return org::apache::commons::lang3::serialization_utils::SerializationUtils::deserialize(&org::apache::commons::lang3::serialization_utils::SerializationUtils::serialize(obj))? as T;
	}

	pub fn serialize(&self, obj: &/* Java */ java::io::Serializable /**/) /* thrown(org.apache.commons.lang3.SerializationException) */ -> &[i8] {
		/* final */ let baos: ByteArrayOutputStream = ByteArrayOutputStream::new(512);
		org::apache::commons::lang3::serialization_utils::SerializationUtils::serialize(obj, baos)?;
		return baos.toByteArray();
	}

	pub fn serialize(&self, obj: &/* Java */ java::io::Serializable /**/, output_stream: &/* Java */ java::io::OutputStream /**/) /* thrown(org.apache.commons.lang3.SerializationException) */ {
		Objects::requireNonNull(output_stream, "outputStream");
		let r0 = 'try0: {
			(let out: ObjectOutputStream = ObjectOutputStream::new(output_stream)) out.writeObject(obj);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				break 'try0 Err(SerializationException::new(ex));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn new() -> org::apache::commons::lang3::serialization_utils::SerializationUtils {
	// empty
	}
}

struct ClassLoaderAwareObjectInputStream {
	class_loader: /* Java */ java::lang::ClassLoader /**/,
}

impl ClassLoaderAwareObjectInputStream {
	fn new(in: &/* Java */ java::io::InputStream /**/, class_loader: &/* Java */ java::lang::ClassLoader /**/) /* thrown(java.io.IOException) */ -> org::apache::commons::lang3::serialization_utils::ClassLoaderAwareObjectInputStream {
		super(in);
		self.classLoader = class_loader;
	}

	fn resolve_class(&self, desc: &/* Java */ java::io::ObjectStreamClass /**/) /* thrown(java.io.IOException | java.lang.ClassNotFoundException) */ -> /* Java */ java::lang::Class /**/ {
		/* final */ let name: String = desc.getName();
		let r0 = 'try0: {
			return Class::forName(name, false, self.class_loader);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ClassNotFoundException) => {
				let r1 = 'try1: {
					return Class::forName(name, false, &Thread::currentThread().getContextClassLoader());
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ ClassNotFoundException) => {
						/* final */ let cls: Class<?> = ClassUtils::get_primitive_class(name);
						if cls != null {
							return cls;
						}
						break 'try1 Err(cnfe);
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}

impl /* Java */ java::io::Closeable /**/ for ClassLoaderAwareObjectInputStream {}

impl /* Java */ java::lang::AutoCloseable /**/ for ClassLoaderAwareObjectInputStream {}

impl /* Java */ java::io::ObjectInput /**/ for ClassLoaderAwareObjectInputStream {}

impl /* Java */ java::io::DataInput /**/ for ClassLoaderAwareObjectInputStream {}

impl /* Java */ java::io::ObjectStreamConstants /**/ for ClassLoaderAwareObjectInputStream {}