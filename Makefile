SHELL=/bin/bash

# Or just CARGO=cargo
CARGO=docker run --rm \
  --user $$(id -u):$$(id -g) \
  --env PATH=/home/rust/.cargo/bin:$$PATH \
  --env OPENSSL_LIB_DIR=/usr/local/musl/lib/ \
  --env OPENSSL_INCLUDE_DIR=/usr/local/musl/include \
  --env OPENSSL_STATIC=true \
  --env PKG_CONFIG_ALLOW_CROOSS=1 \
  -v $$PWD:/home/rust/dev \
  -v $$HOME/.cargo/registry:/home/rust/.cargo/registry \
  -w /home/rust/dev \
  rust:latest \
  cargo


.PHONY: all
all: prebuild build


.PHONY: build
build:
	@$(CARGO) build --lib --target=x86_64-unknown-linux-musl --release


.PHONY: prebuild
prebuild:
	@$(CARGO) +stable fmt
	@$(CARGO) clippy


.PHONY: clean
clean:
	@$(CARGO) clean


.PHONY: test
test:
	@$(CARGO) test


.PHONY: example
example:
	@./target/debug/examples/color_output


.PHONY: version
version:
	@$(CARGO) --version


.PHONY: ldd
ldd:
	@basename `pwd` | xargs -I@ find ./target -type f -name @ -exec ldd {} \;


.PHONY: run
run:
	@basename `pwd` | xargs -I@ find ./target -type f -name @ -exec bash -c {} \;
