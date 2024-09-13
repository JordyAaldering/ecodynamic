LOCAL=$(HOME)/.local

all: debug

debug:
	cargo build --lib --bins --examples

release:
	cargo build --release --lib --bins --examples

install: debug release
	cp target/release/mtdynamic.h $(LOCAL)/include/
	cp target/release/libmtdynamic.so $(LOCAL)/lib/

uninstall:
	$(RM) $(LOCAL)/include/mtdynamic.h
	$(RM) $(LOCAL)/lib/libmtdynamic.so

clean:
	cargo clean
