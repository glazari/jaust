import java.util.List;

public class Example extends B implements C {
	public static String what = "Field"	;
	public int asdfds;
	private double my_double;


	public double lllllll;


	public static void main(String[] args) {
	}

	public long add2(long a, long b) {
		return a + b;
	}
	public int add(int a, int b) {
		return 2;
	}

	public MyClass1 weird(MyClass2 c1, MyClass1 c2, int b) {
		return new MyClass1();
	}
}

interface C {
	int add(int a, int b);
}

class MyClass1 {}
class MyClass2 {}
