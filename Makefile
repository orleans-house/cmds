TOOLS := $(notdir $(wildcard crates/*))
TOOLS := $(filter-out cli-common,$(TOOLS))

.PHONY: build install clean

build:
	cargo build --release

install:
	@for tool in $(TOOLS); do \
		cargo install --path crates/$$tool --force; \
	done

clean:
	cargo clean
