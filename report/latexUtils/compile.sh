#!/bin/bash
cd /reflection && xelatex -synctex=1 -interaction=nonstopmode main.tex
cd /groupreport && xelatex -synctex=1 -interaction=nonstopmode main.tex