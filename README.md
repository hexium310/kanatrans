# Kanatrans

This is a web application that converts an English word to Katakana.

## Dependency

- [flite](https://github.com/festvox/flite)

If the `vendored` Cargo feature is enabled, it compiles and statically links to a copy of flite.

## Usage

```sh
# Get ARPAbet list
$ curl -s localhost:8080/arpabet/kanatrans
{"word":"kanatrans","pronunciation":["k","ae1","n","ax0","t","r","ax0","n","z"]}

# Get Katakana: as a query parameter, pass space-separated ARPAbet to `pronunciation`
# The parameter `word` is optional; if it's set the given `word` will be responded as it is
$ curl -s localhost:8080/katakana --get --data-urlencode pronunciation="k ae1 n ax0 t r ax0 n z"
{"word":null,"pronunciation":"カナトランズ"}

# Oneliner
$ curl -s localhost:8080/katakana --get --data-urlencode pronunciation="$(curl -s localhost:8080/arpabet/kanatrans | jq -r '.pronunciation | join(" ")')"
```
