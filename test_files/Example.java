import java.util.List;

public class Example extends B implements C {
	public static String what = "something in the what field";
	public int asdfds;
	private double my_double;

	public double lllllll;

	public static void main(String[] args) {
	}

	public long add2(long a, long b) throws Exception {
		var c = a + b;
		var d = c + 1;
		if (c > 0) {
			throw new RuntimeException("c is greater than 0");
		} else {
			return d;
		}
	}

	public int add(int a, int b) {
		try {
			return (int) add2(a, b);
		} catch (Exception e) {
			return 0;
		}
	}

	@Deprecated
	public MyClass1 weird(MyClass2 c1, MyClass1 c2, int b) {
		return new MyClass1();
	}
}

interface C {
	int add(int a, int b);
}

class B {
}

class MyClass1 {
}

class MyClass2 {
}

