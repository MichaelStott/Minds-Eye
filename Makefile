# Current SDL 2.0 release to target.
export SDL_URL?=https://github.com/libsdl-org/SDL/releases/download/release-2.26.5/SDL2-devel-2.26.5-VC.zip
export SDL_TTF_URL?=https://github.com/libsdl-org/SDL_ttf/releases/download/release-2.20.2/SDL2_ttf-devel-2.20.2-VC.zip
export SDL_IMG_URL?=https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.3/SDL2_image-devel-2.6.3-VC.zip
export SDL_MIX_URL?=https://github.com/libsdl-org/SDL_mixer/releases/download/release-2.6.3/SDL2_mixer-devel-2.6.3-VC.zip

# Include lib and dll files for compilation and execution.
export RUSTFLAGS=-L lib"
export SDL2_DIR="./lib"
export PATH:=$(SDL2_DIR);$(PATH)

clean: ## Remove all builds and Rust dependencies
	@cargo clean
	@if exist "release" del /S /Q release >nul 2>&1
	@if exist "release" rmdir /S /Q release >nul 2>&1
	@if exist "lib" del /S /Q lib >nul 2>&1
	@if exist "lib"  @rmdir /S /Q lib >nul 2>&1

lib: ## Download SDL2 dependencies
	@if not exist "lib" mkdir lib
	@curl -s -L --url $(SDL_URL) -o lib/sdl2.zip
	@tar -C lib --strip-components=3 -zxf lib/sdl2.zip SDL2-2.26.5/lib/x64/SDL2.dll 
	@tar -C lib --strip-components=3 -zxf lib/sdl2.zip SDL2-2.26.5/lib/x64/SDL2.lib 
	@curl -s -L --url $(SDL_TTF_URL) -o lib/ttf.zip
	@tar -C lib --strip-components=3 -zxf lib/ttf.zip SDL2_ttf-2.20.2/lib/x64/SDL2_ttf.dll 
	@tar -C lib --strip-components=3 -zxf lib/ttf.zip SDL2_ttf-2.20.2/lib/x64/SDL2_ttf.lib 
	@curl -s -L --url $(SDL_IMG_URL) -o lib/img.zip
	@tar -C lib --strip-components=3 -zxf lib/img.zip SDL2_image-2.6.3/lib/x64/SDL2_image.dll 
	@tar -C lib --strip-components=3 -zxf lib/img.zip SDL2_image-2.6.3/lib/x64/SDL2_image.lib 
	@curl -s -L --url $(SDL_MIX_URL) -o lib/mix.zip
	@tar -C lib --strip-components=3 -zxf lib/mix.zip SDL2_mixer-2.6.3/lib/x64/SDL2_mixer.dll 
	@tar -C lib --strip-components=3 -zxf lib/mix.zip SDL2_mixer-2.6.3/lib/x64/SDL2_mixer.lib 
	@echo off
	@del /S /Q lib\*.zip >nul 2>&1

update: ## Update project dependencies
	@cargo update

build: ## Pull dependencies and build project 
	@cargo build

dev: ## Run project in dev mode
	@cargo run

release: ## Compliles release folder with executable, dlls, and content
	md release
	cargo rustc --release -- -C link_args="-Wl,--subsystem,windows" -L lib
	@xcopy target\release\minds_eye.exe release
	@xcopy lib\\*.dll release
	@xcopy /S /Q /I res release\res

version:
	@echo 0.0.0

publish:
	echo TODO

_itch_io:
	echo TODO
