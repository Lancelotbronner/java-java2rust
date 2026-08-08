use crate::com::github::javaparser::utils::TypeUtils;
use java::util::Arrays;
use java::util::Collections;
use java::util::List;
use java::util::Optional;

pub enum ResolvedPrimitiveType {
	name: /* Java */ java::lang::String /**/,
	box_type_class: /* Java */ java::lang::Class /**/,
	promotion_types: /* Java */ java::util::List /**/,
}