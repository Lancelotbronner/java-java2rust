use java::io::Serializable;
use java::util::Comparator;

pub struct ObjectToStringComparator;

impl ObjectToStringComparator {
	pub static INSTANCE: org::apache::commons::lang3::compare::object_to_string_comparator::ObjectToStringComparator = ObjectToStringComparator::new();

	static serialVersionUID: i64 = 1;

	pub fn new() -> org::apache::commons::lang3::compare::object_to_string_comparator::ObjectToStringComparator {
	// empty
	}

	pub fn compare(&self, o1: &/* Java */ java::lang::Object /**/, o2: &/* Java */ java::lang::Object /**/) -> i32 {
		if o1 == o2 {
			return 0;
		}
		if o1 == null {
			return 1;
		}
		if o2 == null {
			return -1;
		}
		/* final */ let string1: String = o1.toString();
		/* final */ let string2: String = o2.toString();
		// No guarantee that toString() returns a non-null value, despite what Spotbugs thinks.
		if string1 == string2 {
			return 0;
		}
		if string1 == null {
			return 1;
		}
		if string2 == null {
			return -1;
		}
		return string1.compareTo(string2);
	}
}

impl /* Java */ java::util::Comparator /**/ for ObjectToStringComparator {}

impl /* Java */ java::io::Serializable /**/ for ObjectToStringComparator {}