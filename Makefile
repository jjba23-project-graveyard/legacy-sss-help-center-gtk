fmt:
	guix shell -m manifest.scm -- rustfmt --verbose \
		./src/main.rs ./src/keybinds.rs ./src/widgets.rs
run:
	RUST_LOG=debug guix shell -f guix.scm -- sss-help
