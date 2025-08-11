(use-modules (guix gexp)
             (guix packages)
             ((guix licenses) #:prefix license:)
             (guix build-system cargo)
             (gnu packages)
             (gnu packages rust))

(define %source-dir
  (dirname (current-filename)))

(define sss-help-center-gtk
  (package
    (name "sss-help-center-gtk")
    (version "0.0.1")
    (source (local-file %source-dir #:recursive? #t
                        #:select? (git-predicate %source-dir)))
    (arguments
     (list
      #:install-source? #f
      #:cargo-test-flags `(list "--release")
      #:cargo-inputs `(        ("rust-gtk" ,rust-gtk-0.14)
                               ("rust-serde" ,rust-serde-1)
                               ("rust-log" ,rust-log-0.4)
                               ("rust-base16ct" ,rust-base16ct-0.2)
                               ("rust-serde-json" ,rust-serde-json-1))))
    (home-page "https://codeberg.org/jjba23/sss-help-center-gtk")
    (synopsis "Help and assistance for SSS")
    (description "")
    (license license:gpl3)))

sss-help-center-gtk
