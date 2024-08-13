RELEASE=target/release
LOCAL=$(HOME)/.local

all: debug

debug:
	cargo build

release:
	cargo build --release

install: debug release
	cp $(RELEASE)/mtdynamic.h $(LOCAL)/include/
	cp $(RELEASE)/libmtdynamic.so $(LOCAL)/lib/

uninstall:
	$(RM) $(LOCAL)/include/mtdynamic.h
	$(RM) $(LOCAL)/lib/libmtdynamic.so
