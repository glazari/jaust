import java.util.List;

public class Example extends B implements C {
	public static String what = "Field"	;
	public int asdfds;
	private double my_double;


	public double lllllll;


	public static void main(String[] args) {
	}

	public int add(int a, int b) {
		return a + b;
	}
}

interface C {
	int add(int a, int b);
}
