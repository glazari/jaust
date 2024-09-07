# Class File Parsing

The class file follows the following structure [docs](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1)


```
ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
```

Parsing is done by reading the number of bytes the documentation describes for each part of the document.
The constant pool is the one that contains all of the strings in the file the rest of the file
contains references to indexes in the constant pool.


# Parsing method and field descriptors

the field and method descriptors are stored in a special
string format that does reminds the java syntax but is not the same.

example:
```
Lcom/example/Example; -> com.example.Example
(II)V -> void method(int, int)
([Ljava/lang/String;)V -> void method(String[])
([I)V -> void method(int[])
```
