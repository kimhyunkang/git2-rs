lib: git2.rs repository.rs conditions.rs ext.rs types.rs index.rs reference.rs
	rustc --lib git2.rs -O

rgit: lib
	cd sample; ${MAKE}

test: git2_test
	RUST_THREADS=1 ./git2_test

git2_test: git2_test.rs lib
	rustc --test $< -L .

clean:
	rm -rf git2_test *.dylib *.dSYM *.so *.o
