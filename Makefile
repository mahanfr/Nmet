INSTALL_PATH ?= /usr/bin 
BIN_FILE ?= target/release/nmet
DEV_FILE ?= target/debug/nmet

.DEFAULT_GOAL: build
.PHONY: install build test

# copies compiled program to system binary path 
# make install
# or
# make install INSTALL_PATH="/usr/local/bin"
install: build $(BIN_FILE) 
	@sudo cp $(BIN_FILE) $(INSTALL_PATH)
	@echo "Executable installed at" $(INSTALL_PATH)

# build the compiler to an executable
build:
	cargo build --release
