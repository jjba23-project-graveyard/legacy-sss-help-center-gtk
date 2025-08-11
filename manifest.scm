(use-modules (gnu packages rust)
             (gnu packages gtk))

(packages->manifest (list rust-analyzer rust gtk))
