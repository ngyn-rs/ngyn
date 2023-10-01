#!/bin/sh
# Copyright 2019 the Deno authors. All rights reserved. MIT license.
# Copyright 2022 the Backpack authors. All rights reserved. MIT license.

set -e

# assumes bin name same as project name
project="ngyn"
bin_name="ngyn"
repo="ngyn-rs/ngyn"

case "$OS" in
	"Windows_NT") target="x86_64-windows" ;;
	*)
		case $(uname -sm) in
			"Darwin x86_64") target="x86_64-macos" ;;
			"Darwin arm64") target="aarch64-macos" ;;
			*) target="x86_64-linux" ;;
		esac
		;;
esac

uri="https://github.com/${repo}/releases/${1:-latest}/download/${project}-${target}.tar.xz"

install="${PROJ_INSTALL:-$HOME/.$project-bin}"
bin_dir="$install"
exe="$bin_dir/$bin_name"

mkdir -p "$bin_dir"

curl --fail --location --progress-bar --output "$exe.tar.xz" "$uri"
tar zxf "$exe.tar.xz" -C "$bin_dir" --strip-components 1 
chmod +x "$exe"
rm "$exe.tar.xz"

echo "$project was installed successfully to $exe"
if command -v $exe >/dev/null; then
	echo "Run '$exe --help' to get started"
else
	shell_profile=".${SHELL##*/}rc"
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export PROJ_INSTALL=\"$install\""
	echo "  export PATH=\"\$PROJ_INSTALL:\$PATH\""
	echo "Run '$exe --help' to get started"
fi

