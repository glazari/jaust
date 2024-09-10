set export

file := "./test_class_files/com/example/RecordTest.class"

compile_test_files:
	rm -r ./test_class_files/*
	javac -d ./test_class_files ./test_files/*.java

# get file from args
diff:
	@echo "Processing " $file
	@javap -p $file > ./javapoutput.txt
	@./target/release/jaustp -p $file > ./jaustpoutput.txt
	@echo "\033[1;32mJaustp output\033[0m"
	@cat ./jaustpoutput.txt
	@echo "\033[1;32mJavap output\033[0m"
	@cat ./javapoutput.txt
	@echo "\033[1;32mDiff\033[0m"
	@delta ./jaustpoutput.txt ./javapoutput.txt


diff_full:	
	@echo "Processing " $file
	@javap -p -c $file > ./javapoutput.txt
	@./target/release/jaustp -p -c $file > ./jaustpoutput.txt
	@echo "\033[1;32mJaustp output\033[0m"
	@cat ./jaustpoutput.txt
	@echo "\033[1;32mJavap output\033[0m"
	@cat ./javapoutput.txt
	@echo "\033[1;32mDiff\033[0m"
	@delta ./jaustpoutput.txt ./javapoutput.txt

fullp:
	./target/release/jaustp -p -c $file
	@echo "\033[1;32mGenerics still in output:\033[0m"
	./target/release/jaustp -p -c $file | grep -i generic

raw:
	./target/release/jaustp --raw $file
	@echo "\033[1;32mGenerics still in output:\033[0m"
	./target/release/jaustp --raw $file | grep -i generic

