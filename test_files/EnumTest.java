import java.util.Arrays;

public enum EnumTest {
	Variable1("Variable1", "v1"),
	Variable2("Variable2", "v2");

	private final String name;
	private final String value;

	EnumTest(String name, String value) {
		this.name = name;
		this.value = value;
	}

	public static EnumTest getByName(String name) {
		return Arrays.stream(EnumTest.values())
				.filter(e -> e.name.equals(name))
				.findFirst()
				.orElseThrow(() -> new IllegalArgumentException("No such enum constant " + name));
	}
}
