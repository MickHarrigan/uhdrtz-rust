#!/usr/bin/bash
pandoc sdd.md -t html --number-sections -V margin-top:"1in" -V margin-bottom:"1in" -V margin-left:"0.5in" -V margin-right:"1in" -o sdd.pdf

# Gnome document viewer (Debian)
evince sdd.pdf
