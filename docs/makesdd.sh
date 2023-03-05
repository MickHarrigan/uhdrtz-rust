#!/usr/bin/bash

pandoc sdd.md -o sdd.pdf -t html --template=templates/default.html5 --number-sections -M title="System Design Document" -V mainfont="Noto Serif" -V fontsize -V maxwidth=8.5in -V backgroundcolor=white -V margin-left=1in -V margin-right=1in -V margin-top=1in -V margin-bottom=1in -V papersize=Letter

firefox --new-window sdd.pdf

#pandoc sdd.md -o sdd.html --template=templates/default.html5 --number-sections -V mainfont="Noto Serif" -V maxwidth=8.5in -V backgroundcolor=white -V margin-left=1in -V margin-right=1in -V margin-top=1in -V margin-bottom=1in -V papersize=Letter --verbose

#firefox --new-window sdd.html

#--shift-heading-level-by=-1
#