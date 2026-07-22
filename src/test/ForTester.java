package test;

import java2rust.Java2Rust;
import org.junit.jupiter.api.Test;

/**
 * @author aschoerk
 */
public class ForTester {
	@Test
	public void canConvertCompleteForToRust() {
		String java = """
			void main() {
			    for (int i = 10; i < 100; i++)
			        System.out.println("i: " + i);
			    for (int i = 10; i < 100; i++) {
			        System.out.println("i: " + i);
			    }
			}
			""";
		String expected = """
			for i in 10..100 {
			   System::out.println("i: " + i);
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	//    @Test
	//    public void canConvertCompleteWithOutCondToRust() {
	//
	//        // for (int i = 10; ; i++) { System.out.println("i: " + i); if (i > 100) break; }
	//        String res = call("for (int i = 10; ; i++) { System.out.println(\"i: \" + i); if (i > 100) break; }");
	//        System.out.println(res);
	//
	//    }
	//
	//    @Test
	//    public void canConvertEmptyToRust() {
	//        // int i = 0; for (;;) { System.out.println("i: " + i); if (i > 100) break; else i++; }
	//        // for (int i = 10; ; i++) { System.out.println("i: " + i); if (i > 100) break; }
	//        String res = call("int i = 0; for (;;) { System.out.println(\"i: \" + i); if (i > 100) break; else { i++; } }");
	//        System.out.println(res);
	//
	//    }
	//
	//    @Test
	//    public void canConvertOnlyWithIncToRust() {
	//        // int i = 0; for (;;) { System.out.println("i: " + i); if (i > 100) break; else i++; }
	//        // for (int i = 10; ; i++) { System.out.println("i: " + i); if (i > 100) break; }
	//        String res = call("int i = 0; for (;;i++) { System.out.println(\"i: \" + i); if (i > 100) break;  }");
	//        System.out.println(res);
	//
	//    }
}
