# Include lib and dll files for compilation and execution.
export RUSTFLAGS=-L sdl2
export SDL2_DIR="./sdl2"
export PATH:=$(SDL2_DIR);$(PATH)

clean:
	@cargo clean

dep: ## Download SDL2 dependencies
	@echo TODO

update:
	@cargo update
	
build: ## Pull dependencies and build project 
	@cargo build

dev: ## Run project in dev mode
	@cargo run

release: ## Build EXE folder and distributable project folder
	@cargo build --release

version:
	@echo 0.0.0