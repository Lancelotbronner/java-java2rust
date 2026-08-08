use java::lang::reflect::AccessibleObject;

struct AccessibleObjects;

impl AccessibleObjects {
	fn is_accessible(&self, accessible_object: &/* Java */ java::lang::reflect::AccessibleObject /**/) -> bool {
		return accessible_object == null || accessible_object.isAccessible();
	}

	fn set_accessible(&self, accessible_object: &/* Java */ java::lang::reflect::AccessibleObject /**/) -> bool {
		if !org::apache::commons::lang3::reflect::accessible_objects::AccessibleObjects::is_accessible(accessible_object) {
			accessible_object.setAccessible(true);
			return true;
		}
		return false;
	}
}