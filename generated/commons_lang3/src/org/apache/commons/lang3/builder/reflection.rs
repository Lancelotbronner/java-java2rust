use java::lang::reflect::Field;
use java::util::Objects;

struct Reflection;

impl Reflection {
	fn get_unchecked(&self, field: &/* Java */ java::lang::reflect::Field /**/, obj: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Object /**/ {
		let r0 = 'try0: {
			return Objects::requireNonNull(field, "field").get(obj);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalAccessException) => {
				break 'try0 Err(IllegalArgumentException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}