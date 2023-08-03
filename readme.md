# cntr

> counts unique lines reading from stdin

## usage

```
rustc cntr.rs
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
