(use-modules (gnu packages rust)
             (gnu packages gtk)
             (gnu packages rust-apps)
             (gnu packages tls)
             (gnu packages pkg-config)
             (gnu packages gnome))

(packages->manifest (list gtk
                          libadwaita
                          openssl
                          rust
                          (list rust "cargo")
                          (list rust "tools")
                          rust-analyzer))
