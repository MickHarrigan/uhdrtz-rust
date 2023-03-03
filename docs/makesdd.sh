#!/usr/bin/bash
#pandoc sdd.md -t html -M document-css=true -V papersize="Letter" --number-sections -o sdd.pdf
pandoc sdd.md -o sdd.pdf --number-sections -V geometry:margin=1in -V fontsize:12pt -V fontfamily:noto

firefox --new-window sdd.pdf
