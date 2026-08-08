use crate::java2rust::Java2Rust;
use commons_lang3::org::junit::jupiter::api::Test;
use commons_lang3::org::junit::jupiter::api::Assertions::assertEquals;

pub struct CommentTests;

impl CommentTests {
	pub fn no_duplicate_javadoc_comments(&self) {
		Java2Rust::assert_conversion(r#"
		/**
		 * Interface comment
		 */
		public interface X {
		  /**
		   * Hello
		   */
		  // World
		  int hello();
		
		  /**
		   * Just javadoc
		   */
		   int ohyes();
		}
		"#, r#"
		/// Interface comment
		pub trait X {
			/// Hello
		    // World
		    fn  hello(&self) -> i32 ;
		
		    /// Just javadoc
		    fn  ohyes(&self) -> i32 ;
		}
		"#);
	}

	pub fn package_declaration_javadoc(&self) {
		Java2Rust::assert_conversion(r#"
		/**
		 * Licence
		 */
		// Comment
		package y;
		
		/**
		 * Class.
		 */
		public class C{}
		"#, r#"
		/// License
		// Comment
		mod y;
		
		/// Class.
		pub struct C {
		}
		"#);
	}

	pub fn case_test(&self) {
		Java2Rust::assert_conversion(r#"
		public class C {
			public void glfwErrorCapture() {
			}
		}
		"#, r#"
		pub struct C {}
		
		impl C {
		    fn glfw_error_capture(&self) {
		    }
		}
		"#);
	}
}