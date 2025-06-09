#!/bin/bash

cd "`dirname "$0"`"

fmt="+%d"

today="`date "$fmt"`"

if [ "$1" != "" ]; then
    today="`date "$fmt" -d "$1 December 2024"`"
fi

clang-16 $CLANG_OPTS -std=c99 -lc -g `cat compile_flags.txt` -o day${today} day${today}.c -lm && ./day${today}
