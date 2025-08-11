(use-modules (guix gexp)
             (guix packages)
             ((guix licenses)
              #:prefix license:)
             (guix build-system cargo)
             (guix git-download)
             (gnu packages)
             (gnu packages crates-io)
             (gnu packages crates-crypto)
             (gnu packages crates-gtk)
             (gnu packages gtk)
             (gnu packages pkg-config)
             (gnu packages rust))

(define %source-dir
  (dirname (current-filename)))

(define sss-help-center-gtk
  (package
    (name "sss-help-center-gtk")
    (version "0.0.1")
    (source
     (local-file %source-dir
                 #:recursive? #t
                 #:select? (git-predicate %source-dir)))
    (build-system cargo-build-system)
    (arguments
     (list
      #:install-source? #f
      #:cargo-test-flags `(list "--release")
      #:cargo-inputs `(("rust-gtk" ,rust-gtk-0.14)
                       ("rust-libadwaita" ,rust-libadwaita-0.6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-base16ct" ,rust-base16ct-0.2)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (native-inputs (list pkg-config gtk))
    (home-page "https://codeberg.org/jjba23/sss-help-center-gtk")
    (synopsis "Help and assistance for SSS")
    (description "")
    (license license:gpl3)))

sss-help-center-gtk
