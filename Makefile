all: debug

debug:
	cargo build

release:
	cargo build --release

install: release
	cp target/release/rapl_energy.h $(HOME)/.local/include/
	cp target/release/librapl_energy.so $(HOME)/.local/lib/

uninstall:
	$(RM) $(HOME)/.local/include/rapl_energy.h
	$(RM) $(HOME)/.local/lib/librapl_energy.so

clean:
	cargo clean
