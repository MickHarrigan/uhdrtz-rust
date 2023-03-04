#!/usr/bin/bash

# html
pandoc sdd.md -o sdd.pdf -t html --template=templates/default.html5 --number-sections -V mainfont="Noto Serif" -V margin-left="1in" -V margin-right="1in" -V margin-top="1in" -V margin-bottom="1in" -V papersize="Letter"

firefox --new-window sdd.pdf
