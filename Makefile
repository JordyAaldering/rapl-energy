TARGET=target/release
LOCAL=/usr/local

all: debug release

debug:
	cargo build

release:
	cargo build --release

install:
	cp $(TARGET)/librapl_energy.a  $(LOCAL)/lib/
	cp $(TARGET)/librapl_energy.so $(LOCAL)/lib/
	cp $(TARGET)/rapl_energy.h     $(LOCAL)/include/

uninstall:
	rm $(LOCAL)/lib/librapl_energy.a
	rm $(LOCAL)/lib/librapl_energy.so
	rm $(LOCAL)/include/rapl_energy.h
