asciidoctor-pdf --verbose -a source-highlighter=rouge -a rouge-style=colorful -r asciidoctor-diagram index.adoc
asciidoctor --verbose \
    -a source-highlighter=rouge \
    -a rouge-style=colorful \
    -r asciidoctor-diagram \
    index.adoc

## add this to the above command to generate multipage html
    # -r asciidoctor-multipage \
    # -b multipage_html5 \
    # -D . \

## add this to the above command to generate html5s
    # -r asciidoctor-html5s \
    # -b html5s \