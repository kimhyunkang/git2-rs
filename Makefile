lib: git2.rs repository.rs conditions.rs ext.rs index.rs reference.rs
	rustc --lib git2.rs -O

rgit: lib
	cd sample; ${MAKE}

test: lib
	cd test; ${MAKE}

clean:
	rm -rf *.dylib *.dSYM *.so *.o
	cd sample; ${MAKE} clean
	cd test; ${MAKE} clean
