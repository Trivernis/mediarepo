default: build

build: install_tauri install_angular
	cargo tauri build --verbose

install_tauri:
	cargo install tauri-cli --version ^1.0.0-rc.4

install_angular:
	npm i --save-dev @angular/cli