use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::expr::TypePatternExpr;
use crate::com::github::javaparser::quality::Nullable;
use crate::com::github::javaparser::resolution::declarations;
use crate::com::github::javaparser::resolution::model::SymbolReference;
use crate::com::github::javaparser::resolution::model::Value;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Collections;
use java::util::List;
use java::util::Optional;

pub trait Context;