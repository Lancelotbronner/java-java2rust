import org.junit.jupiter.api.Test;

/**
 * Tests for if-else statement conversion.
 */
public class IfElseTest {
	@Test
	public void canConvertSimpleIfToRust() {
		String java = """
			class A {
			    void check(int x) {
			        if (x > 0) {
			            System.out.println("positive");
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn check(&self, x: i32) {
			        if x > 0 {
			            System::out.println("positive");
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertIfElseToRust() {
		String java = """
			class A {
			    void check(int x) {
			        if (x > 0) {
			            System.out.println("positive");
			        } else {
			            System.out.println("negative");
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn check(&self, x: i32) {
			        if x > 0 {
			            System::out.println("positive");
			            } else {
			            System::out.println("negative");
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertIfElseIfToRust() {
		String java = """
			class A {
			    void check(int x) {
			        if (x > 10) {
			            System.out.println("large");
			        } else if (x > 0) {
			            System.out.println("medium");
			        } else {
			            System.out.println("small");
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn check(&self, x: i32) {
			        if x > 10 {
			            System::out.println("large");
			            } else if x > 0 {
			            System::out.println("medium");
			            } else {
			            System::out.println("small");
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertTernaryExpressionToRust() {
		String java = """
			class A {
			    int abs(int x) {
			        return x > 0 ? x : -x;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn abs(&self, x: i32) -> i32 { if x > 0 { x } else { -x }; }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
