#!/bin/bash

PYTHON=${PYTHON:-python}
BINDGEN_ENTRY_SCRIPT="../bindgen/main.py"
BINDGEN_ENTRY_SCRIPT=$(realpath ${BINDGEN_ENTRY_SCRIPT})

cd llvm-sys

rm -rf src
${PYTHON} ${BINDGEN_ENTRY_SCRIPT} api src
