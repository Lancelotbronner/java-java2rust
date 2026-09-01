import org.junit.jupiter.api.Test;


/**
 * Tests for array conversion.
 */
public class ArrayTest {
	@Test
	public void canConvertArrayAccessToRust() {
		String java = """
			class A {
			    void access(int[] arr) {
			        int x = arr[0];
			        arr[1] = 10;
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn access(&self, arr: &[i32]) {
			        let x: i32 = arr[0];
			        arr[1] = 10;
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertArrayInitializationToRust() {
		String java = """
			class A {
			    void init() {
			        int[] arr = {1, 2, 3};
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn init(&self) {
			        let arr: vec![i32] = vec![1, 2, 3, ];
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
