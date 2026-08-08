use java::util::Arrays;
use java::util::Collections;
use java::util::List;
use java::util::function::Consumer;
use crate::org::apache::commons::lang3::math::NumberUtils;

pub struct BooleanUtils;

impl BooleanUtils {
	static BOOLEAN_LIST: /* Java */ java::util::List /**/ = Collections::unmodifiableList(&Arrays::asList(Boolean::FALSE, Boolean::TRUE));

	pub static FALSE: /* Java */ java::lang::String /**/ = "false";

	pub static NO: /* Java */ java::lang::String /**/ = "no";

	pub static OFF: /* Java */ java::lang::String /**/ = "off";

	pub static ON: /* Java */ java::lang::String /**/ = "on";

	pub static TRUE: /* Java */ java::lang::String /**/ = "true";

	pub static YES: /* Java */ java::lang::String /**/ = "yes";

	pub fn and(&self, array: bool) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		ObjectUtils::require_non_empty(array, "array")?;
		for /* final */ element in array {
			if !element {
				return false;
			}
		}
		return true;
	}

	pub fn and(&self, array: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		ObjectUtils::require_non_empty(array, "array")?;
		return  if org::apache::commons::lang3::boolean_utils::BooleanUtils::and(&ArrayUtils::to_primitive(array))? { Boolean::TRUE } else { Boolean::FALSE };
	}

	pub fn boolean_values(&self) -> &[/* Java */ java::lang::Boolean /**/] {
		return : [Option<Boolean>; ] = [None; ];
	}

	pub fn compare(&self, x: bool, y: bool) -> i32 {
		if x == y {
			return 0;
		}
		return  if x { 1 } else { -1 };
	}

	pub fn for_each(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		org::apache::commons::lang3::boolean_utils::BooleanUtils::values().forEach(action);
	}

	pub fn is_false(&self, bool: &/* Java */ java::lang::Boolean /**/) -> bool {
		return Boolean::FALSE.equals(bool);
	}

	pub fn is_not_false(&self, bool: &/* Java */ java::lang::Boolean /**/) -> bool {
		return !org::apache::commons::lang3::boolean_utils::BooleanUtils::is_false(bool);
	}

	pub fn is_not_true(&self, bool: &/* Java */ java::lang::Boolean /**/) -> bool {
		return !org::apache::commons::lang3::boolean_utils::BooleanUtils::is_true(bool);
	}

	pub fn is_true(&self, bool: &/* Java */ java::lang::Boolean /**/) -> bool {
		return Boolean::TRUE.equals(bool);
	}

	pub fn negate(&self, bool: &/* Java */ java::lang::Boolean /**/) -> /* Java */ java::lang::Boolean /**/ {
		if bool == null {
			return null;
		}
		return  if bool.booleanValue() { Boolean::FALSE } else { Boolean::TRUE };
	}

	pub fn one_hot(&self, array: bool) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		ObjectUtils::require_non_empty(array, "array")?;
		let result: bool = false;
		for /* final */ element in array {
			if element {
				if result {
					return false;
				}
				result = true;
			}
		}
		return result;
	}

	pub fn one_hot(&self, array: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		return Boolean::valueOf(&org::apache::commons::lang3::boolean_utils::BooleanUtils::one_hot(&ArrayUtils::to_primitive(array))?);
	}

	pub fn or(&self, array: bool) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		ObjectUtils::require_non_empty(array, "array")?;
		for /* final */ element in array {
			if element {
				return true;
			}
		}
		return false;
	}

	pub fn or(&self, array: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		ObjectUtils::require_non_empty(array, "array")?;
		return  if org::apache::commons::lang3::boolean_utils::BooleanUtils::or(&ArrayUtils::to_primitive(array))? { Boolean::TRUE } else { Boolean::FALSE };
	}

	pub fn primitive_values(&self) -> &[bool] {
		return : [bool; ] = [false; ];
	}

	pub fn to_boolean(&self, bool: &/* Java */ java::lang::Boolean /**/) -> bool {
		return bool != null && bool.booleanValue();
	}

	pub fn to_boolean(&self, value: i32) -> bool {
		return value != 0;
	}

	pub fn to_boolean(&self, value: i32, true_value: i32, false_value: i32) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if value == true_value {
			return true;
		}
		if value == false_value {
			return false;
		}
		return Err(IllegalArgumentException::new("The Integer did not match either specified value"));
	}

	pub fn to_boolean(&self, value: &/* Java */ java::lang::Integer /**/, true_value: &/* Java */ java::lang::Integer /**/, false_value: &/* Java */ java::lang::Integer /**/) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if value == null {
			if true_value == null {
				return true;
			}
			if false_value == null {
				return false;
			}
		} else if value.equals(true_value) {
			return true;
		} else if value.equals(false_value) {
			return false;
		}
		return Err(IllegalArgumentException::new("The Integer did not match either specified value"));
	}

	pub fn to_boolean(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_boolean_object(str) == Boolean::TRUE;
	}

	pub fn to_boolean(&self, str: &/* Java */ java::lang::String /**/, true_string: &/* Java */ java::lang::String /**/, false_string: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if str == true_string {
			return true;
		}
		if str == false_string {
			return false;
		}
		if str != null {
			if str.equals(true_string) {
				return true;
			}
			if str.equals(false_string) {
				return false;
			}
		}
		return Err(IllegalArgumentException::new("The String did not match either specified value"));
	}

	pub fn to_boolean_default_if_null(&self, bool: &/* Java */ java::lang::Boolean /**/, value_if_null: bool) -> bool {
		if bool == null {
			return value_if_null;
		}
		return bool.booleanValue();
	}

	pub fn to_boolean_object(&self, value: i32) -> /* Java */ java::lang::Boolean /**/ {
		return  if value == 0 { Boolean::FALSE } else { Boolean::TRUE };
	}

	pub fn to_boolean_object(&self, value: i32, true_value: i32, false_value: i32, null_value: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		if value == true_value {
			return Boolean::TRUE;
		}
		if value == false_value {
			return Boolean::FALSE;
		}
		if value == null_value {
			return null;
		}
		return Err(IllegalArgumentException::new("The Integer did not match any specified value"));
	}

	pub fn to_boolean_object(&self, value: &/* Java */ java::lang::Integer /**/) -> /* Java */ java::lang::Boolean /**/ {
		if value == null {
			return null;
		}
		return  if value.intValue() == 0 { Boolean::FALSE } else { Boolean::TRUE };
	}

	pub fn to_boolean_object(&self, value: &/* Java */ java::lang::Integer /**/, true_value: &/* Java */ java::lang::Integer /**/, false_value: &/* Java */ java::lang::Integer /**/, null_value: &/* Java */ java::lang::Integer /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		if value == null {
			if true_value == null {
				return Boolean::TRUE;
			}
			if false_value == null {
				return Boolean::FALSE;
			}
			if null_value == null {
				return null;
			}
		} else if value.equals(true_value) {
			return Boolean::TRUE;
		} else if value.equals(false_value) {
			return Boolean::FALSE;
		} else if value.equals(null_value) {
			return null;
		}
		return Err(IllegalArgumentException::new("The Integer did not match any specified value"));
	}

	pub fn to_boolean_object(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Boolean /**/ {
		// 'true'/'TRUE' match 4 times slower, 'tRUE'/'True' 7 times slower.
		if str == self.TRUE {
			return Boolean::TRUE;
		}
		if str == null {
			return null;
		}
		match str.length() {
			1 =>  {
				{
					/* final */ let ch0: char = str.charAt(0);
					if ch0 == 'y' || ch0 == 'Y' || ch0 == 't' || ch0 == 'T' || ch0 == '1' {
						return Boolean::TRUE;
					}
					if ch0 == 'n' || ch0 == 'N' || ch0 == 'f' || ch0 == 'F' || ch0 == '0' {
						return Boolean::FALSE;
					}
					break;
				}
			}
			2 =>  {
				{
					/* final */ let ch0: char = str.charAt(0);
					/* final */ let ch1: char = str.charAt(1);
					if (ch0 == 'o' || ch0 == 'O') && (ch1 == 'n' || ch1 == 'N') {
						return Boolean::TRUE;
					}
					if (ch0 == 'n' || ch0 == 'N') && (ch1 == 'o' || ch1 == 'O') {
						return Boolean::FALSE;
					}
					break;
				}
			}
			3 =>  {
				{
					/* final */ let ch0: char = str.charAt(0);
					/* final */ let ch1: char = str.charAt(1);
					/* final */ let ch2: char = str.charAt(2);
					if (ch0 == 'y' || ch0 == 'Y') && (ch1 == 'e' || ch1 == 'E') && (ch2 == 's' || ch2 == 'S') {
						return Boolean::TRUE;
					}
					if (ch0 == 'o' || ch0 == 'O') && (ch1 == 'f' || ch1 == 'F') && (ch2 == 'f' || ch2 == 'F') {
						return Boolean::FALSE;
					}
					break;
				}
			}
			4 =>  {
				{
					/* final */ let ch0: char = str.charAt(0);
					/* final */ let ch1: char = str.charAt(1);
					/* final */ let ch2: char = str.charAt(2);
					/* final */ let ch3: char = str.charAt(3);
					if (ch0 == 't' || ch0 == 'T') && (ch1 == 'r' || ch1 == 'R') && (ch2 == 'u' || ch2 == 'U') && (ch3 == 'e' || ch3 == 'E') {
						return Boolean::TRUE;
					}
					break;
				}
			}
			5 =>  {
				{
					/* final */ let ch0: char = str.charAt(0);
					/* final */ let ch1: char = str.charAt(1);
					/* final */ let ch2: char = str.charAt(2);
					/* final */ let ch3: char = str.charAt(3);
					/* final */ let ch4: char = str.charAt(4);
					if (ch0 == 'f' || ch0 == 'F') && (ch1 == 'a' || ch1 == 'A') && (ch2 == 'l' || ch2 == 'L') && (ch3 == 's' || ch3 == 'S') && (ch4 == 'e' || ch4 == 'E') {
						return Boolean::FALSE;
					}
					break;
				}
			}
			_ =>  {
				break;
			}
		}
		return null;
	}

	pub fn to_boolean_object(&self, str: &/* Java */ java::lang::String /**/, true_string: &/* Java */ java::lang::String /**/, false_string: &/* Java */ java::lang::String /**/, null_string: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		if str == null {
			if true_string == null {
				return Boolean::TRUE;
			}
			if false_string == null {
				return Boolean::FALSE;
			}
			if null_string == null {
				return null;
			}
		} else if str.equals(true_string) {
			return Boolean::TRUE;
		} else if str.equals(false_string) {
			return Boolean::FALSE;
		} else if str.equals(null_string) {
			return null;
		}
		// no match
		return Err(IllegalArgumentException::new("The String did not match any specified value"));
	}

	pub fn to_integer(&self, bool: bool) -> i32 {
		return  if bool { 1 } else { 0 };
	}

	pub fn to_integer(&self, bool: bool, true_value: i32, false_value: i32) -> i32 {
		return  if bool { true_value } else { false_value };
	}

	pub fn to_integer(&self, bool: &/* Java */ java::lang::Boolean /**/, true_value: i32, false_value: i32, null_value: i32) -> i32 {
		if bool == null {
			return null_value;
		}
		return  if bool.booleanValue() { true_value } else { false_value };
	}

	pub fn to_integer_object(&self, bool: bool) -> /* Java */ java::lang::Integer /**/ {
		return  if bool { NumberUtils::INTEGER_ONE } else { NumberUtils::INTEGER_ZERO };
	}

	pub fn to_integer_object(&self, bool: bool, true_value: &/* Java */ java::lang::Integer /**/, false_value: &/* Java */ java::lang::Integer /**/) -> /* Java */ java::lang::Integer /**/ {
		return  if bool { true_value } else { false_value };
	}

	pub fn to_integer_object(&self, bool: &/* Java */ java::lang::Boolean /**/) -> /* Java */ java::lang::Integer /**/ {
		if bool == null {
			return null;
		}
		return  if bool.booleanValue() { NumberUtils::INTEGER_ONE } else { NumberUtils::INTEGER_ZERO };
	}

	pub fn to_integer_object(&self, bool: &/* Java */ java::lang::Boolean /**/, true_value: &/* Java */ java::lang::Integer /**/, false_value: &/* Java */ java::lang::Integer /**/, null_value: &/* Java */ java::lang::Integer /**/) -> /* Java */ java::lang::Integer /**/ {
		if bool == null {
			return null_value;
		}
		return  if bool.booleanValue() { true_value } else { false_value };
	}

	pub fn to_string(&self, bool: bool, true_string: &/* Java */ java::lang::String /**/, false_string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if bool { true_string } else { false_string };
	}

	pub fn to_string(&self, bool: &/* Java */ java::lang::Boolean /**/, true_string: &/* Java */ java::lang::String /**/, false_string: &/* Java */ java::lang::String /**/, null_string: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if bool == null {
			return null_string;
		}
		return  if bool.booleanValue() { true_string } else { false_string };
	}

	pub fn to_string_on_off(&self, bool: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.ON, self.OFF);
	}

	pub fn to_string_on_off(&self, bool: &/* Java */ java::lang::Boolean /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.ON, self.OFF, null);
	}

	pub fn to_string_true_false(&self, bool: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.TRUE, self.FALSE);
	}

	pub fn to_string_true_false(&self, bool: &/* Java */ java::lang::Boolean /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.TRUE, self.FALSE, null);
	}

	pub fn to_string_yes_no(&self, bool: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.YES, self.NO);
	}

	pub fn to_string_yes_no(&self, bool: &/* Java */ java::lang::Boolean /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::boolean_utils::BooleanUtils::to_string(bool, self.YES, self.NO, null);
	}

	pub fn values(&self) -> /* Java */ java::util::List /**/ {
		return self.BOOLEAN_LIST;
	}

	pub fn xor(&self, array: bool) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		ObjectUtils::require_non_empty(array, "array")?;
		// false if the neutral element of the xor operator
		let result: bool = false;
		for /* final */ element in array {
			result ^= element;
		}
		return result;
	}

	pub fn xor(&self, array: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		ObjectUtils::require_non_empty(array, "array")?;
		return  if org::apache::commons::lang3::boolean_utils::BooleanUtils::xor(&ArrayUtils::to_primitive(array))? { Boolean::TRUE } else { Boolean::FALSE };
	}

	pub fn new() -> org::apache::commons::lang3::boolean_utils::BooleanUtils {
	// empty
	}
}