#!/bin/bash

PYTHON=${PYTHON:-python}

cd llvm-sys

rm -rf src
${PYTHON} -m rust_bindgen api src
