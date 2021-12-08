#!/bin/sh

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 NUM" >&2
    exit 2
fi

DIR="adv-$1"

mkdir $DIR
ln -s "../makefiles/day-$1" $DIR/Makefile
