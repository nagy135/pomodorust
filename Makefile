install:
	mkdir -p /usr/share/pomodorust
	cp -R assets /usr/share/pomodorust
	RUSTUP_TOOLCHAIN=stable cargo install --path .
clean:
	rm -rf /user/share/pomodorust
	cargo uninstall
