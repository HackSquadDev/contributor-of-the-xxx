{ pkgs }: {
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  deps = [
    pkgs.rustc
    pkgs.rustfmt
    pkgs.cargo
    pkgs.cargo-edit
    pkgs.rust-analyzer
  ];
}
