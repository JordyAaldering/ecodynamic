LOCAL ?= $(HOME)/.local

.PHONY: all debug release install uninstall clean

all: debug release

debug:
	cargo build

release:
	cargo build --release

install: release
	cp target/release/server $(LOCAL)/bin/mtdynamic

uninstall:
	$(RM) $(LOCAL)/bin/mtdynamic

clean:
	cargo clean
