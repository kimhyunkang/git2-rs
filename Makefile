lib: git2.rs repository.rs ext.rs git_index.rs reference.rs
	rustc --lib git2.rs -O -Z debug-info

rgit: lib
	cd sample; ${MAKE}

test: lib
	cd test; ${MAKE}

doc:
	rustdoc --output-dir=docs --output-format=markdown git2.rs

clean:
	rm -rf *.dylib *.dSYM *.so *.o
	cd sample; ${MAKE} clean
	cd test; ${MAKE} clean
