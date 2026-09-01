import org.junit.jupiter.api.Test;

public class CommentTests {
	@Test
	public void noDuplicateJavadocComments() {
		Java2Rust.assertConversion(
			"""
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
				""", """
				/// Interface comment
				pub trait X {
					/// Hello
				    // World
				    fn  hello(&self) -> i32 ;
				
				    /// Just javadoc
				    fn  ohyes(&self) -> i32 ;
				}
				""");
	}

	@Test
	public void packageDeclarationJavadoc() {
		Java2Rust.assertConversion(
			"""
				/**
				 * Licence
				 */
				// Comment
				package y;
				
				/**
				 * Class.
				 */
				public class C{}
				""", """
				/// License
				// Comment
				mod y;
				
				/// Class.
				pub struct C {
				}
				""");
	}

	@Test
	public void caseTest() {
		Java2Rust.assertConversion(
			"""
				public class C {
					public void glfwErrorCapture() {
					}
				}
				""", """
				pub struct C {}
				
				impl C {
				    fn glfw_error_capture(&self) {
				    }
				}
				""");
	}
}
