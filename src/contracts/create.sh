#!/bin/bash

quicktype \
    --lang=ts \
    --src-lang=schema \
    ./backend.schema.json \
    -o backend.ts 
