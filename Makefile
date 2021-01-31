	
DOTS=$(wildcard *.dot)
PNGS=$(DOTS:.dot=.png)

build_maps: $(PNGS)

%.png : %.dot
	sfdp -Tpng -o $@ $<

clean:
	-rm *.csv
	-rm *.dot
	-rm *.png

.PHONY: build_maps, clean