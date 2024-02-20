#!/usr/bin/env bash

source "$(dirname "$0")/../scripts/common.sh"

file_path=""
dir_path=""

while getopts ":f: d:" opt; do
    case $opt in
    f)
        file_path="$OPTARG"
        ;;
    d)
        dir_path="$OPTARG"
        ;;
    \?)
        echo "Invalid option: -$OPTARG" >&2
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

# Setup Python stuff
if [ ! -f venv/bin/activate ]; then
    echo "Creating python virtual environment"
    python3 -m venv venv
    if [ $? -ne 0 ]; then
        echo "Error: failed to create virtual environment"
        exit 1
    fi
else
    echo "Virtual environment already exists"
fi

echo "Activating python virtual environment"
source venv/bin/activate

if [ $? -ne 0 ]; then
    echo "Error: failed to activate virtual environment"
    exit 1
fi

echo "Tests will run with $VIRTUAL_ENV virtual environment"

pip install --upgrade pip &>/dev/null
pip install --upgrade pytest &>/dev/null
pip install --upgrade icecream &>/dev/null
pip install -e ../python_common &>/dev/null

# check if cairo-lang is installed with pip list
# if not install it
pip list | grep cairo-lang | grep "0.13.0"
if [ $? -ne 0 ]; then
    echo "Installing cairo-lang"
    pip install -U https://github.com/starkware-libs/cairo-lang/releases/download/v0.13.0/cairo-lang-0.13.0.zip
    if [ $? -ne 0 ]; then
        echo "Error: failed to install cairo-lang"
        exit 1
    fi
else
    echo "Cairo-lang already installed"
fi

# needed to fix sys.path, can't make it work with cairo-lang installed with pip
if [ ! -d cairo-lang ]; then
    echo "Cloning cairo-lang"
    git clone git@github.com:starkware-libs/cairo-lang.git
    if [ $? -ne 0 ]; then
        echo "Error: failed to clone cairo-lang"
        exit 1
    fi
else
    echo "Cairo-lang already exists"
fi

pushd cairo-lang
git checkout v0.13.0
if [ $? -ne 0 ]; then
    echo "Error: failed to checkout cairo-lang tag v0.13.0"
    exit 1
fi
popd

build_rust_ffi

if [ -n "$file_path" ]; then
    python main.py -f "$file_path"
elif [ -n "$dir_path" ]; then
    python main.py -d "$dir_path"
else
    python main.py
fi
