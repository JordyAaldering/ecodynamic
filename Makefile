LOCAL ?= $(HOME)/.local

.PHONY: all debug release install uninstall clean

all: debug release

debug:
	cargo build

release:
	cargo build --release

install: release
	cp target/release/server $(LOCAL)/bin/ecodynamic

uninstall:
	$(RM) $(LOCAL)/bin/ecodynamic

clean:
	cargo clean
