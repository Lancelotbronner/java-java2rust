import org.junit.jupiter.api.Test;

/**
 * Tests for integer literal conversion.
 */
public class IntegerConvTest {
	@Test
	public void canConvertIntegerLiteralsToRust() {
		String java = """
			class A {
			    void literals() {
			        int a = 10;
			        int b = +20;
			        int c = -30;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn literals(&self) {
			        let a: i32 = 10;
			        let b: i32 = +20;
			        let c: i32 = -30;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertDoubleLiteralsToRust() {
		String java = """
			class A {
			    void doubles() {
			        double a = 1.5;
			        double b = 2.0;
			        double c = 1e10;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn doubles(&self) {
			        let a: f64 = 1.5;
			        let b: f64 = 2.0;
			        let c: f64 = 1e10;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
