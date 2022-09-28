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

# $(LATEX) $(NOPDF) $(ARGS) $(PAPER).tex

$(PDF): $(REQUIRE) FORCE
	-mkdir -p dist
	$(LATEX) $(ARGS) $(PAPER).tex

clean:
	-rm -rf $(TEMP)

FORCE:
.PHONY: all clean FORCE