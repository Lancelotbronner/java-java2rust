pub struct Java12PreviewValidator;

impl Java12PreviewValidator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java12_preview_validator::Java12PreviewValidator {
		super();
		// Incubator
		// No new incubator language features added within Java 12
		// Preview
		{
			/* 
	             * Switch Expressions (Preview) - first preview within Java 12 - https://openjdk.java.net/jeps/325
	             * <ul>
	             *     <li>Switch permissions are added within this preview.</li>
	             *     <li>Multiple labels is NOT YET PERMITTED -- introduced within Java 14 release.</li>
	             *     <li>Yield keyword is NOT YET PERMITTED -- introduced within Java 13 preview.</li>
	             * </ul>
	             */ 
			self.remove()?;
			self.remove()?;
		}
	// 2nd Preview
	// No new 2nd preview language features added within Java 12
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java12PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java12PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java12PreviewValidator {}