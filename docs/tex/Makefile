PAPER    = main
TEMPLATE = hcr-cumcm
PDF      = $(PAPER).pdf
REQUIRE  = $(PAPER).tex $(TEMPLATE).sty
TEMP     = *.aux sections/*.aux *.log *.out *.toc *.xdv *.bbl *.blg *.bcf *.synctex.gz *.run.xml *.markdown.* *_markdown_* dist/

LATEX    = xelatex
BIBER    = biber
NOPDF    = --no-pdf
ARGS     = --shell-escape

ifndef LAB
	LAB := lab1
endif

all: $(PDF)

$(PDF): images $(REQUIRE) FORCE
	$(LATEX) $(ARGS) $(PAPER).tex
	cp $(PDF) ../$(LAB)/密码学实验-$(LAB).pdf

images:
	-cp -r ../$(LAB)/*.assets .

clean:
	-rm -rf $(TEMP)

FORCE:
.PHONY: all clean images FORCE