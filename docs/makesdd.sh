#!/usr/bin/bash

# html
# pandoc sdd.md -o sdd.pdf -t html --template default.html5 --number-sections -M document-css=true -V papersize="Letter"

# latex
pandoc sdd.md -o sdd.pdf --number-sections -V geometry:margin=1in -V fontsize:12pt -V fontfamily:noto

firefox --new-window sdd.pdf
