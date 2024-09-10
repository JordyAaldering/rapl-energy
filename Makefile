LOCAL=$(HOME)/.local

all: debug

debug:
	cargo build

release:
	cargo build --release

install: release
	cp target/release/rapl_energy.h $(LOCAL)/include/
	cp target/release/librapl_energy.so $(LOCAL)/lib/

uninstall:
	$(RM) $(LOCAL)/include/rapl_energy.h
	$(RM) $(LOCAL)/lib/librapl_energy.so

clean:
	cargo clean
