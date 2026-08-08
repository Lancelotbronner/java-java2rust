use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::Processor;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::ClassExpr;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::VarType;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::List;

pub struct Java10PostProcessor {
	var_node_creator: com::github::javaparser::processor::Processor = Processor::new() {
	pub fn post_process(&self, result: &ParseResult<? extends Node>, configuration: &ParserConfiguration) {
		result.get_result().ifPresent(|node|{
			node.find_all(ClassOrInterfaceType.class).forEach(|n|{
				if "var".equals(&n.get_name_as_string()) && !self.matchForbiddenContext(n) {
					n.replace(VarType::new(&n.get_token_range().orElse(null)));
				}
			});
		});
	}

	fn match_forbidden_context(&self, cit: &ClassOrInterfaceType) -> bool {
		return cit.get_parent_node().isPresent() && FORBIDEN_PARENT_CONTEXT_TO_DETECT_POTENTIAL_VAR_TYPE.stream().anyMatch(|cl|cl.isInstance(&cit.get_parent_node().get()));
	}

},
}

impl Java10PostProcessor {
	static FORBIDEN_PARENT_CONTEXT_TO_DETECT_POTENTIAL_VAR_TYPE: /* Java */ java::util::List /**/ = ArrayList<>::new();

	init {
	    FORBIDEN_PARENT_CONTEXT_TO_DETECT_POTENTIAL_VAR_TYPE.addAll(Arrays.asList(ClassExpr.class));
	}

	pub fn new() -> com::github::javaparser::ast::validator::postprocessors::java10_post_processor::Java10PostProcessor {
		self.add(self.var_node_creator);
	}
}