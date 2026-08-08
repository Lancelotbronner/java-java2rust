pub struct ResolvedLambdaConstraintType {
	bound: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
}

impl ResolvedLambdaConstraintType {
	fn new(bound: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_lambda_constraint_type::ResolvedLambdaConstraintType {
		self.bound = bound;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return "? super " + self.bound.describe();
	}

	pub fn get_bound(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.bound;
	}

	pub fn is_constraint(&self) -> bool {
		return true;
	}

	pub fn as_constraint_type(&self) -> com::github::javaparser::resolution::types::resolved_lambda_constraint_type::ResolvedLambdaConstraintType {
		return self;
	}

	pub fn bound(&self, bound: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_lambda_constraint_type::ResolvedLambdaConstraintType {
		return ResolvedLambdaConstraintType::new(bound);
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		return self.bound.is_assignable_by(other);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "LambdaConstraintType{" + "bound=" + self.bound + '}';
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedLambdaConstraintType {}