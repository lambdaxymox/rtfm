BUILD_ROOT := target
RELEASE_ROOT := $(BUILD_ROOT)/release
DOC_ROOT := docs
DOC_SOURCE := rtfm.1
DOC_TARGET := rtfm.1.gz
RTFM := $(RELEASE_ROOT)/rtfm
DOCS := $(RELEASE_ROOT)/$(DOC_TARGET)
INSTALL_PATH := /usr/local


.PHONY: all clean default install

all: $(RTFM)

clean:
	@cargo clean
	@rm -rf BUILD_ROOT

default: all

install:
	cp "$(RTFM)" "$(INSTALL_PATH)/bin"
	cp "$(DOCS)" "$(INSTALL_PATH)/share/man/man1"


$(RTFM):
	@cargo build --release
	@gzip --keep "$(DOC_ROOT)/$(DOC_SOURCE)"
	@mv "$(DOC_ROOT)/$(DOC_TARGET)" "$(DOCS)"
