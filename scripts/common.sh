#!/usr/bin/env bash

file_path=""
dir_path=""

function parse_args() {
	while getopts ":f:d:" opt $@; do
		case $opt in
		f)
			file_path="$OPTARG"
			;;
		d)
			dir_path="$OPTARG"
			;;
		\?)
			echo "Invalid option: -$OPTARG"
			exit 1
			;;
		:)
			echo "Option -$OPTARG requires an argument." >&2
			exit 1
			;;
		esac
	done
	shift $((OPTIND - 1))

	if [ -n "$file_path" ] && [ -n "$dir_path" ]; then
		echo "Error: options -f and -d are exclusive and cannot be used together."
		exit 1
	fi

	if [ -z "$file_path" ] && [ -z "$dir_path" ]; then
		echo "Error: either -f or -d must be provided."
		exit 1
	fi
}

build_rust_ffi() {
	rust_ffi_path="$(dirname "$0")/../rust_ffi"
	# if we have the sources of the lib, let's build it and use it
	if [[ -f "$rust_ffi_path"/Cargo.toml ]]; then
		echo "Building rust_ffi"
		pushd "$rust_ffi_path"
		cargo xtask || exit 1
		popd
	# else if we have the binary version, let's use it
	elif [ -f "$rust_ffi_path"/librust_ffi.so ] || [ -f "$rust_ffi_path"/librust_ffi.dylib ] || [ -f "$rust_ffi_path"/rust_ffi.dll ]; then
		echo "Using binary library rust_ffi"
	# if none, exit with error
	else
		echo "rust_ffi not found, exiting"
		exit 1
	fi
}
