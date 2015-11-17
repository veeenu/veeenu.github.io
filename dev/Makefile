DIR  := ..
JADE := $(shell find jade -name '*.jade')
HTML := $(JADE:jade/%.jade=$(DIR)/%.html)

$(DIR)/%.html: jade/%.jade
	jade < $< > $@ 

$(DIR)/css/style.css: style.styl reset.styl
	stylus style.styl -o ../css

jade: $(HTML)
	@:

stylus: $(DIR)/css/style.css
	@:

all: jade stylus
	@:

watch:
	while true; do $(MAKE) -s all; sleep 1; done

.PHONY: jade stylus all watch
