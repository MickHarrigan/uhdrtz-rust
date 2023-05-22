#!/usr/bin/sh

pandoc sdd.md -o sdd.pdf -t html --template=templates/sdd.html5 --number-sections -M title="System Design Document" -V mainfont="Noto Serif" -V monofont="Hack" -V monobackgroundcolor=#202020 -V linkcolor=blue -V maxwidth=8.5in -V backgroundcolor=white -V margin-left=1in -V margin-right=1in -V margin-top=1in -V margin-bottom=1in -V papersize=Letter --pdf-engine-opt=--enable-local-file-access

#firefox --new-window sdd.pdf
