import org.junit.jupiter.api.Test;

/**
 * @author aschoerk
 */
public class ForTest {
	@Test
	public void canConvertCompleteForToRust() {
		String java = """
			class A {
			    void main() {
			        for (int i = 10; i < 100; i++)
			            System.out.println("i: " + i);
			        for (int i = 10; i < 100; i++) {
			            System.out.println("i: " + i);
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn main(&self) {
			        {
			            let i: i32 = 10;
			            while i < 100 {
			            System::out.println(format!("i: {}", i));
			            i += 1;
			            }
			            }
			            {
			            let i: i32 = 10;
			            while i < 100 {
			            System::out.println(format!("i: {}", i));
			            i += 1;
			            }
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertForWithDecrementToRust() {
		String java = """
			class A {
			    void countDown() {
			        for (int i = 10; i > 0; i--) {
			            System.out.println(i);
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn count_down(&self) {
			        {
			            let i: i32 = 10;
			            while i > 0 {
			            System::out.println(i);
			            i -= 1;
			            }
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertForEachToRust() {
		String java = """
			class A {
			    void iterate(int[] arr) {
			        for (int x : arr) {
			            System.out.println(x);
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn iterate(&self, arr: &[i32]) {
			        for x in arr {
			            System::out.println(x);
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertWhileLoopToRust() {
		String java = """
			class A {
			    void loopWhile() {
			        int i = 0;
			        while (i < 10) {
			            i++;
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn loop_while(&self) {
			        let i: i32 = 0;
			        while i < 10 {
			            i += 1;
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertDoWhileToRust() {
		String java = """
			class A {
			    void loopDoWhile() {
			        int i = 0;
			        do {
			            i++;
			        } while (i < 10);
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn loop_do_while(&self) {
			        let i: i32 = 0;
			        loop { i += 1;if !(i < 10) break;}
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertInfiniteForLoopToRust() {
		String java = """
			class A {
			    void infiniteLoop() {
			        for (;;) {
			            break;
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn infinite_loop(&self) {
			        loop {
			            break;
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertForWithMultipleInitToRust() {
		String java = """
			class A {
			    void multiInit() {
			        for (int i = 0, j = 10; i < j; i++, j--) {
			            System.out.println(i + " " + j);
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn multi_init(&self) {
			        {
			            let i: i32 = 0; let j: i32 = 10;
			            while i < j {
			            System::out.println(format!("{} {}", i, j));
			            i += 1; j -= 1;
			            }
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertBreakWithLabelToRust() {
		String java = """
			class A {
			    void labeledBreak() {
			        outer: for (int i = 0; i < 5; i++) {
			            for (int j = 0; j < 5; j++) {
			                if (j == 3) break outer;
			            }
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn labeled_break(&self) {
			        'outer: {
			            let i: i32 = 0;
			            while i < 5 {
			            'inner: {
			            let j: i32 = 0;
			            while j < 5 {
			            if j == 3 { break 'outer; }
			            j += 1;
			            }
			            }
			            i += 1;
			            }
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertContinueToRust() {
		String java = """
			class A {
			    void skipEven() {
			        for (int i = 0; i < 10; i++) {
			            if (i % 2 == 0) continue;
			            System.out.println(i);
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn skip_even(&self) {
			        {
			            let i: i32 = 0;
			            while i < 10 {
			            if i % 2 == 0 { continue; }
			            System::out.println(i);
			            i += 1;
			            }
			            }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
