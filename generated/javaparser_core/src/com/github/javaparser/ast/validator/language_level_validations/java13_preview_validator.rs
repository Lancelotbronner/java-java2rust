pub struct Java13PreviewValidator;

impl Java13PreviewValidator {
	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java13_preview_validator::Java13PreviewValidator {
		super();
		// Incubator
		// No new incubator language features added within Java 13
		// Preview
		// Text Block Literals - first preview within Java 13 - https://openjdk.java.net/jeps/355
		self.remove()?;
		// 2nd Preview
		{
			/* 
	             * Switch Expressions (2nd Preview) - 2nd Preview within Java 13 - https://openjdk.java.net/jeps/354
	             * <ul>
	             *     <li>Switch permissions are added within this preview.</li>
	             *     <li>Multiple labels is NOT YET PERMITTED -- introduced within Java 14 release.</li>
	             *     <li>Yield keyword -- introduced within Java 13 preview.</li>
	             * </ul>
	             */ 
			self.remove()?;
			self.remove()?;
			self.remove()?;
		}
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java13PreviewValidator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java13PreviewValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java13PreviewValidator {}