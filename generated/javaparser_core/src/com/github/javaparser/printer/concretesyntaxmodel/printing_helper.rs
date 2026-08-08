use crate::com::github::javaparser::printer::Stringable;

struct PrintingHelper;

impl PrintingHelper {
	fn print_to_string(&self, value: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		if value instanceof Stringable {
			return (value as Stringable).as_string();
		}
		if value instanceof Enum {
			return (value as Enum).name().toLowerCase();
		} else {
			if value != null {
				return value.toString();
			}
		}
		return "";
	}
}