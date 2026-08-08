use crate::org::apache::commons::lang3::BooleanUtils;

pub struct InheritanceUtils;

impl InheritanceUtils {
	pub fn distance(&self, child: &/* Java */ java::lang::Class /**/, parent: &/* Java */ java::lang::Class /**/) -> i32 {
		if child == null || parent == null {
			return -1;
		}
		if child.equals(parent) {
			return 0;
		}
		/* final */ let c_parent: Class<?> = child.getSuperclass();
		let d: i32 = BooleanUtils::to_integer(&parent.equals(c_parent));
		if d == 1 {
			return d;
		}
		d += org::apache::commons::lang3::reflect::inheritance_utils::InheritanceUtils::distance(c_parent, parent);
		return  if d > 0 { d + 1 } else { -1 };
	}

	pub fn new() -> org::apache::commons::lang3::reflect::inheritance_utils::InheritanceUtils {
	// empty
	}
}