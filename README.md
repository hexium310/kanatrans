# Kanatrans

This is a web application that converts a English word to Katakana.

## Dependency

- lex_lookup

You can obtain the `lex_lookup` command by building <https://github.com/festvox/flite>. See [scripts/lex_lookup.sh](scripts/lex_lookup.sh)

## Usage

```sh
# Get ARPAbet list
$ curl -s localhost:8080/arpabet/kanatrans
{"word":"kanatrans","pronunciation":["k","ae1","n","ax0","t","r","ax0","n","z"]}

# Get Katakana: as the query parameter, pass space separated ARPAbet to `pronunciation`
# The `word` parameter is optional. If it's set a `word` value will be responded to as it is
$ curl -s localhost:8080/katakana --get --data-urlencode pronunciation="k ae1 n ax0 t r ax0 n z"
{"word":null,"pronunciation":"カナトランズ"}

# Oneliner
$ curl -s localhost:8080/katakana --get --data-urlencode pronunciation="$(curl -s localhost:8080/arpabet/kanatrans | jq -r '.pronunciation | join(" ")')"
```
