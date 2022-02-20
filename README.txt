# Base64

### Restults for pure Elixir with hardcoded test strings

Benchwarmer.benchmark(fn -> REWASM.Base64.encode() end, [], 1)
0.0 sec      1 iterations   97.0 μs/op

Benchwarmer.benchmark(fn -> REWASM.Base64.decode() end, [], 1)
0.0 sec      1 iterations   55.0 μs/op


### Results for Rust NIF with hardcoded test strings

Benchwarmer.benchmark(fn -> REWASM.NIF.Base64.encode(:standard) end, [], 1)
0.0 sec      1 iterations   104.0 μs/op

Benchwarmer.benchmark(fn -> REWASM.NIF.Base64.decode(:standard) end, [], 1)
0.0 sec      1 iterations   197.0 μs/op
