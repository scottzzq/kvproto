#!/usr/bin/env bash

check_protoc_version() {
    major=$(protoc --version | awk -F"[ .]" '{print $2}')
    minor=$(protoc --version | awk -F"[ .]" '{print $3}')
    if [ "$major" != "3" ] || [ "$minor" != "1" ]; then
        echo "protoc version not match, version 3.1.x is needed"
        exit 1
    fi
}

push () {
    pushd $1 >/dev/null 2>&1
}

pop () {
    popd $1 >/dev/null 2>&1
}

cmd_exists () {
    which "$1" 1>/dev/null 2>&1
}
