install:
	cargo build --release
	nu -c "register $(PWD)/target/release/nu_plugin_from_prometheus"
	