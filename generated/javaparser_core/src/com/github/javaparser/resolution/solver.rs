use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use crate::com::github::javaparser::resolution::model::SymbolReference;
use crate::com::github::javaparser::resolution::model::Value;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::List;
use java::util::Optional;

pub trait Solver;