# cntr

> counts occurence of unique lines reading from stdin

## usage

```sh
cat << EOF | cargo run --quiet
foo
bar
foo
EOF

# output
2       foo
1       bar
```

## author

balazs4

## license

MIT
