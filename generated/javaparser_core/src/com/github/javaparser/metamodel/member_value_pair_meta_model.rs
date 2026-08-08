use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::MemberValuePair;
use java::util::Optional;

pub struct MemberValuePairMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	value_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl MemberValuePairMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::member_value_pair_meta_model::MemberValuePairMetaModel {
		super(super_base_node_meta_model, MemberValuePair.class, "MemberValuePair", "com.github.javaparser.ast.expr", false, false);
	}
}