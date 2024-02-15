#!/usr/bin/env bash

asciidoctor-pdf --verbose -a source-highlighter=rouge -a rouge-style=colorful -r asciidoctor-diagram index.adoc
asciidoctor --verbose \
    -a source-highlighter=rouge \
    -a rouge-style=colorful \
    -r asciidoctor-diagram \
    -r asciidoctor-html5s \
    -b html5s \
    -r asciidoctor-multipage \
    -b multipage_html5 \
    -D . \
    index.adoc


## add this to the above command to generate html5s