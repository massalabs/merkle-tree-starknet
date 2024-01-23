#!/bin/bash

function create_venv() {
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
}
function load_venv() {
    if [ ! -f venv/bin/activate ]; then
        echo "Virtual environment does not exist"
        exit 1
    fi

    echo "Activating python virtual environment"
    source venv/bin/activate

    if [ $? -ne 0 ]; then
        echo "Error: failed to activate virtual environment"
        exit 1
    fi

    echo "Tests will run with $VIRTUAL_ENV virtual environment"
}

function clone_cairo_lang() {
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
}
function checkout_cairo_lang_tag() {
    pushd .
    cd cairo-lang
    git checkout $1
    if [ $? -ne 0 ]; then
        echo "Error: failed to checkout cairo-lang tag $1"
        exit 1
    fi
    popd
}

# are we in the right python virtual environment?
# e.g. $VIRTUAL_ENV == $(pwd)/venv
if [ "$VIRTUAL_ENV" != "$(pwd)/venv" ]; then
    echo "Warning: you shell is not in the right python virtual environment"
    echo "You should run '. venv/bin/activate' to fix this"
fi

# Setup Python stuff
create_venv
load_venv
pip install --upgrade pip &> /dev/null
pip install --upgrade pytest &> /dev/null
pip install --upgrade cffi &> /dev/null

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
clone_cairo_lang
checkout_cairo_lang_tag "v0.13.0"

# build rust_ffi
pushd .
cd ../rust/rust_ffi && cargo build --release && popd

# run our ts test
python -m main >result.txt
cat result.txt
# python -m main >result.txt

# # diff the result
# diff result.txt expected.txt

# # if error print it
# if [ $? -ne 0 ]; then
#     echo "Error: result.txt and expected.txt are not the same"
#     exit 1
# else
#     echo "You're the boss test passed"
#     exit 0
# fi
