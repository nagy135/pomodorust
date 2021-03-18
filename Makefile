install:
	mkdir -p /usr/share/pomodorust
	cp -R assets /usr/share/pomodorust
	RUSTUP_TOOLCHAIN=stable cargo build
	cp ./target/debug/pomodorust /usr/bin/pomodorust
clean:
	rm -rf /user/share/pomodorust
	rm /usr/bin/pomodorust
