pub struct JavadocSnippet {
	text: /* Java */ java::lang::String /**/,
}

impl JavadocSnippet {
	pub fn new(text: &/* Java */ java::lang::String /**/) /* thrown(java.lang.NullPointerException) */ -> com::github::javaparser::javadoc::description::javadoc_snippet::JavadocSnippet {
		if text == null {
			return Err(NullPointerException::new());
		}
		self.text = text;
	}

	pub fn to_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.text;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: JavadocSnippet = o as JavadocSnippet;
		return self.text.equals(that.text);
	}

	pub fn hash_code(&self) -> i32 {
		return self.text.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "JavadocSnippet{" + "text='" + self.text + '\'' + '}';
	}
}

impl com::github::javaparser::javadoc::description::javadoc_description_element::JavadocDescriptionElement for JavadocSnippet {}