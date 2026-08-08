use crate::com::github::javaparser::ast::AccessSpecifier;
use crate::com::github::javaparser::resolution::MethodUsage;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::types::ResolvedReferenceType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::io::Serializable;
use java::util;
use java::util::function::Function;
use java::util::stream::Collectors;

pub trait ResolvedReferenceTypeDeclaration;