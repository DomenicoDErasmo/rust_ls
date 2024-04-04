# LS

Personal LS implementation.

# Building in NixOS
1. Download rustup with `nix-shell ./build/shell.nix`
2. Download rust with  `rustup default stable`

# Notes
- Adding all lints back in doesn't work for some reason, but only having clippy::missing_inline_in_public_items does
- TODO: figure out why