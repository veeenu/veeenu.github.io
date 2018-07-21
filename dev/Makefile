# Actually ignore this file, simply do
# $ pug --watch dev/pug -o .
# $ stylus -w -o css dev
# $ jekyll serve

DIR  := ..
PUG := $(shell find pug -name '*.pug')
HTML := $(PUG:pug/%.pug=$(DIR)/%.html)

$(DIR)/%.html: pug/%.pug
  pug < $< > $@ 

$(DIR)/css/style.css: style.styl reset.styl
  stylus style.styl -o ../css

pug: $(HTML)
  @:

stylus: $(DIR)/css/style.css
  @:

all: pug stylus
  @:

watch:
  while true; do $(MAKE) -s all; sleep 1; done

.PHONY: pug stylus all watch
