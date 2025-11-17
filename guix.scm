(use-modules (guix gexp)
             (guix packages)
             ((guix licenses)
              #:prefix license:)
             (guix build-system cargo)
             (guix git-download)
             (gnu packages)
             (gnu packages gtk)
             (gnu packages gnome)
             (gnu packages tls)
             (gnu packages pkg-config)
             (guix build-system cargo)
             (guix import crate)             
             (gnu packages rust))

(define %source-dir
  (dirname (current-filename)))

(define sss-help-center-gtk
  (package
    (name "sss-help-center-gtk")
    (version "dev")
    (source
     (local-file %source-dir
                 #:recursive? #t
                 #:select? (git-predicate %source-dir)))
    (build-system cargo-build-system)
    (native-inputs (list pkg-config gtk libadwaita))
    (inputs (cons* openssl
                   (cargo-inputs-from-lockfile "Cargo.lock")))
    (arguments
     (list
      #:install-source? #f
      #:tests? #f
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
                                          (string-append share
                                                         "/resources/locale"))))))))
    (home-page "https://codeberg.org/jjba23/sss-help-center-gtk")
    (synopsis "Help and assistance for SSS")
    (description "")
    (license license:gpl3)))

sss-help-center-gtk

