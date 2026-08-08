use crate::com::github::javaparser::ast::NodeList::toNodeList;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ModifierMetaModel;
use java::util::Arrays;

pub struct Modifier {
	keyword: com::github::javaparser::ast::modifier::Keyword,
}

impl Modifier {
	pub fn public_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::PUBLIC);
	}

	pub fn protected_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::PROTECTED);
	}

	pub fn private_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::PRIVATE);
	}

	pub fn abstract_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::ABSTRACT);
	}

	pub fn static_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::STATIC);
	}

	pub fn final_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::FINAL);
	}

	pub fn transient_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::TRANSIENT);
	}

	pub fn volatile_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::VOLATILE);
	}

	pub fn synchronized_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::SYNCHRONIZED);
	}

	pub fn native_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::NATIVE);
	}

	pub fn strictfp_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::STRICTFP);
	}

	pub fn transitive_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::TRANSITIVE);
	}

	pub fn sealed_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::SEALED);
	}

	pub fn non_sealed_modifier(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return Modifier::new(Keyword::NON_SEALED);
	}

	pub fn new() -> com::github::javaparser::ast::modifier::Modifier {
		this(Keyword::PUBLIC);
	}

	pub fn new(keyword: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::modifier::Modifier {
		this(null, keyword);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, keyword: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::modifier::Modifier {
		super(token_range);
		self.set_keyword(keyword);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_keyword(&self) -> com::github::javaparser::ast::modifier::Keyword {
		return self.keyword;
	}

	pub fn set_keyword(&mut self, keyword: &com::github::javaparser::ast::modifier::Keyword) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modifier::Modifier {
		com::github::javaparser::utils::utils::Utils::assert_not_null(keyword)?;
		if keyword == self.keyword {
			return self;
		}
		self.notify_property_change(ObservableProperty::KEYWORD, self.keyword, keyword);
		self.keyword = keyword;
		return self;
	}

	pub fn create_modifier_list(&self, modifiers: &com::github::javaparser::ast::modifier::Keyword) -> com::github::javaparser::ast::node_list::NodeList {
		return Arrays::stream(modifiers).map(Modifier::new).collect(&com::github::javaparser::ast::node_list::NodeList::to_node_list());
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modifier::Modifier {
		return self.accept(CloneVisitor::new(), null) as Modifier;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::modifier_meta_model::ModifierMetaModel {
		return JavaParserMetaModel::modifierMetaModel;
	}
}

impl /* Java */ java::lang::Cloneable /**/ for Modifier {}

impl com::github::javaparser::has_parent_node::HasParentNode for Modifier {}

impl com::github::javaparser::ast::observer::observable::Observable for Modifier {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Modifier {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Modifier {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Modifier {}

pub enum Keyword {
	code_representation: /* Java */ java::lang::String /**/,
}