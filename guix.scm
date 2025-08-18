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
             (gnu packages gnome)
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
      #:phases `(modify-phases %standard-phases
                  (add-before 'build 'install-program-files
                    (lambda* (#:key outputs #:allow-other-keys)
                      (let ((bin (string-append (assoc-ref outputs "out")
                                                "/bin"))
                            (share (string-append (assoc-ref outputs "out")
                                                  "/share")))
                        (mkdir-p (string-append share "/resources/img"))
                        (mkdir-p (string-append share "/resources/locale"))                        
                        (install-file "resources/img/sss.png"
                                      (string-append share "/resources/img/"))
                        (copy-recursively "resources/locale"
                                          (string-append share "/resources/locale"))))))
      #:install-source? #t
      #:cargo-test-flags `(list "--release")
      #:cargo-inputs `(("rust-gtk" ,rust-gtk-0.14)
                       ("rust-libadwaita" ,rust-libadwaita-0.6)
                       ("rust-serde" ,rust-serde-1)
                       ("rust-log" ,rust-log-0.4)
                       ("rust-gettext-sys" ,rust-gettext-sys-0.21)
                       ("rust-gettext-rs" ,rust-gettext-rs-0.7)
                       ("rust-base16ct" ,rust-base16ct-0.2)
                       ("rust-serde-json" ,rust-serde-json-1))))
    (native-inputs (list pkg-config gtk libadwaita))
    (home-page "https://codeberg.org/jjba23/sss-help-center-gtk")
    (synopsis "Help and assistance for SSS")
    (description "")
    (license license:gpl3)))

sss-help-center-gtk
