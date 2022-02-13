default: build

build: build_daemon build_ui

create_output_dir:
	mkdir -p out

build_daemon: create_output_dir
	$(MAKE) -C mediarepo-daemon build
	cp mediarepo-daemon/target/release/mediarepo-daemon* out/

build_ui: create_output_dir
	$(MAKE) -C mediarepo-ui build
	cp mediarepo-ui/src-tauri/target/release/mediarepo-ui* out/
	cp mediarepo-ui/src-tauri/target/release/bundle out/
	cp mediarepo-ui/src-tauri/icons out/