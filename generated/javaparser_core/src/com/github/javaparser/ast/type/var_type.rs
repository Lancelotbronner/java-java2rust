use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use crate::com::github::javaparser::ast::stmt::ForEachStmt;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::VarTypeMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeDeclaration;
use crate::com::github::javaparser::resolution::model::typesystem::ReferenceTypeImpl;
use crate::com::github::javaparser::resolution::types::ResolvedArrayType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct VarType;

impl VarType {
	static JAVA_LANG_OBJECT: /* Java */ java::lang::String /**/ = Object.class.getCanonicalName();

	pub fn new() -> com::github::javaparser::ast::type::var_type::VarType {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::type::var_type::VarType {
		super(token_range);
		self.custom_initialization();
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::var_type::VarType {
		return super.set_annotations(annotations)? as VarType;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return "var";
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::var_type::VarType {
		return self.accept(CloneVisitor::new(), null) as VarType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::var_type_meta_model::VarTypeMetaModel {
		return JavaParserMetaModel::varTypeMetaModel;
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedType.class);
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_var_type(&self) -> bool {
		return true;
	}

	pub fn as_var_type(&self) -> com::github::javaparser::ast::type::var_type::VarType {
		return self;
	}

	pub fn to_var_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_var_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let parent: Node = self.get_parent_node().get();
		if !(parent instanceof VariableDeclarator) {
			return Err(IllegalStateException::new("Trying to resolve a `var` which is not in a variable declaration."));
		}
		/* final */ let variable_declarator: VariableDeclarator = parent as VariableDeclarator;
		let initializer: Optional<Expression> = variable_declarator.get_initializer();
		if !initializer.isPresent() {
			// When a `var` type decl has no initializer it may be part of a
			// for-each statement (e.g. `for(var i : expr)`).
			let for_each_stmt: Optional<ForEachStmt> = self.for_each_stmt_with_variable_declarator(variable_declarator);
			if for_each_stmt.isPresent() {
				let iterable: Expression = for_each_stmt.get().get_iterable();
				let iter_type: ResolvedType = iterable.calculate_resolved_type();
				if iter_type instanceof ResolvedArrayType {
					// is the component type of the array.
					return (iter_type as ResolvedArrayType).get_component_type();
				}
				if iter_type.is_reference_type() {
					// The type of a variable in a for-each loop with an
					// Iterable with parameter type
					let parameters_type: List<ResolvedType> = iter_type.as_reference_type()?.type_parameters_map().get_types();
					if parameters_type.isEmpty() {
						let o_object_declaration: Optional<ResolvedTypeDeclaration> = context.solve_type(self.JAVA_LANG_OBJECT).get_declaration();
						return o_object_declaration.map(|decl|ReferenceTypeImpl::undetermined_parameters(&decl.as_reference_type()?)).orElseThrow(|()|UnsupportedOperationException::new());
					}
					return parameters_type.get(0);
				}
			}
		}
		return initializer.map(Expression::calculateResolvedType).orElseThrow(|()|IllegalStateException::new("Cannot resolve `var` which has no initializer."));
	}

	fn for_each_stmt_with_variable_declarator(&self, variable_declarator: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator) -> /* Java */ java::util::Optional /**/ {
		let node: Optional<Node> = variable_declarator.get_parent_node();
		if !node.isPresent() || !(node.get() instanceof VariableDeclarationExpr) {
			return Optional::empty();
		}
		node = node.get().get_parent_node();
		if !node.isPresent() || !(node.get() instanceof ForEachStmt) {
			return Optional::empty();
		}
		return Optional::of(node.get() as ForEachStmt);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for VarType {}

impl com::github::javaparser::has_parent_node::HasParentNode for VarType {}

impl com::github::javaparser::ast::observer::observable::Observable for VarType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for VarType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for VarType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for VarType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for VarType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for VarType {}