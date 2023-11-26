#!/bin/bash

new_version=$1

# Install cargo-bump if it's not already installed
if ! command -v cargo-bump &> /dev/null; then
  cargo install cargo-bump
fi

# Find all subfolders in the crates directory
crate_names=()
crate_dirs=()
examples_dirs=()

while IFS= read -r -d $'\0' example_dir; do
  examples_dirs+=("$example_dir")
done < <(find examples -maxdepth 1 -type d -print0)

while IFS= read -r -d $'\0' crate_dir; do
  if [[ "$crate_dir" != "crates" ]]; then
    cargo_toml="$crate_dir/Cargo.toml"
    # Bump the version of each crate 
    cargo bump $new_version --manifest-path "$cargo_toml"
    # read the name of the crate from the Cargo.toml file (the very first).
    crate_name=$(grep -m 1 "name =" "$cargo_toml" | sed -e 's/.*= "//' -e 's/"//' | tr -d '[:space:]')
    echo "Found crate: $crate_name"

    crate_names+=("$crate_name")
    crate_dirs+=("$crate_dir")
  fi
done < <(find crates -maxdepth 1 -type d -print0)

echo ""

# Update the version of each crate used in the entire workspace
for ((i=0; i<${#crate_names[@]}; ++i)); do
  crate_name="${crate_names[$i]}"

  echo "Updating version of $crate_name to $new_version"
  # check both `crates` and examples folders for Cargo.toml files and update the version for $crate_name in each
  for dir in "${crate_dirs[@]}" "${examples_dirs[@]}"; do
    cargo_toml="$dir/Cargo.toml"
    if [[ -f "$cargo_toml" ]]; then
      # using perl, update the version which should be in format $crate_name = { version = "0.1.0", ... }
      perl -i -pe "s/($crate_name = \{ version = )\".*?(\",.*)/\1\"$new_version\2/" "$cargo_toml"
    fi
  done
  echo ""
done
