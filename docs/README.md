# Render the documentation

To render this document as pdf and html and see the mermaid diagrams and have syntax highlighting, you need to:

- install diagram extension to asciidoctor `gem install asciidoctor-diagram`
- install mermaid command line `npm install -g @mermaid-js/mermaid-cli`
- install a syntax highlighter for asciidoctor-pdf `gem install rouge`
- install multipage html generator `gem install asciidoctor-multipage`
- render the documentation: `./render_doc.sh`
