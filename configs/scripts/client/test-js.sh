#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# go to parent folder
cd $(dirname $(dirname $(dirname $SCRIPT_DIR)))
WORKING_DIR=$(pwd)

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

for p in ${PROGRAMS[@]}; do
    cd ${WORKING_DIR}/clients/${p}/js
    pnpm install && pnpm build && pnpm test ${ARGS}
done