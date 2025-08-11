nix-run:
	RUST_LOG=debug nix develop -c cargo run
run:
	RUST_LOG=debug guix shell -f guix.scm -- sss-help
