BUILD_ROOT := target
RTFM = BUILD_ROOT/release/rtfm

.PHONY: all clean default install

all: $(RTFM)

clean:
	cargo clean
	@rm -rf BUILD_ROOT

default: all

install:
	echo "foo"

$(RTFM):
	cargo build --release