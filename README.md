# gbemu

## Setup

0. Requirements:
    1. [Rust](https://rust-lang.org/tools/install/) and `cargo`
    1. `rgbasm` from [RGBDS](https://rgbds.gbdev.io/install)
    1. [Make](https://www.gnu.org/software/make/)
0. Remember to clone with submodules: `git clone --recurse-submodules git@github.com:libardia/gbemu.git`
0. Optional: ensure sha256sums.txt in the roms-boot submodule has Unix (LF) line endings, or the hash check will fail. Even if it fails the roms are still built.
0. Run `make` in gb-bootroms
0. Ready.
