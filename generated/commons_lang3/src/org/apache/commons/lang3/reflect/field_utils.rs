use java::lang::annotation::Annotation;
use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::ArrayList;
use java::util::Collections;
use java::util::List;
use java::util::Objects;
use java::util::stream::Collectors;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::JavaVersion;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::SystemUtils;
use crate::org::apache::commons::lang3::Validate;

pub struct FieldUtils;

impl FieldUtils {
	pub fn get_all_fields(&self, cls: &/* Java */ java::lang::Class /**/) -> &[/* Java */ java::lang::reflect::Field /**/] {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_all_fields_list(cls).toArray(ArrayUtils::EMPTY_FIELD_ARRAY);
	}

	pub fn get_all_fields_list(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		Objects::requireNonNull(cls, "cls");
		/* final */ let all_fields: List<Field> = ArrayList<>::new();
		let current_class: Class<?> = cls;
		while current_class != null {
			Collections::addAll(all_fields, &current_class.getDeclaredFields());
			current_class = current_class.getSuperclass();
		}
		return all_fields;
	}

	pub fn get_declared_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::reflect::Field /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_declared_field(cls, field_name, false);
	}

	pub fn get_declared_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::Field /**/ {
		Objects::requireNonNull(cls, "cls");
		Validate::is_true(&StringUtils::is_not_blank(field_name), "The field name must not be blank/empty")?;
		let r0 = 'try0: {
			// only consider the specified class by using getDeclaredField()
			/* final */ let field: Field = cls.getDeclaredField(field_name);
			if !MemberUtils::is_accessible(field) {
				if !force_access {
					return null;
				}
				field.setAccessible(true);
			}
			return field;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchFieldException) => {
			// ignore
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return null;
	}

	pub fn get_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::reflect::Field /**/ {
		return MemberUtils::set_accessible_workaround(&org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_field(cls, field_name, false));
	}

	pub fn get_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::Field /**/ {
		Objects::requireNonNull(cls, "cls");
		Validate::is_true(&StringUtils::is_not_blank(field_name), "The field name must not be blank/empty")?;
		// check up the superclass hierarchy
		 {
			let acls: Class<?> = cls;
			while acls != null {
				{
					let r0 = 'try0: {
						/* final */ let field: Field = acls.getDeclaredField(field_name);
						// and it returns accurate results
						if !MemberUtils::is_public(field) {
							if !force_access {
								continue;
							}
							field.setAccessible(true);
						}
						return field;
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ NoSuchFieldException) => {
						// ignore
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				acls = acls.getSuperclass();
			 }
		 }
	
		// check the public interface case. This must be manually searched for
		// incase there is a public supersuperclass field hidden by a private/package
		// superclass field.
		let match: Field = null;
		for /* final */ class1 in ClassUtils::get_all_interfaces(cls) {
			let r1 = 'try1: {
				/* final */ let test: Field = class1.getField(field_name);
				if let Err(e) = Validate::is_true(match == null, "Reference to field %s is ambiguous relative to %s; a matching field exists on two or more implemented interfaces.", field_name, cls) {
					return Err(e);
				};
				match = test;
				break 'try1 Ok(());
			};
			match r1 {
				Err(e @ NoSuchFieldException) => {
				// ignore
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		return match;
	}

	pub fn get_fields_list_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		Objects::requireNonNull(annotation_cls, "annotationCls");
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_all_fields_list(cls).stream().filter(|field|field.getAnnotation(annotation_cls) != null).collect(&Collectors::toList());
	}

	pub fn get_fields_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/) -> &[/* Java */ java::lang::reflect::Field /**/] {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_fields_list_with_annotation(cls, annotation_cls).toArray(ArrayUtils::EMPTY_FIELD_ARRAY);
	}

	pub fn read_declared_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_declared_field(target, field_name, false);
	}

	pub fn read_declared_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Object /**/ {
		Objects::requireNonNull(target, "target");
		/* final */ let cls: Class<?> = target.getClass();
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_declared_field(cls, field_name, force_access)?;
		Validate::is_true(field != null, "Cannot locate declared field %s.%s", cls, field_name)?;
		// already forced access above, don't repeat it here:
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_field(field, target, false);
	}

	pub fn read_declared_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_declared_static_field(cls, field_name, false);
	}

	pub fn read_declared_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_declared_field(cls, field_name, force_access)?;
		Validate::not_null(field, "Cannot locate declared field %s.%s", &cls.getName(), field_name);
		// already forced access above, don't repeat it here:
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_static_field(field, false);
	}

	pub fn read_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, target: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_field(field, target, false);
	}

	pub fn read_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, target: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		Objects::requireNonNull(field, "field");
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::set_accessible(field, force_access).get(target);
	}

	pub fn read_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_field(target, field_name, false);
	}

	pub fn read_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Object /**/ {
		Objects::requireNonNull(target, "target");
		/* final */ let cls: Class<?> = target.getClass();
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_field(cls, field_name, force_access)?;
		Validate::is_true(field != null, "Cannot locate field %s on %s", field_name, cls)?;
		// already forced access above, don't repeat it here:
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_field(field, target, false)?;
	}

	pub fn read_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_static_field(cls, field_name, false);
	}

	pub fn read_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_field(cls, field_name, force_access)?;
		Validate::not_null(field, "Cannot locate field '%s' on %s", field_name, cls);
		// already forced access above, don't repeat it here:
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_static_field(field, false);
	}

	pub fn read_static_field(&self, field: &/* Java */ java::lang::reflect::Field /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_static_field(field, false);
	}

	pub fn read_static_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Object /**/ {
		Objects::requireNonNull(field, "field");
		Validate::is_true(&MemberUtils::is_static(field), "The field '%s' is not static", &field.getName())?;
		return org::apache::commons::lang3::reflect::field_utils::FieldUtils::read_field(field, null as Object, force_access)?;
	}

	pub fn remove_final_modifier(&self, field: &/* Java */ java::lang::reflect::Field /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::remove_final_modifier(field, true)?;
	}

	pub fn remove_final_modifier(&self, field: &/* Java */ java::lang::reflect::Field /**/, force_access: bool) /* thrown(java.lang.UnsupportedOperationException) */ {
		Objects::requireNonNull(field, "field");
		let r0 = 'try0: {
			if Modifier::isFinal(&field.getModifiers()) {
				// Do all JREs implement Field with a private ivar called "modifiers"?
				/* final */ let modifiers_field: Field = Field.class.getDeclaredField("modifiers");
				/* final */ let do_force_access: bool = force_access && !modifiers_field.isAccessible();
				if do_force_access {
					modifiers_field.setAccessible(true);
				}
				let r1 = 'try1: {
					modifiers_field.setInt(field, field.getModifiers() & ~Modifier::FINAL);
					break 'try1 Ok(());
				};
				match r1 {
					Err(e) => Err(e)?,
					Ok => (),
				}
				if do_force_access {
					modifiers_field.setAccessible(false);
				}
	
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchFieldExceptionIllegalAccessException | ) => {
				if SystemUtils::is_java_version_at_least(JavaVersion::JAVA_12) {
					break 'try1 Err(UnsupportedOperationException::new("In java 12+ final cannot be removed.", e));
				}
			// else no exception is thrown because we can modify final.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn set_accessible(&self, field: &/* Java */ java::lang::reflect::Field /**/, force_access: bool) -> /* Java */ java::lang::reflect::Field /**/ {
		if force_access && !field.isAccessible() {
			field.setAccessible(true);
		} else {
			MemberUtils::set_accessible_workaround(field);
		}
		return field;
	}

	pub fn write_declared_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_declared_field(target, field_name, value, false);
	}

	pub fn write_declared_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ {
		Objects::requireNonNull(target, "target");
		/* final */ let cls: Class<?> = target.getClass();
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_declared_field(cls, field_name, force_access)?;
		Validate::is_true(field != null, "Cannot locate declared field %s.%s", &cls.getName(), field_name)?;
		// already forced access above, don't repeat it here:
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(field, target, value, false);
	}

	pub fn write_declared_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_declared_static_field(cls, field_name, value, false);
	}

	pub fn write_declared_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ {
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_declared_field(cls, field_name, force_access)?;
		Validate::not_null(field, "Cannot locate declared field %s.%s", &cls.getName(), field_name);
		// already forced access above, don't repeat it here:
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(field, null as Object, value, false);
	}

	pub fn write_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, target: &/* Java */ java::lang::Object /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(field, target, value, false);
	}

	pub fn write_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, target: &/* Java */ java::lang::Object /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ {
		Objects::requireNonNull(field, "field");
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::set_accessible(field, force_access).set(target, value);
	}

	pub fn write_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(target, field_name, value, false);
	}

	pub fn write_field(&self, target: &/* Java */ java::lang::Object /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ {
		Objects::requireNonNull(target, "target");
		/* final */ let cls: Class<?> = target.getClass();
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_field(cls, field_name, force_access)?;
		Validate::is_true(field != null, "Cannot locate declared field %s.%s", &cls.getName(), field_name)?;
		// already forced access above, don't repeat it here:
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(field, target, value, false)?;
	}

	pub fn write_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_static_field(cls, field_name, value, false);
	}

	pub fn write_static_field(&self, cls: &/* Java */ java::lang::Class /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException) */ {
		/* final */ let field: Field = org::apache::commons::lang3::reflect::field_utils::FieldUtils::get_field(cls, field_name, force_access)?;
		Validate::not_null(field, "Cannot locate field %s on %s", field_name, cls);
		// already forced access above, don't repeat it here:
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_static_field(field, value, false);
	}

	pub fn write_static_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ {
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_static_field(field, value, false);
	}

	pub fn write_static_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, value: &/* Java */ java::lang::Object /**/, force_access: bool) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException) */ {
		Objects::requireNonNull(field, "field");
		Validate::is_true(&MemberUtils::is_static(field), "The field %s.%s is not static", &field.getDeclaringClass().getName(), &field.getName())?;
		org::apache::commons::lang3::reflect::field_utils::FieldUtils::write_field(field, null as Object, value, force_access)?;
	}

	pub fn new() -> org::apache::commons::lang3::reflect::field_utils::FieldUtils {
	// empty
	}
}