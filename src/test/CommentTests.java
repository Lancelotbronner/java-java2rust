package test;

import java2rust.Java2Rust;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class CommentTests {
	@Test
	public void noDuplicateJavadocComments() {
		String res = Java2Rust.test("""
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
			""");
		assertEquals(
			"""
				/**
				 * Interface comment
				 */
				pub trait X {
				
				    /**
				   * Hello
				   */
				    // World
				    fn  hello(&self) -> i32 ;
				
				    /**
				   * Just javadoc
				   */
				    fn  ohyes(&self) -> i32 ;
				}
				
				""", res);
	}

	@Test
	public void packageDeclarationJavadoc() {
		String actual = Java2Rust.test("""
			/**
			 * Licence
			 */
			// Comment
			package y;
			
			/**
			 * Class.
			 */
			public class C{}""");
		assertEquals(
			"""
				/**
				 * Licence
				 */
				// Comment
				// package y;
				
				/**
				 * Class.
				 */
				pub struct C {
				}
				
				impl C {
				}
				
				""",
			actual);
	}

	@Test
	public void caseTest() {
		assertEquals("glfw_error_capture", Java2Rust.toSnakeCase("GLFWErrorCapture"));
	}
}
