# Kipos

Kipos is a restful active static site generator. The idea is a consolidation of posts from multiple services. Some assumptions and goals:

### Everything is a garden

We view Twitter and a Blog as different things. their streams flow at different speeds but ultimately they are both gardens. When we view our internet as a bunch of gardens, it opens up many opportunities for representation of all these gardens blended together, or individually. Do [[pull]] [[Garden Representations]]!

### Restful and active

Despite being a static site generator, if you leave the daemon running then it will continue to update the static files as new things come in. At any time copy the files elsewhere and use them with zero configuration

### Configurable

Everything should be an option

## Plugins

There are three steps

### seed

Fetch content, make hashmap of metadata. You should export content, but feel no pressure to convert it to HTML. Export what makes sense for the format

- Plaintext
- Markdown
- JSON
- Html

### resn

Format those exports into garden respesentations:

- Stream
- graph

## High level

1. Seed
2. Apply seeds to all templates
3.
