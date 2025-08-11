(use-modules (gnu packages rust)
             (gnu packages gtk)
             (gnu packages rust-apps)
             (gnu packages gnome))

(packages->manifest (list rust-analyzer rust gtk libadwaita rust-cargo))
