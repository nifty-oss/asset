#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
PROGRAMS_OUTPUT="./programs/.bin"
# go to parent folder
cd $(dirname $(dirname $(dirname $SCRIPT_DIR)))

if [ ! -z "$PROGRAM" ]; then
    PROGRAMS='["'${PROGRAM}'"]'
fi

if [ -z "$PROGRAMS" ]; then
    PROGRAMS="$(cat .github/.env | grep "PROGRAMS" | cut -d '=' -f 2)"
fi

# default to input from the command-line
ARGS=$*

# command-line arguments override env variable
if [ ! -z "$ARGS" ]; then
    PROGRAMS="[\"${1}\"]"
    shift
    ARGS=$*
fi

PROGRAMS=$(echo $PROGRAMS | jq -c '.[]' | sed 's/"//g')

WORKING_DIR=$(pwd)
SOLFMT="solfmt"
export SBF_OUT_DIR="${WORKING_DIR}/${PROGRAMS_OUTPUT}"

# client SDK tests
for p in ${PROGRAMS[@]}; do
    cd ${WORKING_DIR}/clients/rust/${p}

    if [ ! "$(command -v $SOLFMT)" = "" ]; then
        CARGO_TERM_COLOR=always cargo test-sbf --sbf-out-dir ${WORKING_DIR}/${PROGRAMS_OUTPUT} ${ARGS} 2>&1 | ${SOLFMT}
    else
        cargo test-sbf --sbf-out-dir ${WORKING_DIR}/${PROGRAMS_OUTPUT} ${ARGS}
    fi
done