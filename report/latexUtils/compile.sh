#!/bin/bash

if [ "$1" == "reflection" ]; then
    cd /reflection && xelatex -synctex=1 -interaction=nonstopmode main.tex && rm -f *.aux *.log *.out *.synctex.gz
elif [ "$1" == "groupreport" ]; then
    cd /groupreport && xelatex -synctex=1 -interaction=nonstopmode main.tex && rm -f *.aux *.log *.out *.synctex.gz
else
    cd /reflection && xelatex -synctex=1 -interaction=nonstopmode main.tex && rm -f *.aux *.log *.out *.synctex.gz
    cd /groupreport && xelatex -synctex=1 -interaction=nonstopmode main.tex && rm -f *.aux *.log *.out *.synctex.gz
fi