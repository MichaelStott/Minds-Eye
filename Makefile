# Include lib and dll files for compilation and execution.
export RUSTFLAGS=-L sdl2"
export SDL2_DIR="./sdl2"
export PATH:=$(SDL2_DIR);$(PATH)

clean:	
	@cargo clean
	@del /S /Q release
	@rmdir /S /Q release
	
dep: ## Download SDL2 dependencies
	@echo TODO

update: ## Update project dependencies
	@cargo update

build: ## Pull dependencies and build project 
	@cargo build

dev: ## Run project in dev mode
	@cargo run

final: ## Compliles release folder with executable, dlls, and content
	md release
	cargo rustc --release -- -C link_args="-Wl,--subsystem,windows" -L sdl2
	@xcopy target\release\minds_eye.exe release
	@xcopy sdl2\\*.dll release
	@xcopy /S /Q /I res release\res

version:
	@echo 0.0.0

publish:
	echo TODO

_itch_io:
	echo TODO
