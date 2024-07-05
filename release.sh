#!/bin/bash

new_version=$1

# Install cocogitto if it's not already installed
if ! command -v cog &> /dev/null; then
  cargo install cocogitto
fi

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

# Bump the version of all crates
for crate_name in "${crate_names[@]}"; do
  echo "Bumping & Publishing version of $crate_name to $new_version"
  cog bump --minor --package "ngyn-$crate_name" --skip-untracked --skip-ci -d
  # sleep for two minutes to avoid rate limiting
  sleep 120 # 2 minutes
done


