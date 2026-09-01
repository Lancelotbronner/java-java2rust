import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Created by aschoerk on 01.05.16.
 */
public class SnakeTest {
	@Test
	public void testMethodCall() {
		String result = Java2Rust.test("class A { void m() { xAA.xAAB(); } }");
		assertTrue(result.contains("x_a_a.x_a_a_b"));
	}

	@Test
	public void testVarDecl() {
		String result = Java2Rust.test("class A { void m() { String xTestString; } }");
		assertTrue(result.contains("x_test_string"));
	}

	@Test
	public void testMethodDecl() {
		String result = Java2Rust.test("class A { String methodMIs(); }");
		assertTrue(result.contains("method_m_is"));
	}

	@Test
	public void testTestAnnotation() {
		String result1 = Java2Rust.test("class A { @Test void testMethod() { int i; } }");
		assertTrue(result1.contains("#[test]"));

		String result2 = Java2Rust.test("class A { @Test\\n void testMethod() { int a; } }");
		assertTrue(result2.contains("#[test]"));

		String result3 = Java2Rust.test(
			"class A { @Test(expected = Exception.class)\\n void testMethod() { int b; } }");
		assertTrue(result3.contains("#[test]"));
	}
}
