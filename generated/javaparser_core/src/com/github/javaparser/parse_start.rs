use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::PackageDeclaration;
use crate::com::github::javaparser::ast::body::BodyDeclaration;
use crate::com::github::javaparser::ast::body::MethodDeclaration;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::TypeDeclaration;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use crate::com::github::javaparser::ast::modules::ModuleDirective;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::stmt::Statement;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::TypeParameter;

pub trait ParseStart<R>;