#!/bin/sh
# Copyright 2019 the lsctl authors. All rights reserved. MIT license.
# TODO(everyone): Keep this script simple and easily auditable.

set -e

if ! command -v unzip >/dev/null; then
	echo "Error: unzip is required to install lsctl." 1>&2
	exit 1
fi

if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="aarch64-apple-darwin" ;;
	*) target="x86_64-unknown-linux-gnu" ;;
	esac
fi

if [ $# -eq 0 ]; then
	lsctl_uri="https://github.com/lukeshay/lsctl/releases/latest/download/lsctl-${target}.zip"
else
	lsctl_uri="https://github.com/lukeshay/lsctl/releases/download/${1}/lsctl-${target}.zip"
fi

lsctl_install="${DENO_INSTALL:-$HOME/.lsctl}"
bin_dir="$lsctl_install/bin"
exe="$bin_dir/lsctl"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.zip" "$lsctl_uri"
unzip -d "$bin_dir" -o "$exe.zip"
chmod +x "$exe"
rm "$exe.zip"

echo "lsctl was installed successfully to $exe"
if command -v lsctl >/dev/null; then
	echo "Run 'lsctl --help' to get started"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export DENO_INSTALL=\"$lsctl_install\""
	echo "  export PATH=\"\$DENO_INSTALL/bin:\$PATH\""
	echo "Run '$exe --help' to get started"
fi
