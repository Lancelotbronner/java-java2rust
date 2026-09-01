import org.junit.jupiter.api.Test;

/**
 * Tests for lambda expression conversion.
 */
public class LambdaTest {
	@Test
	public void canConvertLambdaToRust() {
		String java = """
			class A {
			    void process() {
			        Runnable r = () -> System.out.println("hello");
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn process(&self) {
			        let r: Runnable = || System::out.println("hello");
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertLambdaWithParamsToRust() {
		String java = """
			class A {
			    void process() {
			        java.util.function.Function<Integer, Integer> f = x -> x * 2;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn process(&self) {
			        let f: java::util::function::Function<Integer, Integer> = |x| x * 2;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
