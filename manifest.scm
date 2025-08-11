(use-modules (gnu packages rust)
             (gnu packages gtk)
             (gnu packages gnome))

(packages->manifest (list rust-analyzer rust gtk libadwaita))
