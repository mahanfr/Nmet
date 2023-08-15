SHELL= bash
.DEFAULT_GOAL := install
.PHONY : install build bin 

install: build bin
	@echo "\n---successfully installed---"

.hiddentarget: build:
	cargo build
	@echo "---successfully built---"

.hiddentarget: bin:
	@sudo cp target/debug/nemet /usr/bin/
	@echo "---added to /user/bin/---"
