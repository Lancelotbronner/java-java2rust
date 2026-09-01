import org.junit.jupiter.api.Test;

/**
 * Tests for expression conversions.
 */
public class ExpressionTest {
	@Test
	public void canConvertBinaryExpressionsToRust() {
		String java = """
			class A {
			    void calc(int a, int b) {
			        int x = a + b;
			        int y = a - b;
			        int z = a * b;
			        int w = a / b;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn calc(&self, a: i32, b: i32) {
			        let x: i32 = a + b;
			        let y: i32 = a - b;
			        let z: i32 = a * b;
			        let w: i32 = a / b;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertComparisonOperatorsToRust() {
		String java = """
			class A {
			    void compare(int a, int b) {
			        boolean eq = a == b;
			        boolean ne = a != b;
			        boolean lt = a < b;
			        boolean gt = a > b;
			        boolean le = a <= b;
			        boolean ge = a >= b;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn compare(&self, a: i32, b: i32) {
			        let eq: bool = a == b;
			        let ne: bool = a != b;
			        let lt: bool = a < b;
			        let gt: bool = a > b;
			        let le: bool = a <= b;
			        let ge: bool = a >= b;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertLogicalOperatorsToRust() {
		String java = """
			class A {
			    void logic(int a, int b) {
			        boolean x = a > 0 && b > 0;
			        boolean y = a > 0 || b > 0;
			        boolean z = !(a > 0);
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn logic(&self, a: i32, b: i32) {
			        let x: bool = a > 0 && b > 0;
			        let y: bool = a > 0 || b > 0;
			        let z: bool = !(a > 0);
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertStringConcatenationToRust() {
		String java = """
			class A {
			    void greet(String name) {
			        System.out.println("Hello, " + name + "!");
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn greet(&self, name: String) {
			        System::out.println(format!("Hello, {}!", name));
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertUnaryOperatorsToRust() {
		String java = """
			class A {
			    void increment(int x) {
			        x++;
			        x--;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn increment(&self, x: i32) {
			        x += 1;
			        x -= 1;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
