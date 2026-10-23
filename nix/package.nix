{
  rustPlatform,
  lib,
}:
let
  fs = lib.fileset;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "mini_lang";
  inherit (finalAttrs.passthru.cargoToml.package) version;

  src = fs.toSource {
    root = ../.;
    fileset = fs.unions [
      ../src
      ../Cargo.lock
      ../Cargo.toml
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  passthru = {
    cargoToml = lib.importTOML ../Cargo.toml;
  };

  meta = {
    description = "Mini lang 'compiler'";
    homepage = "https://tangled.org/macuguita.com/mini_lang_rs";
    mainProgram = finalAttrs.pname;
    license = lib.licenses.eupl12;
    sourceProvenance = with lib.sourceTypes; [
      fromSource
    ];
  };
})
