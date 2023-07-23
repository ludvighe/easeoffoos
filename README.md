# Ease of Foos

A simple utility api.

## Lorem

Generates _lorem ipsum_ in the sense that it returns the given number of words and paragraphs from an excerpt of text containing a placeholder text known as "lorem ipsum".

```
GET /lorem/:words?p=:paragraphs&html=:html

:words      <int>
            The number of words
            clamped to 1 - 1 000 000

:paragraphs <optional int>
            The number of paragraphs
            clamped to 0 - :words
            separated by "\n\n" if not :html

:html       <optional bool>
            Determines if response should be with HTML-tags (<p></p>)
            accepts "true" or "1"

```

**Note** that the excerpt in use is, unlike classic _lorem ipsum_, from the start of section 1.10.32 through 1.10.33 (Book I. x) of "de Finibus Bonorum et Malorum" by Marcus Tullius Cicero, compiled by Harris Rackham 1914.

**Sourced from** https://archive.org/details/definibusbonoru02cicegoog

## Usage

### Debug

```sh
# Run run.sh to run server and autoreload on save (watch)
# cargo watch -q -c -w src/ -x run
./run.sh
```

```sh
# Test api (requires running server on default address)
cargo test
```

### Release

```sh
# Build
cargon build --release

# Run/Distribute
./target/release/easeoffoos
```

## Arguments

### Address

When running easeoffoos you can pass the IP-address and port as the first argument. Will default to `127.0.0.1:8080`.

```sh
# Debug example
cargo watch -q -c -w src/ -x 'run -- 0.0.0.0:8080'
```

```sh
# Release example
./easeoffoos 0.0.0.0:8080
```
