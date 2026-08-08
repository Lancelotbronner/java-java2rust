use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::SimpleName;
use java::util::Optional;

pub trait NodeWithOptionalLabel<T: com::github::javaparser::ast::node::Node>;