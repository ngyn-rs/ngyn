#!/bin/bash

new_version=$1

# Install smart-release if it's not already installed
cargo install smart-release

root_dir=$(git rev-parse --show-toplevel)

# Find all subfolders in the crates directory
crate_names=("shared" "macros" "hyper" "vercel" "shuttle")
examples_dirs=()

while IFS= read -r -d $'\0' example_dir; do
  if [[ $example_dir != "examples" ]]; then
    echo "Found example: $example_dir"
    examples_dirs+=("$example_dir")
  fi
done < <(find examples -maxdepth 1 -type d -print0)

# bump core version used in the examples
for example_dir in "${examples_dirs[@]}"; do
  echo "Bumping version in $example_dir"
  sed -i '' -e "s/^ngyn = .*/ngyn = \"$new_version\"/" $example_dir/Cargo.toml
done

# publish the version of all crates
for crate_name in "${crate_names[@]}"; do
  echo "Publishing version $new_version of $crate_name"
  cargo smart-release --execute --no-changelog -b keep -d keep $crate_name
  # sleep for two minutes to avoid rate limiting
  sleep 120 # 2 minutes
done

cd $root_dir/crates/core
# publish the version of the core crate
echo "Publishing version $new_version of ngyn"
cargo publish
