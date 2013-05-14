lib: git2.rs repository.rs
	rustc --lib git2.rs -O

test: git2_test
	./git2_test

git2_test: git2_test.rs lib
	rustc --test $< -L .

clean:
	rm -f git2_test *.dylib *.dSYM
