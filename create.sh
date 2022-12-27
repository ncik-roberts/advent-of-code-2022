#!/bin/bash
set -euo pipefail

[ "$#" = 1 ] || exit 1

name=day"$1"
cargo init "$name" --vcs none
echo 'advent = { path = "../advent" }' >> "$name"/Cargo.toml
cat > "$name"/src/main.rs <<EOF
  fn part1(_lines: advent::LineStream) {
    ()
  }

  fn part2(_lines: advent::LineStream) {
    ()
  }

  fn main() {
    advent::of_code(advent::PARSE_AS_STREAM, part1, part2)
  }
EOF
