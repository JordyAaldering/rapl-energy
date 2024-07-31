RELEASE=target/release
LOCAL=$(HOME)/.local

all: debug

debug:
	cargo build

release:
	cargo build --release

install: release
	cp $(RELEASE)/rapl_energy.h $(LOCAL)/include/
	cp $(RELEASE)/librapl_energy.so $(LOCAL)/lib/

uninstall:
	rm $(LOCAL)/include/rapl_energy.h
	rm $(LOCAL)/lib/librapl_energy.so
