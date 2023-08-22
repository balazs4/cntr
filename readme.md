# cntr

> counts unique lines reading from stdin

## usage

```
rustc --crate-type=bin --crate-name=cntr main.rs
```

```sh
cat << EOF | ./cntr
foo
bar
foo
EOF

# output
2       foo
1       bar
```
