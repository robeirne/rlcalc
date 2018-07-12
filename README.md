# rlcalc
Calculate length of rolled goods (fabric, paper, etc.) based on roll diameter, core diameter, and material thickness

## Usage
```sh
rlcalc -r <ROLL_DIAMETER> -c <CORE_DIAMETER> -t <MATERIAL_THICKNESS>
```

## Install
#### * Requires `rustc` and `cargo` to build
```sh
git clone https://github.com/robeirne/rlcalc
cd rlcalc
cargo build && cargo install
```

## Notes
- All values supplied on the command line must be in the same units (e.g. inches, centimeters, etc.).

### TODO
- Add support for differing units (e.g. roll diameter in inches, material thickness in mils, etc.).
- The flags for `-rct` are probably unnecessary as the values can be inferred by the size (i.e. roll > core > thickness).