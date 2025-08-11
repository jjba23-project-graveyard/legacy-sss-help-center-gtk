(use-modules (gnu packages rust)
             (gnu packages gtk)
             (gnu packages rust-apps)
             (gnu packages crates-io)
             (gnu packages gnome))

(packages->manifest (list rust-analyzer
                          rust
                          gtk
                          libadwaita
                          rust-cargo
                          (specification->package "rust-clippy")))

