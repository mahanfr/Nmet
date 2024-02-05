SHELL= bash
.DEFAULT_GOAL := install
.PHONY : install build bin 

install: --build --bin
	@echo "---successfully installed---"

--build:
	cargo build --release
	@echo "---successfully built---"

--bin:
	@sudo cp target/debug/nemet /usr/bin/
	@echo "---added to /user/bin/---"
