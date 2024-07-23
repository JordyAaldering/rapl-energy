all: debug release

debug:
	cargo build

release:
	cargo build --release

install:
	cp target/release/librapl_energy.a /usr/local/lib/
	cp target/rapl_energy.h /usr/local/include/

uninstall:
	rm /usr/local/lib/librapl_energy.a
	rm /usr/local/include/rapl_energy.h
